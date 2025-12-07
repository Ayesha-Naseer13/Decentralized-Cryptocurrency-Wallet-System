use mongodb::Database;
use crate::models::{User, Block, Transaction, UTXO, BlockchainState, SystemLog};
use crate::error::ApiError;
use mongodb::bson::{doc, Document};
use chrono::Utc;

pub struct Database;

impl Database {
    pub async fn create_user(db: &mongodb::Database, user: &User) -> Result<(), ApiError> {
        db.collection("users")
            .insert_one(user, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn find_user_by_email(db: &mongodb::Database, email: &str) -> Result<Option<User>, ApiError> {
        let collection = db.collection::<User>("users");
        collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub async fn find_user_by_wallet_id(db: &mongodb::Database, wallet_id: &str) -> Result<Option<User>, ApiError> {
        let collection = db.collection::<User>("users");
        collection
            .find_one(doc! { "wallet_id": wallet_id }, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub async fn update_user_zakat(db: &mongodb::Database, wallet_id: &str, zakat_amount: f64) -> Result<(), ApiError> {
        db.collection("users")
            .update_one(
                doc! { "wallet_id": wallet_id },
                doc! { "$set": { "zakat_deduction": zakat_amount } },
                None,
            )
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn get_utxos_for_wallet(db: &mongodb::Database, wallet_id: &str) -> Result<Vec<UTXO>, ApiError> {
        let collection = db.collection::<UTXO>("utxos");
        let mut cursor = collection
            .find(doc! { "wallet_id": wallet_id, "status": "unspent" }, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let mut utxos = Vec::new();
        while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
            utxos.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
        }
        Ok(utxos)
    }

    pub async fn create_utxo(db: &mongodb::Database, utxo: &UTXO) -> Result<(), ApiError> {
        db.collection("utxos")
            .insert_one(utxo, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn mark_utxo_spent(db: &mongodb::Database, utxo_id: &str) -> Result<(), ApiError> {
        db.collection("utxos")
            .update_one(
                doc! { "utxo_id": utxo_id },
                doc! { "$set": { "status": "spent" } },
                None,
            )
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn create_transaction(db: &mongodb::Database, tx: &Transaction) -> Result<(), ApiError> {
        db.collection("transactions")
            .insert_one(tx, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn get_transactions_for_wallet(db: &mongodb::Database, wallet_id: &str) -> Result<Vec<Transaction>, ApiError> {
        let collection = db.collection::<Transaction>("transactions");
        let filter = doc! {
            "$or": [
                { "sender_wallet_id": wallet_id },
                { "recipient_wallet_id": wallet_id }
            ]
        };

        let mut cursor = collection
            .find(filter, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let mut transactions = Vec::new();
        while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
            transactions.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
        }
        Ok(transactions)
    }

    pub async fn get_transactions_by_block_hash(db: &mongodb::Database, block_hash: &str) -> Result<Vec<Transaction>, ApiError> {
        let collection = db.collection::<Transaction>("transactions");
        let mut cursor = collection
            .find(doc! { "block_hash": block_hash }, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let mut transactions = Vec::new();
        while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
            transactions.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
        }
        Ok(transactions)
    }

    pub async fn create_block(db: &mongodb::Database, block: &Block) -> Result<(), ApiError> {
        db.collection("blocks")
            .insert_one(block, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn get_all_blocks(db: &mongodb::Database) -> Result<Vec<Block>, ApiError> {
        let collection = db.collection::<Block>("blocks");
        let mut cursor = collection
            .find(doc! {}, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let mut blocks = Vec::new();
        while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
            blocks.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
        }
        Ok(blocks)
    }

    pub async fn get_latest_block(db: &mongodb::Database) -> Result<Option<Block>, ApiError> {
        let collection = db.collection::<Block>("blocks");
        collection
            .find_one(doc! {}, Some(mongodb::options::FindOneOptions::builder().sort(doc! { "index": -1 }).build()))
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub async fn get_block_by_hash(db: &mongodb::Database, hash: &str) -> Result<Option<Block>, ApiError> {
        let collection = db.collection::<Block>("blocks");
        collection
            .find_one(doc! { "hash": hash }, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub async fn log_event(db: &mongodb::Database, log: &SystemLog) -> Result<(), ApiError> {
        db.collection("logs")
            .insert_one(log, None)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
