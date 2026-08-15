use crate::{command, constants, error};
use axum::{
    response::IntoResponse,
    {extract, http, routing},
};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InstagramWebhookParams {
    hub_mode: String,
    hub_verify_token: String,
    hub_challenge: i32,
}

pub async fn run(command_tx: mpsc::Sender<command::Command>) -> Result<(), error::Error> {
    let instagram_webhook_token = match std::env::var("INSTAGRAM_WEBHOOK_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'INSTAGRAM_WEBHOOK_TOKEN' environment variable"
                    .to_string(),
            ));
        }
    };

    let server_state = Arc::new(command_tx);

    let router = axum::Router::new().route(
        "/{platform}",
        routing::get(
            |extract::Path(platform_id): extract::Path<String>,
             extract::Query(query_params): extract::Query<InstagramWebhookParams>| async move {
                if platform_id != "instagram" {
                    return (http::StatusCode::METHOD_NOT_ALLOWED, "NOT SUPPORTED").into_response();
                }
                if query_params.hub_verify_token == instagram_webhook_token {
                    tracing::info!("shit is verified");
                    return (http::StatusCode::OK, query_params.hub_challenge.to_string())
                        .into_response();
                }
                tracing::info!(
                    "GET request to {} with query params: {:#?}",
                    platform_id,
                    query_params
                );
                (http::StatusCode::OK, "HEALTHY").into_response()
            },
        )
        .post(
            |extract::State(server_state): extract::State<Arc<mpsc::Sender<command::Command>>>,
            extract::Path(platform_id): extract::Path<String>,
             extract::Json(body): extract::Json<serde_json::Value>| async move {
                if platform_id != "instagram" {
                    return (http::StatusCode::METHOD_NOT_ALLOWED, "NOT SUPPORTED").into_response();
                }
                if let Err(e) = server_state.send(command::Command::try_from(body).unwrap()).await {
                    tracing::error!("error sending command: {}", e);
                    return (http::StatusCode::INTERNAL_SERVER_ERROR, "FAILED").into_response();
                }
                (http::StatusCode::OK, "HEALTHY").into_response()
            },
        ),
    ).with_state(server_state);

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
