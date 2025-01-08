use anyhow::Result;
use clap::Parser;

mod cli;
pub use cli::*;
use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use fastrace::prelude::*;
use im_config::AppConfig;
use logforth::append;
use logforth::filter::EnvFilter;

#[trace]
fn plus(a: u64, b: u64) -> Result<u64, std::io::Error> {
    Ok(a + b)
}

#[tokio::main]
async fn main() -> Result<()> {
    fastrace::set_reporter(ConsoleReporter, Config::default());
    // Set up a custom logger. [`logforth`](https://github.com/fast/logforth)
    // is easy to start and integrated with `fastrace`.
    logforth::builder()
        .dispatch(|d| {
            d.filter(EnvFilter::from_default_env())
                .append(append::Stderr::default())
        })
        .dispatch(|d| d.append(append::FastraceEvent::default()))
        .apply();

    let opts = Opts::parse();
    let config = AppConfig::load(opts.config).expect("Failed to load config file");
    opts.cmd.execute(&config).await?;
    Ok(())
}
