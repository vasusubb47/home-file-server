use ::serde::{Deserialize, Serialize};
use rocket::{
    serde::json::{serde_json::json, Value},
    *,
};
use sqlx::{self, postgres::PgPool, FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub passcode: String,
}

#[get("/")]
pub async fn get_all_users(pool: &rocket::State<PgPool>) -> Value {
    let q = "SELECT * FROM userinfo";

    let query = sqlx::query_as::<_, UserInfo>(q);

    let users = query
        .fetch_all(pool.inner())
        .await
        .expect("Failed To load Users.");

    json!(users)
}
