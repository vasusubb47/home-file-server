use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppData {
    pub pg_conn: PgPool,
}
