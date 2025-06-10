# Owa TestToken Distribution Plan

## Total Supply
- 1,000,000,000 (1 billion) Owa TestTokens

## Allocation
1. **Community Faucet (60%)**:
   - 600,000,000 tokens for public distribution
   - Daily limit: 1,000 tokens per address
   - USSD/SMS allocation: 200,000 tokens reserved

2. **Developer Grants (25%)**:
   - 250,000,000 tokens for DApp developers
   - Apply via GitHub PR showing working prototype
   - Grants range from 10,000 to 1M tokens

3. **Network Incentives (10%)**:
   - 100,000,000 tokens for:
     - Node operators
     - Bug bounties
     - Testnet challenges

4. **Team Reserve (5%)**:
   - 50,000,000 tokens for internal testing
   - Used to simulate large transactions

## Distribution Mechanism
```rust
// Sample distribution function
fn distribute_tokens(address: &str, amount: u64) -> Result<(), Error> {
    // Verify address
    // Check daily limit    
    // Transfer tokens
}
```

## USSD/SMS Integration
- Dial *123*456# to:
  - Check balance
  - Request tokens
  - Send to other phone numbers

## Faucet Rules
- Max 3 requests per day
- KYC not required
- Tokens expire after 30 days
- No value - for testing only