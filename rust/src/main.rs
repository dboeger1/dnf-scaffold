use dnf_scaffold::{
    command::{
        options::{Arg, CommandOption, CommandOptions},
        subcommand::Subcommand,
    },
    Command,
};

pub fn main() {
    let mut command_options = CommandOptions::new();

    let arg = Arg::new("advisory_arg".to_string()).unwrap();
    command_options.set(&CommandOption::Advisory(arg));

    let arg = Arg::new("bz_arg".to_string()).unwrap();
    command_options.set(&CommandOption::Bz(arg));

    println!("options: \"{command_options}\"");

    let command = Command{
        options: command_options,
        subcommand: Some(Subcommand::Upgrade),
    };
    println!("command: \"{command}\"");
}
