use http::macros::{code_to_country, fancy_time, map_thumbnail, ordinal, time_format};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;

mod error;
mod http;
mod models;

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

    let mut template = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    template.register_filter("map_thumbnail", map_thumbnail);
    template.register_filter("fancy_time", fancy_time);
    template.register_filter("code_to_country", code_to_country);
    template.register_filter("ordinal", ordinal);
    template.register_filter("time_format", time_format);

    // build our application with a route
    http::serve(db, template).await;
}
