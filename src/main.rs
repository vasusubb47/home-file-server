use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::{self, Pool, Postgres};
use std::env::var;

use crate::controlers::bucket::bucket_config;
use crate::controlers::user_file::user_file_config;
use crate::controlers::user_info::*;
use crate::middlewares::auth::jwt_validator;

mod app_data;
mod controlers;
mod middlewares;
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

    let data_path = var("DATA_PATH").expect("Couldn't find DATA_PATH from environment variable.");

    println!("Starting web server.");

    let app_data_var = app_data::AppData {
        pg_conn: db_connection().await,
        data_path,
    };

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(jwt_validator);

        App::new()
            .app_data(web::Data::new(app_data_var.clone()))
            // .service(web::scope("/api").service(index))
            .service(
                web::scope("/api/auth")
                    .service(user_login)
                    .service(register_user),
            )
            .service(
                web::scope("/api")
                    .wrap(bearer_middleware)
                    .configure(user_info_config)
                    .configure(user_file_config)
                    .configure(bucket_config),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
