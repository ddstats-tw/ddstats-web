use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn init_db() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(
            env::var("DATABASE_MAX_CONNECTIONS")
                .map(|x| x.parse().expect("not a number"))
                .unwrap_or(50),
        )
        .min_connections(
            env::var("DATABASE_MIN_CONNECTIONS")
                .map(|x| x.parse().expect("not a number"))
                .unwrap_or(1),
        )
        .connect(&env::var("DATABASE_URL").expect("failed to get DATABASE_URL"))
        .await
        .expect("could not connect to database")
}
