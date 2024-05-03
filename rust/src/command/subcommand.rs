mod alias;
mod autoremove;
mod check;
mod checkupdate;
mod clean;
mod deplist;
mod distrosync;
mod downgrade;
mod group;
mod help;
mod history;
mod info;
mod install;
mod list;
mod makecache;
mod mark;
mod module;
mod provides;
mod reinstall;
mod remove;
mod repoinfo;
mod repolist;
mod repoquery;
mod repositorypackages;
mod search;
mod shell;
mod swap;
mod updateinfo;
mod upgrade;
mod upgrademinimal;


use std::fmt::Display;


pub enum Subcommand {
    Alias,
    Autoremove,
    Check,
    CheckUpdate,
    Clean,
    Deplist,
    DistroSync,
    Downgrade,
    Group,
    Help,
    History,
    Info,
    Install,
    List,
    Makecache,
    Mark,
    Module,
    Provides,
    Reinstall,
    Remove,
    Repoinfo,
    Repolist,
    Repoquery,
    RepositoryPackages,
    Search,
    Shell,
    Swap,
    Updateinfo,
    Upgrade,
    UpgradeMinimal,
}

fn subcommand_str(subcommand: &Subcommand) -> &'static str {
    match subcommand {
        Subcommand::Alias => alias::STR,
        Subcommand::Autoremove => autoremove::STR,
        Subcommand::Check => check::STR,
        Subcommand::CheckUpdate => checkupdate::STR,
        Subcommand::Clean => clean::STR,
        Subcommand::Deplist => deplist::STR,
        Subcommand::DistroSync => distrosync::STR,
        Subcommand::Downgrade => downgrade::STR,
        Subcommand::Group => group::STR,
        Subcommand::Help => help::STR,
        Subcommand::History => history::STR,
        Subcommand::Info => info::STR,
        Subcommand::Install => install::STR,
        Subcommand::List => list::STR,
        Subcommand::Makecache => makecache::STR,
        Subcommand::Mark => mark::STR,
        Subcommand::Module => module::STR,
        Subcommand::Provides => provides::STR,
        Subcommand::Reinstall => reinstall::STR,
        Subcommand::Remove => remove::STR,
        Subcommand::Repoinfo => repoinfo::STR,
        Subcommand::Repolist => repolist::STR,
        Subcommand::Repoquery => repoquery::STR,
        Subcommand::RepositoryPackages => repositorypackages::STR,
        Subcommand::Search => search::STR,
        Subcommand::Shell => shell::STR,
        Subcommand::Swap => swap::STR,
        Subcommand::Updateinfo => updateinfo::STR,
        Subcommand::Upgrade => upgrade::STR,
        Subcommand::UpgradeMinimal => upgrademinimal::STR,
    }
}

impl Display for Subcommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", subcommand_str(self))
    }
}
