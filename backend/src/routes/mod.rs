use actix_web::web;
mod auth;
mod auth_middleware;
mod playlist;
mod track;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let guard = HttpAuthentication::bearer(auth_middleware::ok_validator);
    cfg.service(web::scope("/auth").configure(auth::config))
        .service(
            web::scope("/track")
                .wrap(guard.clone())
                .configure(track::config),
        )
        .service(
            web::scope("/playlist")
                .wrap(guard.clone())
                .configure(playlist::config),
        );
}
