use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use settings::Settings;
use sqlx::PgPool;

mod db;
mod proto;
mod routes;
mod settings;
mod storage;
mod utils;

pub struct State {
    pool: PgPool,
    config: settings::Settings,
    s3: storage::S3Storage,
}
// ffmpeg -ss 00:00:00 -i input.mp4 -to 00:00:15 -vcodec libwebp -preset photo -loop 0 -an -vsync 0 -compression_level 6 -s 960x720 output.webp
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let config = Settings::new().unwrap();
    let pool = db::create_pool(&config).await;
    let s3 = storage::S3Storage::new(config.clone());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(State {
                pool: pool.clone(),
                config: config.clone(),
                s3: s3.clone(),
            }))
            .configure(routes::config)
    })
    .bind("127.0.0.1:56750")?
    .bind_uds("/tmp/avelio.sock")?
    .run()
    .await
}
