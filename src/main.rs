use std::{collections::HashMap, env};

use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use tera::{to_value, try_get_value, Error, Tera, Value};

mod http;
mod models;

pub fn map_thumbnail(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let mut s = try_get_value!("map_thumbnail", "value", String, value);
    let re1 = Regex::new(r"[À-ž]").unwrap();
    let re2 = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    s = re1.replace_all(&s, "__").to_string();
    s = re2.replace_all(&s, "_").to_string();
    Ok(to_value(s).unwrap())
}

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

    // build our application with a route
    http::serve(db, template).await;
}
