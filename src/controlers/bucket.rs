use actix_web::{
    delete, get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    app_data::AppData,
    models::bucket::{
        create_user_bucket, delete_user_buckets, get_all_user_bucket_info, BucketDeletionError,
        NewBucket,
    },
    utility::jwt_token::Claims,
};

pub fn bucket_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/bucket")
        .service(get_all_buckets)
        .service(create_bucket)
        .service(delete_bucket);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_buckets(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let buckets = get_all_user_bucket_info(&data.pg_conn, &user_id).await;

    HttpResponse::Ok().json(json!(buckets))
}

#[post("/")]
pub async fn create_bucket(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
    bucket_name: web::Json<NewBucket>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let bucket = create_user_bucket(&data.pg_conn, &user_id, &bucket_name.bucket_name).await;

    match bucket {
        Some(bucket) => HttpResponse::Ok().json(json!(bucket)),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/")]
pub async fn delete_bucket(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let delete_user_bucket = delete_user_buckets(&data.pg_conn, &user_id).await;

    match delete_user_bucket {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            println!("error while deleting user buckets: {:?}", err);
            // HttpResponse::BadRequest().body(format!("{:?}", err))
            match err {
                BucketDeletionError::FailedToDeleteBucket => {
                    HttpResponse::InternalServerError().body(format!("{:?}", err))
                }
                _ => HttpResponse::BadRequest().body(format!("{:?}", err)),
            }
        }
    }
}
