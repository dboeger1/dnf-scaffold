use dnf_scaffold::command::Command;
use dnf_scaffold::command::subcommand::Subcommand;
use dnf_scaffold::command::options::Options;


#[test]
fn stuff() {
    let command = Command {
        subcommand: Some(Subcommand::DistroSync),
        options: Some(Options {
            allowerasing: true,
            disable: true,
            randomwait: Some(10),
            ..Options::default()
        }),
    };

    println!("COMMAND: \"{command}\"");
}
