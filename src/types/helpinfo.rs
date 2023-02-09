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
