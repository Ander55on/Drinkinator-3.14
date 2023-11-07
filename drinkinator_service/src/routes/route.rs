use core::hash::Hash;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
type Handler = fn(Request<Body>) -> Result<Response<Body>, Infallible>;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Route {
    path: String,
    method: Method,
}

pub struct Router {
    routes: HashMap<Route, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, route: Route, handler: Handler) {
        self.routes.insert(route, handler);
    }

    pub fn route(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route = Route::new(req.uri().path().to_string(), req.method().clone());
        match self.routes.get(&route) {
            Some(handler) => handler(req),
            None => {
                let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap();

                Ok(response)
            }
        }
    }
}

impl Route {
    pub fn new(path: String, method: Method) -> Self {
        Self { path, method }
    }
}
