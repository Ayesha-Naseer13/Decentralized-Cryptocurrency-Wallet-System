use actix_web::{web, HttpResponse};
use mongodb::Database;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{SendMoneyRequest, Transaction, UTXO, TransactionResponse};
use crate::db::Database as DbOps;
use crate::error::ApiError;
use crate::crypto::{hash_sha256, sign_transaction, verify_signature, calculate_zakat};

pub async fn send_money(
    db: web::Data<Database>,
    req: web::Json<SendMoneyRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate sender wallet exists
    let sender = DbOps::find_user_by_wallet_id(&db, &req.sender_wallet_id)
        .await?
        .ok_or_else(|| ApiError::InvalidWalletId)?;

    // Validate recipient wallet exists
    let _recipient = DbOps::find_user_by_wallet_id(&db, &req.recipient_wallet_id)
        .await?
        .ok_or_else(|| ApiError::InvalidWalletId)?;

    // Get sender's unspent UTXOs
    let mut utxos = DbOps::get_utxos_for_wallet(&db, &req.sender_wallet_id).await?;
    let total_balance: f64 = utxos.iter().map(|u| u.amount).sum();

    // Check sufficient balance
    if total_balance < req.amount {
        return Err(ApiError::InsufficientBalance);
    }

    // Select UTXOs for transaction
    let mut selected_utxos = Vec::new();
    let mut accumulated = 0.0;
    
    for utxo in utxos.iter_mut() {
        selected_utxos.push(utxo.clone());
        accumulated += utxo.amount;
        if accumulated >= req.amount {
            break;
        }
    }

    // Create transaction
    let input_utxo_ids: Vec<String> = selected_utxos.iter().map(|u| u.utxo_id.clone()).collect();
    let output_utxo_id = Uuid::new_v4().to_string();
    let change_amount = accumulated - req.amount;
    let change_utxo_id = if change_amount > 0.0 {
        Some(Uuid::new_v4().to_string())
    } else {
        None
    };

    // Create signature payload
    let sig_payload = format!(
        "{}{}{}{}",
        req.sender_wallet_id, req.recipient_wallet_id, req.amount, Utc::now().timestamp()
    );
    let signature = sign_transaction(&sig_payload, &sender.private_key_encrypted);

    let transaction = Transaction {
        id: None,
        transaction_hash: hash_sha256(sig_payload.as_bytes()),
        sender_wallet_id: req.sender_wallet_id.clone(),
        recipient_wallet_id: req.recipient_wallet_id.clone(),
        amount: req.amount,
        note: req.note.clone(),
        timestamp: Utc::now(),
        sender_public_key: sender.public_key.clone(),
        digital_signature: signature,
        input_utxos: input_utxo_ids.clone(),
        output_utxo: output_utxo_id.clone(),
        change_utxo: change_utxo_id.clone(),
        status: "pending".to_string(),
        block_hash: None,
    };

    // Save transaction
    DbOps::create_transaction(&db, &transaction).await?;

    // Create output UTXO
    let output_utxo = UTXO {
        id: None,
        utxo_id: output_utxo_id,
        wallet_id: req.recipient_wallet_id.clone(),
        amount: req.amount,
        status: "unspent".to_string(),
        block_hash: "pending".to_string(),
        transaction_hash: transaction.transaction_hash.clone(),
        created_at: Utc::now(),
    };
    DbOps::create_utxo(&db, &output_utxo).await?;

    // Create change UTXO if needed
    if let Some(change_id) = change_utxo_id {
        let change_utxo = UTXO {
            id: None,
            utxo_id: change_id,
            wallet_id: req.sender_wallet_id.clone(),
            amount: change_amount,
            status: "unspent".to_string(),
            block_hash: "pending".to_string(),
            transaction_hash: transaction.transaction_hash.clone(),
            created_at: Utc::now(),
        };
        DbOps::create_utxo(&db, &change_utxo).await?;
    }

    // Mark input UTXOs as spent
    for utxo_id in input_utxo_ids {
        DbOps::mark_utxo_spent(&db, &utxo_id).await?;
    }

    // Process zakat deduction
    process_zakat_deduction(&db, &req.sender_wallet_id).await?;

    Ok(HttpResponse::Created().json(TransactionResponse {
        transaction_hash: transaction.transaction_hash,
        status: "pending".to_string(),
    }))
}

pub async fn get_history(
    db: web::Data<Database>,
    wallet_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let wallet_id = wallet_id.into_inner();
    let transactions = DbOps::get_transactions_for_wallet(&db, &wallet_id).await?;

    let response: Vec<_> = transactions
        .iter()
        .map(|tx| {
            let tx_type = if tx.sender_wallet_id == wallet_id {
                "sent"
            } else {
                "received"
            };

            json!({
                "id": tx.id.map(|id| id.to_string()).unwrap_or_default(),
                "type": tx_type,
                "amount": tx.amount,
                "counterparty": if tx.sender_wallet_id == wallet_id {
                    tx.recipient_wallet_id.clone()
                } else {
                    tx.sender_wallet_id.clone()
                },
                "timestamp": tx.timestamp,
                "status": tx.status,
                "blockHash": tx.block_hash.clone().unwrap_or_default(),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

pub async fn process_zakat_deduction(
    db: web::Data<Database>,
    wallet_id: &str,
) -> Result<(), ApiError> {
    let user = DbOps::find_user_by_wallet_id(&db, wallet_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let utxos = DbOps::get_utxos_for_wallet(&db, wallet_id).await?;
    let balance: f64 = utxos.iter().map(|u| u.amount).sum();

    let zakat_amount = calculate_zakat(balance);

    if zakat_amount > 0.0 {
        // Create zakat transaction from user to Zakat Pool
        let output_utxo_id = Uuid::new_v4().to_string();
        
        let zakat_tx = Transaction {
            id: None,
            transaction_hash: hash_sha256(
                format!("zakat_{}{}", wallet_id, Utc::now().timestamp()).as_bytes()
            ),
            sender_wallet_id: wallet_id.to_string(),
            recipient_wallet_id: "ZAKAT_POOL".to_string(),
            amount: zakat_amount,
            note: Some("Monthly Zakat Deduction (2.5%)".to_string()),
            timestamp: Utc::now(),
            sender_public_key: user.public_key.clone(),
            digital_signature: sign_transaction(
                &format!("zakat_{}", wallet_id),
                &user.private_key_encrypted,
            ),
            input_utxos: Vec::new(),
            output_utxo: output_utxo_id.clone(),
            change_utxo: None,
            status: "confirmed".to_string(),
            block_hash: Some("zakat_block".to_string()),
        };

        DbOps::create_transaction(&db, &zakat_tx).await?;

        // Create UTXO for zakat pool
        let zakat_utxo = UTXO {
            id: None,
            utxo_id: output_utxo_id,
            wallet_id: "ZAKAT_POOL".to_string(),
            amount: zakat_amount,
            status: "unspent".to_string(),
            block_hash: "zakat_block".to_string(),
            transaction_hash: zakat_tx.transaction_hash,
            created_at: Utc::now(),
        };

        DbOps::create_utxo(&db, &zakat_utxo).await?;
    }

    Ok(())
}

pub async fn get_transaction_details(
    db: web::Data<Database>,
    tx_hash: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let collection = db.collection::<Transaction>("transactions");
    let tx = collection
        .find_one(mongodb::bson::doc! { "transaction_hash": tx_hash.into_inner() }, None)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    Ok(HttpResponse::Ok().json(tx))
}

pub async fn get_pending_transactions(
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let collection = db.collection::<Transaction>("transactions");
    let mut cursor = collection
        .find(mongodb::bson::doc! { "status": "pending" }, None)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let mut transactions = Vec::new();
    while cursor.advance().await.map_err(|e| ApiError::DatabaseError(e.to_string()))? {
        transactions.push(cursor.deserialize_current().map_err(|e| ApiError::DatabaseError(e.to_string()))?);
    }

    Ok(HttpResponse::Ok().json(transactions))
}
