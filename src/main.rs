use anyhow::Result;
use clap::Parser;

mod cli;
pub use cli::*;
use im_config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();
    let opts = Opts::parse();
    let config = AppConfig::load(opts.config).expect("Failed to load config file");
    opts.cmd.execute(&config).await?;
    Ok(())
}
