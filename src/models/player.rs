use serde::{Deserialize, Serialize};
use sqlx::{self, types::chrono::NaiveDateTime, Pool, Postgres};
use std::{collections::HashMap, env, fmt::Debug};
use strum::IntoEnumIterator;

use crate::points::{Category, Leaderboard, LeaderboardRank};

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
    pub team_rank: Option<i32>,
    pub seconds_played: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopRank {
    pub map: Map,
    pub name: String,
    pub time: f64,
    pub rank: i32,
    pub team_rank: Option<i32>,
    pub team_time: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentRank {
    pub rank_type: String, // "rank" or "teamrank"
    pub map: String,
    pub time: f64,
    pub rank: i32,
    pub timestamp: NaiveDateTime,
    pub server: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentActivity {
    pub name: String,
    pub date: String, /* TODO: This shouldn't be as string */
    pub map_name: String,
    pub map: Option<Map>,
    pub seconds_played: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentPlayerInfo {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub skin_name: String,
    pub skin_color_body: Option<i32>,
    pub skin_color_feet: Option<i32>,
    pub last_seen: String, /* TODO: This shouldn't be as string */
    pub seconds_played: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MostPlayedMaps {
    pub map_name: String,
    pub seconds_played: i64,
    pub map: Option<Map>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Points {
    pub weekly_points: Option<LeaderboardRank>,
    pub monthly_points: Option<LeaderboardRank>,
    pub yearly_points: Option<LeaderboardRank>,
    pub points: HashMap<Category, Option<LeaderboardRank>>,
    pub rank_points: HashMap<Category, Option<LeaderboardRank>>,
    pub team_points: HashMap<Category, Option<LeaderboardRank>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MostPlayedChart {
    pub key: String,
    pub seconds_played: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaytimePerMonth {
    pub year_month: String,
    pub month: String,
    pub seconds_played: i64,
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

    /// Get the `n` favourite rank1s teammates of a `player`.
    pub async fn favourite_rank1s_teammates(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RanksTogether>, sqlx::Error> {
        sqlx::query_file_as!(
            RanksTogether,
            "sql/player/favourite_rank1s_teammates.sql",
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

    /// Get all top 10s of a `player`.
    pub async fn all_top_10s(
        db: &Pool<Postgres>,
        player: &str,
    ) -> Result<Vec<TopRank>, sqlx::Error> {
        sqlx::query_file_as!(TopRank, "sql/player/all_top_10s.sql", player,)
            .fetch_all(db)
            .await
    }

    /// Get `n` recent top 10s of a `player`.
    pub async fn recent_top_10s(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RecentRank>, sqlx::Error> {
        sqlx::query_file_as!(RecentRank, "sql/player/recent_top_10s.sql", player, n)
            .fetch_all(db)
            .await
    }

    /// Get the `n` recent playtime of a `player`.
    pub async fn recent_activity(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RecentActivity>, sqlx::Error> {
        sqlx::query_file_as!(RecentActivity, "sql/player/recent_activity.sql", player, n)
            .fetch_all(db)
            .await
    }

    /// Get the `n` recent playtime of a `player`.
    pub async fn recent_player_info(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<RecentPlayerInfo>, sqlx::Error> {
        sqlx::query_file_as!(
            RecentPlayerInfo,
            "sql/player/recent_player_info.sql",
            player,
            n
        )
        .fetch_all(db)
        .await
    }

    /// Get the `n` most played maps of a `player`.
    pub async fn most_played_maps(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<MostPlayedMaps>, sqlx::Error> {
        sqlx::query_file_as!(MostPlayedMaps, "sql/player/most_played_maps.sql", player, n)
            .fetch_all(db)
            .await
    }

    /// Get the `n` most played gametypes of a `player`.
    pub async fn most_played_gametypes(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<MostPlayedChart>, sqlx::Error> {
        sqlx::query_file_as!(
            MostPlayedChart,
            "sql/player/most_played_gametypes.sql",
            player,
            n
        )
        .fetch_all(db)
        .await
    }

    /// Get the `n` most played categories of a `player`.
    pub async fn most_played_categories(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<MostPlayedChart>, sqlx::Error> {
        sqlx::query_file_as!(
            MostPlayedChart,
            "sql/player/most_played_categories.sql",
            player,
            n
        )
        .fetch_all(db)
        .await
    }

    /// Get the `n` most played categories of a `player`.
    pub async fn most_played_locations(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<MostPlayedChart>, sqlx::Error> {
        sqlx::query_file_as!(
            MostPlayedChart,
            "sql/player/most_played_locations.sql",
            player,
            n
        )
        .fetch_all(db)
        .await
    }

    /// Get the `n` most played categories of a `player`.
    pub async fn playtime_per_month(
        db: &Pool<Postgres>,
        player: &str,
        n: Option<i64>,
    ) -> Result<Vec<PlaytimePerMonth>, sqlx::Error> {
        sqlx::query_file_as!(
            PlaytimePerMonth,
            "sql/player/playtime_per_month.sql",
            player,
            n
        )
        .fetch_all(db)
        .await
    }

    /// Get points of a `player`.
    pub fn points(players_msgpack: &Leaderboard, player: &str) -> Points {
        let mut points: HashMap<Category, Option<LeaderboardRank>> = HashMap::new();
        let mut rank_points: HashMap<Category, Option<LeaderboardRank>> = HashMap::new();
        let mut team_points: HashMap<Category, Option<LeaderboardRank>> = HashMap::new();

        let parse_points = env::var("PARSE_POINTS")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap();

        if parse_points {
            for category in Category::iter() {
                points.insert(
                    category,
                    players_msgpack
                        .points
                        .get(&category)
                        .unwrap()
                        .get(player)
                        .cloned(),
                );
                rank_points.insert(
                    category,
                    players_msgpack
                        .rank_points
                        .get(&category)
                        .unwrap()
                        .get(player)
                        .cloned(),
                );
                team_points.insert(
                    category,
                    players_msgpack
                        .team_points
                        .get(&category)
                        .unwrap()
                        .get(player)
                        .cloned(),
                );
            }
        }

        Points {
            weekly_points: players_msgpack.weekly_points.get(player).cloned(),
            monthly_points: players_msgpack.monthly_points.get(player).cloned(),
            yearly_points: players_msgpack.yearly_points.get(player).cloned(),
            points,
            rank_points,
            team_points,
        }
    }
}
