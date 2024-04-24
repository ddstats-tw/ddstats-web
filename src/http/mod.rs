use axum::{Extension, Router};
use sqlx::{PgPool, Pool, Postgres};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
mod misc;
mod templates;

#[derive(Clone)]
struct WebContext {
    db: PgPool,
}

pub async fn serve(db: Pool<Postgres>) {
    let app = router(db);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "12345".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn router(db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(misc::router())
        .route_layer(Extension(WebContext { db }))
        .route_layer(TraceLayer::new_for_http())
}
