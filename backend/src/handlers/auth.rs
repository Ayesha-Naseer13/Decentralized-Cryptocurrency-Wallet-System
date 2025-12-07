use actix_web::{web, HttpResponse, HttpRequest};
use mongodb::Database;
use serde_json::json;
use crate::models::{RegisterRequest, LoginRequest, VerifyOtpRequest, User, UserResponse, AuthResponse};
use crate::crypto::{generate_keypair, generate_wallet_id, hash_password, generate_otp};
use crate::db::Database as DbOps;
use crate::error::ApiError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;

// In-memory OTP storage (use Redis in production)
lazy_static::lazy_static! {
    static ref OTP_STORAGE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub async fn register(
    db: web::Data<Database>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    // Check if user already exists
    if DbOps::find_user_by_email(&db, &req.email).await?.is_some() {
        return Err(ApiError::BadRequest("User already exists".to_string()));
    }

    let (private_key, public_key) = generate_keypair();
    let wallet_id = generate_wallet_id(&public_key);

    let user = User {
        id: None,
        email: req.email.clone(),
        full_name: req.full_name.clone(),
        cnic: req.cnic.clone(),
        password_hash: hash_password(&req.password),
        wallet_id: wallet_id.clone(),
        public_key: public_key.clone(),
        private_key_encrypted: private_key,
        created_at: Utc::now(),
        beneficiaries: Vec::new(),
        zakat_deduction: 0.0,
    };

    DbOps::create_user(&db, &user).await?;

    let token = format!("token_{}", wallet_id);
    
    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: UserResponse {
            email: user.email,
            full_name: user.full_name,
            wallet_id: user.wallet_id,
            public_key: user.public_key,
        },
    }))
}

pub async fn login(
    db: web::Data<Database>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = DbOps::find_user_by_email(&db, &req.email)
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    // Send OTP (simplified - in production use email service)
    let otp = generate_otp();
    OTP_STORAGE.lock().unwrap().insert(req.email.clone(), otp.clone());
    
    log::info!("OTP for {}: {} (check logs for demo)", req.email, otp);

    Ok(HttpResponse::Ok().json(json!({
        "message": "OTP sent to email",
        "email": req.email
    })))
}

pub async fn verify_otp(
    db: web::Data<Database>,
    req: web::Json<VerifyOtpRequest>,
) -> Result<HttpResponse, ApiError> {
    let stored_otp = OTP_STORAGE
        .lock()
        .unwrap()
        .get(&req.email)
        .cloned()
        .ok_or_else(|| ApiError::Unauthorized("OTP not found".to_string()))?;

    if stored_otp != req.otp {
        return Err(ApiError::Unauthorized("Invalid OTP".to_string()));
    }

    let user = DbOps::find_user_by_email(&db, &req.email)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let token = format!("token_{}", user.wallet_id);

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            email: user.email,
            full_name: user.full_name,
            wallet_id: user.wallet_id,
            public_key: user.public_key,
        },
    }))
}
