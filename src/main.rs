#![feature(rust_2018_preview, uniform_paths)]
#![feature(await_macro, async_await, futures_api)]

extern crate chrono;
extern crate dotenv;
extern crate ipnetwork;
extern crate pretty_env_logger;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate ring; // For crypto
#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

extern crate actix_web;
extern crate actix_http;
extern crate actix_server_config;
extern crate actix_service;

extern crate rand; // For deck shuffling RNG

extern crate futures;
#[macro_use]
extern crate tokio;

extern crate chess_clock;

mod db;
mod util;
//mod login;
mod game;
mod hash;
mod http;

use ring::rand::SystemRandom;

use actix_web::{
    web::{
        resource,
        get,
        post,
        service,
    },
    HttpServer,
    Route,
    middleware,
    App,
};

/// According to `ring` docs, one (threadsafe) instance of SystemRandom should be used for the
/// entire app
lazy_static! {
    pub static ref SYSRAND: SystemRandom = SystemRandom::new();
}

pub fn index() {
    unimplemented!()
}
pub fn login() {
    unimplemented!()
}
pub fn ws_index() {
    unimplemented!()
}

fn main() {
    pretty_env_logger::init();

    match dotenv::dotenv() {
        Err(e) => warn!("Error reading .env file: {}", e),
        _ => info!("Parsed .env file successfully"),
    }

    let db = db::Db::connect();
    let name = "testname0";
    let pw = b"asdf";
    use ipnetwork::IpNetwork;
    let ip = IpNetwork::V4("192.168.0.2/16".parse().unwrap());
    match db.authenticate_user(name, pw.to_vec(), ip) {
        Err(_) => warn!("Invalid credentials for {}", name),
        _ => warn!("User {} logged in successfully", name),
    }
    let server = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(resource("/"     ).route( get() .to(index   )))
            .service(resource("/login").route( post().to(login   )))
            .service(resource("/ws"   ).route( get() .to(ws_index)))
    });
    server.bind("127.0.0.1:8080")?.run()
}

#[cfg(test)]
mod test {
    pub static NORMAL_VARIANT: &'static str = "---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors:
      - 0
  - dist: *def_dist
    colors:
      - 1
  - dist: *def_dist
    colors:
      - 2
  - dist: *def_dist
    colors:
      - 3
  - dist: *def_dist
    colors:
      - 4
";

    pub static ACID_TRIP_VARIANT: &'static str = "---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
";

    pub static WILD_CRAZY_VARIANT: &'static str = "---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors:
      - 0
      - 1
  - dist: *def_dist
    colors:
      - 0
      - 2
  - dist: *def_dist
    colors:
      - 1
      - 2
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors:
      - 0
      - 1
      - 2
      - 3
  - dist:
      - 1
      - 1
      - 1
      - 1
      - 1
    colors:
      - 3
";
}
