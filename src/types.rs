use crate::actions::{
    add_commit_push, ask_google, ask_stack_overflow, checkout_new, get_status, log_commits,
    open_azure, open_devops, reset_branch, show_help, update_all,
};
use crate::types::Actions::{
    Acp, Cloud, Cob, DevOps, Go, Help, Log, PullAll, So, SoftReset, Status,
};
use colored::*;
use structopt::StructOpt;

pub struct CrustConfig {
    pub verbosity: Option<u8>,
    pub hide: Option<bool>,
    pub dump_location: Option<String>,
    pub ee_img_path: Option<String>,
}

pub struct Crust {
    pub config: CrustConfig,
}

impl Crust {
    pub fn new(crust_fig: Option<CrustConfig>) -> Self {
        let default_config = CrustConfig {
            verbosity: Some(2),
            hide: Some(false),
            dump_location: Some(String::from("./")),
            ee_img_path: None,
        };
        let config = match crust_fig {
            Some(config) => config,
            None => default_config,
        };

        Crust { config }
    }
}

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    arg: Option<String>,
}

impl Crust {
    pub fn run(config: Option<CrustConfig>) {
        if Some(&config).is_some() {
            Crust::new(config);
        } else {
            Crust::new(None);
        }

        let args: Cli = Cli::from_args();

        let sub_cmd = args.arg.unwrap_or_else(|| "".to_string());
        let output = match args.command {
            x if x == Acp.value() || x == Acp.short_value() => Acp.method(sub_cmd),
            x if x == Cloud.value() || x == Cloud.short_value() => Cloud.method(sub_cmd),
            x if x == Cob.value() || x == Cob.short_value() => Cob.method(sub_cmd),
            x if x == DevOps.value() || x == DevOps.short_value() => DevOps.method(sub_cmd),
            x if x == PullAll.value() || x == PullAll.short_value() => PullAll.method(sub_cmd),
            x if x == Help.value() || x == Help.short_value() => Help.method(sub_cmd),
            x if x == Log.value() || x == Log.short_value() => Log.method(sub_cmd),
            x if x == Go.value() || x == Go.short_value() => Go.method(sub_cmd),
            x if x == So.value() || x == So.short_value() => So.method(sub_cmd),
            x if x == SoftReset.value() || x == SoftReset.short_value() => {
                SoftReset.method(sub_cmd)
            }
            x if x == Status.value() || x == Status.short_value() => Status.method(sub_cmd),
            x if x == "*" => display_msg(),
            _ => Help.method(sub_cmd),
        };
        println!("{}", output);
    }
}

fn display_msg() -> String {
    String::from(" Welcome to Crust \u{1F35E} \nTry typing `crust help`")
        .green()
        .to_string()
}

pub enum Actions {
    Acp,
    Cloud,
    Cob,
    DevOps,
    Help,
    PullAll,
    Log,
    Go,
    So,
    Status,
    SoftReset,
}

impl Actions {
    pub fn value(&self) -> String {
        match *self {
            Actions::Acp => String::from("done"),
            Actions::Cloud => String::from("azure"),
            Actions::Cob => String::from("cob"),
            Actions::DevOps => String::from("devops"),
            Actions::PullAll => String::from("pullall"),
            Actions::Help => String::from("help"),
            Actions::Log => String::from("log"),
            Actions::Go => String::from("google"),
            Actions::So => String::from("stof"),
            Actions::SoftReset => String::from("soft"),
            Actions::Status => String::from("st"),
        }
    }
}

impl Actions {
    pub fn method(&self, sub_cmd: String) -> String {
        match *self {
            Actions::Acp => add_commit_push(sub_cmd),
            Actions::Cloud => open_azure(),
            Actions::Cob => checkout_new(sub_cmd),
            Actions::DevOps => open_devops(),
            Actions::PullAll => update_all(),
            Actions::Help => show_help(),
            Actions::Log => log_commits(sub_cmd),
            Actions::Go => ask_google(sub_cmd),
            Actions::So => ask_stack_overflow(&sub_cmd),
            Actions::SoftReset => reset_branch(sub_cmd),
            Actions::Status => get_status(),
        }
    }
}

impl Actions {
    pub fn short_value(&self) -> String {
        match *self {
            Actions::Acp => String::from("acp"),
            Actions::Cloud => String::from("az"),
            Actions::Cob => String::from("nb"),
            Actions::DevOps => String::from("ops"),
            Actions::PullAll => String::from("pa"),
            Actions::Help => String::from("h"),
            Actions::Log => String::from("l"),
            Actions::Go => String::from("go"),
            Actions::So => String::from("so"),
            Actions::SoftReset => String::from("sr"),
            Actions::Status => String::from("s"),
        }
    }
}

pub enum RootCmd {
    // FindStr,
    Git,
    Grep,
}

impl RootCmd {
    pub fn value(&self) -> String {
        match *self {
            // RootCmd::FindStr => String::from("findstr"),
            RootCmd::Git => String::from("git"),
            RootCmd::Grep => String::from("grep"),
        }
    }
}

pub enum GitCommands {
    Add,
    Branch,
    Checkout,
    Commit,
    Log,
    Push,
    Status,
    Reset,
    Pull,
    Stash,
    Revert,
    Pop,
    Apply,
    Hard,
    Soft,
}

impl GitCommands {
    pub fn value(&self) -> String {
        match *self {
            GitCommands::Add => String::from("add"),
            GitCommands::Branch => String::from("branch"),
            GitCommands::Checkout => String::from("checkout"),
            GitCommands::Commit => String::from("commit"),
            GitCommands::Log => String::from("log"),
            GitCommands::Push => String::from("push"),
            GitCommands::Status => String::from("status"),
            GitCommands::Reset => String::from("reset"),
            GitCommands::Pull => String::from("pull"),
            GitCommands::Stash => String::from("stash"),
            GitCommands::Revert => String::from("revert"),
            GitCommands::Pop => String::from("pop"),
            GitCommands::Apply => String::from("apply"),
            GitCommands::Hard => String::from("hard"),
            GitCommands::Soft => String::from("soft"),
        }
    }
}

pub struct HelpInfo {
    pub descriptions: Vec<String>,
    pub commands: Vec<String>,
}

impl HelpInfo {
    pub fn display(&self) -> String {
        println!(
            "{}",
            "\n\u{1F419}   Welcome to crust".yellow().bold().blink()
        );
        println!("{}", "     v0.0.1".blue().dimmed());
        let mut table = vec![];
        let mut row: String;
        for (i, cmd) in self.commands.iter().enumerate() {
            row = format!(
                "\u{1F680}   {0: <10}   \u{1F9AE}   {1: <10}\n",
                &self.descriptions[i].cyan(),
                cmd.magenta()
            );
            table.push(row);
        }
        println!(
            "\n{0: <10}   {1: <10}\n",
            "\u{1F680}   Description          "
                .green()
                .bold()
                .to_string(),
            "\u{1F9AE}   Command     ".red().bold().to_string()
        );
        table.join("")
    }
}
