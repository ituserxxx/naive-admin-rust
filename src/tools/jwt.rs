use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: i64,
    exp: usize,
}

const SECRET: &str = "jwt_secret";

pub async fn en_token(uid: i64) -> String {
    let my_claims = Claims {
        uid,
        exp: SystemTime::now()
            .checked_add(Duration::from_secs(3600 * 12))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap();
    return token;
}

pub async fn dn_token(token: String) -> Result<i64, String> {
    let token_without_bearer = token.trim_start_matches("Bearer ");
    match decode::<Claims>(
        &token_without_bearer,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            let uid = token_data.claims.uid;
            Ok(uid)
        }
        Err(err) => Err("Error decoding token".to_string()),
    }
}
