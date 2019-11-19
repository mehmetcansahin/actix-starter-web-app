#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod app;
mod config;
mod error;
mod routes;
mod schema;

use actix_web::{http::StatusCode, middleware::errhandlers::ErrorHandlers, App, HttpServer};
use listenfd::ListenFd;
use std::env;

fn main() {
    let pool = config::db::get_db_pool();

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(app::middleware::jwt::Token)
            .wrap(ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, error::render_400))
            .configure(routes::config)
    });
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };
    server.run().unwrap();
}
