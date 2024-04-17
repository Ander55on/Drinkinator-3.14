use crate::models::models::{ModelOperationError, PostDrink};
use crate::responses::responses::response_error_not_found;
use hyper::body::{self, Bytes};
use hyper::{Body, Method, Response, StatusCode};
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use super::drinks::{get_all_drinks, insert_drink};

impl From<ModelOperationError> for Response<Body> {
    fn from(error: ModelOperationError) -> Self {
        match error {
            ModelOperationError::InternalError => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Error"))
                .unwrap(),
            ModelOperationError::NotFound => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Not Found"))
                .unwrap(),
        }
    }
}

fn new_response<T: Serialize>(data: T, status: StatusCode) -> Response<Body> {
    let bytes = serde_json::to_vec(&data).unwrap();
    Response::builder()
        .status(status)
        .body(Body::from(bytes))
        .unwrap()
}

pub async fn route(
    path: String,
    method: Method,
    body: Bytes,
    pool: Pool<Sqlite>,
) -> Response<Body> {
    match (path.as_str(), method) {
        ("/drinks", Method::GET) => get_all_drinks(pool).unwrap(),
        ("/drinks", Method::POST) => {
            let drink = match serde_json::from_slice(&body) {
                Ok(drink) => drink,
                Err(e) => {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Bad Request"))
                        .unwrap()
                }
            };
            let result = insert_drink(drink, pool).await;

            match result {
                Ok(data) => new_response(data, StatusCode::CREATED),
                Err(e) => Response::from(e),
            }
        }
        _ => response_error_not_found(),
    }
}
