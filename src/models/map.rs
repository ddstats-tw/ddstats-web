use serde::{Deserialize, Serialize};
use sqlx::{
    self,
    postgres::{PgQueryResult, PgRow},
    types::chrono::NaiveDateTime,
    Pool, Postgres,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Map {
    pub map: String,
    pub server: String,
    pub points: i32,
    pub stars: i32,
    pub mapper: String,
    pub timestamp: Option<NaiveDateTime>,
}

impl Map {
    /// Search for a Map
    pub async fn search(db: &Pool<Postgres>, query: &str, n: i64) -> Result<Vec<Map>, sqlx::Error> {
        sqlx::query_file_as!(Map, "sql/map/search.sql", query, n)
            .fetch_all(db)
            .await
    }
}
