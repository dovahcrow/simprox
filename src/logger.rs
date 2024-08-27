use anyhow::Error;
use culpa::throws;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
    Registry,
};

#[throws(Error)]
pub fn setup_logger() {
    tracing::subscriber::set_global_default(
        Registry::default()
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_filter(LevelFilter::INFO),
            )
            .with(EnvFilter::builder().parse(format!("simprox=info")).unwrap()),
    )?;
}
