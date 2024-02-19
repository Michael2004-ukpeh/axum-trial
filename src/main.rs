use std::net::AddrParseError;
use serde::Deserialize;
use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use axum::response::{Html, IntoResponse};
#[tokio::main]
async fn main() {
    let routes_hello=  Router::new().route("/hello", get(handler_hello));
     
     let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
     println!("->> Listening on 5000 \n");
     axum::serve(listener, routes_hello).await.unwrap();
}


async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
    println!("->> {:<12}  -  {params:?} handler_hello", "HANDLER");

    let name  =  params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong> {name}</strong> "))

}

#[derive(Debug, Deserialize)]
struct HelloParams{
    name:Option<String>
}