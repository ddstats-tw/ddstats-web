use axum::{routing::get, Extension, Router};
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;

use super::{
    templates::{faq, landing, not_found},
    WebContext,
};

pub fn router() -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .route("/", get(landing))
        .route("/faq", get(faq))
}
