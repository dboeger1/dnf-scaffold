use dnf_scaffold::command::{
    Command,
    options::{
        advisory,
        allowerasing,
        assumeno,
        assumeyes,
        best,
        bugfix,
        bz,
        cacheonly,
        color,
        CommandOption,
        CommandOptions,
        comment,
        config,
        cve,
        debuglevel,
        debugsolver,
        disable,
        disableexcludes,
        disableplugin,
        disablerepo,
        downloaddir,
        downloadonly,
        enable,
        enableplugin,
        enablerepo,
        enhancement,
        errorlevel,
        exclude,
        excludepkgs,
        forcearch,
        help,
        installroot,
        ipv4,
        ipv6,
        newpackage,
        noautoremove,
        nobest,
        nodocs,
        nogpgcheck,
        noplugins,
        obsoletes,
        quiet,
        randomwait,
        refresh,
        releasever,
        repo,
        //repofrompath,
        rpmverbosity,
        secseverity,
        security,
        setopt,
        showduplicates,
        skipbroken,
        verbose,
        version,
    },
    subcommand::Subcommand,
};
use itertools::{
    chain,
    Itertools,
};
use lazy_static::lazy_static;


#[test]
fn test_combinations() {
    let mut combinations = 0u64;

    for command_option_count in chain(0..=4, 49..=52) {
        for combination in Itertools::combinations(COMMAND_OPTION_INFO.iter(), command_option_count) {
            combinations += 1;

            let mut command_options = CommandOptions::default();
            for command_option_info in combination.iter() {
                command_options.set(&command_option_info.command_option);
            }

            let command = Command {
                subcommand: Some(Subcommand::DistroSync),
                options: Some(command_options),
            };

            for command_option_info in combination.iter() {
                assert!(command.to_string().contains(&command_option_info.text));
            }
        }
    }

    println!("combinations: {combinations}");
}


struct CommandOptionInfo {
    command_option: CommandOption,
    text: String,
}

lazy_static! {
    static ref COMMAND_OPTION_INFO: [CommandOptionInfo; 52] = [
        CommandOptionInfo{
            command_option: CommandOption::Advisory{arg: advisory::Arg::default()},
            text: format!(
                "--{} {}",
                advisory::NAME,
                advisory::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Allowerasing,
            text: format!(
                "--{}",
                allowerasing::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Assumeno,
            text: format!(
                "--{}",
                assumeno::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Assumeyes,
            text: format!(
                "--{}",
                assumeyes::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Best,
            text: format!(
                "--{}",
                best::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Bugfix,
            text: format!(
                "--{}",
                bugfix::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Bz{arg: bz::Arg::default()},
            text: format!(
                "--{} {}",
                bz::NAME,
                bz::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Cacheonly,
            text: format!(
                "--{}",
                cacheonly::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Color{arg: color::Arg::default()},
            text: format!(
                "--{} {}",
                color::NAME,
                color::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Comment{arg: comment::Arg::default()},
            text: format!(
                "--{} {}",
                comment::NAME,
                comment::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Config{arg: config::Arg::default()},
            text: format!(
                "--{} {}",
                config::NAME,
                config::Arg::default().to_string_lossy(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Cve{arg: cve::Arg::default()},
            text: format!(
                "--{} {}",
                cve::NAME,
                cve::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Debuglevel{arg: debuglevel::Arg::default()},
            text: format!(
                "--{} {}",
                debuglevel::NAME,
                debuglevel::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Debugsolver,
            text: format!(
                "--{}",
                debugsolver::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Disable,
            text: format!(
                "--{}",
                disable::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Disableexcludes{arg: disableexcludes::Arg::default()},
            text: format!(
                "--{} {}",
                disableexcludes::NAME,
                disableexcludes::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Disableplugin{arg: disableplugin::Arg::default()},
            text: format!(
                "--{} {}",
                disableplugin::NAME,
                disableplugin::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Disablerepo{arg: disablerepo::Arg::default()},
            text: format!(
                "--{} {}",
                disablerepo::NAME,
                disablerepo::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Downloaddir{arg: downloaddir::Arg::default()},
            text: format!(
                "--{} {}",
                downloaddir::NAME,
                downloaddir::Arg::default().to_string_lossy(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Downloadonly,
            text: format!(
                "--{}",
                downloadonly::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Enable,
            text: format!(
                "--{}",
                enable::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Enableplugin{arg: enableplugin::Arg::default()},
            text: format!(
                "--{} {}",
                enableplugin::NAME,
                enableplugin::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Enablerepo{arg: enablerepo::Arg::default()},
            text: format!(
                "--{} {}",
                enablerepo::NAME,
                enablerepo::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Enhancement,
            text: format!(
                "--{}",
                enhancement::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Errorlevel{arg: errorlevel::Arg::default()},
            text: format!(
                "--{} {}",
                errorlevel::NAME,
                errorlevel::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Exclude{arg: exclude::Arg::default()},
            text: format!(
                "--{} {}",
                exclude::NAME,
                exclude::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Excludepkgs{arg: excludepkgs::Arg::default()},
            text: format!(
                "--{} {}",
                excludepkgs::NAME,
                excludepkgs::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Forcearch{arg: forcearch::Arg::default()},
            text: format!(
                "--{} {}",
                forcearch::NAME,
                forcearch::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Help,
            text: format!(
                "--{}",
                help::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Installroot{arg: installroot::Arg::default()},
            text: format!(
                "--{} {}",
                installroot::NAME,
                installroot::Arg::default().to_string_lossy(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Ipv4,
            text: format!(
                "--{}",
                ipv4::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Ipv6,
            text: format!(
                "--{}",
                ipv6::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Newpackage,
            text: format!(
                "--{}",
                newpackage::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Noautoremove,
            text: format!(
                "--{}",
                noautoremove::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Nobest,
            text: format!(
                "--{}",
                nobest::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Nodocs,
            text: format!(
                "--{}",
                nodocs::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Nogpgcheck,
            text: format!(
                "--{}",
                nogpgcheck::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Noplugins,
            text: format!(
                "--{}",
                noplugins::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Obsoletes,
            text: format!(
                "--{}",
                obsoletes::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Quiet,
            text: format!(
                "--{}",
                quiet::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Randomwait{arg: randomwait::Arg::default()},
            text: format!(
                "--{} {}",
                randomwait::NAME,
                randomwait::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Refresh,
            text: format!(
                "--{}",
                refresh::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Releasever{arg: releasever::Arg::default()},
            text: format!(
                "--{} {}",
                releasever::NAME,
                releasever::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Repo{arg: repo::Arg::default()},
            text: format!(
                "--{} {}",
                repo::NAME,
                repo::Arg::default(),
            ),
        },
        //CommandOptionInfo{
        //    command_option: CommandOption::Repofrompath((String::default(), PathBuf::default())),
        //    text: format!(
        //        "--{} {}",
        //        repofrompath::NAME,
        //        String::default(),
        //    ),
        //},
        CommandOptionInfo{
            command_option: CommandOption::Rpmverbosity{arg: rpmverbosity::Arg::default()},
            text: format!(
                "--{} {}",
                rpmverbosity::NAME,
                rpmverbosity::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Secseverity{arg: secseverity::Arg::default()},
            text: format!(
                "--{} {}",
                secseverity::NAME,
                secseverity::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Security,
            text: format!(
                "--{}",
                security::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Setopt{arg: setopt::Arg::default()},
            text: format!(
                "--{} {}",
                setopt::NAME,
                setopt::Arg::default(),
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Showduplicates,
            text: format!(
                "--{}",
                showduplicates::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Skipbroken,
            text: format!(
                "--{}",
                skipbroken::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Verbose,
            text: format!(
                "--{}",
                verbose::NAME,
            ),
        },
        CommandOptionInfo{
            command_option: CommandOption::Version,
            text: format!(
                "--{}",
                version::NAME,
            ),
        },
    ];
}
