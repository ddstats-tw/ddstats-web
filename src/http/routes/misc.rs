use crate::http::error::Error;
use crate::http::render;
use crate::http::AppState;
use crate::models::map::Info;
use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::player::Profile;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;
use tera::Context;

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchJson {
    pub maps: Vec<Info>,
    pub players: Vec<Profile>,
}

pub async fn landing(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let mut context = Context::new();
    context.insert("hide_search", &true);

    render(state.template, "landing.html", &context)
}

pub async fn search(
    query: Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let maps = Map::search(&state.db, &query.q, Some(20)).await?;
    let players = Player::search(&state.db, &query.q, Some(30)).await?;

    let mut context = Context::new();
    context.insert("query", &query.q.clone());
    context.insert("maps", &maps);
    context.insert("players", &players);
    context.insert("hide_search", &true);

    render(state.template, "search.html", &context)
}

pub async fn search_api(
    query: Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Response, Error> {
    let maps = Map::search(&state.db, &query.q, Some(5)).await?;
    let players = Player::search(&state.db, &query.q, Some(5)).await?;

    Ok(Json(SearchJson { maps, players }).into_response())
}

pub async fn faq(State(state): State<AppState>) -> Result<Html<String>, Error> {
    render(state.template, "faq.html", &Context::new())
}

pub async fn not_found(State(state): State<AppState>) -> Result<Html<String>, Error> {
    render(state.template, "404.html", &Context::new())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(landing))
        .route("/faq", get(faq))
        .route("/search", get(search))
        .route("/search/api", get(search_api))
}
