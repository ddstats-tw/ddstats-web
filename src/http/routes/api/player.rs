use crate::http::error::Error;
use crate::http::AppState;
use crate::models::map::Info;
use crate::models::mapper::Mapper;
use crate::models::player::*;
use axum::extract::Query;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerJson {
    pub points_graph: Vec<PointsGraph>,
    pub recent_finishes: Vec<RaceFinish>,
    pub favourite_teammates: Vec<RanksTogether>,
    pub profile: Profile,
    pub is_mapper: bool,
    pub finishes: Vec<Finish>,
    pub unfinished_maps: Vec<Info>,
    pub points: Points,
    pub completion_progress: Vec<CompletionProgress>,
    pub recent_activity: Vec<RecentActivity>,
    pub recent_player_info: Vec<RecentPlayerInfo>,
    pub most_played_maps: Vec<MostPlayedMaps>,
    pub most_played_gametypes: Vec<MostPlayedChart>,
    pub most_played_categories: Vec<MostPlayedChart>,
    pub most_played_locations: Vec<MostPlayedChart>,
    pub playtime_per_month: Vec<PlaytimePerMonth>,
    pub favourite_rank1s_teammates: Vec<RanksTogether>,
    pub all_top_10s: Vec<TopRank>,
    pub recent_top_10s: Vec<RecentRank>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonError {
    error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerQuery {
    player: String,
}

pub async fn player_json(
    query: Query<PlayerQuery>,
    State(state): State<AppState>,
) -> Result<Response, Error> {
    let name = query.player.clone();
    // General
    let profile = Player::get_profile(&state.db, &name).await?;
    let is_mapper = !Mapper::maps(&state.db, &name, Some(1))
        .await
        .unwrap()
        .is_empty();
    let points_graph = Player::points_graph(&state.db, &name).await?;

    // Overview
    let recent_finishes = Player::recent_finishes(&state.db, &name, Some(10)).await?;
    let favourite_teammates = Player::favourite_teammates(&state.db, &name, Some(10)).await?;

    // Finishes
    let finishes = Player::finishes(&state.db, &name).await?;
    let unfinished_maps = Player::unfinished_maps(&state.db, &name).await?;
    let points = Player::points(&state.points, &name);
    let completion_progress = Player::completion_progress(&state.db, &name).await?;

    // Activity
    let recent_activity = Player::recent_activity(&state.db, &name, Some(11)).await?;
    let recent_player_info = Player::recent_player_info(&state.db, &name, Some(5)).await?;
    let most_played_maps = Player::most_played_maps(&state.db, &name, Some(11)).await?;
    let most_played_gametypes = Player::most_played_gametypes(&state.db, &name, Some(15)).await?;
    let most_played_categories = Player::most_played_categories(&state.db, &name, Some(15)).await?;
    let most_played_locations = Player::most_played_locations(&state.db, &name, Some(15)).await?;
    let playtime_per_month = Player::playtime_per_month(&state.db, &name, Some(12)).await?;

    // Rank1s
    let favourite_rank1s_teammates =
        Player::favourite_rank1s_teammates(&state.db, &name, Some(10)).await?;
    let all_top_10s = Player::all_top_10s(&state.db, &name).await?;
    let recent_top_10s = Player::recent_top_10s(&state.db, &name, Some(9)).await?;

    Ok(Json(PlayerJson {
        profile,
        is_mapper,
        points_graph,
        recent_finishes,
        favourite_teammates,
        finishes,
        unfinished_maps,
        points,
        completion_progress,
        recent_activity,
        recent_player_info,
        most_played_maps,
        most_played_gametypes,
        most_played_categories,
        most_played_locations,
        playtime_per_month,
        favourite_rank1s_teammates,
        all_top_10s,
        recent_top_10s,
    })
    .into_response())
}

pub async fn player_middleware(
    query: Query<PlayerQuery>,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    match Player::get_profile(&state.db, &query.player).await.is_ok() {
        true => next.run(request).await,
        false => (StatusCode::NOT_FOUND, {
            Json(JsonError {
                error: "player not found".to_string(),
            })
        })
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(player_json))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            player_middleware,
        ))
}
