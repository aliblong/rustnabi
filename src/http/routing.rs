// Ok, I'm giving up on this. Every time I try to factor anything out it's exploding complexity
/*
#![feature(rust_2018_preview, uniform_paths)]
#![feature(async_await, futures_api)]

use futures::Future;

use actix_web::{
    http::Method,
};

enum Synchronicity<I, E> {
    Sync(fn()),
    Async(fn() -> impl Future),
}

pub const SYNC_ROUTES: [RouteSpec; 2] = [
    RouteSpec::new("/",      Method::GET,  index),
    RouteSpec::new("/ws",    Method::GET,  ws_index),
];

pub const ASYNC_ROUTES: [RouteSpec; 1] = [
    RouteSpec::new("/login", Method::POST, Box::new(login)),
];

pub struct RouteSpec<'a, I, E> {
    route: &'a str,
    http_method: Method,
    handler: Synchronicity<I, E>,
}

impl<'a> RouteSpec<'a> {
    pub fn new(
        route: &'a str,
        http_method: Method,
        handler: fn(),
    ) -> Self {
        Self  {
            route,
            http_method,
            handler,
        }
    }
}

pub fn index() {
    unimplemented!()
}
async fn login() {
    unimplemented!()
}
pub fn ws_index() {
    unimplemented!()
}
*/
