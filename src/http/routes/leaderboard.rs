use crate::http::error::Error;
use crate::http::{render, AppState};
use crate::models::leaderboard::Leaderboard;
use crate::points::Category;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;
use tera::Context;

#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RankType {
    #[serde(rename = "rank1s")]
    Rank1s,
    #[serde(rename = "teamrank1s")]
    Teamrank1s,
}

#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PointsType {
    #[strum(to_string = "points")]
    #[serde(rename = "points")]
    Points,
    #[strum(to_string = "rankpoints")]
    #[serde(rename = "rankpoints")]
    RankPoints,
    #[strum(to_string = "teampoints")]
    #[serde(rename = "teampoints")]
    TeamPoints,
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

    let page: i16 = params
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let leaderboard = match rank_type {
        RankType::Rank1s => Leaderboard::most_rank1s(&state.db, sorting, &category, page).await?,
        RankType::Teamrank1s => {
            Leaderboard::most_team_rank1s(&state.db, sorting, &category, page).await?
        }
    };

    let mut context = Context::new();
    context.insert("sorting", &sorting);
    context.insert("leaderboard", &leaderboard);
    context.insert("type", &rank_type);
    context.insert("current_category", &category);
    context.insert("current_page", &page);

    render(state.template, "leaderboard/rank1s.html", &context)
}

pub async fn worst_times(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let category = params
        .get("category")
        .unwrap_or(&"Total".to_string())
        .to_owned();

    let page: i16 = params
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let leaderboard = Leaderboard::worst_times(&state.db, &category, page).await?;

    let mut context = Context::new();
    context.insert("leaderboard", &leaderboard);
    context.insert("current_category", &category);
    context.insert("current_page", &page);

    render(state.template, "leaderboard/worst_times.html", &context)
}

pub async fn most_played(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let category = params
        .get("category")
        .unwrap_or(&"Total".to_string())
        .to_owned();

    let leaderboard = Leaderboard::most_played(&state.db, &category).await?;

    let mut context = Context::new();
    context.insert("leaderboard", &leaderboard);
    context.insert("current_category", &category);

    render(state.template, "leaderboard/most_played.html", &context)
}

pub async fn most_points(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let category: Category = params
        .get("category")
        .unwrap_or(&"Total".to_string())
        .to_owned()
        .parse()
        .unwrap();

    let points_type: PointsType = params
        .get("type")
        .unwrap_or(&"points".to_string())
        .to_owned()
        .parse()
        .unwrap();

    let page: usize = params
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let leaderboard = match points_type {
        PointsType::Points => {
            Leaderboard::most_points(&state.points.points[&category], page).unwrap()
        }
        PointsType::RankPoints => {
            Leaderboard::most_points(&state.points.rank_points[&category], page).unwrap()
        }
        PointsType::TeamPoints => {
            Leaderboard::most_points(&state.points.team_points[&category], page).unwrap()
        }
    };
    let mut context = Context::new();
    context.insert("leaderboard", &leaderboard);
    context.insert("current_category", &category);
    context.insert("current_type", &points_type);
    context.insert("current_page", &page);

    render(state.template, "leaderboard/points.html", &context)
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Leaderboards
        .route("/leaderboards", get(leaderboards))
        // Rank1s
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
            "/leaderboard/rank1s/category/:category/sortby/:sorting/page/:page",
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
        .route(
            "/leaderboard/teamrank1s/category/:category/sortby/:sorting/page/:page",
            get(move |params, state| rank1s(RankType::Teamrank1s, params, state)),
        )
        // Worst times
        .route("/leaderboard/worsttimes", get(worst_times))
        .route(
            "/leaderboard/worsttimes/category/:category",
            get(worst_times),
        )
        .route(
            "/leaderboard/worsttimes/category/:category/page/:page",
            get(worst_times),
        )
        // Most played
        .route("/leaderboard/mostplayed", get(most_played))
        .route(
            "/leaderboard/mostplayed/category/:category",
            get(most_played),
        )
        // Points
        .route("/leaderboard/points", get(most_points))
        .route("/leaderboard/points/category/:category", get(most_points))
        .route(
            "/leaderboard/points/category/:category/type/:type",
            get(most_points),
        )
        .route(
            "/leaderboard/points/category/:category/type/:type/page/:page",
            get(most_points),
        )
}
