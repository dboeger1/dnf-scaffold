pub mod command_options;
pub mod subcommand;


use command_options::CommandOptions;
use std::fmt::Display;
use subcommand::Subcommand;


pub const BIN_NAME: &str = "dnf";


#[derive(Default)]
pub struct Command {
    pub options: CommandOptions,
    pub subcommand: Option<Subcommand>,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BIN_NAME}")?;

        let options = self.options.to_string();
        if !options.is_empty() {
            write!(f, " {options}")?;
        }

        if let Some(subcommand) = self.subcommand.as_ref() {
            write!(f, " {subcommand}")?;
        }

        Ok(())
    }
}
