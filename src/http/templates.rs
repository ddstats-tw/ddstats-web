use askama::Template;
use axum::{extract::Query, response::Html, Extension};
use serde::Deserialize;
use sqlx::query;

use crate::models::map::Map;
use crate::models::player::{Player, Profile};

use super::WebContext;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn landing(ext: Extension<WebContext>) -> LandingTemplate {
    println!("{:?}", Player::search(&ext.db, "test", 20).await);

    LandingTemplate
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    query: String,
    maps: Vec<Map>,
    players: Vec<Profile>,
}

pub async fn search(query: Query<SearchQuery>, ext: Extension<WebContext>) -> SearchTemplate {
    let maps = Map::search(&ext.db, &query.q, 20).await.unwrap();
    let players = Player::search(&ext.db, &query.q, 30).await.unwrap();
    SearchTemplate {
        query: query.q.clone(),
        maps,
        players,
    }
}

#[derive(Template)]
#[template(path = "search-api.html")]
pub struct SearchApiTemplate {
    query: String,
    maps: Vec<Map>,
    players: Vec<Profile>,
}

pub async fn search_api(
    query: Query<SearchQuery>,
    ext: Extension<WebContext>,
) -> SearchApiTemplate {
    let maps = Map::search(&ext.db, &query.q, 5).await.unwrap();
    let players = Player::search(&ext.db, &query.q, 5).await.unwrap();
    SearchApiTemplate {
        query: query.q.clone(),
        maps,
        players,
    }
}

#[derive(Template)]
#[template(path = "faq.html")]
pub struct FaqTemplate;

pub async fn faq() -> FaqTemplate {
    FaqTemplate
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}
