struct ServiceHandler {
    routes: HashMap<String, Box<dyn Fn(Request<Body>) -> Result<Response<Body>, Infallible>>>,
}

impl ServiceHandler {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new("Hello World".into()))
    }
}
