pub mod routing;

use actix_web::{middleware, App, HttpServer};
use actix_http::{body::MessageBody, Error, Request, Response};
use actix_server_config::ServerConfig;
use actix_service::{IntoNewService, NewService};

pub fn server<F, I, S, B>() -> HttpServer<F, I, S, B>
    where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoNewService<S>,
    S: NewService<Config = ServerConfig, Request = Request>,
    S::Error: Into<Error>,
    S::InitError: std::fmt::Debug,
    S::Response: Into<Response<B>>,
    S::Service: 'static,
    B: MessageBody + 'static,
{
    let server = HttpServer::new(|| {
        let app = App::new()
            // enable logger
            .wrap(middleware::Logger::default());
        app = routing::build_routes(app);
        app
    });
    server
}
