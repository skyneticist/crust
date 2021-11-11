mod actions;
mod types;

// use crate::types::CrustConfig;
use crate::actions::*;
use crate::types::Crust;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    message: String,
}

fn main() {
    // Crust::new(Some(crusty_config));
    Crust::run_cmd(Cli::from_args());
}
