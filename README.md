# Decentralized Cryptocurrency Wallet System

A complete blockchain-based cryptocurrency wallet application with React frontend and Rust backend, featuring UTXO model, Proof-of-Work mining, digital signatures, and automatic Zakat (2.5%) deduction.

## Project Structure

\`\`\`
.
├── frontend/          # React (JSX) frontend application
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── pages/         # Page components
│   │   ├── styles/        # CSS stylesheets
│   │   ├── services/      # API services
│   │   └── App.jsx        # Main app component
│   ├── package.json
│   └── index.html
│
└── backend/           # Rust Actix-web backend
    ├── src/
    │   ├── main.rs        # Application entry point
    │   ├── models.rs      # Data models
    │   ├── handlers/      # API route handlers
    │   ├── blockchain.rs  # Blockchain logic
    │   ├── crypto.rs      # Cryptographic functions
    │   ├── db.rs          # Database operations
    │   ├── error.rs       # Error handling
    │   └── middleware.rs  # Middleware setup
    ├── Cargo.toml
    └── Cargo.lock
\`\`\`

## Features

### Authentication
- Email and OTP-based login/registration
- Secure password hashing with bcrypt
- Automatic wallet ID generation from public key

### Wallet Management
- UTXO (Unspent Transaction Output) model
- Real-time balance calculation
- Multi-wallet support with beneficiary management
- Wallet profile with user information

### Blockchain
- Custom blockchain implementation with SHA-256 hashing
- Proof-of-Work (PoW) mining with adjustable difficulty
- Merkle tree root calculation
- Block validation and chain integrity verification
- Genesis block creation

### Transactions
- Digital signature verification
- Transaction input/output management
- Change UTXO calculation
- Pending transaction pool
- Transaction history tracking
- Support for transaction notes/messages

### Zakat System
- Automatic 2.5% monthly Zakat deduction
- Zakat pool wallet management
- Zakat deduction logging
- System transactions for Zakat operations

### Reporting & Logging
- Transaction history per wallet
- System event logging
- Block explorer to view all blocks
- Transaction details and blockchain status
- Chain validation utilities

## Technology Stack

### Frontend
- **Framework**: React 18.2
- **Build Tool**: Vite
- **Styling**: Vanilla CSS (no frameworks)
- **HTTP Client**: Fetch API

### Backend
- **Language**: Rust 2021 Edition
- **Web Framework**: Actix-web 4.4
- **Async Runtime**: Tokio
- **Database**: MongoDB 2.6
- **Cryptography**: SHA-256, bcrypt
- **Serialization**: Serde, BSON

## Installation & Setup

### Prerequisites
- Node.js 16+
- Rust 1.70+
- MongoDB 5.0+
- Git

### Backend Setup

1. Navigate to backend directory:
\`\`\`bash
cd backend
\`\`\`

2. Create `.env` file:
\`\`\`
MONGODB_URL=mongodb://localhost:27017
RUST_LOG=info
PORT=3001
\`\`\`

3. Run MongoDB locally:
\`\`\`bash
mongod --dbpath ./data
\`\`\`

4. Build and run:
\`\`\`bash
cargo build --release
cargo run
\`\`\`

The backend will start on `http://0.0.0.0:3001`

### Frontend Setup

1. Navigate to frontend directory:
\`\`\`bash
cd frontend
\`\`\`

2. Install dependencies:
\`\`\`bash
npm install
\`\`\`

3. Start development server:
\`\`\`bash
npm run dev
\`\`\`

The frontend will start on `http://localhost:5173`

## API Documentation

### Authentication Endpoints

#### Register User
\`\`\`
POST /api/auth/register
Content-Type: application/json

{
  "full_name": "John Doe",
  "email": "john@example.com",
  "password": "secure_password",
  "cnic": "12345-6789012-3"
}

Response: 201 Created
{
  "token": "token_[wallet_id]",
  "user": {
    "email": "john@example.com",
    "full_name": "John Doe",
    "wallet_id": "[auto-generated]",
    "public_key": "[generated]"
  }
}
\`\`\`

#### Login
\`\`\`
POST /api/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "secure_password"
}

Response: 200 OK
{
  "message": "OTP sent to email",
  "email": "john@example.com"
}
\`\`\`

#### Verify OTP
\`\`\`
POST /api/auth/verify-otp
Content-Type: application/json

{
  "email": "john@example.com",
  "otp": "123456"
}

Response: 200 OK
{
  "token": "token_[wallet_id]",
  "user": { ... }
}
\`\`\`

### Wallet Endpoints

#### Get Wallet Info
\`\`\`
GET /api/wallet/{wallet_id}
Authorization: Bearer {token}

Response: 200 OK
{
  "wallet_id": "...",
  "email": "john@example.com",
  "full_name": "John Doe",
  "public_key": "...",
  "zakat_deduction": 0.0
}
\`\`\`

#### Get Wallet Balance
\`\`\`
GET /api/wallet/{wallet_id}/balance
Authorization: Bearer {token}

Response: 200 OK
{
  "balance": 1000.0,
  "utxos": [
    {
      "id": "utxo_id",
      "amount": 500.0,
      "status": "unspent",
      "block_hash": "..."
    }
  ],
  "zakat_deduction": 25.0
}
\`\`\`

### Transaction Endpoints

#### Send Money
\`\`\`
POST /api/transactions/send
Authorization: Bearer {token}
Content-Type: application/json

{
  "sender_wallet_id": "...",
  "recipient_wallet_id": "...",
  "amount": 100.0,
  "note": "Payment for services"
}

Response: 201 Created
{
  "transaction_hash": "...",
  "status": "pending"
}
\`\`\`

#### Get Transaction History
\`\`\`
GET /api/transactions/history/{wallet_id}
Authorization: Bearer {token}

Response: 200 OK
[
  {
    "id": "...",
    "type": "sent|received|zakat_deduction",
    "amount": 100.0,
    "counterparty": "...",
    "timestamp": "2024-01-01T00:00:00Z",
    "status": "pending|confirmed",
    "blockHash": "..."
  }
]
\`\`\`

### Blockchain Endpoints

#### Get All Blocks
\`\`\`
GET /api/blockchain/blocks

Response: 200 OK
[
  {
    "index": 0,
    "timestamp": "2024-01-01T00:00:00Z",
    "transactions": [...],
    "previous_hash": "0",
    "nonce": 0,
    "hash": "...",
    "merkle_root": "...",
    "difficulty": 1
  }
]
\`\`\`

#### Mine Block
\`\`\`
POST /api/blockchain/mine

Response: 201 Created
{
  "block_hash": "...",
  "index": 1,
  "nonce": 12345
}
\`\`\`

#### Mine Pending Transactions
\`\`\`
POST /api/blockchain/mine-pending

Response: 201 Created
{
  "block_hash": "...",
  "index": 1,
  "nonce": 12345,
  "transactions_mined": 5
}
\`\`\`

#### Get Blockchain Status
\`\`\`
GET /api/blockchain/status

Response: 200 OK
{
  "latest_block": {
    "hash": "...",
    "index": 0
  },
  "status": "running"
}
\`\`\`

#### Validate Blockchain
\`\`\`
GET /api/blockchain/validate

Response: 200 OK
{
  "valid": true,
  "total_blocks": 5,
  "error": ""
}
\`\`\`

## Database Schema

### Users Collection
\`\`\`javascript
{
  _id: ObjectId,
  email: String,
  full_name: String,
  cnic: String,
  password_hash: String,
  wallet_id: String (unique),
  public_key: String,
  private_key_encrypted: String,
  created_at: DateTime,
  beneficiaries: [String],
  zakat_deduction: Number
}
\`\`\`

### UTXOs Collection
\`\`\`javascript
{
  _id: ObjectId,
  utxo_id: String (unique),
  wallet_id: String,
  amount: Number,
  status: String ("unspent" | "spent"),
  block_hash: String,
  transaction_hash: String,
  created_at: DateTime
}
\`\`\`

### Transactions Collection
\`\`\`javascript
{
  _id: ObjectId,
  transaction_hash: String (unique),
  sender_wallet_id: String,
  recipient_wallet_id: String,
  amount: Number,
  note: String (optional),
  timestamp: DateTime,
  sender_public_key: String,
  digital_signature: String,
  input_utxos: [String],
  output_utxo: String,
  change_utxo: String (optional),
  status: String ("pending" | "confirmed"),
  block_hash: String (optional)
}
\`\`\`

### Blocks Collection
\`\`\`javascript
{
  _id: ObjectId,
  index: Number,
  timestamp: DateTime,
  transactions: [Transaction],
  previous_hash: String,
  nonce: Number,
  hash: String (unique),
  merkle_root: String,
  difficulty: Number
}
\`\`\`

## Security Considerations

1. **Private Key Encryption**: Private keys are encrypted before storage
2. **Digital Signatures**: All transactions require valid digital signatures
3. **Password Hashing**: Passwords are hashed using bcrypt with salt rounds of 4
4. **Double-Spend Prevention**: UTXOs are marked as spent after use
5. **Blockchain Validation**: All blocks are validated before acceptance
6. **OTP Verification**: Email-based OTP for secure authentication

## Deployment

### Frontend Deployment (Vercel/Netlify)
1. Push to GitHub repository
2. Connect repository to Vercel or Netlify
3. Set environment variables for backend URL
4. Deploy automatically on push

### Backend Deployment (Fly.io/Render/Railway)

1. Create account on your chosen platform
2. Create new application
3. Connect MongoDB Atlas cluster
4. Set environment variables:
   \`\`\`
   MONGODB_URL=mongodb+srv://user:pass@cluster.mongodb.net/crypto_wallet
   RUST_LOG=info
   PORT=3001
   \`\`\`
5. Deploy Rust application

## Testing

### Manual Testing Workflow

1. **Register User**
   - Create account with email and password
   - Wallet ID generated automatically

2. **Fund Wallet** (Development)
   - Manually add initial UTXOs to MongoDB for testing

3. **Send Transaction**
   - Send money to another user
   - Verify UTXO changes

4. **Mine Block**
   - Mine pending transactions
   - Verify transactions are confirmed

5. **View History**
   - Check transaction history
   - View block explorer

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - See LICENSE file for details

## Support

For issues and questions, please open an issue on the GitHub repository.
#
