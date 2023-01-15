use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, future::Future};
use tracing::{info, debug, error};

pub async fn listen() {
    info!("starting server");
    let app = Router::new()
        .route("/", get(status_api))
        .route("/admin", post(admin));
        // or
        // .route("/stop", post(admin));
        // .route("/start", post(admin));
        // .route("/send", post(admin));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn admin() -> String {
    "get post request with command, authorize master and execute command if needed\n".to_string()
}

async fn status_api() -> String {
    format!("status post request {}\n", "not working yet")
}

pub async fn status() -> String {
    "DB status\n".to_string()
}