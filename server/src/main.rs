#[macro_use]
extern crate diesel;
extern crate rand;

mod controllers {
    pub mod users;
}
mod db;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};

use crate::controllers::users::*;

fn main() -> std::io::Result<()> {

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/test")
                            .service(show_users)
                            .service(create_user)
                            .service(delete_user)
                    )
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
