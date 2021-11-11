mod actions;
mod types;

use crate::types::Crust;
use crate::actions::*;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    message: String,
}

fn run_cmd(args: Cli) {
    let _acp_cmd = String::from("acp");
    let _log = String::from("log");
    let _status = String::from("status");
    let output = match args.command {
        x if x == _acp_cmd => add_commit_push(Some(true), args.message),
        x if x == _status => get_status(),
        x if x == _log => log_commits(true, false, None),
        _ => String::from("command not found"),
    };
    println!("{}", String::from(output));
}

fn main() {
    let crust = Crust::new(None);
    Crust::run_cmd(Cli::from_args());
    run_cmd(Cli::from_args());
}
