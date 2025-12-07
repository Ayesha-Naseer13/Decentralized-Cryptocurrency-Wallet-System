mod models;
mod handlers;
mod blockchain;
mod crypto;
mod db;
mod middleware;
mod error;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mongo_url = env::var("MONGODB_URL")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let client = Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to create MongoDB client");

    let db = client.database("crypto_wallet");

    log::info!("Starting CryptoWallet Backend on 0.0.0.0:3001");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Content-Type", "application/json"))
                    .add(("Access-Control-Allow-Origin", "*"))
            )
            // Auth routes
            .route("/api/auth/register", web::post().to(handlers::auth::register))
            .route("/api/auth/login", web::post().to(handlers::auth::login))
            .route("/api/auth/verify-otp", web::post().to(handlers::auth::verify_otp))
            
            // Wallet routes
            .route("/api/wallet/{wallet_id}", web::get().to(handlers::wallet::get_wallet))
            .route("/api/wallet/{wallet_id}/balance", web::get().to(handlers::wallet::get_balance))
            
            // Transaction routes
            .route("/api/transactions/send", web::post().to(handlers::transactions::send_money))
            .route("/api/transactions/history/{wallet_id}", web::get().to(handlers::transactions::get_history))
            .route("/api/transactions/{tx_hash}", web::get().to(handlers::transactions::get_transaction_details))
            .route("/api/transactions/pending/list", web::get().to(handlers::transactions::get_pending_transactions))
            
            // Blockchain routes
            .route("/api/blockchain/blocks", web::get().to(handlers::blockchain::get_blocks))
            .route("/api/blockchain/mine", web::post().to(handlers::blockchain::mine_block))
            .route("/api/blockchain/status", web::get().to(handlers::blockchain::get_status))
            .route("/api/blockchain/mine-pending", web::post().to(handlers::blockchain::mine_pending_transactions))
            .route("/api/blockchain/validate", web::get().to(handlers::blockchain::validate_blockchain))
            .route("/api/blockchain/info", web::get().to(handlers::blockchain::get_chain_info))
            
            // Zakat operations
            .route("/api/zakat/process/{wallet_id}", web::post().to(handlers::transactions::process_zakat_deduction))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
