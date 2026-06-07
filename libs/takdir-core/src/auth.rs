use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::UserRole;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: Uuid,
    pub username: String,
    pub role: UserRole,
    pub exp: usize,
    pub jti: String,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash_str: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash_str)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub fn generate_jwt(
    user_id: Uuid,
    username: &str,
    role: UserRole,
    secret: &[u8],
    expiry_secs: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let claims = UserClaims {
        sub: user_id,
        username: username.to_string(),
        role,
        exp: (now + expiry_secs) as usize,
        jti: Uuid::new_v4().to_string(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<UserClaims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret),
        &validation,
    )?;
    Ok(token_data.claims)
}
