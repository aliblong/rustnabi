////extern crate chrono;
////extern crate dotenv;
////extern crate ipnetwork;
//use pretty_env_logger;
//use log::{warn, info};
//#[macro_use]
//extern crate diesel_derive_enum;
//#[macro_use]
//extern crate diesel;
//use lazy_static::lazy_static;
//use std::io;
//
//#[macro_use]
//extern crate serde_derive;
//use futures::future::{
//    Future,
//    ok,
//};
////extern crate chess_clock;

//mod db;
//mod util;
//mod login;
mod game;
//mod hash;
//mod http;
//use vec_map;

//use ring::rand::SystemRandom;
//
//use actix_web::{
//    web::{
//        get,
//        post,
//        resource,
//        service,
//        method,
//    },
//    Error,
//    HttpResponse,
//    HttpServer,
//    Route,
//    Resource,
//    middleware,
//    App,
//};
//
fn main() {
    println!("Hello, world!");
}
////use http::routing::{RouteSpec, SYNC_ROUTES, ASYNC_ROUTES};
//
///// According to `ring` docs, one (threadsafe) instance of SystemRandom should be used for the
///// entire app
//lazy_static! {
//    pub static ref SYSRAND: SystemRandom = SystemRandom::new();
//}
//
//async fn index() -> Result<HttpResponse, Error>{
//    unimplemented!()
//}
////async fn login() -> Result<HttpResponse, Error> {
////    ok(HttpResponse::Ok().finish())
////}
//async fn ws_index() -> Result<HttpResponse, Error> {
//    unimplemented!()
//}
//
//#[actix_rt::main]
//async fn main() -> std::io::Result<()> {
//    pretty_env_logger::init();
//
//    match dotenv::dotenv() {
//        Err(e) => warn!("Error reading .env file: {}", e),
//        _ => info!("Parsed .env file successfully"),
//    }
//
//    let db = db::Db::connect();
//    let name = "testname0";
//    let pw = b"asdf";
//    use ipnetwork::IpNetwork;
//    let ip = IpNetwork::V4("192.168.0.2/16".parse().unwrap());
//    match db.authenticate_user(name, pw.to_vec(), ip) {
//        Err(_) => warn!("Invalid credentials for {}", name),
//        _ => warn!("User {} logged in successfully", name),
//    }
//    HttpServer::new(|| {
//        App::new()
//            // enable logger
//            .wrap(middleware::Logger::default())
//            .service(resource( "/"      ).route( get()  .to( index    )))
////            .service(resource( "/login" ).route( post() .to( login    )))
//            .service(resource( "/ws"    ).route( get()  .to( ws_index )))
//    }).bind("127.0.0.1:8080")?.run().await
//}
//
//#[cfg(test)]
//mod test {
//    pub static NORMAL_VARIANT: &'static str = "---
//default_dist: &def_dist
//  - 3
//  - 2
//  - 2
//  - 2
//  - 1
//
//suits:
//  - dist: *def_dist
//    colors:
//      - 0
//  - dist: *def_dist
//    colors:
//      - 1
//  - dist: *def_dist
//    colors:
//      - 2
//  - dist: *def_dist
//    colors:
//      - 3
//  - dist: *def_dist
//    colors:
//      - 4
//";
//
//    pub static ACID_TRIP_VARIANT: &'static str = "---
//default_dist: &def_dist
//  - 3
//  - 2
//  - 2
//  - 2
//  - 1
//
//suits:
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors: []
//";
//
//    pub static WILD_CRAZY_VARIANT: &'static str = "---
//default_dist: &def_dist
//  - 3
//  - 2
//  - 2
//  - 2
//  - 1
//
//suits:
//  - dist: *def_dist
//    colors:
//      - 0
//      - 1
//  - dist: *def_dist
//    colors:
//      - 0
//      - 2
//  - dist: *def_dist
//    colors:
//      - 1
//      - 2
//  - dist: *def_dist
//    colors: []
//  - dist: *def_dist
//    colors:
//      - 0
//      - 1
//      - 2
//      - 3
//  - dist:
//      - 1
//      - 1
//      - 1
//      - 1
//      - 1
//    colors:
//      - 3
//";
//}
