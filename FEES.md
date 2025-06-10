# Transaction Fee Structure

Our blockchain network implements a transaction fee mechanism to:
- Prevent spam transactions
- Incentivize miners/validators
- Prioritize high-value transactions

## Fee Rules

1. Minimum Fee: 1 token
2. Recommended Fee: 10 tokens
3. Fees are deducted from sender's balance
4. Fees are added to miner/validator's balance

## Fee Calculation

```rust
// Transaction::new signature
pub fn new(
 sender_key: &SigningKey,
 receiver_pk: [u8; 32],
 amount: u64,  
 fee: u64
) -> Self {
 // implementation
}
```

## Recommendations

- Higher fees increase transaction priority
- Complex transactions may require higher fees
- Fee market will evolve with network usage