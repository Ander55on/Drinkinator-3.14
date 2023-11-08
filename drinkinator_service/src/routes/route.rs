use hyper::body::Bytes;
use hyper::{Body, Method, Response, StatusCode};
use std::convert::Infallible;

use super::drinks::{get_drinks, post_drink};

pub fn route(path: String, method: Method, body: Bytes) -> Result<Response<Body>, Infallible> {
    match (path.as_str(), method) {
        ("/drinks", Method::GET) => get_drinks(body),
        ("/drinks", Method::POST) => post_drink(body),
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap();
            Ok(response)
        }
    }
}
