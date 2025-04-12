use serde::{Deserialize, Serialize};
use sqlx::{self, types::chrono::NaiveDateTime, Pool, Postgres};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
pub struct Map {
    pub map: String,
    pub server: String,
    pub points: i32,
    pub stars: i32,
    pub mapper: String,
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub map: Map,
    pub finishes: Option<i32>,
    pub finishes_rank: Option<i32>,
    pub median_time: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rankings {
    pub rank: i32,
    pub timestamp: Option<NaiveDateTime>,
    pub name: String,
    pub time: f64,
    pub map: String,
    pub server: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamRankings {
    pub rank: i32,
    pub timestamp: Option<NaiveDateTime>,
    pub id: Vec<u8>,
    pub players: Vec<String>,
    pub time: f64,
    pub map: String,
    pub server: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeCps {
    pub name: String,
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
    pub time: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Playtime {
    pub name: String,
    pub seconds_played: i64,
}

impl Map {
    /// Search for a Map
    pub async fn search(
        db: &Pool<Postgres>,
        query: &str,
        n: Option<i64>,
    ) -> Result<Vec<Info>, sqlx::Error> {
        sqlx::query_file_as!(Info, "sql/map/search.sql", query, n)
            .fetch_all(db)
            .await
    }

    /// Get information about a map
    pub async fn info(db: &Pool<Postgres>, map: &str) -> Result<Info, sqlx::Error> {
        sqlx::query_file_as!(Info, "sql/map/info.sql", map)
            .fetch_one(db)
            .await
    }

    /// Get information about all maps
    pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<Map>, sqlx::Error> {
        sqlx::query_file_as!(Map, "sql/map/get_all.sql")
            .fetch_all(db)
            .await
    }

    /// Get rankings of a map
    pub async fn rankings(
        db: &Pool<Postgres>,
        map: &str,
        n: Option<i32>,
    ) -> Result<Vec<Rankings>, sqlx::Error> {
        sqlx::query_file_as!(Rankings, "sql/map/rankings.sql", map, n)
            .fetch_all(db)
            .await
    }

    /// Get teamrankings of a map
    pub async fn team_rankings(
        db: &Pool<Postgres>,
        map: &str,
        n: Option<i32>,
    ) -> Result<Vec<TeamRankings>, sqlx::Error> {
        sqlx::query_file_as!(TeamRankings, "sql/map/team_rankings.sql", map, n)
            .fetch_all(db)
            .await
    }

    /// Get timecps of a map
    pub async fn time_cps(
        db: &Pool<Postgres>,
        map: &str,
        sort_by: Option<String>,
        n: Option<i64>,
    ) -> Result<Vec<TimeCps>, sqlx::Error> {
        if sort_by.is_none() {
            sqlx::query_file_as!(TimeCps, "sql/map/time_cps.sql", map, n)
                .fetch_all(db)
                .await
        } else {
            sqlx::query_file_as!(TimeCps, "sql/map/time_cps_sort.sql", map, sort_by, n)
                .fetch_all(db)
                .await
        }
    }

    /// Get playtime of a map
    pub async fn playtime(
        db: &Pool<Postgres>,
        map: &str,
        n: Option<i64>,
    ) -> Result<Vec<Playtime>, sqlx::Error> {
        sqlx::query_file_as!(Playtime, "sql/map/playtime.sql", map, n)
            .fetch_all(db)
            .await
    }
}
