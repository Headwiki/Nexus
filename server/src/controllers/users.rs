use crate::db;
use crate::models::User;
use crate::diesel::prelude::*;


pub fn show_users() {
    use crate::schema::users::dsl::*;

    let connection = db::establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("Username: {}", user.username);
        println!("----------\n");
        println!("AccessId: {}", user.access_id);
    }
}