use actix_web::web;
use self::super::controllers::*;

// Actix config
pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::get().to(show_users))
            .route("/", web::post().to(|| {}))
            .route("/{name}", web::get().to(create_user))
            .route("/del/{name}", web::get().to(delete_user))
    );
}