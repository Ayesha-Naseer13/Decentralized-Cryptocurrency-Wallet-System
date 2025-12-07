use sha2::{Sha256, Digest};
use hex;
use rand::Rng;
use bcrypt;

pub fn hash_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn generate_keypair() -> (String, String) {
    let mut rng = rand::thread_rng();
    let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let private_key_hex = hex::encode(&private_key);
    
    // Simplified public key generation (SHA256 of private key)
    let public_key = hash_sha256(&private_key);
    
    (private_key_hex, public_key)
}

pub fn generate_wallet_id(public_key: &str) -> String {
    hash_sha256(public_key.as_bytes())
}

pub fn sign_transaction(data: &str, private_key: &str) -> String {
    // Simplified signing: hash(data || private_key)
    let combined = format!("{}{}", data, private_key);
    hash_sha256(combined.as_bytes())
}

pub fn verify_signature(data: &str, signature: &str, public_key: &str) -> bool {
    // Simplified verification: regenerate signature and compare
    // In production, use proper RSA or ECDSA
    let expected_signature = hash_sha256(format!("{}{}", data, public_key).as_bytes());
    signature == expected_signature
}

pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    otp.to_string()
}

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, 4).unwrap_or_else(|_| password.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn calculate_zakat(balance: f64) -> f64 {
    // 2.5% Zakat calculation for balances above 85 grams of gold equivalent
    if balance > 100.0 {
        balance * 0.025
    } else {
        0.0
    }
}

pub fn validate_transaction_signature(
    sender_wallet_id: &str,
    recipient_wallet_id: &str,
    amount: f64,
    timestamp: i64,
    signature: &str,
    public_key: &str,
) -> bool {
    let data = format!("{}{}{}{}", sender_wallet_id, recipient_wallet_id, amount, timestamp);
    verify_signature(&data, signature, public_key)
}
