use ::tera::Tera;
use axum::{middleware, Router};
use sqlx::{PgPool, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

use crate::{error::error_middleware, points::Leaderboard};
pub mod filters;
mod misc;
mod templates;
pub mod tera;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub template: Tera,
    pub points: Arc<Leaderboard>,
}

pub async fn serve(db: Pool<Postgres>, template: Tera, points: Leaderboard) {
    let state = AppState {
        db,
        template,
        points: Arc::new(points),
    };
    let app = router(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "12345".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn router(state: AppState) -> Router {
    Router::new()
        .merge(misc::router())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            error_middleware,
        ))
        .with_state(state)
}
