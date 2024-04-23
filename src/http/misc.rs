use axum::{routing::get, Router};

use tower_http::services::ServeDir;

use super::templates::{faq, landing, not_found, search, search_api};

pub fn router() -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .route("/", get(landing))
        .route("/faq", get(faq))
        .route("/search", get(search))
        .route("/search/api", get(search_api))
}
