use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use serde::Deserialize;

use super::AppState;
use crate::error::Error;
use crate::models::map::Map;
use crate::models::player::Player;
use tera::Context;

pub async fn landing(State(state): State<AppState>) -> Result<Html<String>, Error> {
    Ok(Html(
        state.template.render("landing.html", &Context::new())?,
    ))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
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

    Ok(Html(state.template.render("search.html", &context)?))
}

pub async fn search_api(
    query: Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let maps = Map::search(&state.db, &query.q, Some(5)).await?;
    let players = match !query.q.is_empty() {
        true => Player::search(&state.db, &query.q, Some(5)).await?,
        false => Vec::new(),
    };
    let mut context = Context::new();
    context.insert("query", &query.q.clone());
    context.insert("maps", &maps);
    context.insert("players", &players);

    Ok(Html(state.template.render("search-api.html", &context)?))
}

pub async fn faq(State(state): State<AppState>) -> Result<Html<String>, Error> {
    Ok(Html(state.template.render("faq.html", &Context::new())?))
}

pub async fn not_found(State(state): State<AppState>) -> Result<Html<String>, Error> {
    Ok(Html(state.template.render("404.html", &Context::new())?))
}
