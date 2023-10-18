use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>
}

impl FnPointerRouter {
    fn new() -> FnPointerRouter {
        FnPointerRouter { routes: HashMap::new() }
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response)
    {
        self.routes.insert(url.to_string(), callback);
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            Some(callback) => callback(request),
            None => not_found_response(),
        }
    }
} 

fn not_found_response() -> Response {
    panic!("404");
}

fn main() {
    let mut router = FnPointerRouter::new();
    router.add_route("/", |req| panic!("good job!"));
}
