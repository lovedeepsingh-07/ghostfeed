use crate::{constants, error};
use axum::{
    response::IntoResponse,
    {extract, routing, http},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InstagramWebhookParams {
    // this will always be equal to "subscribe"
    hub_mode: String,
    // this must be verified upon receiving
    hub_verify_token: String,
    // this must be passed back to the instagram app dashboard upon request
    hub_challenge: i32,
}

pub async fn run() -> Result<(), error::Error> {
    let instagram_webhook_token = match std::env::var("INSTAGRAM_WEBHOOK_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'INSTAGRAM_WEBHOOK_TOKEN' environment variable".to_string(),
            ));
        }
    };

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
            |extract::Path(platform_id): extract::Path<String>| async move {
                tracing::info!("POST request to {}", platform_id);
                "HEALTHY".into_response()
            },
        ),
    );

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
