# Owami Testnet Fee Structure

## Transaction Fees
1. **Base Fees**:
   - Simple transfer: 0.1 Owa TestToken  
   - Contract deployment: 1.0 Owa
   - DApp interaction: 0.5 Owa

2. **Dynamic Fees**:
   ```rust
   // Sample fee calculation
   fn calculate_fee(tx_size: usize, urgency: u8) -> f64 {
       let base_fee = 0.1;
       let size_fee = tx_size as f64 * 0.0001;
       let urgency_fee = urgency as f64 * 0.01;
       base_fee + size_fee + urgency_fee
   }
   ```

## Fee Distribution
- 70% to block producers
- 20% to network treasury
- 10% burned

## Free Tier
- First 10 transactions/day: free
- USSD transactions: 50% discount
- Developer accounts: first 100 free

## Fee Collection
- Deducted automatically
- Minimum balance requirement: 1 Owa
- Failed transactions still pay 50% fee

## Testnet Special Rules
- Fees are simulated but not enforced
- Real fees will apply on mainnet
- Fee structure may change during testing