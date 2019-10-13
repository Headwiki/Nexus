use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub access_id: String,
    pub access_secret: String,
    pub access_level: i16
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub access_id: &'a str,
    pub access_secret: &'a str,
    pub access_level: i16
}
