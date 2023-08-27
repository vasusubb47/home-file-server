use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    app_data::AppData,
    models::user_info::{
        get_all_user_info, insert_user, login_user_by_email, NewUser, NewUserError, UserError,
        UserLogin,
    },
};

pub fn user_info_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/user")
        .service(get_all_users)
        .service(register_user)
        .service(user_login);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_users(data: web::Data<AppData>) -> impl Responder {
    let users = get_all_user_info(&data.pg_conn).await;

    match users {
        Some(users) => HttpResponse::Ok().json(json!(users)),
        None => HttpResponse::NoContent().into(),
    }
}

#[post("/register")]
pub async fn register_user(
    data: web::Data<AppData>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let user = insert_user(&data.pg_conn, &new_user).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(json!(user)),
        Err(error) => {
            println!("{:#?}", error);
            match error {
                NewUserError::InvalidEmail => HttpResponse::BadRequest().body(stringify!(error)),
            }
        }
    }
}

#[post("/login")]
pub async fn user_login(
    data: web::Data<AppData>,
    login_user: web::Json<UserLogin>,
) -> impl Responder {
    let user = login_user_by_email(&data.pg_conn, &login_user).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(json!(user)),
        Err(error) => {
            println!("{:#?}", error);
            match error {
                UserError::InvalidEmail | UserError::WrongPasscode => {
                    HttpResponse::BadRequest().body(stringify!(e))
                }
            }
        }
    }
}
