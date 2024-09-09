use std::{net::SocketAddr, convert::Infallible};
use hyper::{Server, Request, Response, StatusCode, Body};
use routerify::{RouterService, Router, RequestInfo};

async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world!")))
}

async fn get_name_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Name: Victor Aguayo")))
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .get("/", home_handler)
        .get("/name", get_name_handler)  // Nueva ruta para "/nombre"
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let server = Server::bind(&addr).serve(service);

    println!("Server is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
   }
}