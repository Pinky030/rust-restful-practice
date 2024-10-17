mod model;
mod response;
mod handler;
mod db;

use std::sync::Arc;

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let (client, connection) = db::connect_to_db().await.unwrap();
    let client = Arc::new(client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    db::create_category_table(&client).await.unwrap();
    db::create_budget_table(&client).await.unwrap();

    let app = Router::new()
    .route("/category", get(handler::get_category_list).post(handler::create_category))
    .route("/budget", get(handler::get_budget_list).post(handler::create_budget))
    .route("/budget/:id", get(handler::get_single_budget).patch(handler::update_budget).delete(handler::remove_budget))
    .route("/budget/sum", get(handler::get_amount))
    .layer(Extension(client))
    .layer(cors);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
