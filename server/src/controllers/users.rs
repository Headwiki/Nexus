use crate::db;
use crate::models::{ User, NewUser };
use crate::diesel::prelude::*;
use crate::schema::users::dsl::*;

pub fn show_users() {

    let connection = db::establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users\n", results.len());
    for user in results {
        println!("Username: {}\nAccessId: {}", user.username, user.access_id);
        println!("----------");
    }
}

pub fn create_user(new_username: String) {

    let connection = db::establish_connection();

    let new_user = NewUser {
        username: &new_username,
        password: "Test1234",
        access_id: "asdf123",
        access_secret: "123asdf"
    };

    let created_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&connection)
        .expect("Error creating new user");

    println!("Username: {}, Access Id: {}", created_user.username, created_user.access_id);
}

pub fn delete_user(del_username: String) {
    
    let connection = db::establish_connection();

    let deleted_user = diesel::delete(users.filter(username.like(del_username)))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted user: {}", deleted_user);
}