table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        access_id -> Varchar,
        access_secret -> Varchar,
        access_level -> Int2,
    }
}
