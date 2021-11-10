use std::io;
use std::io::Write;
use std::process::Command;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    command: String,
    message: Option<String>,
}

#[derive(StructOpt)]
struct GitCmd {
    command: String,
}

impl GitCmd {
    pub fn execute(sub_commands: GitCmd) -> std::process::Output {
        let output = Command::new("git")
            .arg(sub_commands.command)
            .output()
            .expect("problem executing git command");

        return output;
    }
}

fn main() {
    let args = Cli::from_args();
    let git_cmd = GitCmd::from_args();

    // GitCmd::execute(git_cmd);

    let msg: Result<String, Err> = match args.message {
        Some(msg) => Ok(msg),
        None => Ok(String::from("")),
    };

    if args.command == "acp" {
        add_commit_push(msg.unwrap());
    } else if args.command == "st" {
        let data = log_commits();
        io::stdout().write_all(&data).unwrap();

        // println!("{:?}", log_commits());
    }
}

fn add_commit_push(commit_msg: String) {
    let git_cmd = GitCmd {
        command: "status".to_string(),
    };
    GitCmd::execute(git_cmd);
    Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("crust acp did not run successfully");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_msg)
        .output()
        .expect("trouble committing to repo");

    Command::new("git")
        .arg("push")
        .output()
        .expect("trouble pushing commit to repo");
}

fn log_commits() -> Vec<u8> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=oneline")
        .output()
        .expect("problem running git log");

    return output.stdout;
}
