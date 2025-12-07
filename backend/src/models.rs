use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub full_name: String,
    pub cnic: String,
    pub password_hash: String,
    pub wallet_id: String,
    pub public_key: String,
    pub private_key_encrypted: String,
    pub created_at: DateTime<Utc>,
    pub beneficiaries: Vec<String>,
    pub zakat_deduction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub utxo_id: String,
    pub wallet_id: String,
    pub amount: f64,
    pub status: String, // "unspent" or "spent"
    pub block_hash: String,
    pub transaction_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub transaction_hash: String,
    pub sender_wallet_id: String,
    pub recipient_wallet_id: String,
    pub amount: f64,
    pub note: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub sender_public_key: String,
    pub digital_signature: String,
    pub input_utxos: Vec<String>,
    pub output_utxo: String,
    pub change_utxo: Option<String>,
    pub status: String, // "pending" or "confirmed"
    pub block_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub merkle_root: String,
    pub difficulty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainState {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub latest_block_hash: String,
    pub chain_length: u64,
    pub pending_transactions: Vec<String>,
    pub difficulty: u32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub event_type: String,
    pub user_email: Option<String>,
    pub wallet_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub details: String,
    pub ip_address: String,
    pub status: String,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub cnic: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub email: String,
    pub full_name: String,
    pub wallet_id: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct SendMoneyRequest {
    pub sender_wallet_id: String,
    pub recipient_wallet_id: String,
    pub amount: f64,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: f64,
    pub utxos: Vec<UtxoResponse>,
    pub zakat_deduction: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct UtxoResponse {
    pub id: String,
    pub amount: f64,
    pub status: String,
    pub block_hash: String,
}
