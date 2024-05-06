use axum::{routing::get, Router};

use tower_http::services::ServeDir;

use super::{
    templates::{
        faq, landing, not_found, player_overview, player_overview_finishes,
        player_overview_partners, search, search_api,
    },
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .route("/", get(landing))
        .route("/faq", get(faq))
        .route("/search", get(search))
        .route("/search/api", get(search_api))
        .route("/player/:name", get(player_overview))
        .route("/player/:name/overview", get(player_overview))
        .route(
            "/player/:name/overview/finishes",
            get(player_overview_finishes),
        )
        .route(
            "/player/:name/overview/partners",
            get(player_overview_partners),
        )
}
