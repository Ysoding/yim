use clap::Parser;
use im_config::AppConfig;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct IpConfigOpts {}

impl CmdExector for IpConfigOpts {
    async fn execute(self, config: &AppConfig) -> anyhow::Result<()> {
        im_ipconfig::run_server(config).await;
        Ok(())
    }
}
