use hyper::{Body, Response, StatusCode};

pub fn response_error_invalid_json() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Invalid JSON data"))
        .expect("Failed to create response body")
}

pub fn response_error_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal server error"))
        .expect("Failed to create response body")
}

pub fn response_error_not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .expect("Failed to create response body")
}

pub fn response_created() -> Response<Body> {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("Created"))
        .expect("Failed to create response body")
}
