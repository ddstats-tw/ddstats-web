use askama::Template;
use axum::Extension;

use crate::models::player::Player;

use super::WebContext;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn landing(ext: Extension<WebContext>) -> LandingTemplate {
    println!("{:?}", Player::search(&ext.db, "test", 20).await);

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
