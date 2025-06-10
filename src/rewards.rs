use std::collections::HashMap;

pub struct RewardCalculator {
    epoch_rewards: u64,
    commission_rate: f64,
    last_distribution_epoch: u64,
}

impl RewardCalculator {
    pub fn new(base_reward: u64, commission: f64) -> Self {
        RewardCalculator {
            epoch_rewards: base_reward,
            commission_rate: commission,
            last_distribution_epoch: 0,
        }
    }

    pub fn calculate_rewards(
        &mut self,
        current_epoch: u64,
        active_validators: &HashMap<[u8; 32], u64>, // validator -> stake amount
    ) -> HashMap<[u8; 32], u64> {
        if current_epoch <= self.last_distribution_epoch {
            return HashMap::new();
        }

        let total_stake: u64 = active_validators.values().sum();
        let mut rewards = HashMap::new();

        for (validator, stake) in active_validators {
            let validator_share = *stake as f64 / total_stake as f64;
            let reward = (self.epoch_rewards as f64 * validator_share) as u64;
            rewards.insert(*validator, reward);
        }

        self.last_distribution_epoch = current_epoch;
        rewards
    }

    pub fn update_epoch_rewards(&mut self, new_reward: u64) {
        self.epoch_rewards = new_reward;
    }
}