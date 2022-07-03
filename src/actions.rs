use crate::types::GitCommands::{Add, Branch, Checkout, Commit, Log, Push, Reset, Status};
use crate::types::RootCmd::{Git, Grep};
use crate::types::{GitCommands, HelpInfo};
use std::fs;
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

pub fn ask_stack_overflow(query: String) -> String {
    let question_url = "https://stackoverflow.com";
    let output = Command::new("explorer")
        .arg(question_url)
        .output()
        .expect("problem opening stack overflow!");
    match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => String::from("Error occurred at ask_stack_overflow"),
    }
}

pub fn open_azure() -> String {
    let url = "https://portal.azure.com/";
    let output = Command::new("explorer")
        .arg(url)
        .output()
        .expect("could not open Azure portal");
    match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => String::from("Error occurred at open_azure"),
    }
}

// not working currently
pub fn open_devops() -> String {
    let url = "https://dev.azure.com/";
    let output = Command::new("explorer")
        .arg(url)
        .output()
        .expect("could not open Azure DevOps");
    match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => String::from("Error occurred at open_devops"),
    }
}

// pub fn open_environment() -> String {
//     String::from("")
//     // Need to open dev ops, azure portal, jenkins, octopus, vs code, vs, docker, etc
//     // Need to handle anything that promotes convenience
// }

pub fn ask_google(query: String) -> String {
    let url = "https://google.com/";
    let output = Command::new("explorer")
        .arg(url)
        .arg(query)
        .output()
        .expect("problem asking google");
    match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => String::from("Error occurred at ask_google"),
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

pub fn update_all() -> String {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        // run_git_cmd(Stash, None);
        // run_git_cmd(Pull, None);
        println!("Name: {}", path.unwrap().path().display())
    }
    String::from("are you happy now")
}

pub fn check_new_branch(branch: String) -> bool {
    let br_copy = branch.clone();
    let empty_string = String::from("");
    match run_git_cmd(
        Branch,
        Some(vec![
            String::from("-r"),
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
            "Open Azure Portal    ".to_string(),
            "Checkout new branch  ".to_string(),
            "Stash, Pull, Apply   ".to_string(),
            "Reset staged changes ".to_string(),
            "Get status of branch ".to_string(),
            "Open google for help ".to_string(),
            "Open stackoverflow   ".to_string(),
            "Log git history      ".to_string(),
        ],
        commands: vec![
            "crust acp [commit msg]".to_string(),
            "crust az".to_string(),
            "crust cob [branch_name]".to_string(),
            "crust spa".to_string(),
            "crust soft".to_string(),
            "crust st  ".to_string(),
            "crust go  ".to_string(),
            "crust so  ".to_string(),
            "crust log".to_string(),
        ],
    })
}
