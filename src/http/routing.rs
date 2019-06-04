use actix_web::{
    web::{
        get,
        post,
	resource,
    },
    HttpServer,
    Route,
};
use actix_http::{body::MessageBody, Error, Request, Response};
use actix_server_config::ServerConfig;
use actix_service::{IntoNewService, NewService};

pub struct RouteSpec<'a> {
    route: &'a str,
    http_method: Route,
    handler_type: fn() -> Route,
    handler: fn(),
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

pub const ROUTE_SPECS: &[&RouteSpec] = &[
    ("/",      get(),  Route::to, index),
    ("/login", post(), Route::to, login),
    ("/ws",    get(),  Route::to, ws_index),
];

pub fn build_routes<F, I, S, B>(app: App) -> App
{
    // server
    //     .service(resource("/").route(get().to(index)))
    //     .service(resource("/login").route(post().to(login)))
    //     .service(resource("/ws").route(get().to(ws_index)))
    for RouteSpec { route, http_method, handler_type, handler } in ROUTE_SPECS {
        server = server.service(resource(route).route(http_method, handler_type(handler)))
    }
}
