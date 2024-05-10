use database::init_db;
use points::parse_points;
use std::env;
use tera::init_tera;

mod database;
mod http;
mod models;
mod points;
mod tera;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("failed to get DATABASE_URL");
    let db = init_db(db_url).await;
    let tera = init_tera();
    let points = parse_points();

    http::serve(db, tera, points).await;
}
