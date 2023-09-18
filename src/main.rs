mod error;
mod model;
mod web;

use crate::model::TicketService;

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let ticket_service = TicketService::new().await?;

    let routes_api = web::tickets::routes(ticket_service)
        .route_layer(middleware::from_fn(web::middleware::require_auth));

    let routes = Router::new()
        .merge(routes_hello())
        .merge(web::login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = "127.0.0.1:8080"
        .parse()
        .expect("failed to parse socket address");
    println!("->> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("failed to start server");

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RESPONSE_MAPPER");

    println!();

    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}!!</strong>"))
}
