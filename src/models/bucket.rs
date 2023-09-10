use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{self, postgres::PgPool, FromRow};
use uuid::Uuid;

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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewBucket {
    pub bucket_name: String,
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
    let query = "SELECT * FROM bucket WHERE user_id = $1";

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
