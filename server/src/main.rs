#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_json::{Value};

mod schema;


fn main() -> std::io::Result<()> {

    let _connection = establish_connection();

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

fn establish_connection() -> PgConnection {

    // Read config file
    let content: String = std::fs::read_to_string("config.json")
      .expect("Something went wrong reading config.json");

    // Parse config file
    let config: Value = serde_json::from_str(&content)
      .expect("JSON was not well-formatted");

    // Remove leading and trailing double quotes
    let db_url = config["database_url"].to_string().trim_matches('"').to_owned();

    // Return PgConnection object
    PgConnection::establish(&db_url)
      .expect(&format!("Error connection to {}", &db_url))
}
