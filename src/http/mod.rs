use axum::{Extension, Router};
use sqlx::{PgPool, Pool, Postgres};
use tower_http::trace::TraceLayer;
mod misc;
mod templates;

#[derive(Clone)]
struct WebContext {
    db: PgPool,
}

pub async fn serve(db: Pool<Postgres>) {
    let app = router(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router(db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(misc::router())
        .route_layer(Extension(WebContext { db }))
        .route_layer(TraceLayer::new_for_http())
}
