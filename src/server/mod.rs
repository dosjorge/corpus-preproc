use std::str::FromStr;
use std::net::SocketAddr;

use crate::pipeline::{run as clean_text, PreprocOpts};

use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::StatusCode,
    routing::post,
    Router,
};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Serve {
    #[clap(parse(try_from_str = SocketAddr::from_str))]
    address: SocketAddr,
}

#[tokio::main]
pub async fn run(opts: Serve) {
    let app = Router::new()
        .route("/preproc", post(accept_form))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    tracing::info!("listening on {}", opts.address);

    axum::Server::bind(&opts.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { 250 * 1024 * 1024 }>,
) -> Result<String, StatusCode> {
    let mut text: String = "".to_string();
    let mut options = PreprocOpts::default();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return Err(StatusCode::BAD_REQUEST),
        };
        let content_type = match field.content_type() {
            Some(content_type) => content_type.to_string(),
            None => return Err(StatusCode::BAD_REQUEST),
        };
        let data = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Err(StatusCode::UNPROCESSABLE_ENTITY),
        };
        match name.as_str() {
            "config" => {
                if content_type == "application/json" {
                    options = match serde_json::from_slice::<PreprocOpts>(&data) {
                        Ok(opts) => opts,
                        Err(e) => {
                            tracing::debug!("Error while parsing options: `{}`", e.to_string());
                            return Err(StatusCode::BAD_REQUEST);
                        }
                    };
                } else {
                    tracing::debug!("Unsupported content type `{content_type}`");
                    return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
                }
            }
            "data" => {
                if content_type.starts_with("text") {
                    text.push_str(&clean_text(options.clone(), &data));
                } else {
                    tracing::debug!("Unsupported content type `{content_type}`");
                    return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
                }
            }
            _ => {
                tracing::debug!("Unexpected field name `{name}` in multipart form");
                return Err(StatusCode::BAD_REQUEST);
            }
        };
    }
    if text.is_empty() {
        tracing::debug!("No text received");
        Err(StatusCode::NO_CONTENT)
    } else {
        Ok(text)
    }
}
