use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::settings::Settings;
pub mod playlist;
pub mod song;
pub mod user;

pub async fn create_pool(config: &Settings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.url)
        .await
        .unwrap()
}
