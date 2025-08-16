pub mod api_simplified;
pub mod wasm;
pub mod blockchain;
pub mod block;
pub mod transaction;
pub mod wallet;

// Re-export public interfaces
pub use transaction::Transaction;
pub use blockchain::{Blockchain, create_shared_blockchain};
pub use block::Block;