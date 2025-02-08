use clap::Parser;
use im_config::AppConfig;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct StateConfigOpts {}

impl CmdExector for StateConfigOpts {
    async fn execute(self, config: &AppConfig) -> anyhow::Result<()> {
        im_ipconfig::run_server(config).await;
        Ok(())
    }
}
