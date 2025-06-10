use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents a vesting schedule for an investor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    pub total_amount: u64,
    pub start_time: DateTime<Utc>,
    pub cliff_duration: Duration,
    pub vesting_duration: Duration,
    pub claimed_amount: u64,
}

impl VestingSchedule {
    /// Calculate the amount currently vested and claimable
    pub fn claimable_amount(&self, now: DateTime<Utc>) -> u64 {
        if now < self.start_time + self.cliff_duration {
            return 0;
        }
        let elapsed = now - self.start_time;
        if elapsed >= self.vesting_duration {
            return self.total_amount - self.claimed_amount;
        }
        let vested = (self.total_amount as f64) * (elapsed.num_seconds() as f64) / (self.vesting_duration.num_seconds() as f64);
        let vested_u64 = vested as u64;
        if vested_u64 > self.claimed_amount {
            vested_u64 - self.claimed_amount
        } else {
            0
        }
    }

    /// Claim vested tokens
    pub fn claim(&mut self, now: DateTime<Utc>) -> u64 {
        let amount = self.claimable_amount(now);
        self.claimed_amount += amount;
        amount
    }
}

/// Manages vesting schedules for multiple investors
#[derive(Default)]
pub struct VestingManager {
    pub schedules: HashMap<String, VestingSchedule>, // key: investor address
}

impl VestingManager {
    /// Add a new vesting schedule
    pub fn add_schedule(&mut self, address: String, schedule: VestingSchedule) {
        self.schedules.insert(address, schedule);
    }

    /// Claim tokens for an investor
    pub fn claim_vested_tokens(&mut self, address: &str) -> u64 {
        if let Some(schedule) = self.schedules.get_mut(address) {
            schedule.claim(Utc::now())
        } else {
            0
        }
    }

    /// Get claimable amount for an investor
    pub fn get_claimable_amount(&self, address: &str) -> u64 {
        if let Some(schedule) = self.schedules.get(address) {
            schedule.claimable_amount(Utc::now())
        } else {
            0
        }
    }
}