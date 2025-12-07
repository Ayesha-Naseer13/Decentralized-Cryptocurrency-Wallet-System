use crate::models::{Block, Transaction, BlockchainState};
use crate::crypto::hash_sha256;
use chrono::Utc;
use serde_json::to_string;

pub const DIFFICULTY_PREFIX: &str = "00000";

pub struct Blockchain;

impl Blockchain {
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return hash_sha256(b"");
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| hash_sha256(tx.transaction_hash.as_bytes()))
            .collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                hashes.push(hashes[hashes.len() - 1].clone());
            }

            let mut new_hashes = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let combined = format!("{}{}", hashes[i], hashes[i + 1]);
                new_hashes.push(hash_sha256(combined.as_bytes()));
            }
            hashes = new_hashes;
        }

        hashes[0].clone()
    }

    pub fn calculate_block_hash(block: &Block) -> String {
        let block_data = format!(
            "{}{}{}{}{}{}{}",
            block.index,
            block.timestamp,
            to_string(&block.transactions).unwrap_or_default(),
            block.previous_hash,
            block.nonce,
            block.merkle_root,
            block.difficulty
        );
        hash_sha256(block_data.as_bytes())
    }

    pub fn mine_block(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: u32,
    ) -> Block {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let mut nonce = 0u64;

        loop {
            let mut block = Block {
                id: None,
                index,
                timestamp: Utc::now(),
                transactions: transactions.clone(),
                previous_hash: previous_hash.clone(),
                nonce,
                hash: String::new(),
                merkle_root: merkle_root.clone(),
                difficulty,
            };

            block.hash = Self::calculate_block_hash(&block);

            if block.hash.starts_with(DIFFICULTY_PREFIX) {
                return block;
            }

            nonce += 1;
        }
    }

    pub fn validate_block(block: &Block, previous_hash: &str, difficulty: u32) -> bool {
        // Check if hash matches difficulty
        if !block.hash.starts_with(DIFFICULTY_PREFIX) {
            return false;
        }

        // Check if previous hash matches
        if block.previous_hash != previous_hash {
            return false;
        }

        // Verify merkle root
        if block.merkle_root != Self::calculate_merkle_root(&block.transactions) {
            return false;
        }

        // Verify calculated hash
        let calculated_hash = Self::calculate_block_hash(block);
        calculated_hash == block.hash
    }

    pub fn create_genesis_block() -> Block {
        Block {
            id: None,
            index: 0,
            timestamp: Utc::now(),
            transactions: Vec::new(),
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: hash_sha256(b"genesis"),
            merkle_root: hash_sha256(b""),
            difficulty: 1,
        }
    }
}
