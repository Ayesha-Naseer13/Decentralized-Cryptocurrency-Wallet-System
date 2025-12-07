use actix_web::{web, HttpResponse};
use mongodb::Database;
use serde_json::json;
use crate::db::Database as DbOps;
use crate::blockchain::Blockchain;
use crate::error::ApiError;
use crate::models::Transaction;

pub async fn get_blocks(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let blocks = DbOps::get_all_blocks(&db).await?;
    Ok(HttpResponse::Ok().json(blocks))
}

pub async fn mine_block(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let latest_block = DbOps::get_latest_block(&db)
        .await?
        .unwrap_or_else(|| Blockchain::create_genesis_block());

    let new_block = Blockchain::mine_block(
        latest_block.index + 1,
        Vec::new(),
        latest_block.hash.clone(),
        latest_block.difficulty,
    );

    DbOps::create_block(&db, &new_block).await?;

    Ok(HttpResponse::Created().json(json!({
        "block_hash": new_block.hash,
        "index": new_block.index,
        "nonce": new_block.nonce
    })))
}

pub async fn get_status(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let latest_block = DbOps::get_latest_block(&db).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "latest_block": latest_block.map(|b| json!({
            "hash": b.hash,
            "index": b.index
        })),
        "status": "running"
    })))
}

pub async fn mine_pending_transactions(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let collection = db.collection::<Transaction>("transactions");
    let mut cursor = collection
        .find(mongodb::bson::doc! { "status": "pending" }, None)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let mut pending_txs = Vec::new();
    while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
        pending_txs.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
    }

    if pending_txs.is_empty() {
        return Err(ApiError::BadRequest("No pending transactions to mine".to_string()));
    }

    let latest_block = DbOps::get_latest_block(&db)
        .await?
        .unwrap_or_else(|| Blockchain::create_genesis_block());

    let new_block = Blockchain::mine_block(
        latest_block.index + 1,
        pending_txs.clone(),
        latest_block.hash.clone(),
        latest_block.difficulty,
    );

    let tx_collection = db.collection::<Transaction>("transactions");
    for tx in &pending_txs {
        tx_collection
            .update_one(
                mongodb::bson::doc! { "transaction_hash": &tx.transaction_hash },
                mongodb::bson::doc! {
                    "$set": {
                        "status": "confirmed",
                        "block_hash": &new_block.hash
                    }
                },
                None,
            )
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    }

    DbOps::create_block(&db, &new_block).await?;

    Ok(HttpResponse::Created().json(json!({
        "block_hash": new_block.hash,
        "index": new_block.index,
        "nonce": new_block.nonce,
        "transactions_mined": pending_txs.len()
    })))
}

pub async fn validate_blockchain(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let blocks = DbOps::get_all_blocks(&db).await?;

    if blocks.is_empty() {
        return Ok(HttpResponse::Ok().json(json!({
            "valid": true,
            "message": "Empty blockchain"
        })));
    }

    let mut is_valid = true;
    let mut error_message = String::new();

    for i in 1..blocks.len() {
        let current = &blocks[i];
        let previous = &blocks[i - 1];

        if !Blockchain::validate_block(current, &previous.hash, previous.difficulty) {
            is_valid = false;
            error_message = format!("Block {} is invalid", i);
            break;
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "valid": is_valid,
        "total_blocks": blocks.len(),
        "error": error_message
    })))
}

pub async fn get_chain_info(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let blocks = DbOps::get_all_blocks(&db).await?;
    let tx_collection = db.collection::<Transaction>("transactions");
    let total_transactions = tx_collection
        .count_documents(mongodb::bson::doc! {}, None)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(json!({
        "chain_length": blocks.len(),
        "total_transactions": total_transactions,
        "latest_block_hash": blocks.last().map(|b| &b.hash),
        "difficulty": blocks.last().map(|b| b.difficulty)
    })))
}
