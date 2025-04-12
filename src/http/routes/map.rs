use crate::http::error::Error;
use crate::http::render;
use crate::http::AppState;
use crate::models::map::Map;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use tera::Context;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeCpsQuery {
    sort_by: Option<String>,
}

pub async fn map_context(db: &Pool<Postgres>, name: &str, page: &str) -> Result<Context, Error> {
    let info = Map::info(db, name).await?;
    let mut context = Context::new();
    context.insert("info", &info);
    context.insert("page", &page);

    Ok(context)
}

pub async fn map_overview(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let rankings = Map::rankings(&state.db, &name, Some(100)).await?;
    let team_rankings = Map::team_rankings(&state.db, &name, Some(100)).await?;

    let mut context = map_context(&state.db, &name, "overview").await?;
    context.insert("rankings", &rankings);
    context.insert("team_rankings", &team_rankings);

    render(state.template, "map/overview.html", &context)
}

pub async fn map_time_cps(
    Path(name): Path<String>,
    query: Query<TimeCpsQuery>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let time_cps = Map::time_cps(&state.db, &name, query.sort_by.clone(), Some(100)).await?;

    let mut context = map_context(&state.db, &name, "timecps").await?;
    context.insert("time_cps", &time_cps);
    context.insert(
        "sorting",
        &query.sort_by.clone().unwrap_or("time".to_string()),
    );

    render(state.template, "map/time_cps.html", &context)
}

pub async fn map_playtime(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let playtime = Map::playtime(&state.db, &name, Some(100)).await?;

    let mut context = map_context(&state.db, &name, "playtime").await?;
    context.insert("playtime", &playtime);

    render(state.template, "map/playtime.html", &context)
}

pub async fn map_middleware(
    Path(name): Path<String>,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    match Map::info(&state.db, &name).await.is_ok() {
        true => next.run(request).await,
        false => (StatusCode::NOT_FOUND, {
            let mut context = Context::new();
            context.insert("hide_search", &true);
            render(state.template, "map/404.html", &context).unwrap()
        })
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(map_overview))
        .route("/overview", get(map_overview))
        .route("/timecps", get(map_time_cps))
        .route("/playtime", get(map_playtime))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            map_middleware,
        ))
}
