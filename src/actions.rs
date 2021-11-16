use crate::types::GitCommands::{Add, Branch, Checkout, Commit, Log, Push, Reset, Status};
use crate::types::RootCmd::{Git, Grep};
use crate::types::{GitCommands, HelpInfo};
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
    run_git_cmd(Add, Some(vec![String::from(".")]));
    run_git_cmd(Commit, Some(vec![String::from("-m"), commit_msg]));

    let remote_push_args = match check_new_branch(get_branch()) {
        true => vec![
            String::from("-u"),
            String::from("origin"),
            String::from("HEAD"),
        ],
        false => vec![],
    };
    run_git_cmd(Push, Some(remote_push_args))
}

pub fn get_status() -> String {
    run_git_cmd(Status, None)
}

pub fn log_commits(pithy: String) -> String {
    let sub_cmd = match pithy {
        x if x == "ol" || x == "oneline" => {
            vec![String::from("--pretty=oneline")]
        }
        _ => vec![],
    };
    run_git_cmd(Log, Some(sub_cmd))
}

pub fn checkout_new(branch_name: String) -> String {
    run_git_cmd(Checkout, Some(vec![String::from("-b"), branch_name]))
}

pub fn reset_branch(density: String) -> String {
    run_git_cmd(Reset, Some(vec![density]))
}

pub fn check_new_branch(branch: String) -> bool {
    let br_copy = branch.clone();
    let empty_string = String::from("");
    match run_git_cmd(
        Branch,
        Some(vec![
            String::from("--contains"),
            branch,
            String::from("|"),
            Grep.value(),
            String::from("-w"),
            br_copy,
        ]),
    ) {
        x if x != empty_string => false,
        x if x == empty_string => true,
        _ => false,
    }
}

pub fn get_branch() -> String {
    run_git_cmd(Branch, Some(vec![String::from("--show-current")]))
}

pub fn show_help() -> String {
    HelpInfo::display(&HelpInfo {
        descriptions: vec![
            "Add, Commit, Push    ".to_string(),
            "Checkout new branch  ".to_string(),
            "Stash, Pull, Apply   ".to_string(),
            "Reset staged changes ".to_string(),
            "Get status of branch ".to_string(),
            "Log git history      ".to_string(),
        ],
        commands: vec![
            "crust acp [commit msg]".to_string(),
            "crust cob [branch_name]".to_string(),
            "crust spa".to_string(),
            "crust soft".to_string(),
            "crust st  ".to_string(),
            "crust log".to_string(),
        ],
    })
}
