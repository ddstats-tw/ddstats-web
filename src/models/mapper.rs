use sqlx::{self, Pool, Postgres};

use super::map::{Info, Map};

pub struct Mapper;

impl Mapper {
    pub async fn maps(
        db: &Pool<Postgres>,
        mapper: &str,
        n: Option<i64>,
    ) -> Result<Vec<Info>, sqlx::Error> {
        sqlx::query_file_as!(Info, "sql/mapper/maps.sql", mapper, n)
            .fetch_all(db)
            .await
    }
}
