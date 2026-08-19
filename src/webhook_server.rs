use crate::{constants, error, state};
use axum::{
    response::IntoResponse,
    {extract, http, routing},
};
use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InstagramWebhookParams {
    hub_mode: String,
    hub_verify_token: String,
    hub_challenge: i32,
}

#[axum::debug_handler]
async fn instagram_get(
    extract::State(state): extract::State<Arc<state::State>>,
    extract::Query(query_params): extract::Query<InstagramWebhookParams>,
) -> impl IntoResponse {
    if state
        .engine
        .is_webhook_token_valid(&query_params.hub_verify_token)
    {
        return (http::StatusCode::OK, query_params.hub_challenge.to_string()).into_response();
    } else {
        tracing::info!("Invalid instagram webhook token");
    }
    (http::StatusCode::OK, "").into_response()
}

#[axum::debug_handler]
async fn instagram_post(
    extract::State(state): extract::State<Arc<state::State>>,
    extract::Json(body): extract::Json<serde_json::Value>,
) -> impl IntoResponse {
    let _ = state;
    let _ = body;
    (http::StatusCode::OK, "").into_response()
}

pub async fn run(state: Arc<state::State>) -> Result<(), error::Error> {
    let router = axum::Router::new()
        .route(
            "/instagram",
            routing::get(instagram_get).post(instagram_post),
        )
        .route(
            "/health",
            routing::get(|| async move { (http::StatusCode::OK, "HEALTHY").into_response() }),
        )
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, constants::SERVER_PORT)).await?;

    tracing::info!(
        "ghostfeed webhook server started on {}:{}",
        constants::SERVER_ADDRESS,
        constants::SERVER_PORT
    );
    axum::serve(listener, router).await?;

    Ok(())
}
