use axum::extract::Path;
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

pub async fn player_overview(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let profile = Player::get_profile(&state.db, &name).await?;
    let recent_finishes = Player::recent_finishes(&state.db, &name, Some(10)).await?;
    let favourite_teammates = Player::favourite_teammates(&state.db, &name, Some(10)).await?;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("profile", &profile);
    context.insert("recent_finishes", &recent_finishes);
    context.insert("favourite_teammates", &favourite_teammates);
    context.insert("is_mapper", &false);
    context.insert("page", &"overview");

    Ok(Html(
        state
            .template
            .render("player/overview/overview.html", &context)?,
    ))
}

pub async fn player_overview_partners(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let profile = Player::get_profile(&state.db, &name).await?;
    let favourite_teammates = Player::favourite_teammates(&state.db, &name, None).await?;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("profile", &profile);
    context.insert("favourite_teammates", &favourite_teammates);
    context.insert("is_mapper", &false);
    context.insert("page", &"overview");

    Ok(Html(state.template.render(
        "player/overview/favourite_partners.html",
        &context,
    )?))
}

pub async fn player_overview_finishes(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let profile = Player::get_profile(&state.db, &name).await?;
    let recent_finishes = Player::recent_finishes(&state.db, &name, Some(100)).await?;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("profile", &profile);
    context.insert("recent_finishes", &recent_finishes);
    context.insert("is_mapper", &false);
    context.insert("page", &"overview");

    Ok(Html(state.template.render(
        "player/overview/recent_finishes.html",
        &context,
    )?))
}
