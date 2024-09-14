mod server;
use server::server::{Server, Handler, Middleware};
use http::{Request, Response, StatusCode, Method, uri::Uri};
mod config;
use config::config::Config;


struct HelloHandler;

impl Handler for HelloHandler {
    fn handle(&self, _request: Request<()>) -> Response<String> {
        Response::builder()
            .status(StatusCode::OK)
            .body("Hello, World!".to_string())
            .unwrap()
    }
}

struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut Request<()>) {
        println!("Received request: {} {}", request.method(), request.uri().path());
    }
}

fn main() {
    let config = Config::new("src/config.json").unwrap();
    println!("Server host: {}", config.server.host);
    println!("Server port: {}", config.server.port);
    println!("Database URL: {}", config.database.url);
    println!("Database username: {}", config.database.username);
    println!("Database password: {}", config.database.password);

    let mut server = Server::new("127.0.0.1:8080");
    server.add_route(Method::GET, "/hello".to_string(), Box::new(HelloHandler));
    server.add_middleware(Box::new(LoggingMiddleware));
    //server.start();
}