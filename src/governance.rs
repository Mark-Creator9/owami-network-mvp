use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct SlashingParams {
    pub slash_percent_first_offense: u8, // 0-100
    pub slash_percent_repeat_offense: u8,
    pub jail_epochs_first_offense: u64,
    pub jail_epochs_repeat_offense: u64,
    pub missed_block_threshold: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GovernanceState {
    pub current_params: SlashingParams,
    pub pending_proposals: HashMap<u64, GovernanceProposal>,
    pub next_proposal_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: u64,
    pub proposer: [u8; 32],
    pub new_params: SlashingParams,
    pub stake_commitment: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub voting_end_epoch: u64,
}

impl GovernanceState {
    pub fn new(default_params: SlashingParams) -> Self {
        GovernanceState {
            current_params: default_params,
            pending_proposals: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    pub fn create_proposal(
        &mut self,
        proposer: [u8; 32],
        new_params: SlashingParams,
        stake_commitment: u64,
        voting_period: u64,
        current_epoch: u64,
    ) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;

        self.pending_proposals.insert(id, GovernanceProposal {
            id,
            proposer,
            new_params,
            stake_commitment,
            votes_for: 0,
            votes_against: 0,
            voting_end_epoch: current_epoch + voting_period,
        });

        id
    }

    pub fn vote_on_proposal(
        &mut self,
        proposal_id: u64,
        voter: [u8; 32],
        stake_amount: u64,
        support: bool,
    ) -> Result<(), String> {
        if let Some(proposal) = self.pending_proposals.get_mut(&proposal_id) {
            if support {
                proposal.votes_for += stake_amount;
            } else {
                proposal.votes_against += stake_amount;
            }
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    pub fn finalize_proposals(&mut self, current_epoch: u64) -> Vec<u64> {
        let mut finalized = Vec::new();
        let mut to_remove = Vec::new();

        for (id, proposal) in &self.pending_proposals {
            if current_epoch >= proposal.voting_end_epoch {
                let total_votes = proposal.votes_for + proposal.votes_against;
                if proposal.votes_for * 2 > total_votes { // Simple majority
                    self.current_params = proposal.new_params.clone();
                }
                finalized.push(*id);
                to_remove.push(*id);
            }
        }

        for id in to_remove {
            self.pending_proposals.remove(&id);
        }

        finalized
    }
}