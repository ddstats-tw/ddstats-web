use askama::Template;
use axum::{extract::Query, Extension};
use serde::Deserialize;

use crate::models::map::Map;
use crate::models::player::{Player, Profile};

use super::WebContext;

mod filters {
    use regex::Regex;

    pub fn map_thumbnail<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let mut s = s.to_string();
        let re1 = Regex::new(r"[À-ž]").unwrap();
        let re2 = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        s = re1.replace_all(&s, "__").to_string();
        s = re2.replace_all(&s, "_").to_string();
        Ok(s)
    }
}

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn landing() -> LandingTemplate {
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
    let maps = Map::search(&ext.db, &query.q, Some(20)).await.unwrap();
    let players = Player::search(&ext.db, &query.q, Some(30)).await.unwrap();
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
    let maps = Map::search(&ext.db, &query.q, Some(5)).await.unwrap();
    let players = match !query.q.is_empty() {
        true => Player::search(&ext.db, &query.q, Some(5)).await.unwrap(),
        false => Vec::new(),
    };
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
