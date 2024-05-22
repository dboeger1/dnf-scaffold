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
    let advisory = &CommandOption::new_advisory(
        "advisory_arg".to_string(),
    ).unwrap();
    println!("advisory: \"{advisory}\"");
    println!("arg: \"{}\"", advisory.arg().unwrap());

    let bz = &CommandOption::new_bz(
        "bz_arg".to_string(),
    ).unwrap();
    println!("\nbz: \"{bz}\"");
    println!("arg: \"{}\"", bz.arg().unwrap());

    let mut command_options = CommandOptions::new();
    command_options.set(&advisory);
    command_options.set(&bz);
    println!("\noptions: \"{command_options}\"");

    let command = Command{
        options: command_options,
        subcommand: Some(Subcommand::Upgrade),
    };
    println!("\ncommand: \"{command}\"");
}
