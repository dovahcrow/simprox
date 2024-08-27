use warp::hyper::body::Bytes;
use warp::{
    http::{method::Method, HeaderMap},
    path::FullPath,
};

pub struct OriginalRequest {
    pub method: Method,
    pub path: FullPath,
    pub query: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl OriginalRequest {
    pub fn new(
        method: Method,
        path: FullPath,
        query: String,
        headers: HeaderMap,
        body: Bytes,
    ) -> Self {
        OriginalRequest {
            method,
            path,
            query,
            headers,
            body,
        }
    }

    pub fn query_string(&self) -> String {
        if self.query.is_empty() {
            String::default()
        } else {
            format!("?{}", self.query)
        }
    }
}
