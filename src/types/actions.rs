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
