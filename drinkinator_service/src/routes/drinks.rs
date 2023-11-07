use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub fn get_drinks(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Getting drinks".into()))
}
