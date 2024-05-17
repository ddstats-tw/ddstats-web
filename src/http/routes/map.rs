use crate::http::error::Error;
use crate::http::AppState;
use crate::models::map::Map;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::{routing::get, Router};
use sqlx::Pool;
use sqlx::Postgres;
use tera::Context;

pub async fn map_context(db: &Pool<Postgres>, name: &str, page: &str) -> Result<Context, Error> {
    let info = Map::info(db, &name).await?;
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

    Ok(Html(state.template.render("map/overview.html", &context)?))
}

pub async fn map_time_cps(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let time_cps = Map::time_cps(&state.db, &name, Some(100)).await?;

    let mut context = map_context(&state.db, &name, "timecps").await?;
    context.insert("time_cps", &time_cps);

    Ok(Html(state.template.render("map/time_cps.html", &context)?))
}

pub async fn map_playtime(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let playtime = Map::playtime(&state.db, &name, Some(100)).await?;

    let mut context = map_context(&state.db, &name, "playtime").await?;
    context.insert("playtime", &playtime);

    Ok(Html(state.template.render("map/playtime.html", &context)?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(map_overview))
        .route("/overview", get(map_overview))
        .route("/timecps", get(map_time_cps))
        .route("/playtime", get(map_playtime))
}
