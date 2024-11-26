/*An axum Microservice that has multiple routes:
A.  / that returns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use rand::seq::SliceRandom;
use std::env;
use tokio::main;

mod deck;

async fn hello() -> impl IntoResponse {
    "Hello World!"
}

async fn fruit() -> impl IntoResponse {
    let fruits = vec![
        "Apple", "Banana", "Cherry", "Date", "Elderberry", "Fig", "Grape", "Honeydew", "Jackfruit", "Kiwi",
        "Lemon", "Mango", "Nectarine", "Orange", "Papaya", "Quince", "Raspberry", "Strawberry", "Tangerine",
        "Ugli", "Vineyard Peach", "Watermelon", "Xigua", "Yellow Passion Fruit", "Zucchini"
    ];
    let fruit = fruits.choose(&mut rand::thread_rng()).unwrap();
    (*fruit).to_string()
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn version() -> impl IntoResponse {
    env!("CARGO_PKG_VERSION").to_string()
}

async fn deck() -> impl IntoResponse {
    let deck = deck::Deck::new();
    Json(deck).into_response()
}

#[main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(hello))
        .route("/fruit", get(fruit))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/deck", get(deck));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}