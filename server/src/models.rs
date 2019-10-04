#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub access_id: String,
    pub access_secret: String,
    pub access_level: i16
}