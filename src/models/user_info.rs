use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use sqlx::{self, postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::utility::{genarate_salt, jwt_token::generate_token, u8_to_hex_str};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub created_date: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub passcode: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub passcode: String,
}

#[derive(Debug, Serialize)]
pub enum NewUserError {
    InvalidEmail,
}

#[derive(Debug, Serialize)]
pub enum UserError {
    InvalidEmail,
    WrongPasscode,
}

pub async fn get_all_user_info(pool: &PgPool) -> Option<Vec<UserInfo>> {
    let query = "SELECT user_id, user_name, email, created_date FROM userinfo";

    let query = sqlx::query_as::<_, UserInfo>(query);

    let users = query.fetch_all(pool).await;

    match users {
        Ok(users) => Some(users),
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<UserInfo, NewUserError> {
    let mut sha = Sha256::new();
    let salt = genarate_salt(64);

    sha.update(new_user.passcode.to_owned() + &salt.to_owned());
    let passcode_hash = sha.finalize();

    let passcode_hash = u8_to_hex_str(passcode_hash.as_slice()) + ":" + &salt;

    let query = "INSERT INTO userinfo (user_name, email, passcode) VALUES($1, $2, $3)";

    let query = sqlx::query(query)
        .bind(new_user.user_name.to_owned())
        .bind(new_user.email.to_owned())
        .bind(passcode_hash)
        .execute(pool)
        .await;

    match query {
        Ok(_) => {
            let user = get_user_by_email(pool, &new_user.email).await;
            Ok(user.unwrap())
        }
        Err(e) => {
            println!("{:#?}", e);
            Err(NewUserError::InvalidEmail)
        }
    }
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Option<UserInfo> {
    let query = "SELECT user_id, user_name, email, created_date FROM userinfo where email = $1";

    let query = sqlx::query_as::<_, UserInfo>(query).bind(email);

    let users = query.fetch_all(pool).await;

    match users {
        Ok(mut users) => {
            let user = users.pop().unwrap();
            Some(user)
        }
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

async fn get_user_info_by_email_passcode(
    pool: &PgPool,
    user_login: &UserLogin,
) -> Result<UserInfo, UserError> {
    let query = "SELECT passcode, email FROM userinfo where email = $1";

    let query = sqlx::query_as::<_, UserLogin>(query).bind(user_login.email.to_owned());

    let users = query.fetch_all(pool).await;

    match users {
        Ok(users) => {
            let passcode_parts = users[0].passcode.split(':').collect::<Vec<&str>>();
            let passcode_hash = passcode_parts[0];
            let passcode_salt = passcode_parts[1];

            let user_passcode = user_login.passcode.as_str();

            let mut sha = Sha256::new();
            sha.update(user_passcode.to_owned() + passcode_salt);
            let user_passcode_hash = sha.finalize();

            let user_passcode_hash = u8_to_hex_str(user_passcode_hash.as_slice());

            if passcode_hash == user_passcode_hash {
                let user = get_user_by_email(pool, &user_login.email).await.unwrap();
                Ok(user)
            } else {
                Err(UserError::WrongPasscode)
            }
        }
        Err(error) => {
            println!("{}", error);
            Err(UserError::InvalidEmail)
        }
    }
}

pub async fn get_user_info_by_user_id(pool: &PgPool, user_id: Uuid) -> Option<UserInfo> {
    let query = "SELECT user_id, user_name, email, created_date FROM userinfo where user_id = $1";

    let query = sqlx::query_as::<_, UserInfo>(query).bind(user_id);

    let users = query.fetch_all(pool).await;

    match users {
        Ok(mut users) => {
            let user = users.pop().unwrap();
            Some(user)
        }
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

pub async fn login_user_by_email(
    pool: &PgPool,
    user_login: &UserLogin,
) -> Result<String, UserError> {
    let user_info = get_user_info_by_email_passcode(pool, user_login).await;

    match user_info {
        Ok(user_info) => Ok(generate_token(&user_info)),
        Err(user_err) => Err(user_err),
    }
}
