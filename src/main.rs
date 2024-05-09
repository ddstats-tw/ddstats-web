use http::tera::init_tera;
use points::parse_points;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod error;
mod http;
mod models;
pub mod points;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    tracing_subscriber::fmt::init();

    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .min_connections(25)
        .connect(&db_url)
        .await
        .expect("could not connect to database_url");

    let tera = init_tera();
    let points = parse_points();

    // build our application with a route
    http::serve(db, tera, points).await;
}
