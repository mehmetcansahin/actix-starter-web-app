use crate::app::controllers::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .service(
                    web::resource("/register").route(web::post().to_async(user::auth::register)),
                )
                .service(web::resource("/login").route(web::post().to_async(user::auth::login)))
                .service(web::resource("/me").route(web::get().to_async(user::auth::me))),
        ),
    );
}
