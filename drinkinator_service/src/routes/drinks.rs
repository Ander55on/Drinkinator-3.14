use hyper::{Body, Response};
use std::convert::Infallible;
use hyper::body::Bytes;

pub fn get_drinks(_bytes: Bytes) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Getting drinks".into()))
}

pub fn post_drink(bytes: Bytes) -> Result<Response<Body>, Infallible> {
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    Ok(Response::new(body.into()))
}
