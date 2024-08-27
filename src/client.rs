use hyper_tls::{native_tls::TlsConnector, HttpsConnector};
use warp::hyper::{body::Body, client::connect::HttpConnector, Client};

pub type HttpsClient = Client<HttpsConnector<HttpConnector>, Body>;

pub fn https_client(skip_ssl_verify: bool) -> HttpsClient {
    let mut tls_builder = TlsConnector::builder();
    let tls_builder = tls_builder.danger_accept_invalid_certs(skip_ssl_verify);
    let tls_builder = tls_builder.danger_accept_invalid_hostnames(skip_ssl_verify);

    let tls = tls_builder.build().unwrap();

    let mut http = HttpConnector::new();
    http.enforce_http(false);
    let https = HttpsConnector::from((http, tls.into()));

    Client::builder().build(https)
}
