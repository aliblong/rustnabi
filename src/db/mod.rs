//https://github.com/rust-lang/rust/issues/50504#issuecomment-410550021
//https://github.com/diesel-rs/diesel/issues/1785
#[allow(proc_macro_derive_resolution_fallback, unused_imports)]
mod schema;
#[allow(proc_macro_derive_resolution_fallback, dead_code)]
pub mod models;

use diesel;
use diesel::prelude::*;

use std::vec::Vec;
use std::env;

use ring::rand::SecureRandom;

use self::schema::users;
// A bunch of aliases including one that allows directly referring to a table by its name
//use self::schema::users::dsl::*;

use ipnetwork::IpNetwork;

pub type AuthResult = Result<(), AuthError>;
pub enum AuthError {
    WrongPw,
}

pub struct Db {
    conn: PgConnection,
}

impl Db {
    pub fn connect() -> Db {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set (check `.env`)");
        Db {
            conn: PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }
    // This function has an uncomfortable number of nested scopes, but splitting it into more
    // functions is a pain since all the types from Diesel expressions are very complicated.
    pub fn authenticate_user<'a>(&self, name: &'a str, pw: Vec<u8>, ip: IpNetwork) -> AuthResult { //-> QueryResult<models::User> {
        let user_record = users::table.filter(users::name.eq(name));
        let user_exists: QueryResult<(Vec<u8>, Vec<u8>, Vec<IpNetwork>)> =
            user_record.select((
                users::pw,
                users::salt,
                users::ip,
            ))
            .first(&self.conn);
        match user_exists {
            // If user doesn't exist, add them to the database;
            Err(_) => {
                self.add_user(name, pw, ip);
                Ok(())
            },
            // otherwise, authenticate and update their list of ip addresses and last login time
            Ok((auth_pw, salt, mut ips)) => {
                match check_pw(pw, salt, &auth_pw) {
                    false => Err(AuthError::WrongPw),
                    true => {
                        diesel::update(user_record)
                            .set(users::datetime_last_login.eq(diesel::dsl::now))
                            .execute(&self.conn)
                            .expect("Failed to update user");

                        // if their list of ip addresses already contains the current one, don't
                        // update the list
                        match ips.contains(&ip) {
                            false => {
                                ips.push(ip);
                                diesel::update(user_record)
                                    .set(users::ip.eq(ips))
                                    .execute(&self.conn)
                                    .expect("Failed to update user");
                            },
                            true => (),
                        };
                        Ok(())
                    },
                }
            },
        }
    }
    fn add_user<'a>(&self, name: &'a str, pw: Vec<u8>, ip: IpNetwork) {
        let salt = generate_salt();
        let salted_pw = apply_salt(pw, salt.clone());
        let hashed_salted_pw = hash(salted_pw.as_slice());

        // IP address must be put in a vec because of how it's stored in the DB
        let ip_vec = vec![ip];
        let ip_slice = ip_vec.as_slice();

        let new_user = models::NewUser {
            name: name,
            pw: hashed_salted_pw.as_slice(),
            salt: salt.as_slice(),
            ip: ip_slice,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&self.conn)
            .expect("Failed to add user");
    }

}

fn generate_salt() -> Vec<u8> {
    let mut salt: Vec<u8> = vec![b'0'; 32];
    super::SYSRAND.fill(salt.as_mut_slice()).expect("System RNG error");
    salt
}

fn apply_salt(mut pw: Vec<u8>, mut salt: Vec<u8>) -> Vec<u8> {
    // Can't chain clone() into append() because the expression needs an explicit lifetime
    // https://github.com/rust-lang/rust/issues/27063
    salt.append(&mut pw);
    salt
}

fn hash(pw: &[u8]) -> Vec<u8> {
    use ring::digest;
    digest::digest(&digest::SHA256, pw).as_ref().to_vec()
}

fn check_pw(pw: Vec<u8>, salt: Vec<u8>, auth_pw: &Vec<u8>) -> bool {
    let salted_pw = apply_salt(pw, salt);
    let hashed_salted_pw = hash(salted_pw.as_slice());
    hashed_salted_pw == *auth_pw
}
