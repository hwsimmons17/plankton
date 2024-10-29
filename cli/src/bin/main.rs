use anyhow::Result;
use clap::Parser;
use plankton_cli::Opts;

fn main() -> Result<()> {
    plankton_cli::entry(Opts::parse())
}
