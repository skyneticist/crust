mod actions;
mod types;

// use crate::types::CrustConfig;
use crate::actions::{add_commit_push, get_status, log_commits};
use crate::types::Crust;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    arg1: Option<String>,
    arg2: Option<String>,
    arg3: Option<String>,
}

fn main() {
    // Crust::new(Some(crusty_config));
    Crust::run_cmd(Cli::from_args());
}
