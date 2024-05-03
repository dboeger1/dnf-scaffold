pub mod options;
pub mod subcommand;


use options::Options;
use std::fmt::Display;
use subcommand::Subcommand;


#[derive(Default)]
pub struct Command {
    pub subcommand: Option<Subcommand>,
    pub options: Option<Options>,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dnf")?;
        if let Some(options) = self.options.as_ref() {
            write!(f, "{options}")?;
        }
        if let Some(subcommand) = self.subcommand.as_ref() {
            write!(f, " {subcommand}")?;
        }

        Ok(())
    }
}
