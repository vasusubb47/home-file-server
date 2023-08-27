use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{self, Pool, Postgres};
use std::env::var;

use crate::controlers::user_info::*;
use crate::models::user_file::*;

mod app_data;
mod controlers;
mod models;
mod utility;

#[get("/")]
async fn index() -> web::Json<String> {
    web::Json("hello world!".to_owned())
}

async fn db_connection() -> Pool<Postgres> {
    let url = var("DATABASE_URL").expect("Couldn't find database url from environment variable.");
    let pool = sqlx::postgres::PgPool::connect(&url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate");

    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting web server.");

    let app_data_var = app_data::AppData {
        pg_conn: db_connection().await,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data_var.clone()))
            .service(index)
            .configure(user_info_config)
            .configure(user_file_config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
