use crate::http::error::Error;
use crate::http::AppState;
use crate::models::map::*;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MapsJson {
    pub maps: Vec<Map>,
}

pub async fn maps_json(State(state): State<AppState>) -> Result<Response, Error> {
    let maps = Map::get_all(&state.db).await?;

    Ok(Json(maps).into_response())
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(maps_json))
}
