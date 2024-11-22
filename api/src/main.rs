/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rand::seq::SliceRandom;
use std::env;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn fruit() -> impl Responder {
    let fruits = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry", "Fig", "Grape", "Honeydew", "Jackfruit", "Kiwi", "Lemon", "Mango", "Nectarine", "Orange", "Papaya", "Quince", "Raspberry", "Strawberry", "Tangerine", "Ugli", "Vineyard Peach", "Watermelon", "Xigua", "Yellow Passion Fruit", "Zucchini"];
    let fruit = fruits.choose(&mut rand::thread_rng()).unwrap();
    // return fruit as repsonse body
    HttpResponse::Ok().body(*fruit)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn version() -> impl Responder {
    let version = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().body(version)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/fruit", web::get().to(fruit))
            .route("/health", web::get().to(health))
            .route("/version", web::get().to(version))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}