use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use ring;
use std::vec::Vec;
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

// TODO define types Hash and Salt as String aliases to prevent output type mixup
fn hash_and_salt(pw: &str, rand: &ring::rand::SecureRandom) -> (String, String) {
    use ring::digest::{digest, Algorithm};
    // 32 bytes is the size of SHA256 output
    let mut salt: Vec<u8> = Vec::with_capacity(32);
    rand.fill(salt.as_mut_slice()).expect("System RNG error");
    let salted_pw = format!("{}{}", salt, pw);
    digest(Algorithm::SHA256, 
}
