use axum::response::Html;
use axum::{extract::Query, Extension};
use serde::Deserialize;

use super::WebContext;
use crate::models::map::Map;
use crate::models::player::Player;
use tera::Context;

pub async fn landing(ext: Extension<WebContext>) -> Html<String> {
    axum::response::Html(
        ext.template
            .render("landing.html", &Context::new())
            .unwrap(),
    )
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search(query: Query<SearchQuery>, ext: Extension<WebContext>) -> Html<String> {
    let maps = Map::search(&ext.db, &query.q, Some(20)).await.unwrap();
    let players = Player::search(&ext.db, &query.q, Some(30)).await.unwrap();

    let mut context = Context::new();
    context.insert("query", &query.q.clone());
    context.insert("maps", &maps);
    context.insert("players", &players);

    axum::response::Html(ext.template.render("search.html", &context).unwrap())
}

pub async fn search_api(query: Query<SearchQuery>, ext: Extension<WebContext>) -> Html<String> {
    let maps = Map::search(&ext.db, &query.q, Some(5)).await.unwrap();
    let players = match !query.q.is_empty() {
        true => Player::search(&ext.db, &query.q, Some(5)).await.unwrap(),
        false => Vec::new(),
    };
    let mut context = Context::new();
    context.insert("query", &query.q.clone());
    context.insert("maps", &maps);
    context.insert("players", &players);

    axum::response::Html(ext.template.render("search-api.html", &context).unwrap())
}

pub async fn faq(ext: Extension<WebContext>) -> Html<String> {
    axum::response::Html(ext.template.render("faq.html", &Context::new()).unwrap())
}

pub async fn not_found(ext: Extension<WebContext>) -> Html<String> {
    axum::response::Html(ext.template.render("404.html", &Context::new()).unwrap())
}
