#[derive()]
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
