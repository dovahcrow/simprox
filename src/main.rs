mod client;
mod logger;
mod original_request;
mod proxy;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Error};
use clap::Parser;
use culpa::throws;
use tracing::info;
use url::Url;
use warp::Filter;

use crate::{
    client::https_client, logger::setup_logger, original_request::OriginalRequest,
    proxy::proxy_request,
};

#[derive(Debug, Parser)]
#[command(version, about = "Simple proxy server", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        env,
        help = "Set the host for the proxy server itself",
        default_value = "127.0.0.1:7000"
    )]
    listen: SocketAddr,

    #[arg(short, long, env, help = "Sets the proxy target (required)")]
    target_host: Url,

    #[arg(long, env, help = "Disable ssl certificate verification")]
    skip_ssl_verify: bool,

    #[arg(long, env, help = "Whether to rewrite the host to target_host")]
    rewrite_host: bool,
}

#[throws(Error)]
#[tokio::main]
async fn main() {
    setup_logger()?;

    ctrlc::set_handler(|| {
        info!("Stopping simprox...");
        std::process::exit(0);
    })
    .with_context(|| "Error setting exit handler")?;

    let mut cli = Cli::parse();

    info!("Listening on: {}", cli.listen);
    info!("Proxy target: {}", cli.target_host);
    info!("Skip ssl verify: {}", cli.skip_ssl_verify);
    info!("Rewrite host: {}", cli.rewrite_host);

    cli.target_host.set_path("");
    cli.target_host.set_query(None);

    let target_host = Arc::new(cli.target_host);
    let rewrite_host = cli.rewrite_host;

    let client = Arc::new(https_client(cli.skip_ssl_verify));
    let routes = warp::method()
        .and(warp::path::full())
        .and({
            warp::filters::query::raw()
                .or(warp::any().map(String::default))
                .unify()
        })
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .map(OriginalRequest::new)
        .and(warp::any().map(move || client.clone()))
        .and(warp::any().map(move || target_host.clone()))
        .and(warp::any().map(move || rewrite_host))
        .and_then(proxy_request)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(cli.listen).await;
}
