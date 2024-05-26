use serde::{Deserialize, Serialize};
use sqlx::{self, Pool, Postgres};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MostRank1s {
    pub rank: i64,
    pub name: String,
    pub rank1s: i64,
    pub rank2s: i64,
    pub rank3s: i64,
    pub rank4s: i64,
    pub rank5s: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorstTimes {
    pub rank: i64,
    pub map: String,
    pub name: String,
    pub worst: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MostPlayed {
    pub rank: i64,
    pub server: String,
    pub map: String,
    pub seconds: i64,
    pub mostaddicted: String,
    pub mostaddicted_seconds: i64,
}

pub struct Leaderboard;

impl Leaderboard {
    /// Get leaderboard with most rank placements for the given `sorting` within `category`
    pub async fn most_rank1s(
        db: &Pool<Postgres>,
        sorting: i32,
        category: &str,
        page: i16,
    ) -> Result<Vec<MostRank1s>, sqlx::Error> {
        sqlx::query_file_as!(
            MostRank1s,
            "sql/leaderboard/most_rank1s.sql",
            sorting,
            category,
            page - 1
        )
        .fetch_all(db)
        .await
    }

    /// Get leaderboard with most teamrank placements for the given `sorting` within `category`
    pub async fn most_team_rank1s(
        db: &Pool<Postgres>,
        sorting: i32,
        category: &str,
        page: i16,
    ) -> Result<Vec<MostRank1s>, sqlx::Error> {
        sqlx::query_file_as!(
            MostRank1s,
            "sql/leaderboard/most_team_rank1s.sql",
            sorting,
            category,
            page - 1
        )
        .fetch_all(db)
        .await
    }

    /// Get leaderboard with worst times within `category`
    pub async fn worst_times(
        db: &Pool<Postgres>,
        category: &str,
        page: i16,
    ) -> Result<Vec<WorstTimes>, sqlx::Error> {
        sqlx::query_file_as!(
            WorstTimes,
            "sql/leaderboard/worst_times.sql",
            category,
            page - 1
        )
        .fetch_all(db)
        .await
    }

    /// Get leaderboard for most played maps within `category`
    pub async fn most_played(
        db: &Pool<Postgres>,
        category: &str,
        page: i16,
    ) -> Result<Vec<MostPlayed>, sqlx::Error> {
        sqlx::query_file_as!(
            MostPlayed,
            "sql/leaderboard/most_played.sql",
            category,
            page - 1
        )
        .fetch_all(db)
        .await
    }
}
