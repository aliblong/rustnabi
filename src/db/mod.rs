use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
pub mod schema;
pub mod models;

pub fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user<'a>(conn: &PgConnection, name: &'a str, pw: &'a str) -> QueryResult<models::User> {
    use self::schema::users;
    let new_user = models::NewUser {
        name: name,
        pw: pw,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}
