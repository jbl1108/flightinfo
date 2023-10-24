mod services;
mod model;

use crate::services::flight_service;
use std::{env, io};
use actix_web::{App, HttpServer, middleware, web, guard::{Guard, self}};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::scope("/flight")
            // register HTTP requests handlers
            .route("",web::route().guard(guard::Get()).to(flight_service::list))
            .route("/insert",web::route().guard(guard::Post()).to(flight_service::insert))
            .route("/{id}/delete",web::get().to(flight_service::delete)))

    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}