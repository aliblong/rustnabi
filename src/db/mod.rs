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
use self::schema::users;
// A bunch of aliases including one that allows directly referring to a table by its name
//use self::schema::users::dsl::*;
#[allow(proc_macro_derive_resolution_fallback)]
pub mod models;

pub type AuthResult = Result<(), ()>;

pub struct Db {
    conn: PgConnection,
}

impl Db {
    pub fn connect() -> Db {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        Db {
            conn: PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }
    pub fn authenticate_user<'a>(&self, name: &'a str, mut pw: Vec<u8>) -> AuthResult { //-> QueryResult<models::User> {
        let res: QueryResult<(Vec<u8>, Vec<u8>)> = users::table.filter(users::name.eq(name)).select((users::pw, users::salt)).first(&self.conn);
        match res {
            Err(_) => {
                self.add_user(name, pw);
                Ok(())
            },
            Ok((auth_pw, salt)) => {
                let salted_pw = apply_salt(&mut pw, salt);
                let hashed_salted_pw = hash(salted_pw.as_slice());
                match hashed_salted_pw == auth_pw {
                    false => Err(()),
                    true => Ok(()),
                }
            },
        }
    }
    pub fn add_user<'a>(&self, name: &'a str, mut pw: Vec<u8>) {
        let salt = generate_salt();
        let salted_pw = apply_salt(&mut pw, salt.clone());
        let hashed_salted_pw = hash(salted_pw.as_slice());
        let new_user = models::NewUser {
            name: name,
            pw: hashed_salted_pw.as_slice(),
            salt: salt.as_slice(),
        };
        let res: models::User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.conn)
            .expect("Failed to add user");
    }

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
