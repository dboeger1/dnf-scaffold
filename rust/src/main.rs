use dnf_scaffold::{
    command::{
        command_options::{
            CommandOptions,
            command_option::CommandOption,
        },
        subcommand::Subcommand,
    },
    Command,
};

pub fn main() {
    let mut command_options = CommandOptions::new();

    command_options.set(
        &CommandOption::new_advisory(
            "advisory_arg".to_string(),
        ).unwrap(),
    );
    command_options.set(
        &CommandOption::new_bz(
            "bz_arg".to_string(),
        ).unwrap(),
    );

    println!("options: \"{command_options}\"");

    let command = Command{
        options: command_options,
        subcommand: Some(Subcommand::Upgrade),
    };
    println!("command: \"{command}\"");
}
