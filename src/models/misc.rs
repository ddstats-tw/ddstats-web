use serde::{Deserialize, Serialize};
use sqlx::{self, types::chrono::NaiveDateTime, Pool, Postgres};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rank1sSearchResults {
    pub rank: i32,
    pub name: String,
    pub map: String,
    pub server: String,
    pub time: f64,
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamRank1sSearchResults {
    pub rank: i32,
    pub players: Vec<String>,
    pub map: String,
    pub server: String,
    pub time: f64,
    pub timestamp: Option<NaiveDateTime>,
}

pub struct Misc;

impl Misc {
    /// Get all rank1s
    pub async fn search_rank1s(
        db: &Pool<Postgres>,
        category: &str,
    ) -> Result<Vec<Rank1sSearchResults>, sqlx::Error> {
        sqlx::query_file_as!(Rank1sSearchResults, "sql/misc/rank1s-search.sql", category)
            .fetch_all(db)
            .await
    }

    /// Get all teamrank1s
    pub async fn search_teamrank1s(
        db: &Pool<Postgres>,
        category: &str,
    ) -> Result<Vec<TeamRank1sSearchResults>, sqlx::Error> {
        sqlx::query_file_as!(
            TeamRank1sSearchResults,
            "sql/misc/teamrank1s-search.sql",
            category
        )
        .fetch_all(db)
        .await
    }
}
