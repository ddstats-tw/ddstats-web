use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init_db(db_url: String) -> Pool<Postgres> {
    PgPoolOptions::new()
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
        .expect("could not connect to database")
}
