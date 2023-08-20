use dotenv::dotenv;
use rocket::{
    serde::json::{serde_json::json, Value},
    *,
};
use sqlx::{self, postgres::PgPool};
use std::env::var;

use crate::controlers::user_info::*;
use crate::models::user_file::*;

mod controlers;
mod models;
mod utility;

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
        .mount(
            "/api/user/",
            routes![get_all_users, user_login, register_user],
        )
        .mount("/api/file/", routes![get_all_files])
}
