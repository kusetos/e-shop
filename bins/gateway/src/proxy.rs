use axum::{
    body::Body,
    extract::Request,
    http::{HeaderName, StatusCode},
    response::{IntoResponse, Response},
};
use reqwest::Client;

use crate::auth::Claims;

pub async fn forward(
    client: &Client,
    target_base: &str,
    req: Request,
    claims: Option<&Claims>,
) -> Response {
    let method  = req.method().clone();
    let headers = req.headers().clone();
    let uri     = req.uri().clone();

    let path_and_query = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");

    let target_url = format!("{}{}", target_base, path_and_query);

    let body_bytes = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
        Ok(b)  => b,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut builder = client.request(method, &target_url);

    for (name, value) in &headers {
        if !is_hop_by_hop(name) {
            builder = builder.header(name, value);
        }
    }

    if let Some(c) = claims {
        builder = builder.header("x-user-id", c.sub.to_string());
    }

    builder = builder.body(body_bytes.to_vec());

    match builder.send().await {
        Ok(resp) => convert_response(resp).await,
        Err(e) => {
            tracing::error!("upstream error: {e}");
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

async fn convert_response(resp: reqwest::Response) -> Response {
    let status = StatusCode::from_u16(resp.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let headers = resp.headers().clone();

    let body = match resp.bytes().await {
        Ok(b)  => b,
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
    };

    let mut builder = Response::builder().status(status);

    for (name, value) in &headers {
        if !is_hop_by_hop(name) {
            builder = builder.header(name, value);
        }
    }

    builder
        .body(Body::from(body))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "connection"
            | "keep-alive"
            | "transfer-encoding"
            | "te"
            | "trailer"
            | "upgrade"
            | "proxy-authorization"
            | "host"
    )
}
