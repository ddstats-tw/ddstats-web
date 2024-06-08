use crate::http::error::Error;
use crate::http::AppState;
use crate::models::map::*;
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
pub struct MapJson {
    pub info: Info,
    pub rankings: Vec<Rankings>,
    pub team_rankings: Vec<TeamRankings>,
    pub time_cps: Vec<TimeCps>,
    pub playtime: Vec<Playtime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonError {
    error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MapQuery {
    map: String,
}

pub async fn map_json(
    query: Query<MapQuery>,
    State(state): State<AppState>,
) -> Result<Response, Error> {
    let name = query.map.clone();

    // Overview
    let info = Map::info(&state.db, &name).await?;
    let rankings = Map::rankings(&state.db, &name, Some(100)).await?;
    let team_rankings = Map::team_rankings(&state.db, &name, Some(100)).await?;

    // TimeCPs
    let time_cps = Map::time_cps(&state.db, &name, Some(100)).await?;

    // Playtime
    let playtime = Map::playtime(&state.db, &name, Some(100)).await?;

    Ok(Json(MapJson {
        info,
        rankings,
        team_rankings,
        time_cps,
        playtime,
    })
    .into_response())
}

pub async fn map_middleware(
    query: Query<MapQuery>,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    match Map::info(&state.db, &query.map).await.is_ok() {
        true => next.run(request).await,
        false => (StatusCode::NOT_FOUND, {
            Json(JsonError {
                error: "map not found".to_string(),
            })
        })
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(map_json))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            map_middleware,
        ))
}
