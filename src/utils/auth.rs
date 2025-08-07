use actix_web::HttpRequest;
use jsonwebtoken::decode;
use crate::auth::jwt::{Claims, decode_token}; // depends on how your jwt is setup

pub fn get_user_id_from_request(req: &HttpRequest) -> Result<uuid::Uuid, ()> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(token_data) = decode_token(token) {
                    return Ok(token_data.claims.sub);
                }
            }
        }
    }
    Err(())
}
