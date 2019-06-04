use actix_web::{
    web::{
        get,
        post,
	resource,
    },
    App,
    Route,
};

pub struct RouteSpec<'a> {
    route: &'a str,
    http_method: Route,
    handler_type: fn(Route) -> Route,
    handler: fn(),
}

impl<'a> RouteSpec<'a> {
    pub fn new(
        route: &'a str,
        http_method: Route,
        handler_type: fn(Route) -> Route,
        handler: fn(),
    ) -> Self {
        Self  {
            route,
            http_method,
            handler_type,
            handler,
        }
    }
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

pub const ROUTE_SPECS: [RouteSpec; 3] = [
    RouteSpec::new("/",      get(),  Route::to, index),
    RouteSpec::new("/login", post(), Route::to, login),
    RouteSpec::new("/ws",    get(),  Route::to, ws_index),
];

pub fn build_routes<T, B>(app: App<T, B>) -> App<T, B>
{
    // server
    //     .service(resource("/").route(get().to(index)))
    //     .service(resource("/login").route(post().to(login)))
    //     .service(resource("/ws").route(get().to(ws_index)))
    for RouteSpec { route, http_method, handler_type, handler } in ROUTE_SPECS {
        app = app.service(resource(route).route(http_method, handler_type(handler)))
    }
}
