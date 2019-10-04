#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

mod controllers {
    pub mod users;
}
mod db;
mod models;
mod schema;

fn main() -> std::io::Result<()> {

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/test")
                            .route(web::get().to(||{controllers::users::show_users()})),
                    )
                    .service(
                        web::resource("/test2")
                            .route(web::post().to(||{})),
                    )
                    .service(
                        web::resource("/test3")
                            .route(web::post().to(||{}))
                            .route(web::delete().to(||{}))
                            .route(web::get().to(||{})),
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
