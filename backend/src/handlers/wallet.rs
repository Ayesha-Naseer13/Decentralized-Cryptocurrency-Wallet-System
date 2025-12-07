use actix_web::{web, HttpResponse};
use mongodb::Database;
use serde_json::json;
use crate::models::BalanceResponse;
use crate::db::Database as DbOps;
use crate::error::ApiError;

pub async fn get_wallet(
    db: web::Data<Database>,
    wallet_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let wallet_id = wallet_id.into_inner();
    
    let user = DbOps::find_user_by_wallet_id(&db, &wallet_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Wallet not found".to_string()))?;

    Ok(HttpResponse::Ok().json(json!({
        "wallet_id": user.wallet_id,
        "email": user.email,
        "full_name": user.full_name,
        "public_key": user.public_key,
        "zakat_deduction": user.zakat_deduction
    })))
}

pub async fn get_balance(
    db: web::Data<Database>,
    wallet_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let wallet_id = wallet_id.into_inner();

    let utxos = DbOps::get_utxos_for_wallet(&db, &wallet_id).await?;
    let balance: f64 = utxos.iter().map(|u| u.amount).sum();

    let user = DbOps::find_user_by_wallet_id(&db, &wallet_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Wallet not found".to_string()))?;

    let utxo_responses = utxos
        .iter()
        .map(|u| crate::models::UtxoResponse {
            id: u.utxo_id.clone(),
            amount: u.amount,
            status: u.status.clone(),
            block_hash: u.block_hash.clone(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(BalanceResponse {
        balance,
        utxos: utxo_responses,
        zakat_deduction: user.zakat_deduction,
    }))
}
