use crate::responses::responses::response_error_not_found;
use hyper::body::Bytes;
use hyper::{Body, Method, Response};
use sqlx::{Sqlite, Pool};
use std::error::Error;

use super::drinks::{get_all_drinks, insert_drink};

pub async fn route(path: String, method: Method, body: Bytes, pool: Pool<Sqlite>) -> Result<Response<Body>, Box<dyn Error>> {
    match (path.as_str(), method) {
        ("/drinks", Method::GET) => get_all_drinks(pool).map_err(|e| e.into()),
        ("/drinks", Method::POST) =>  {
            let result = insert_drink(body, pool).await;
            return result;
        },
        _ => {
            let response = response_error_not_found();
            Ok(response)
        }
    }
}
