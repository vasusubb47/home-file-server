use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{
    delete, get, post,
    web::{self, ReqData},
    HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_data::AppData,
    models::user_file::{
        delete_user_file_by_file_id, get_all_user_files, get_user_file_by_file_id, save_user_file,
        UploadFile, UserFileErrors,
    },
    utility::jwt_token::Claims,
};

pub fn user_file_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/file")
        .service(get_all_files)
        .service(save_file)
        .service(get_file_by_id)
        .service(delete_file_by_id);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_files(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let files = get_all_user_files(&data.pg_conn, &user_id).await;

    HttpResponse::Ok().json(json!(files))
}

#[post("/save")]
pub async fn save_file(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
    form: MultipartForm<UploadFile>,
) -> impl Responder {
    /*
      part of the solution was referenced
      from a post on stackoverflow

      post : https://stackoverflow.com/a/75849261/13026811

      refere for more information
    */

    // 10 MB
    const MAX_FILE_SIZE: usize = 1024 * 1024 * 10;
    // const MAX_FILE_COUNT: i32 = 1;

    // reject malformed requests
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest().body(format!(
                "The uploaded file is too large. Maximum size is {} bytes.",
                MAX_FILE_SIZE
            ));
        }
        _ => {}
    };

    let user_id = req_user.unwrap().id;

    let saved_file = save_user_file(&data.pg_conn, &data.data_path, &user_id, form.0).await;

    match saved_file {
        Some(saved_file) => HttpResponse::Created().json(json!(saved_file)),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{file_id}")]
pub async fn get_file_by_id(
    req: HttpRequest,
    file_id: web::Path<Uuid>,
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let file_path =
        get_user_file_by_file_id(&data.pg_conn, &data.data_path, &user_id, &file_id).await;

    match file_path {
        Ok(file_path) => NamedFile::open_async(file_path)
            .await
            .unwrap()
            .into_response(&req),
        Err(error) => match error {
            UserFileErrors::Forbidden => HttpResponse::Forbidden().finish(),
            UserFileErrors::NotFound => HttpResponse::NotFound().finish(),
            UserFileErrors::Deleted => HttpResponse::Gone().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[delete("/{file_id}")]
pub async fn delete_file_by_id(
    file_id: web::Path<Uuid>,
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let file_data =
        delete_user_file_by_file_id(&data.pg_conn, &data.data_path, &user_id, &file_id).await;

    match file_data {
        Ok(is_deleted) => {
            if is_deleted {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::Accepted().finish()
            }
        }
        Err(error) => match error {
            UserFileErrors::Forbidden => HttpResponse::Forbidden().finish(),
            UserFileErrors::NotFound => HttpResponse::NotFound().finish(),
            UserFileErrors::FailedToDelete => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
