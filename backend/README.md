# CryptoWallet Backend

Rust-based backend for the decentralized cryptocurrency wallet system, implementing a complete blockchain with UTXO model, Proof-of-Work mining, and transaction management.

## Building

\`\`\`bash
cargo build --release
\`\`\`

## Running

\`\`\`bash
cargo run
\`\`\`

## Environment Variables

- `MONGODB_URL`: MongoDB connection string (default: mongodb://localhost:27017)
- `RUST_LOG`: Logging level (default: info)
- `PORT`: Server port (default: 3001)

## Project Structure

- `src/main.rs` - Application entry point and route definitions
- `src/models.rs` - Data models and DTOs
- `src/handlers/` - API request handlers
- `src/blockchain.rs` - Blockchain implementation
- `src/crypto.rs` - Cryptographic functions
- `src/db.rs` - Database operations
- `src/error.rs` - Error handling

## Key Modules

### Blockchain Module
- `calculate_merkle_root()` - Calculate Merkle tree root
- `calculate_block_hash()` - Hash block data
- `mine_block()` - Perform PoW mining
- `validate_block()` - Validate block structure
- `create_genesis_block()` - Initialize blockchain

### Crypto Module
- `hash_sha256()` - SHA-256 hashing
- `generate_keypair()` - Generate public/private keys
- `generate_wallet_id()` - Create wallet from public key
- `sign_transaction()` - Sign transaction data
- `verify_signature()` - Verify transaction signature
- `calculate_zakat()` - Calculate 2.5% Zakat

### Database Operations
- User management
- UTXO tracking
- Transaction storage
- Block persistence
- System logging
