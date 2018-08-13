use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use ring::rand::SecureRandom;
use std::vec::Vec;
//https://github.com/rust-lang/rust/issues/50504#issuecomment-410550021
//https://github.com/diesel-rs/diesel/issues/1785
#[allow(proc_macro_derive_resolution_fallback)]
#[allow(unused_imports)]
pub mod schema;
#[allow(proc_macro_derive_resolution_fallback)]
pub mod models;

pub fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user<'a>(conn: &PgConnection, name: &'a str, mut pw: Vec<u8>) -> QueryResult<models::User> {
    let salt = generate_salt();
    let salted_pw = apply_salt(&mut pw, salt.clone());
    let hashed_salted_pw = hash(salted_pw.as_slice());
    use self::schema::users;
    let new_user = models::NewUser {
        name: name,
        pw: hashed_salted_pw.as_slice(),
        salt: salt.as_slice(),
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

fn generate_salt() -> Vec<u8> {
    let mut salt: Vec<u8> = vec![b'0'; 32];
    super::SYSRAND.fill(salt.as_mut_slice()).expect("System RNG error");
    salt
}

fn apply_salt(pw: &mut Vec<u8>, mut salt: Vec<u8>) -> Vec<u8> {
    // Can't chain clone() into append() because the expression needs an explicit lifetime
    // https://github.com/rust-lang/rust/issues/27063
    salt.append(pw);
    salt
}

fn hash(pw: &[u8]) -> Vec<u8> {
    use ring::digest;
    digest::digest(&digest::SHA256, pw).as_ref().to_vec()
}
