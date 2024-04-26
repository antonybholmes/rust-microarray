use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::{MicroarrayError, MicroarrayResult};

const FIND_SAMPLES_SQL: &'static str = r#"SELECT
id, uuid, name, array
FROM samples
WHERE users.name LIKE $1 OR users.uuid LIKE $1"#;

impl From<sqlx::Error> for MicroarrayError {
    fn from(e: sqlx::Error) -> MicroarrayError {
        return MicroarrayError::DatabaseError(e.to_string());
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, FromRow)]
pub struct MicroarraySample {
    pub id: u32,
    pub uuid: String,
    pub array: String,
    pub name: String,
}

pub type MicroarraySamplesResult = MicroarrayResult<Vec<MicroarraySample>>;

#[derive(Clone)]
pub struct MicroarrayDb {
    pool: Pool<Sqlite>,
}

impl MicroarrayDb {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_samples(&self, search: &str) -> MicroarraySamplesResult {
        let samples = sqlx::query_as::<_, MicroarraySample>(FIND_SAMPLES_SQL)
            .bind(format!("%{}%", search))
            .fetch_all(&self.pool)
            .await?;

        Ok(samples)
    }

    // Returns element
}
