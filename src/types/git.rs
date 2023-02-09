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
