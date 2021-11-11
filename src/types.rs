pub enum RootCmd {
    Git,
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