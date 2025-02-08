mod ipconfig;
mod state;
use std::path::PathBuf;

use clap::Parser;
use state::StateConfigOpts;

use crate::AppConfig;

use self::ipconfig::IpConfigOpts;
use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self, config: &AppConfig) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(name = "im", version, author, about, long_about = None)]
pub struct Opts {
    #[arg(short, long, default_value = "./config.yaml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "ipconfig")]
    IpConfig(IpConfigOpts),
    #[command(name = "state")]
    State(StateConfigOpts),
}
