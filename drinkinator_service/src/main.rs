use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

mod routes;
use routes::route::{Route, Router};

async fn handle_request(
    req: Request<Body>,
    router: Arc<Router>,
) -> Result<Response<Body>, Infallible> {
    let test = router.as_ref().route(req);
    match test {
        Ok(response) => Ok(response),
        Err(_e) => {
            let response = Response::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap();
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let mut router = Router::new();
    let route = Route::new("/drinks".to_string(), Method::GET);

    router.add_route(route, routes::drinks::get_drinks);

    let router = Arc::new(router);

    let make_svc = make_service_fn(move |_conn: &hyper::server::conn::AddrStream| {
        let router = Arc::clone(&router);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, Arc::clone(&router))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
