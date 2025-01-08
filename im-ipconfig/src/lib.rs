mod domain;
mod source;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use domain::DP;
use im_config::AppConfig;

pub async fn run_server(config: &AppConfig) {
    source::init(config.discovery.clone());
    if config.global.env == "debug" {
        source::mock::start_mock();
    }
    domain::init();

    let app = Router::new()
        .route("/ips", get(get_ips))
        .route("/healthy", get(healthy));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn healthy() -> &'static str {
    "Ok"
}

async fn get_ips() -> impl IntoResponse {
    let eds = {
        let mut dp = DP.get().unwrap().lock().await;
        dp.dispatch().await
    };

    (StatusCode::OK, Json(eds))
}
