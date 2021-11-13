use crate::types::GitCommands::{Add, Branch, Commit, Log, Push, Reset, Status};
use crate::types::RootCmd::{Git, Grep};
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

pub fn add_commit_push(commit_msg: String) -> String {
    // run_git_cmd(Add, Some(vec![String::from(".")]));

    let sub_args = vec![String::from("-m"), commit_msg];
    // run_git_cmd(Commit, Some(sub_args));

    let br = get_branch();
    let br_exists = check_remote_exists(br);
    // println!("{}", br_exists);
    let remote_push_args = match br_exists {
        true => vec![],
        false => vec![
            String::from("-u"),
            String::from("origin"),
            String::from("HEAD"),
        ],
    };
    // println!("{:?}", remote_push_args);
    remote_push_args.join(", ")
    // run_git_cmd(Push, Some(remote_push_args))
}

pub fn get_status() -> String {
    run_git_cmd(Status, None)
}

pub fn log_commits(pithy: String) -> String {
    let mut sub_cmd: Vec<String> = vec![];
    if pithy == "ol" {
        sub_cmd.push(String::from("--pretty=oneline"));
    }
    run_git_cmd(Log, Some(sub_cmd))
}

pub fn reset_branch(density: String) -> String {
    run_git_cmd(Reset, Some(vec![density]))
}

pub fn check_remote_exists(branch: String) -> bool {
    let br_copy = branch.clone(); 
    println!("{}", br_copy);
    let empty_string = String::from("");
    let is_new_remote = match run_git_cmd(
        Branch,
        Some(vec![
            String::from("-r"),
            String::from("--contains"),
            branch,
            // String::from("|"),
            // Grep.value(),
            // String::from("-w"),
            // branch,
        ])){  
            x if x == empty_string => true,
            _ => false,
        };  
    println!("{:?}", is_new_remote);
    is_new_remote
}

pub fn get_branch() -> String {
    let args = vec![String::from("--show-current")];
    run_git_cmd(Branch, Some(args))
    // println!("{}", String::from(result))
}

// gm
