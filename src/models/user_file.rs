use std::fs;

use ::serde::{Deserialize, Serialize};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use chrono::NaiveDateTime;
use sqlx::{self, FromRow, PgPool};
use uuid::Uuid;

use crate::utility::{get_current_working_dir, get_file_type};

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

pub async fn get_file_info_by_id(pool: &PgPool, file_id: &Uuid) -> Option<UserFile> {
    let query = "SELECT * FROM userfile where file_id = $1";

    let query = sqlx::query_as::<_, UserFile>(query).bind(file_id);

    let files = query
        .fetch_one(pool)
        .await
        .expect("Failed To load file by id {file_id}.");

    Some(files)
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
                    let user_file = get_file_info_by_id(pool, &file_uuid).await.unwrap();

                    let temp_file_path = file.file.path();
                    let mut file_path = get_current_working_dir().unwrap();
                    file_path.push(data_path);
                    file_path.push(&user_file.file_name);

                    println!("Savinf the file at: {}", file_path.display());

                    match fs::copy(temp_file_path, file_path) {
                        Ok(_) => {
                            let _ = fs::remove_file(temp_file_path);
                            Some(user_file)
                        }
                        Err(error) => {
                            let _ = fs::remove_file(temp_file_path);
                            println!("file info : {:#?}", user_file);
                            println!("error while saving into the disk user file: {}", error);
                            None
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
