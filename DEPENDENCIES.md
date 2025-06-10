# Project Dependencies

## Web Framework and Related
- `actix-web = "4.10.2"`: Web framework for building the API server
- `actix-rt = "2.10.0"`: Runtime for Actix-web
- `actix-cors = "0.7.1"`: CORS middleware for Actix-web
- `actix-files = "0.6.2"`: File serving for Actix-web
- `actix-web-httpauth = "0.8.0"`: HTTP authentication middleware for Actix-web

## Serialization/Deserialization
- `serde = { version = "1.0", features = ["derive"] }`: Serialization/Deserialization framework
- `serde_json = "1.0"`: JSON serialization/deserialization with Serde

## Cryptography
- `ed25519-dalek = "2.1.1"`: Ed25519 digital signatures
- `blake3 = "1.8.1"`: Cryptographic hash function
- `jsonwebtoken = "8.3"`: JSON Web Tokens implementation
- `bcrypt = "0.17.0"`: Password hashing

## Logging and Tracing
- `tracing = { version = "0.1", optional = true, features = ["log", "std"] }`: Tracing framework
- `tracing-subscriber = { version = "0.3", optional = true }`: Tracing subscriber
- `env_logger = "0.10"`: Logger configuration via environment variables
- `log = "0.4"`: Logging facade

## Database
- `sqlx = { version = "0.7.4", features = ["sqlite", "runtime-tokio-native-tls", "chrono"] }`: SQL database driver

## Utilities
- `anyhow = "1.0"`: Error handling library
- `chrono = { version = "0.4", features = ["serde"] }`: Date and time library
- `dotenv = "0.15"`: Environment variable loading from .env files
- `hex = "0.4"`: Hexadecimal encoding/decoding
- `lazy_static = "1.4"`: Static variables with lazy initialization
- `rand = "0.8"`: Random number generation
- `rand_core = { version = "0.6", features = ["std"] }`: Random number generation core traits
- `uuid = { version = "1.0", features = ["v4", "serde"] }`: UUID generation
- `parking_lot = "0.12"`: Synchronization primitives
- `futures = "0.3"`: Futures and async utilities
- `rayon = "1.7"`: Parallel computing library

## Testing
- `criterion = "0.4"`: Benchmarking framework
- `pretty_assertions = "1.0"`: Improved assertion messages
- `tokio-test = "0.4"`: Tokio testing utilities
- `rstest = "0.18"`: Test framework with fixtures