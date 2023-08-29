use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env::var;
use uuid::Uuid;

use crate::models::user_info::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub id: Uuid,
    pub user_name: String,
}

pub fn generate_token(user_info: &UserInfo) -> String {
    let jwt_secret =
        var("JWT_SECRET").expect("Couldn't find JWT SECRET from environment variable.");

    let current_time = Utc::now();

    let claims = Claims {
        iat: current_time.timestamp() as u64,
        exp: (current_time + Duration::minutes(20)).timestamp() as u64,
        id: user_info.user_id,
        user_name: user_info.user_name.to_owned(),
    };

    let token_str = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    );

    token_str.unwrap()
}
