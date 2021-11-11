use crate::types::RootCmd::Git;
use crate::types::GitCommands::{Add, Commit, Log, Push, Status};
use crate::types::*;
use std::process::Command;

pub fn run_git_cmd(arg: GitCommands, sub_args: Option<Vec<String>>) -> String {
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

pub fn add_commit_push(remote: Option<bool>, commit_msg: String) -> String {
    run_git_cmd(Add, Some(vec![String::from(".")]));

    let sub_args = vec![String::from("-m"), commit_msg];
    run_git_cmd(Commit, Some(sub_args));

    let is_fresh = remote.unwrap_or(false);
    let remote_push_args = match is_fresh {
        true => vec![String::from("-u"), String::from("origin"), String::from("HEAD")],
        false => vec![],
    };

    return run_git_cmd(Push, Some(remote_push_args));
}

pub fn get_status() -> String {
    return run_git_cmd(Status, None);
}

pub fn log_commits(pithy: bool, _dump: bool, _filter: Option<String>) -> String {
    let mut sub_cmd: Vec<String> = Vec::new();
    if pithy {
        sub_cmd.push(String::from("--pretty=oneline"));
    }
    return run_git_cmd(Log, Some(sub_cmd));
}
