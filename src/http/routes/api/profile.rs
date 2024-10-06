use crate::http::AppState;
use crate::models::player::*;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonError {
    error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerQuery {
    player: String,
}

pub async fn player_json(query: Query<PlayerQuery>, State(state): State<AppState>) -> Response {
    let name = query.player.clone();
    // General
    let profile = Player::get_profile(&state.db, &name).await;
    match profile {
        Ok(profile) => Json(profile).into_response(),
        Err(_) => {
            let profile_old = Player::get_profile_old(&state.db, &name).await;
            match profile_old {
                Ok(profile_old) => Json(profile_old).into_response(),
                Err(_) => (StatusCode::NOT_FOUND, {
                    Json(JsonError {
                        error: "no profile found".to_string(),
                    })
                })
                    .into_response(),
            }
        }
    } // what the actual fuck
}

pub fn router(_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(player_json))
}
