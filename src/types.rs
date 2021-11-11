use crate::actions::log_commits;
use crate::add_commit_push;
use crate::get_status;
use crate::Cli;

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

impl Crust {
    pub fn run_cmd(args: Cli) {
        // enum candidates? mmhmm
        let _acp_cmd = String::from("acp");
        let _log = String::from("log");
        let _status = String::from("status");

        let sub_cmd = args.arg.unwrap_or_else(|| "".to_string());

        let output = match args.command {
            x if x == _acp_cmd => add_commit_push(Some(true), sub_cmd),
            x if x == _status => get_status(),
            x if x == _log => log_commits(sub_cmd),
            _ => String::from("unknown command: ") + &args.command,
        };
        println!("{}", output);
    }
}

pub enum RootCmd {
    Git,
    // Bash,
    // Ps,
    // Node,
    // Python,
}

impl RootCmd {
    pub fn value(&self) -> String {
        match *self {
            RootCmd::Git => String::from("git"),
        }
    }
}

// #[derive(StructOpt)]
pub enum GitCommands {
    Add,
    Commit,
    Log,
    Push,
    Status,
    // Pull,
    // Stash,
    // Revert,
    // Pop,
    // Apply,
    // Reset,
    // Hard,
    // Soft,
}

impl GitCommands {
    pub fn value(&self) -> String {
        match *self {
            GitCommands::Add => String::from("add"),
            GitCommands::Commit => String::from("commit"),
            GitCommands::Log => String::from("log"),
            GitCommands::Push => String::from("push"),
            GitCommands::Status => String::from("status"),
            // GitCommands::Pull => String::from("pull"),
            // GitCommands::Stash => String::from("stash"),
            // GitCommands::Reset => String::from("reset"),
            // GitCommands::Revert => String::from("revert"),
            // GitCommands::Pop => String::from("pop"),
            // GitCommands::Apply => String::from("apply"),
            // GitCommands::Hard => String::from("hard"),
            // GitCommands::Soft => String::from("soft"),
        }
    }
}
