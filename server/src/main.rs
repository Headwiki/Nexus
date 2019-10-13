#[macro_use]
extern crate diesel;
extern crate rand;

mod modules;
mod db;
mod schema;

use actix_web::{web, App, HttpServer};

use crate::modules::users::routes::users_config;


fn main() -> std::io::Result<()> {

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .configure(users_config)    // Load actix route config for 'users' module
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}