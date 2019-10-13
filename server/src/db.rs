use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde_json::Value;

pub fn establish_connection() -> PgConnection {

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