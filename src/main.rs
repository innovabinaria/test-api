use std::{net::SocketAddr, convert::Infallible};
use hyper::{Server, Request, Response, StatusCode, Body};
use routerify::{RouterService, Router, RequestInfo};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Penguin {
    common_name: String,
    length_cm: f32,
}


async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world!")))
}

async fn get_name_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Name: Victor Aguayo")))
}


async fn get_penguin_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data
        ";

    /*let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
    
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}",
                record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
*/

    let mut penguins = Vec::new();

    for (i, record) in penguin_data.lines().enumerate() {
        if i == 0 || record.trim().is_empty() {
            continue;
        }
    
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if fields.len() == 2 {
            if let Ok(length) = fields[1].parse::<f32>() {
                penguins.push(Penguin {
                    common_name: fields[0].to_string(),
                    length_cm: length,
                });
            }
        }
    }

    let json = serde_json::to_string(&penguins).unwrap_or_else(|_| "[]".to_string());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap())
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
        .get("/name", get_name_handler)  
        .get("/penguin", get_penguin_handler)  
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