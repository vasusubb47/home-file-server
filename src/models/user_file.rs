use ::serde::{Deserialize, Serialize};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use chrono::NaiveDateTime;
use sqlx::{self, FromRow, PgPool};
use std::{fs, path::PathBuf};
use uuid::Uuid;

use crate::utility::get_file_type;

use super::user_info::get_user_info_by_user_id;

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

#[derive(MultipartForm)]
pub struct UploadFile {
    pub file: TempFile,
}

#[derive(Debug)]
pub enum UserFileErrors {
    Forbidden,
    NotFound,
    Deleted,
    FailedToDelete,
}

pub async fn get_all_user_files(pool: &PgPool, user_id: &Uuid) -> Option<Vec<UserFile>> {
    let user_info = get_user_info_by_user_id(pool, user_id).await;

    match user_info {
        Some(user_info) => {
            let query = "SELECT * FROM userfile where user_id = $1";

            let query = sqlx::query_as::<_, UserFile>(query).bind(user_info.user_id);

            let files = query.fetch_all(pool).await.expect("Failed To load Files.");

            Some(files)
        }
        None => None,
    }
}

pub async fn get_file_info_by_id(
    pool: &PgPool,
    file_id: &Uuid,
) -> Result<UserFile, UserFileErrors> {
    let query = "SELECT * FROM userfile where file_id = $1";

    let query = sqlx::query_as::<_, UserFile>(query).bind(file_id);

    let file_info = query.fetch_one(pool).await;

    match file_info {
        Ok(file_info) => Ok(file_info),
        Err(error) => {
            println!(
                "Error occurred while fetching file info for file_id {}, error: {}",
                file_id, error
            );
            Err(UserFileErrors::NotFound)
        }
    }

    // Some(files)
}

pub async fn save_user_file(
    pool: &PgPool,
    data_path: &str,
    user_id: &Uuid,
    upload_file: UploadFile,
) -> Option<UserFile> {
    let user_info = get_user_info_by_user_id(pool, user_id).await;

    match user_info {
        Some(user_info) => {
            let file = upload_file.file;

            let file_uuid = Uuid::new_v4();
            let file_name =
                file_uuid.to_string() + "." + &get_file_type(&file.file_name.unwrap().to_owned());

            let query = "INSERT INTO UserFile (file_id, user_id, file_name, file_size, file_hash) VALUES($1, $2, $3, $4, $5)";

            let query = sqlx::query(query)
                .bind(&file_uuid)
                .bind(&user_info.user_id)
                .bind(&file_name)
                .bind(file.size as i32)
                .bind("This is file hash.")
                .execute(pool)
                .await;

            match query {
                Ok(_) => {
                    let user_file = get_file_info_by_id(pool, &file_uuid).await;

                    if user_file.is_err() {}

                    match user_file {
                        Ok(user_file) => {
                            let temp_file_path = file.file.path();
                            let mut file_path = PathBuf::from(data_path);
                            file_path.push(&user_file.file_name);

                            println!("Saving the file at: {}", file_path.display());

                            match fs::copy(temp_file_path, file_path) {
                                Ok(_) => {
                                    let _ = fs::remove_file(temp_file_path);
                                    Some(user_file)
                                }
                                Err(error) => {
                                    let _ = fs::remove_file(temp_file_path);
                                    println!("file info : {:#?}", user_file);
                                    println!(
                                        "error while saving into the disk user file: {}",
                                        error
                                    );
                                    None
                                }
                            }
                        }
                        Err(error) => {
                            println!("Error while saving the file info to db, error {:?}.", error);
                            return None;
                        }
                    }
                }
                Err(error) => {
                    println!("error while saving user file: {}", error);
                    None
                }
            }
        }
        None => None,
    }
}

fn user_can_read(file_info: &UserFile, user_id: &Uuid) -> bool {
    &file_info.user_id == user_id
}

fn user_can_delete(file_info: &UserFile, user_id: &Uuid) -> bool {
    &file_info.user_id == user_id
}

async fn check_health_and_reterive_file(
    pool: &PgPool,
    data_path: &str,
    file_id: &Uuid,
) -> Result<(UserFile, PathBuf), UserFileErrors> {
    let file_info = get_file_info_by_id(pool, file_id).await;

    match file_info {
        Ok(file_info) => {
            let mut file_path = PathBuf::from(data_path);
            file_path.push(&file_info.file_name);

            if file_path.exists() {
                Ok((file_info, file_path))
            } else {
                Err(UserFileErrors::Deleted)
            }
        }
        Err(error) => Err(error),
    }
}

pub async fn get_user_file_by_file_id(
    pool: &PgPool,
    data_path: &str,
    user_id: &Uuid,
    file_id: &Uuid,
) -> Result<PathBuf, UserFileErrors> {
    let file_data = check_health_and_reterive_file(pool, data_path, file_id).await;

    match file_data {
        Ok((file_info, file_path)) => {
            if !user_can_read(&file_info, user_id) {
                Err(UserFileErrors::Forbidden)
            } else {
                Ok(file_path)
            }
        }
        Err(error) => Err(error),
    }
}

pub async fn delete_user_file_by_file_id(
    pool: &PgPool,
    data_path: &str,
    user_id: &Uuid,
    file_id: &Uuid,
) -> Result<bool, UserFileErrors> {
    let file_data = check_health_and_reterive_file(pool, data_path, file_id).await;

    match file_data {
        Ok((file_info, file_path)) => {
            let query = "delete from userfile where file_id = $1";
            let query = sqlx::query(query)
                .bind(&file_info.file_id)
                .execute(pool)
                .await;

            match query {
                Ok(_) => {
                    if !user_can_delete(&file_info, user_id) {
                        Err(UserFileErrors::Forbidden)
                    } else {
                        match fs::remove_file(file_path) {
                            Ok(_) => Ok(true),
                            Err(error) => {
                                println!(
                                    "error while deleting the file: \n{}\n, file info : {:#?}",
                                    error, file_info
                                );
                                Err(UserFileErrors::FailedToDelete)
                            }
                        }
                    }
                }
                Err(error) => {
                    println!("error occurred while deleting the file record in db, file id : {} error : {}", file_id, error);
                    Err(UserFileErrors::FailedToDelete)
                }
            }
        }
        Err(error) => Err(error),
    }
}
