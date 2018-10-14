use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

pub fn init() {
    let addr = ([127, 0, 0, 1], 3000).into();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = || {
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function that
        // returns a Response into a `Service`.
        service_fn_ok(|_| {
            Response::new(Body::from("test"))
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    rt::run(server);
}
