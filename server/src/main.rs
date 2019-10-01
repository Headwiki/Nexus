#![allow(dead_code)] // usful in dev mode
///#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate serde_derive;

//use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use serde_json::{Value};

mod config;


fn main() -> std::io::Result<()> {
    //dotenv::dotenv().ok();
    //std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    //env_logger::init();

    // Read config file
    let content: String = std::fs::read_to_string("config.json")
    .expect("Something went wrong reading config.json");

    // Parse config file
    let config: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");

    // create db connection pool
    //let manager = ConnectionManager::<PgConnection>::new(config["database_url"]);
    //let pool: models::Pool = r2d2::Pool::builder()
        //.build(manager)
        //.expect("Failed to create pool.");
    //let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            //.data(pool.clone())
            // enable logger
            //.wrap(middleware::Logger::default())
            //.wrap(IdentityService::new(
                //CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    //.name("auth")
                    //.path("/")
                    //.domain(domain.as_str())
                    //.max_age_time(chrono::Duration::days(1))
                    //.secure(false), // this can only be true if you have https
            //))
            // limit the maximum amount of data that server will accept
            //.data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/test")
                            .route(web::post().to(||{})),
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