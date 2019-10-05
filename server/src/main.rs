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
                            .route(web::get().to(||{ controllers::users::show_users() }))
                    )
                    .service(
                        web::resource("/test2")
                            .route(web::get().to(||{ controllers::users::create_user("testuser".to_owned()) })),
                    )
                    .service(
                        web::resource("/test3")
                            .route(web::get().to(||{ controllers::users::delete_user("testuser".to_owned()) }))
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
