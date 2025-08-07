use sqlx::PgPool;
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use crate::models::users::{RegisterUser, LoginUser, User}; // fixed "user" → "users"
use crate::auth::jwt;

pub async fn register_user(pool: &PgPool, payload: RegisterUser) -> Result<String, String> {
    // Generate salt and hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| "Failed to hash password".to_string())?
        .to_string();

    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        user_id,
        payload.username,
        payload.email,
        password_hash
    )
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("DB insert error: {:?}", e);
        "Failed to register user".to_string()
    })?;

    Ok(jwt::create_jwt(&user_id.to_string()))
}

pub async fn login_user(pool: &PgPool, payload: LoginUser) -> Result<String, String> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(pool)
    .await
    .map_err(|_| "User not found".to_string())?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| "Invalid stored hash format".to_string())?;

    let argon2 = Argon2::default();
    let is_valid = argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if is_valid {
        Ok(jwt::create_jwt(&user.id.to_string()))
    } else {
        Err("Invalid credentials".to_string())
    }
}
