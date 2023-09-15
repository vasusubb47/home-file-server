use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{self, postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::utility::get_vec_to_sql_str;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Bucket {
    pub bucket_id: Uuid,
    pub user_id: Uuid,
    pub bucket_name: String,
    pub bucket_size: i64,
    pub max_bucket_size: i64,
    pub created_date: NaiveDateTime,
    pub is_shared: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
pub struct BucketNames {
    pub bucket_name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewBucket {
    pub bucket_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BucketDeletionError {
    InvalidBucket,
    CanNotDeleteSharedBucket,
    CanNotDeleteBucketWithFiles,
    FailedToDeleteBucket,
}

pub async fn get_all_user_bucket_info(pool: &PgPool, user_id: &Uuid) -> Option<Vec<Bucket>> {
    let query = "SELECT * FROM bucket WHERE user_id = $1";

    let query = sqlx::query_as::<_, Bucket>(query).bind(user_id);

    let buckets = query.fetch_all(pool).await;

    match buckets {
        Ok(buckets) => Some(buckets),
        Err(error) => {
            println!(
                "Error occurred while fetching buckets for user {:?}: {}",
                user_id, error
            );
            None
        }
    }
}

pub async fn get_bucket_by_name(pool: &PgPool, bucket_name: &str) -> Option<Bucket> {
    let query = "SELECT * FROM bucket WHERE bucket_name = $1";

    let query = sqlx::query_as::<_, Bucket>(query).bind(bucket_name);

    let bucket = query.fetch_one(pool).await;

    match bucket {
        Ok(bucket) => Some(bucket),
        Err(error) => {
            println!(
                "Error occurred while fetching buckets by bucket_name {}: {}",
                bucket_name, error
            );
            None
        }
    }
}

pub async fn create_user_bucket(
    pool: &PgPool,
    user_id: &Uuid,
    bucket_name: &str,
) -> Option<Bucket> {
    let query = "INSERT INTO bucket (user_id, bucket_name) VALUES($1, $2)";

    let query = sqlx::query(query)
        .bind(user_id)
        .bind(bucket_name)
        .execute(pool)
        .await;

    match query {
        Ok(_) => {
            let buckets = get_bucket_by_name(pool, &bucket_name).await;

            match buckets {
                Some(buckets) => Some(buckets),
                None => None,
            }
        }
        Err(error) => {
            println!("error: {}", error);
            None
        }
    }
}

// async fn delete_bucket_with_out_check(pool: &PgPool, bucket_id: &Uuid) -> Option<()> {
//     let query = "delete from bucket where bucket_id = $1";

//     None
// }
//lhZzhNIOEmzMYiel
async fn delete_buckets_with_out_check(pool: &PgPool, bucket_ids: &Vec<Uuid>) -> Option<()> {
    println!("Vec of UUids: {}", get_vec_to_sql_str(&bucket_ids));

    let query = format!(
        "delete from bucket where bucket_id in {}",
        get_vec_to_sql_str(&bucket_ids)
    );

    let query = sqlx::query(&query).execute(pool).await;

    match query {
        Ok(_) => Some(()),
        Err(error) => {
            println!("error occurred while deleting buckets : {}", error);
            None
        }
    }
}

pub async fn delete_user_buckets(pool: &PgPool, user_id: &Uuid) -> Result<(), BucketDeletionError> {
    let user_buckets = get_all_user_bucket_info(pool, user_id).await;

    if user_buckets.is_none() {
        return Err(BucketDeletionError::InvalidBucket);
    }
    let user_buckets = user_buckets.unwrap();

    println!("deleting user_buckets \n{:#?}", user_buckets);

    // this line is just for error initialization
    // if after the loop error is same as below
    // which means that the buckets can be deleted
    let mut error = BucketDeletionError::InvalidBucket;

    let mut bucket_ids = Vec::<Uuid>::new();

    for bucket in &user_buckets {
        if bucket.is_shared {
            error = BucketDeletionError::CanNotDeleteSharedBucket;
        }
        if bucket.bucket_size > 0 && error != BucketDeletionError::CanNotDeleteSharedBucket {
            error = BucketDeletionError::CanNotDeleteBucketWithFiles;
        }
        bucket_ids.push(bucket.bucket_id.to_owned());
    }

    if error != BucketDeletionError::InvalidBucket {
        return Err(error);
    }

    match delete_buckets_with_out_check(pool, &bucket_ids).await {
        Some(_) => Ok(()),
        None => Err(BucketDeletionError::FailedToDeleteBucket),
    }
}

pub async fn get_all_bucket_names(pool: &PgPool) -> Option<Vec<BucketNames>> {
    let query = "SELECT bucket_name FROM bucket";

    let query = sqlx::query_as::<_, BucketNames>(query);

    let bucket_names = query.fetch_all(pool).await;

    match bucket_names {
        Ok(bucket_names) => Some(bucket_names),
        Err(error) => {
            println!("error: {}", error);
            None
        }
    }
}
