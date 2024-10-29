use anyhow::Result;
use clap::Parser;

pub const VERSION: &str = "0.0.1";

#[derive(Debug, Parser)]
#[clap(version = VERSION)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Initializes a workspace.
    Init {
        /// Workspace name
        name: String,
    },
}

pub fn entry(opts: Opts) -> Result<()> {
    println!("{:?}", opts);

    Ok(())
}
