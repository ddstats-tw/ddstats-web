use serde::{Deserialize, Serialize};
use sqlx::{self, types::chrono::NaiveDateTime, Pool, Postgres};
use std::fmt::Debug;

use super::map::Map;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaceFinish {
    pub map: Map,
    pub name: String,
    pub time: f64,
    pub timestamp: NaiveDateTime,
    pub server: String,
    pub cp1: f64,
    pub cp2: f64,
    pub cp3: f64,
    pub cp4: f64,
    pub cp5: f64,
    pub cp6: f64,
    pub cp7: f64,
    pub cp8: f64,
    pub cp9: f64,
    pub cp10: f64,
    pub cp11: f64,
    pub cp12: f64,
    pub cp13: f64,
    pub cp14: f64,
    pub cp15: f64,
    pub cp16: f64,
    pub cp17: f64,
    pub cp18: f64,
    pub cp19: f64,
    pub cp20: f64,
    pub cp21: f64,
    pub cp22: f64,
    pub cp23: f64,
    pub cp24: f64,
    pub cp25: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Finish {
    pub map: Map,
    pub name: String,
    pub time: f64,
    pub timestamp: NaiveDateTime,
    pub server: String,
    pub rank: i32,
    pub teamrank: Option<i32>,
    pub seconds_played: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub points: i32,
    pub clan: Option<String>,
    pub country: Option<i32>,
    pub skin_name: Option<String>,
    pub skin_color_body: Option<i32>,
    pub skin_color_feet: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RanksTogether {
    pub name: String,
    pub ranks_together: i64,
}

pub struct Player;

impl Player {
    /// Get the `n` latest finishes of a `player`.
    pub async fn recent_finishes(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RaceFinish>, sqlx::Error> {
        sqlx::query_file_as!(RaceFinish, "sql/player/recent_finishes.sql", player, n,)
            .fetch_all(db)
            .await
    }

    /// Search for players matching `query` and return `n` results
    pub async fn search(
        db: &Pool<Postgres>,
        query: &str,
        n: Option<i64>,
    ) -> Result<Vec<Profile>, sqlx::Error> {
        sqlx::query_file_as!(Profile, "sql/player/search.sql", query, n)
            .fetch_all(db)
            .await
    }

    /// Get profile of `player`
    pub async fn get_profile(db: &Pool<Postgres>, player: &str) -> Result<Profile, sqlx::Error> {
        sqlx::query_file_as!(Profile, "sql/player/profile_by_name.sql", player)
            .fetch_one(db)
            .await
    }

    /// Get the `n` favourite teammates of a `player`.
    pub async fn favourite_teammates(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RanksTogether>, sqlx::Error> {
        sqlx::query_file_as!(
            RanksTogether,
            "sql/player/favourite_teammates.sql",
            player,
            n,
        )
        .fetch_all(db)
        .await
    }

    /// Get all finishes of a `player`.
    pub async fn finishes(db: &Pool<Postgres>, player: &str) -> Result<Vec<Finish>, sqlx::Error> {
        sqlx::query_file_as!(Finish, "sql/player/finishes.sql", player,)
            .fetch_all(db)
            .await
    }
}
