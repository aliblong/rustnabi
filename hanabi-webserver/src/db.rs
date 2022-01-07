//https://github.com/rust-lang/rust/issues/50504#issuecomment-410550021
//https://github.com/diesel-rs/diesel/issues/1785
#[allow(proc_macro_derive_resolution_fallback, dead_code)]
pub mod models;
#[allow(proc_macro_derive_resolution_fallback, unused_imports)]
mod schema;

use diesel;
use diesel::prelude::*;

use std::env;
use std::vec::Vec;

use hash::hash;

use self::schema::{ips, user_ips, users};
// A bunch of aliases including one that allows directly referring to a table by its name
//use self::schema::users::dsl::*;

use super::SYSRAND;
use ipnetwork::IpNetwork;
use ring::rand::SecureRandom;

pub type AuthResult = Result<(), AuthError>;
pub enum AuthError {
    WrongPw,
}

pub struct Db {
    conn: PgConnection,
}

impl Db {
    pub fn connect() -> Db {
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set (check `.env`)");
        Db {
            conn: PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }
    // This function has an uncomfortable number of nested scopes, but splitting it into more
    // functions is a pain since all the types from Diesel expressions are very complicated.
    pub fn authenticate_user<'a>(&self, name: &'a str, pw: Vec<u8>, ip: IpNetwork) -> AuthResult {
        //-> QueryResult<models::User> {
        let user_record = users::table.filter(users::name.eq(name));
        let user_exists: QueryResult<(i32, Vec<u8>, Vec<u8>)> = user_record
            .select((users::id, users::pw, users::salt))
            .first(&self.conn);
        match user_exists {
            // If user doesn't exist, add them to the database;
            Err(_) => {
                self.add_user(name, pw, ip);
                Ok(())
            }
            // otherwise, authenticate and update their list of ip addresses and last login time
            Ok((user_id, auth_pw, salt)) => match check_pw(pw, salt, &auth_pw) {
                false => Err(AuthError::WrongPw),
                true => {
                    diesel::update(user_record)
                        .set(users::datetime_last_login.eq(diesel::dsl::now))
                        .execute(&self.conn)
                        .expect("Failed to update user");
                    self.add_ip(user_id, ip);
                    Ok(())
                }
            },
        }
    }

    fn add_user<'a>(&self, name: &'a str, pw: Vec<u8>, ip: IpNetwork) {
        let salt = generate_salt();
        let salted_pw = apply_salt(pw, salt.clone());
        let hashed_salted_pw = hash(salted_pw.as_slice());

        let new_user = models::NewUser {
            name: name,
            pw: hashed_salted_pw.as_slice(),
            salt: salt.as_slice(),
        };
        let user_id = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(users::id)
            .execute(&self.conn)
            .expect("Failed to add user");
        // TODO: figure out why user ID is usize rather than i32
        self.add_ip(user_id as i32, ip);
    }

    fn add_ip(&self, user_id: i32, ip: IpNetwork) {
        let ip_exists = ips::table
            .filter(ips::ip.eq(ip))
            .select(ips::id)
            .first(&self.conn);
        match ip_exists {
            Err(_) => {
                let ip_id = diesel::insert_into(ips::table)
                    .values(models::NewIP { ip: &ip })
                    .returning(ips::id)
                    .get_result(&self.conn)
                    .expect("Failed to add IP address to db");
                diesel::insert_into(user_ips::table)
                    .values(models::NewUserIP { user_id, ip_id })
                    .execute(&self.conn)
                    .expect("Logic error: IP addr exists but wasn't selected");
            }
            Ok(ip_id) => {
                diesel::insert_into(user_ips::table)
                    .values(models::NewUserIP { user_id, ip_id })
                    .on_conflict_do_nothing()
                    .execute(&self.conn)
                    .expect("Failed to add user-IP pair to db");
            }
        }
    }
}

fn generate_salt() -> Vec<u8> {
    let mut salt: Vec<u8> = vec![b'0'; 32];
    SYSRAND.fill(salt.as_mut_slice()).expect("System RNG error");
    salt
}

fn apply_salt(mut pw: Vec<u8>, mut salt: Vec<u8>) -> Vec<u8> {
    // Can't chain clone() into append() because the expression needs an explicit lifetime
    // https://github.com/rust-lang/rust/issues/27063
    salt.append(&mut pw);
    salt
}

fn check_pw(pw: Vec<u8>, salt: Vec<u8>, auth_pw: &Vec<u8>) -> bool {
    let salted_pw = apply_salt(pw, salt);
    let hashed_salted_pw = hash(salted_pw.as_slice());
    hashed_salted_pw == *auth_pw
}
