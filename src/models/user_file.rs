use ::serde::{Deserialize, Serialize};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde_json::json;
use sqlx::{self, FromRow};
use uuid::Uuid;

use crate::app_data::AppData;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserFile {
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub file_name: String,
    pub created_date: NaiveDateTime,
    pub file_size: i32,
    pub file_hash: String,
    pub is_shared: bool,
}

pub fn user_file_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/file").service(get_all_files);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_files(data: web::Data<AppData>) -> impl Responder {
    let q = "SELECT * FROM userfile";

    let query = sqlx::query_as::<_, UserFile>(q);

    let files = query
        .fetch_all(&data.pg_conn)
        .await
        .expect("Failed To load Files.");

    HttpResponse::Ok().json(json!(files))
}
