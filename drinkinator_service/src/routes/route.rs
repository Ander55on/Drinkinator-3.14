use crate::responses::responses::response_error_not_found;
use hyper::body::Bytes;
use hyper::{Body, Method, Response};
use std::error::Error;

use super::drinks::{get_drinks, post_drink};

pub fn route(path: String, method: Method, body: Bytes) -> Result<Response<Body>, Box<dyn Error>> {
    match (path.as_str(), method) {
        ("/drinks", Method::GET) => get_drinks(body).map_err(|e| e.into()),
        ("/drinks", Method::POST) => post_drink(body),
        _ => {
            let response = response_error_not_found();
            Ok(response)
        }
    }
}
