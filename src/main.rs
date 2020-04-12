#![feature(matches_macro)]
use actix::{Actor, StreamHandler};
use actix_identity::Identity;
use itsdangerous::{SignerBuilder, IntoTimestampSigner, TimestampSigner};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_files as fs;
use serde::Deserialize;
use mailgun_rs;
////extern crate chrono;
use dotenv;
use std::env;
////extern crate ipnetwork;
use pretty_env_logger;
use log::{warn, info};
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
//mod game;
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
/// Define http actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    println!("hi");
    resp
}

#[derive(Deserialize)]
struct LoginFormData {
    email_address: String,
}

async fn send_login_email(
    email_address: &str,
    timestamp_signed_email_address: &str,
    credentials: &MailgunCredentials
) {
    let resp = mailgun_rs::Mailgun {
        api_key: credentials.api_key.clone(),
        domain: credentials.domain.clone(),
        message: mailgun_rs::Message {
            to: vec![mailgun_rs::EmailAddress::address(email_address)],
            subject: "Login link to playhanabi.com".to_owned(),
            cc: vec![],
            bcc: vec![],
            text: format!(
                "Follow this link to sign into playhanabi.com:\r\n\
                https://playhanabi.com/login/{}", timestamp_signed_email_address
            ),
            html: "".to_owned(),
        }
    }.send(
        &mailgun_rs::EmailAddress::name_address(
            "playhanabi admin",
            "admin@playhanabi.com",
        )
    );
    match resp {
        Ok(response) => {
            println!("success! {}; {}", response.message, response.id)
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}

async fn handle_request_email(
    form: web::Form<LoginFormData>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    println!("{}", &form.email_address);
    let key = data.get_ref().crypto_signer_key.clone();
    let signer = itsdangerous::default_builder(key).build().into_timestamp_signer();
    send_login_email(
        &form.email_address,
        &*signer.sign(&form.email_address),
        &data.mailgun_credentials
    ).await;
    Ok(HttpResponse::Ok().finish())
}

struct AppState {
    // apparently the signer itself doesn't implement Clone, so have to build it each time an
    // email request is handled
    crypto_signer_key: String,
    mailgun_credentials: MailgunCredentials,
}

struct MailgunCredentials {
    api_key: String,
    domain: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    match dotenv::dotenv() {
        Err(e) => warn!("Error reading .env file: {}", e),
        _ => info!("Parsed .env file successfully"),
    }
    let crypto_signer_key = "xddddddddd";
    let api_key = env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY must be set (check `.env`)");
    let domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set (check `.env`)");
    // let app_data = itsdangerous::default_builder(crypto_signer_key).build();
    HttpServer::new(move || {
        App::new()
            .data(AppState{
                crypto_signer_key: crypto_signer_key.to_owned(),
                mailgun_credentials: MailgunCredentials {
                    api_key: api_key.clone(),
                    domain: domain.clone(),
                },
            })
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
                    .secure(false),
            ))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/index.html")
                    .finish()
            })))
            .service(fs::Files::new("/static/", "static/"))
            .service(web::resource("/request_email").route(web::post().to(handle_request_email)))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

//fn main() {
//    println!("Hello, world!");
//}
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
