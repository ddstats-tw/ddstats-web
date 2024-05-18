use std::fmt::Display;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tera::Context;

use crate::http::AppState;

use super::render;

#[derive(Debug)]
pub enum Error {
    TeraError(tera::Error),
    SQLxError(sqlx::Error),
}

pub async fn error_middleware(
    State(state): State<AppState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    let status = response.status();

    if status != 200 {
        return (
            status,
            render(state.template, "error.html.html", &Context::new()).unwrap(),
        )
            .into_response();
    }
    response
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self)).into_response()
    }
}

impl From<tera::Error> for Error {
    fn from(error: tera::Error) -> Self {
        Self::TeraError(error)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::SQLxError(error)
    }
}
