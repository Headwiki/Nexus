use crate::db;
use crate::diesel::prelude::*;
use crate::schema::users::dsl::*;

use actix_web::{
    web, Responder, HttpResponse
};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use self::super::models::*;


pub fn show_users() -> impl Responder {

    let connection = db::establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users\n", results.len());
    for user in results {
        println!("Username: {}\nAccessId: {}\nAccessSecret: {}", user.username, user.access_id, user.access_secret);
        println!("----------");
    }
    HttpResponse::Ok()
}

pub fn create_user(data: web::Path<String>) -> impl Responder {

    let connection = db::establish_connection();

    let gen_secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    let new_user = NewUser {
        username: &data,
        password: "asdasda",
        access_id: "asdf123",
        access_secret: &gen_secret,
        access_level: 0
    };

    let created_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&connection)
        .expect("Error creating new user");

    println!("Username: {}, Access Id: {}", created_user.username, created_user.access_id);
    HttpResponse::Ok()
}

pub fn delete_user(name: web::Path<String>) -> impl Responder {
    
    let connection = db::establish_connection();

    let deleted_user = diesel::delete(users.filter(username.like(name.to_string())))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted user: {}", deleted_user);
    HttpResponse::Ok()
}