use std::collections::HashMap;

use crate::http::error::Error;
use crate::http::{render, AppState};
use crate::models::leaderboard::Leaderboard;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use tera::Context;

#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RankType {
    #[serde(rename = "rank1s")]
    Rank1s,
    #[serde(rename = "teamrank1s")]
    Teamrank1s,
}

pub async fn leaderboards(State(state): State<AppState>) -> Result<Html<String>, Error> {
    render(
        state.template,
        "leaderboard/leaderboards.html",
        &Context::new(),
    )
}

pub async fn rank1s(
    rank_type: RankType,
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let category = params
        .get("category")
        .unwrap_or(&"Total".to_string())
        .to_owned();

    let sorting: i32 = params
        .get("sorting")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);
    let leaderboard = match rank_type {
        RankType::Rank1s => Leaderboard::most_rank1s(&state.db, sorting, &category).await?,
        RankType::Teamrank1s => {
            Leaderboard::most_team_rank1s(&state.db, sorting, &category).await?
        }
    };

    let mut context = Context::new();
    context.insert("sorting", &sorting);
    context.insert("leaderboard", &leaderboard);
    context.insert("type", &rank_type);
    context.insert("current_category", &category);

    render(state.template, "leaderboard/rank1s.html", &context)
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Landing
        .route("/leaderboards", get(leaderboards))
        // Leaderboards
        // There is probably a better a way of doing it.
        .route(
            "/leaderboard/rank1s",
            get(move |params, state| rank1s(RankType::Rank1s, params, state)),
        )
        .route(
            "/leaderboard/rank1s/category/:category",
            get(move |params, state| rank1s(RankType::Rank1s, params, state)),
        )
        .route(
            "/leaderboard/rank1s/category/:category/sortby/:sorting",
            get(move |params, state| rank1s(RankType::Rank1s, params, state)),
        )
        .route(
            "/leaderboard/teamrank1s",
            get(move |params, state| rank1s(RankType::Teamrank1s, params, state)),
        )
        .route(
            "/leaderboard/teamrank1s/category/:category",
            get(move |params, state| rank1s(RankType::Teamrank1s, params, state)),
        )
        .route(
            "/leaderboard/teamrank1s/category/:category/sortby/:sorting",
            get(move |params, state| rank1s(RankType::Teamrank1s, params, state)),
        )
}
