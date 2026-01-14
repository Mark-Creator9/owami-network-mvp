use crate::{block::Block, transaction::Transaction};
use chrono::Utc;
use ed25519_dalek::{SignatureError, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a simple wrapper for VerifyingKey that implements Serialize and Deserialize
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializableVerifyingKey(pub VerifyingKey);

impl Serialize for SerializableVerifyingKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_bytes())
    }
}

impl<'de> Deserialize<'de> for SerializableVerifyingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let key_bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("Invalid key length"))?;
        let verifying_key = VerifyingKey::from_bytes(&key_bytes)
            .map_err(|_| serde::de::Error::custom("Invalid verifying key"))?;
        Ok(SerializableVerifyingKey(verifying_key))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: SerializableVerifyingKey,
    pub stake: u64,
    pub uptime: f64,
    pub missed_blocks: u64,
    pub last_active: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DposConfig {
    pub block_interval: u64,
    pub stake_threshold: u64,
    pub validator_count: u64, // Changed from u32 to u64
}

#[derive(Debug, Clone)]
pub struct DposConsensus {
    pub validators: Vec<Validator>,
    pub voting_power: HashMap<SerializableVerifyingKey, u64>,
    pub current_round: u64,
    pub block_producer: Option<SerializableVerifyingKey>,
    pub last_block_time: i64,
    pub block_interval: u64,
    pub stake_threshold: u64,
    pub validator_count: u64,
}

impl DposConsensus {
    pub fn new(config: &crate::config::DposConfig, initial_validators: Vec<Validator>) -> Self {
        DposConsensus {
            validators: initial_validators,
            voting_power: HashMap::new(),
            current_round: 0,
            block_producer: None,
            last_block_time: Utc::now().timestamp(),
            block_interval: config.block_interval,
            stake_threshold: config.stake_threshold,
            validator_count: config.validator_count as u64, // Cast to u64
        }
    }

    pub fn elect_block_producer(&mut self) -> SerializableVerifyingKey {
        // Sort validators by total voting power (stake + delegated)
        self.validators.sort_by(|a, b| {
            let a_power = self
                .voting_power
                .get(&a.address)
                .cloned()
                .unwrap_or(a.stake);
            let b_power = self
                .voting_power
                .get(&b.address)
                .cloned()
                .unwrap_or(b.stake);
            b_power.cmp(&a_power)
        });

        // Keep only top N validators (configurable)
        if self.validators.len() > self.validator_count as usize {
            self.validators.truncate(self.validator_count as usize);
        }

        // Round-robin selection from top validators
        let index = self.current_round as usize % self.validators.len();
        let producer = self.validators[index].address.clone();
        self.block_producer = Some(producer.clone());
        self.current_round += 1;

        // Update last active time
        self.validators[index].last_active = Utc::now().timestamp();

        producer
    }

    pub fn validate_block(&self, _block: &Block, validator: &SerializableVerifyingKey) -> bool {
        // Check if validator is in current validator set
        let is_validator = self.validators.iter().any(|v| &v.address == validator);

        // Check if validator is the expected block producer for this round
        let is_expected_producer = self.block_producer.as_ref() == Some(validator);

        // Check block timing
        let current_time = Utc::now().timestamp();
        let time_diff = current_time - self.last_block_time;
        let is_valid_time = time_diff >= self.block_interval as i64;

        is_validator && is_expected_producer && is_valid_time
    }

    pub fn process_vote(&mut self, vote: &Transaction) -> Result<bool, String> {
        // Process voting transaction
        if vote.data.as_deref() != Some("vote") {
            return Err("Invalid vote transaction".to_string());
        }

        // Extract validator address from 'to' field
        let validator_address = match hex::decode(&vote.to) {
            Ok(bytes) if bytes.len() == 32 => {
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(&bytes);
                match VerifyingKey::from_bytes(&key_bytes) {
                    Ok(pk) => SerializableVerifyingKey(pk),
                    Err(_) => return Err("Invalid validator address".to_string()),
                }
            }
            _ => return Err("Invalid validator address format".to_string()),
        };

        // Update voting power
        *self
            .voting_power
            .entry(validator_address.clone())
            .or_insert(0) += vote.amount;

        Ok(true)
    }

    pub fn update_validator_set(&mut self) {
        // Sort validators by total voting power (stake + delegated)
        self.validators.sort_by(|a, b| {
            let a_power = self
                .voting_power
                .get(&a.address)
                .cloned()
                .unwrap_or(a.stake);
            let b_power = self
                .voting_power
                .get(&b.address)
                .cloned()
                .unwrap_or(b.stake);
            b_power.cmp(&a_power)
        });

        // Keep only top N validators (configurable)
        if self.validators.len() > self.validator_count as usize {
            self.validators.truncate(self.validator_count as usize);
        }

        // Update uptime metrics
        let current_time = Utc::now().timestamp();
        for validator in &mut self.validators {
            let time_diff = current_time - validator.last_active;
            if time_diff < 300 {
                // 5 minutes
                validator.uptime = (validator.uptime * 0.99) + 0.01;
            } else {
                validator.uptime = validator.uptime * 0.99;
            }
        }
    }

    pub fn get_active_validators(&self) -> Vec<SerializableVerifyingKey> {
        self.validators.iter().map(|v| v.address.clone()).collect()
    }

    pub fn get_validator_info(&self, address: &SerializableVerifyingKey) -> Option<&Validator> {
        self.validators.iter().find(|v| &v.address == address)
    }

    pub fn slash_validator(&mut self, address: &SerializableVerifyingKey, penalty: u64) -> bool {
        if let Some(validator) = self.validators.iter_mut().find(|v| &v.address == address) {
            if validator.stake >= penalty {
                validator.stake -= penalty;
                validator.missed_blocks += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn verify_block_signature(
        &self,
        block: &Block,
        signature: &[u8],
    ) -> Result<bool, SignatureError> {
        let public_key = match self.block_producer {
            Some(ref pk) => &pk.0,
            None => return Ok(false),
        };

        let message = block.hash_data();
        if signature.len() != 64 {
            return Ok(false);
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);
        let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        Ok(public_key.verify(&message, &signature).is_ok())
    }
}

// Network message types for P2P communication
#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
    ValidatorSetRequest,
    ValidatorSetResponse(Vec<ValidatorInfo>),
    BlockRequest(String),
    BlockResponse(Option<Block>),
    StatusUpdate {
        height: u64,
        network: String,
        version: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorInfo {
    pub address: String,
    pub stake: u64,
    pub uptime: f64,
    pub missed_blocks: u64,
}

impl From<&Validator> for ValidatorInfo {
    fn from(validator: &Validator) -> Self {
        ValidatorInfo {
            address: hex::encode(validator.address.0.to_bytes()),
            stake: validator.stake,
            uptime: validator.uptime,
            missed_blocks: validator.missed_blocks,
        }
    }
}

// Light client protocol
#[derive(Serialize, Deserialize, Debug)]
pub struct LightClientRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightClientResponse {
    pub result: serde_json::Value,
    pub error: Option<String>,
}

pub fn handle_light_request(
    request: LightClientRequest,
    consensus: &DposConsensus,
) -> LightClientResponse {
    match request.method.as_str() {
        "get_validator_set" => {
            let validators: Vec<ValidatorInfo> = consensus
                .validators
                .iter()
                .map(ValidatorInfo::from)
                .collect();

            LightClientResponse {
                result: serde_json::to_value(validators).unwrap(),
                error: None,
            }
        }
        _ => LightClientResponse {
            result: serde_json::Value::Null,
            error: Some("Unknown method".to_string()),
        },
    }
}
