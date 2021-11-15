mod actions;
mod types;

// use crate::types::CrustConfig;
use crate::actions::show_help;
use crate::actions::{add_commit_push, get_status};
use crate::types::Crust;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    arg: Option<String>,
}

fn main() {
    Crust::new(None);
    show_help();
    Crust::run_cmd(Cli::from_args());
}
