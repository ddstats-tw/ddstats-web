use serde::{Deserialize, Serialize};
use sqlx::{self, Pool, Postgres};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
pub struct MostRank1s {
    pub rank: i64,
    pub name: String,
    pub rank1s: i64,
    pub rank2s: i64,
    pub rank3s: i64,
    pub rank4s: i64,
    pub rank5s: i64,
}

pub struct Leaderboard;

impl Leaderboard {
    /// Get leaderboard with most rank placements for the given `sorting` within `category`
    pub async fn most_rank1s(
        db: &Pool<Postgres>,
        sorting: i32,
        category: &str,
    ) -> Result<Vec<MostRank1s>, sqlx::Error> {
        sqlx::query_file_as!(
            MostRank1s,
            "sql/leaderboard/most_rank1s.sql",
            sorting,
            category
        )
        .fetch_all(db)
        .await
    }

    /// Get leaderboard with most teamrank placements for the given `sorting` within `category`
    pub async fn most_team_rank1s(
        db: &Pool<Postgres>,
        sorting: i32,
        category: &str,
    ) -> Result<Vec<MostRank1s>, sqlx::Error> {
        sqlx::query_file_as!(
            MostRank1s,
            "sql/leaderboard/most_team_rank1s.sql",
            sorting,
            category
        )
        .fetch_all(db)
        .await
    }
}