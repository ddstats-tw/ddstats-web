use askama::Template;
use axum::Extension;

use super::WebContext;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn landing(ext: Extension<WebContext>) -> LandingTemplate {
    println!(
        "{:?}",
        sqlx::query!("SELECT COUNT(*) FROM maps")
            .fetch_one(&ext.db)
            .await
    );

    LandingTemplate
}

#[derive(Template)]
#[template(path = "faq.html")]
pub struct FaqTemplate;

pub async fn faq() -> FaqTemplate {
    FaqTemplate
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}
