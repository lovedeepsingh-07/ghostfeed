use crate::{command, constants, error};
use axum::{
    response::IntoResponse,
    {extract, http, routing},
};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InstagramWebhookParams {
    hub_mode: String,
    hub_verify_token: String,
    hub_challenge: i32,
}

#[axum::debug_handler]
async fn instagram_get(
    extract::State(command_tx): extract::State<Arc<mpsc::Sender<command::Command>>>,
    extract::Query(query_params): extract::Query<InstagramWebhookParams>,
) -> impl IntoResponse {
    let (response_tx, response_rx) = oneshot::channel::<bool>();
    if let Err(e) = command_tx
        .send(command::Command::VerifyInstagramWebhookToken(
            query_params.hub_verify_token.clone(),
            response_tx,
        ))
        .await
    {
        tracing::error!("Failed to send command in the channel, {}", e);
    }
    match response_rx.await {
        Ok(is_verified) => {
            if is_verified {
                tracing::info!("verified webhook connection");
                return (http::StatusCode::OK, query_params.hub_challenge.to_string())
                    .into_response();
            } else {
                tracing::info!("invalid verify token");
            }
        }
        Err(e) => {
            tracing::error!("Failed to receive response from orchestrator, {}", e);
        }
    }
    (http::StatusCode::OK, "").into_response()
}

#[axum::debug_handler]
async fn instagram_post(
    extract::State(command_tx): extract::State<Arc<mpsc::Sender<command::Command>>>,
    extract::Json(body): extract::Json<serde_json::Value>,
) -> impl IntoResponse {
    let _ = command_tx;
    let _ = body;
    // if let Err(e) = command_tx
    //     .send(command::Command::try_from(body).unwrap())
    //     .await
    // {
    //     tracing::error!("error sending command: {}", e);
    //     return (http::StatusCode::INTERNAL_SERVER_ERROR, "FAILED").into_response();
    // }
    (http::StatusCode::OK, "").into_response()
}

pub async fn run(command_tx: mpsc::Sender<command::Command>) -> Result<(), error::Error> {
    let router = axum::Router::new()
        .route(
            "/instagram",
            routing::get(instagram_get).post(instagram_post),
        )
        .route(
            "/health",
            routing::get(|| async move { (http::StatusCode::OK, "HEALTHY").into_response() }),
        )
        .with_state(Arc::new(command_tx));

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
