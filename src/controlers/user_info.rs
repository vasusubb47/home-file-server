use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json, Value},
    *,
};
use sqlx::PgPool;

use crate::{
    models::user_info::{
        get_all_user_info, insert_user, login_user_by_email, NewUser, NewUserError, UserError,
        UserInfo, UserLogin,
    },
    utility::api::ApiResponse,
};

#[get("/")]
pub async fn get_all_users(pool: &rocket::State<PgPool>) -> Result<Value, Status> {
    let users = get_all_user_info(pool).await;

    match users {
        Some(users) => Ok(json!(users)),
        None => Err(Status::InternalServerError),
    }
}

#[post("/register", data = "<new_user>")]
pub async fn register_user(
    pool: &rocket::State<PgPool>,
    new_user: Json<NewUser>,
) -> ApiResponse<UserInfo, NewUserError> {
    let user = insert_user(pool, &new_user).await;

    match user {
        Ok(user) => ApiResponse::success_data(user, Status::Ok),
        Err(e) => {
            println!("{:#?}", e);
            ApiResponse::error_data(e, Status::BadRequest)
        }
    }
}

#[post("/login", data = "<login_user>")]
pub async fn user_login(
    pool: &rocket::State<PgPool>,
    login_user: Json<UserLogin>,
) -> ApiResponse<UserInfo, UserError> {
    let user = login_user_by_email(pool, &login_user).await;

    match user {
        Ok(user) => ApiResponse::success_data(user, Status::Accepted),
        Err(e) => {
            println!("{:#?}", e);
            ApiResponse::error_data(e, Status::Unauthorized)
        }
    }
}
