use crate::models::models::PostDrink;
use crate::responses::responses::{response_created, response_error_invalid_json};
use hyper::body::Bytes;
use hyper::{Body, Response};
use std::convert::Infallible;
use std::error::Error;

pub fn get_drinks(_bytes: Bytes) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Getting drinks".into()))
}

pub fn post_drink(bytes: Bytes) -> Result<Response<Body>, Box<dyn Error>> {
    
    let body = String::from_utf8(bytes.to_vec())?;
    let result: Result<PostDrink, serde_json::Error> = serde_json::from_str(&body);

    let response = match result {
        Ok(_drink) => response_created(),
        Err(_e) => response_error_invalid_json(),
    };

    Ok(response)
}
