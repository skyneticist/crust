use std::io;
use std::io::Write;
use std::process::Command;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    command: String,
    message: String,
}

fn main() {
    let args = Cli::from_args();

    if args.command == "acp" {
        add_commit_push(args.message);
    } else {
        let data = log_commits();
        io::stdout().write_all(&data).unwrap();

        // println!("{:?}", log_commits());
    }
}

fn add_commit_push(commit_msg: String) {
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
