use std::sync::Arc;

use tracing::{error, info};
use url::Url;
use warp::hyper::{body::Body, Request};
use warp::{http::Response, Rejection};

use crate::client::HttpsClient;
use crate::original_request::OriginalRequest;

pub async fn proxy_request(
    original_request: OriginalRequest,
    client: Arc<HttpsClient>,
    target_host: Arc<Url>,
    rewrite_host: bool,
) -> Result<Response<Body>, Rejection> {
    info!(
        "[{}] {}{}",
        &original_request.method.as_str(),
        &original_request.path.as_str(),
        &original_request.query_string()
    );

    let request = build_request(original_request, &target_host, rewrite_host);

    match client.request(request).await {
        Ok(proxy_response) => {
            let proxy_status = proxy_response.status();
            let proxy_headers = proxy_response.headers().clone();
            let proxy_body = proxy_response.into_body();

            let mut response = Response::new(proxy_body);
            *response.status_mut() = proxy_status;
            *response.headers_mut() = proxy_headers;
            info!(" => {}", proxy_status);

            Ok(response)
        }
        Err(e) => {
            error!("FAILED: proxy server unavailable");
            error!("{:?}", e);
            Ok(Response::builder()
                .status(503)
                .body("proxy target unavailable".into())
                .unwrap())
        }
    }
}

fn build_request(
    original_request: OriginalRequest,
    target_host: &Url,
    rewrite_host: bool,
) -> Request<Body> {
    let location = format!(
        "{}{}{}",
        target_host,
        original_request.path.as_str(),
        original_request.query_string()
    );

    let mut request = Request::new(Body::from(original_request.body));
    *request.method_mut() = original_request.method;
    *request.uri_mut() = location.parse().expect("invalid uri");
    *request.headers_mut() = original_request.headers;
    if rewrite_host {
        if let Some(v) = request.headers_mut().get_mut("HOST") {
            *v = target_host.host_str().unwrap().parse().unwrap();
        }
    }

    request
}
