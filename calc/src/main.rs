use actix_web::{get, web, App, HttpServer, Responder};
// import funcitons from ./lib.rs

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(add_endpoint)
            .service(subtract_endpoint)
            .service(multiply_endpoint)
            .service(divide_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// actix-web endpoint for add function
#[get("/add/{a}/{b}")]
async fn add_endpoint(info: web::Path<(f64, f64)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    format!("Result: {}", result)
}

// actix-web endpoint for subtract function
#[get("/subtract/{a}/{b}")]
async fn subtract_endpoint(info: web::Path<(f64, f64)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    format!("Result: {}", result)
}

// actix-web endpoint for multiply function
#[get("/multiply/{a}/{b}")]
async fn multiply_endpoint(info: web::Path<(f64, f64)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    format!("Result: {}", result)
}

// actix-web endpoint for divide function
#[get("/divide/{a}/{b}")]
async fn divide_endpoint(info: web::Path<(f64, f64)>) -> impl Responder {
    let result = calc::divide(info.0, info.1);
    format!("Result: {:?}", result)
}
