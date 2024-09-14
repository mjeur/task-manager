use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::FromStr;
use http::{Request, Response, StatusCode, Method, uri::Uri};
use httparse;

// Handler Trait
pub trait Handler {
    fn handle(&self, request: Request<()>) -> Response<String>;
}

// Middleware Trait
pub trait Middleware {
    fn handle(&self, request: &mut Request<()>);
}

// Router struct with routing logic
pub struct Router {
    routes: Vec<(Method, String, Box<dyn Handler>)>,
}

impl Router {
    fn new() -> Self {
        Router { routes: Vec::new() }
    }

    fn add_route(&mut self, method: Method, path: String, handler: Box<dyn Handler>) {
        self.routes.push((method, path, handler));
    }

    fn route(&self, request: Request<()>) -> Response<String> {
        let path = request.uri().path().to_string();
        let method = request.method().clone();
        
        for (route_method, route_path, handler) in &self.routes {
            if &path == route_path && method == *route_method {
                return handler.handle(request);
            }
        }
        
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".to_string())
            .unwrap()
    }
}

// Server struct
pub struct Server {
    listener: TcpListener,
    router: Router,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let router = Router::new();
        let middlewares = Vec::new();

        Server { listener, router, middlewares }
    }

    pub fn add_middleware(&mut self, mw: Box<dyn Middleware>) {
        self.middlewares.push(mw);
    }

    pub fn add_route(&mut self, method: Method, path: String, handler: Box<dyn Handler>) {
        self.router.add_route(method, path, handler);
    }

    fn parse_request(&self, buffer: &[u8]) -> Request<()> {
        // Parse the HTTP request using httparse
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        req.parse(buffer).unwrap();

        let method = Method::from_str(req.method.unwrap()).unwrap();
        let uri: Uri = req.path.unwrap().parse().unwrap();

        Request::builder()
            .method(method)
            .uri(uri)
            .body(())
            .unwrap()
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).unwrap();
                    
                    let mut request = self.parse_request(&buffer);
                    
                    for middleware in &self.middlewares {
                        middleware.handle(&mut request);
                    }

                    let response = self.router.route(request);
                    
                    let response_string = format!(
                        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
                        response.status(),
                        response.status().canonical_reason().unwrap(),
                        response.body().len(),
                        response.body()
                    );

                    stream.write(response_string.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

// Example handler
pub struct HelloWorldHandler;

impl Handler for HelloWorldHandler {
    fn handle(&self, _request: Request<()>) -> Response<String> {
        Response::builder()
            .status(StatusCode::OK)
            .body("Hello, World!".to_string())
            .unwrap()
    }
}

// Example middleware
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut Request<()>) {
        println!("Received request: {} {}", request.method(), request.uri().path());
    }
}


