use axum::{routing::get, Router};

use tower_http::services::ServeDir;

use super::{templates::*, AppState};

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
        .route("/player/:name/finishes", get(player_finishes))
        .route("/player/:name/activity", get(player_activity))
        .route(
            "/player/:name/activity/playtime",
            get(player_activity_playtime),
        )
        .route(
            "/player/:name/activity/playerinfo",
            get(player_activity_player_info),
        )
        .route(
            "/player/:name/activity/mostplayed",
            get(player_activity_most_played_maps),
        )
}
