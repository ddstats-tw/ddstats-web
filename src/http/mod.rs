use self::{error::error_middleware, routes::misc::not_found};
use crate::points::Leaderboard;
use ::tera::Tera;
use axum::{middleware, Router};
use sqlx::{PgPool, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};

mod error;
mod routes;

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
        .nest("/", routes::misc::router())
        .nest("/player", routes::player::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            error_middleware,
        ))
        .fallback(not_found)
        .with_state(state)
}
