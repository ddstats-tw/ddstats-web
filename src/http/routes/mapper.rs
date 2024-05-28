use crate::http::error::Error;
use crate::http::render;
use crate::http::AppState;
use crate::models::mapper::Mapper;
use crate::models::player::Player;
use axum::extract::Path;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{routing::get, Router};
use sqlx::Pool;
use sqlx::Postgres;
use tera::Context;

pub async fn mapper_context(db: &Pool<Postgres>, name: &str, page: &str) -> Result<Context, Error> {
    let profile = Player::get_profile(db, name).await.ok();
    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("profile", &profile);
    context.insert("page", &page);

    Ok(context)
}

pub async fn mapper_maps(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let maps = Mapper::maps(&state.db, &name, None).await?;

    let mut context = mapper_context(&state.db, &name, "maps").await?;
    context.insert("maps", &maps);

    render(state.template, "mapper/maps.html", &context)
}

pub async fn mapper_middleware(
    Path(name): Path<String>,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    match Mapper::maps(&state.db, &name, Some(1))
        .await
        .unwrap()
        .is_empty()
    {
        false => next.run(request).await,
        true => (StatusCode::NOT_FOUND, {
            let mut context = Context::new();
            context.insert("hide_search", &true);
            render(state.template, "mapper/404.html", &context).unwrap()
        })
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(mapper_maps))
        .route("/maps", get(mapper_maps))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mapper_middleware,
        ))
}
