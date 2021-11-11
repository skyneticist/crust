use crate::GitCommands::*;
use crate::RootCmd::*;
use ::std::error::Error;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    command: String,
    message: String,
}

enum RootCmd {
    Git,
}

impl RootCmd {
    pub fn value(&self) -> String {
        match *self {
            RootCmd::Git => String::from("git"),
        }
    }
}

impl RootCmd {
    pub fn as_ref(&self) -> &RootCmd {
        (*self).as_ref()
    }
}

#[derive(StructOpt)]
enum GitCommands {
    Add,
    Commit,
    Log,
    Push,
    Pull,
    Stash,
    Revert,
    Pop,
    Apply,
    Reset,
    Hard,
    Soft,
    Status,
}

impl GitCommands {
    pub fn value(&self) -> String {
        match *self {
            GitCommands::Add => String::from("add"),
            GitCommands::Commit => String::from("commit"),
            GitCommands::Log => String::from("log"),
            GitCommands::Push => String::from("push"),
            GitCommands::Pull => String::from("pull"),
            GitCommands::Stash => String::from("stash"),
            GitCommands::Reset => String::from("reset"),
            GitCommands::Revert => String::from("revert"),
            GitCommands::Pop => String::from("pop"),
            GitCommands::Apply => String::from("apply"),
            GitCommands::Hard => String::from("hard"),
            GitCommands::Soft => String::from("soft"),
            GitCommands::Status => String::from("status"),
        }
    }
}

impl GitCommands {
    pub fn as_ref(&self) -> &GitCommands {
        (*self).as_ref()
    }
}

fn main() {
    let args = Cli::from_args();

    // let msg: Result<String, Box<dyn Error>> = match args.message {
    //     Some(msg) => Ok(msg),
    //     None => Ok(String::from("")),
    // };
    // println!("{:?}", msg);

    let log_out = log_commits(true, false, None);
    println!("{}", log_out);

    let status_out = get_status();
    println!("{}", status_out);

    let acpr_out = add_commit_push(Some(true), args.message);
    println!("{}", acpr_out);
}

fn run_git_cmd(arg: GitCommands, sub_args: Option<Vec<String>>) -> String {
    let sub_args = match sub_args {
        Some(values) => values,
        None => Vec::new(),
    };

    let output = Command::new(Git.value())
        .arg(arg.value())
        .args(sub_args)
        .output()
        .expect("error at run_git_cmd");

    match String::from_utf8(output.stdout) {
        Ok(output_str) => output_str,
        Err(_) => String::from("Error has occurred at from_utf8 method"),
    }
}

fn add_commit_push(remote: Option<bool>, commit_msg: String) -> String {
    run_git_cmd(Add, Some(vec![String::from(".")]));

    let mut sub_args = Vec::new();
    sub_args.push(String::from("-m"));
    sub_args.push(commit_msg);
    run_git_cmd(Commit, Some(sub_args));

    let is_fresh = match remote {
        Some(value) => value,
        None => false,
    };

    if is_fresh {
        let mut remote_push_args = Vec::new();
        remote_push_args.push(String::from("-u"));
        remote_push_args.push(String::from("origin"));
        remote_push_args.push(String::from("HEAD"));

        return run_git_cmd(Push, Some(remote_push_args));
    }

    return String::from("");
}

fn get_status() -> String {
    return run_git_cmd(Status, None);
}

fn log_commits(pithy: bool, dump: bool, filter: Option<String>) -> String {
    let mut sub_cmd: Vec<String> = Vec::new();
    if pithy {
        &sub_cmd.push(String::from("--pretty=oneline"));
    }
    println!("{:?}", sub_cmd);
    return run_git_cmd(Log, Some(sub_cmd));
}
