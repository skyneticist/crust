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

pub fn get_status() -> String {
    return run_git_cmd(Status, None);
}

pub fn log_commits(pithy: String, dump: String, filter: Option<String>) -> String {

    if dump == "save" {
        println!("{}", String::from("This be a save"));
    };
    let f = match filter {
        Some(filter_val) => filter_val,
        None => String::from(""), 
    };

    if f != String::from("") {
        println!("filter was applied!");
    };

    let mut sub_cmd: Vec<String> = Vec::new();
    if pithy == "ol" {
        &sub_cmd.push(String::from("--pretty=oneline"));
    }
    return run_git_cmd(Log, Some(sub_cmd));
}
