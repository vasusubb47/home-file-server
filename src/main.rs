use rocket::{
    serde::json::{serde_json::json, Value},
    *,
};
use sqlx::{self, postgres::PgPool};

use crate::models::file::*;
use crate::models::user_info::*;
use dotenv::dotenv;
use std::env::var;

mod models;

#[get("/")]
async fn index() -> Value {
    json!({
        "Hello": "welcome",
    })
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    // let url = "postgres://postgres:Home_File_Server@db:5432/HFS_Db";
    let url = var("DATABASE_URL").expect("Couldn't find database url from environment variable.");

    let pool = sqlx::postgres::PgPool::connect(&url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate");

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/api/", routes![index])
        .mount("/api/user/", routes![get_all_users])
        .mount("/api/file/", routes![get_all_files])
}
