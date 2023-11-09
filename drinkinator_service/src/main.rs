use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use responses::responses::response_error_server_error;
use std::convert::Infallible;
use std::net::SocketAddr;

mod models;
mod responses;
mod routes;
use routes::route::route;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = String::from(req.uri().path());
    let method = req.method().clone();
    let body = req.into_body();

    let body_bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: {e}");
            return Ok(response_error_server_error())
        }
    };

    let response = route(path, method, body_bytes)
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        response_error_server_error()
    });

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| async {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req)))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
