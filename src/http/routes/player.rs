use crate::http::error::Error;
use crate::http::render;
use crate::http::AppState;
use crate::models::mapper::Mapper;
use crate::models::player::Finish;
use crate::models::player::Player;
use crate::utils::create_index_by_field;
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

pub async fn player_context(db: &Pool<Postgres>, name: &str, page: &str) -> Result<Context, Error> {
    let profile = Player::get_profile(db, name).await?;
    let is_mapper = !Mapper::maps(db, name, Some(1)).await.unwrap().is_empty();
    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("profile", &profile);
    context.insert("is_mapper", &is_mapper);
    context.insert("page", &page);

    Ok(context)
}

pub async fn player_overview(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let recent_finishes = Player::recent_finishes(&state.db, &name, Some(10)).await?;
    let favourite_teammates = Player::favourite_teammates(&state.db, &name, Some(10)).await?;

    let mut context = player_context(&state.db, &name, "overview").await?;
    context.insert("recent_finishes", &recent_finishes);
    context.insert("favourite_teammates", &favourite_teammates);

    render(state.template, "player/overview/overview.html", &context)
}

pub async fn player_overview_partners(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let favourite_teammates = Player::favourite_teammates(&state.db, &name, None).await?;

    let mut context = player_context(&state.db, &name, "overview").await?;
    context.insert("favourite_teammates", &favourite_teammates);

    render(
        state.template,
        "player/overview/favourite_partners.html",
        &context,
    )
}

pub async fn player_overview_finishes(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let recent_finishes = Player::recent_finishes(&state.db, &name, Some(100)).await?;

    let mut context = player_context(&state.db, &name, "overview").await?;
    context.insert("recent_finishes", &recent_finishes);

    render(
        state.template,
        "player/overview/recent_finishes.html",
        &context,
    )
}

pub async fn player_finishes(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let finishes = create_index_by_field::<Finish, String>(
        Player::finishes(&state.db, &name).await?,
        |finish| finish.map.server.clone(),
    );
    let points = Player::points(&state.points, &name);

    let mut context = player_context(&state.db, &name, "finishes").await?;
    context.insert("points", &points);
    context.insert("finishes", &finishes);

    render(state.template, "player/finishes/finishes.html", &context)
}

pub async fn player_activity(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let recent_activity = Player::recent_activity(&state.db, &name, Some(11)).await?;
    let recent_player_info = Player::recent_player_info(&state.db, &name, Some(5)).await?;
    let most_played_maps = Player::most_played_maps(&state.db, &name, Some(11)).await?;
    let most_played_gametypes = Player::most_played_gametypes(&state.db, &name, Some(15)).await?;
    let most_played_categories = Player::most_played_categories(&state.db, &name, Some(15)).await?;
    let most_played_locations = Player::most_played_locations(&state.db, &name, Some(15)).await?;
    let playtime_per_month = Player::playtime_per_month(&state.db, &name, Some(12)).await?;

    let mut context = player_context(&state.db, &name, "activity").await?;
    context.insert("recent_activity", &recent_activity);
    context.insert("recent_player_info", &recent_player_info);
    context.insert("most_played_maps", &most_played_maps);
    context.insert("most_played_gametypes", &most_played_gametypes);
    context.insert("most_played_categories", &most_played_categories);
    context.insert("most_played_locations", &most_played_locations);
    context.insert("playtime_per_month", &playtime_per_month);

    render(state.template, "player/activity/activity.html", &context)
}

pub async fn player_activity_playtime(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let recent_activity = Player::recent_activity(&state.db, &name, Some(1000)).await?;

    let mut context = player_context(&state.db, &name, "activity").await?;
    context.insert("recent_activity", &recent_activity);

    render(state.template, "player/activity/playtime.html", &context)
}

pub async fn player_activity_player_info(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let recent_player_info = Player::recent_player_info(&state.db, &name, Some(100)).await?;

    let mut context = player_context(&state.db, &name, "activity").await?;
    context.insert("recent_player_info", &recent_player_info);

    render(state.template, "player/activity/player_info.html", &context)
}

pub async fn player_activity_most_played_maps(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let most_played_maps = Player::most_played_maps(&state.db, &name, None).await?;

    let mut context = player_context(&state.db, &name, "activity").await?;
    context.insert("most_played_maps", &most_played_maps);

    render(
        state.template,
        "player/activity/most_played_maps.html",
        &context,
    )
}

pub async fn player_rank1s(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let favourite_rank1s_teammates =
        Player::favourite_rank1s_teammates(&state.db, &name, Some(10)).await?;
    let all_top_10s = Player::all_top_10s(&state.db, &name).await?;
    let recent_top_10s = Player::recent_top_10s(&state.db, &name, Some(9)).await?;

    let mut context = player_context(&state.db, &name, "rank1s").await?;
    context.insert("favourite_rank1s_teammates", &favourite_rank1s_teammates);
    context.insert("all_top_10s", &all_top_10s);
    context.insert("recent_top_10s", &recent_top_10s);

    render(state.template, "player/rank1s/rank1s.html", &context)
}

pub async fn player_rank1s_partners(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, Error> {
    let favourite_rank1s_teammates =
        Player::favourite_rank1s_teammates(&state.db, &name, None).await?;

    let mut context = player_context(&state.db, &name, "rank1s").await?;
    context.insert("favourite_rank1s_teammates", &favourite_rank1s_teammates);

    render(state.template, "player/rank1s/partners.html", &context)
}

pub async fn player_middleware(
    Path(name): Path<String>,
    State(state): State<AppState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Response {
    match Player::get_profile(&state.db, &name).await.is_ok() {
        true => next.run(request).await,
        false => (
            StatusCode::NOT_FOUND,
            render(state.template, "player/404.html", &Context::new()).unwrap(),
        )
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        // Overview
        .route("/", get(player_overview))
        .route("/overview", get(player_overview))
        .route("/overview/finishes", get(player_overview_finishes))
        .route("/overview/partners", get(player_overview_partners))
        // Finishes
        .route("/finishes", get(player_finishes))
        // Activity
        .route("/activity", get(player_activity))
        .route("/activity/playtime", get(player_activity_playtime))
        .route("/activity/playerinfo", get(player_activity_player_info))
        .route(
            "/activity/mostplayed",
            get(player_activity_most_played_maps),
        )
        // Rank 1s
        .route("/rank1s", get(player_rank1s))
        .route("/rank1s/partners", get(player_rank1s_partners))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            player_middleware,
        ))
}
