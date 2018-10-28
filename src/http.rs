use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use futures::future;

/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn request_handler(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, _) => {
            *response.body_mut() = Body::from("Here will be a login page");
	}
	// The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

pub fn init() {
    let addr = ([127, 0, 0, 1], 3000).into();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let server = Server::bind(&addr)
        .serve(|| service_fn(request_handler))
        .map_err(|e| eprintln!("server error: {}", e));

    rt::run(server);
}
