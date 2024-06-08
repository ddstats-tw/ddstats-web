use self::{
    error::{error_middleware, Error},
    routes::misc::not_found,
};
use crate::points::Leaderboard;
use ::tera::Tera;
use axum::{extract::Request, middleware, response::Html, Router, ServiceExt};
use sqlx::{PgPool, Pool, Postgres};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tera::Context;
use tower::Layer;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir, trace::TraceLayer};

mod error;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub template: Arc<RwLock<Tera>>,
    pub points: Arc<Leaderboard>,
}

pub async fn serve(db: Pool<Postgres>, template: Arc<RwLock<Tera>>, points: Leaderboard) {
    let state = AppState {
        db,
        template,
        points: Arc::new(points),
    };

    let app = NormalizePathLayer::trim_trailing_slash().layer(router(state));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "12345".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

pub fn render(
    template: Arc<RwLock<Tera>>,
    template_name: &str,
    context: &Context,
) -> Result<Html<String>, Error> {
    Ok(Html(
        template.read().unwrap().render(template_name, context)?,
    ))
}

fn router(state: AppState) -> Router {
    Router::new()
        .nest("/", routes::misc::router())
        .nest("/", routes::leaderboard::router())
        .nest("/player/json", routes::api::player::router(state.clone()))
        .nest("/player/:name", routes::player::router(state.clone()))
        .nest("/map/json", routes::api::map::router(state.clone()))
        .nest("/map/:name", routes::map::router(state.clone()))
        .nest("/mapper/:name", routes::mapper::router(state.clone()))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            error_middleware,
        ))
        .fallback(not_found)
        .with_state(state)
}
