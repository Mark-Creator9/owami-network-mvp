//! Simple Rust WASM Smart Contract Example for Owami Network
//! 
//! Demonstrates a basic contract with state management and function calls

use wasm_bindgen::prelude::*;

/// Contract state stored in memory
#[wasm_bindgen]
pub struct SimpleContract {
    counter: i32,
}

#[wasm_bindgen]
impl SimpleContract {
    /// Create a new contract instance
    #[wasm_bindgen(constructor)]
    pub fn new(initial_value: i32) -> Self {
        SimpleContract { counter: initial_value }
    }

    /// Increment the counter
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    /// Get the current counter value
    pub fn get_count(&self) -> i32 {
        self.counter
    }

    /// Add two numbers (demonstrates pure computation)
    pub fn add_numbers(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// Required by wasm-bindgen to handle memory
#[wasm_bindgen]
pub fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Required by wasm-bindgen to handle memory
#[wasm_bindgen]
pub fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}