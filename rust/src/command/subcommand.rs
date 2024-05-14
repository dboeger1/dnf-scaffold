pub mod alias;
pub mod autoremove;
pub mod check;
pub mod checkupdate;
pub mod clean;
pub mod deplist;
pub mod distrosync;
pub mod downgrade;
pub mod group;
pub mod help;
pub mod history;
pub mod info;
pub mod install;
pub mod list;
pub mod makecache;
pub mod mark;
pub mod module;
pub mod provides;
pub mod reinstall;
pub mod remove;
pub mod repoinfo;
pub mod repolist;
pub mod repoquery;
pub mod repositorypackages;
pub mod search;
pub mod shell;
pub mod swap;
pub mod updateinfo;
pub mod upgrade;
pub mod upgrademinimal;


use std::fmt::Display;


#[derive(Clone)]
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
        Subcommand::Alias => alias::NAME,
        Subcommand::Autoremove => autoremove::NAME,
        Subcommand::Check => check::NAME,
        Subcommand::CheckUpdate => checkupdate::NAME,
        Subcommand::Clean => clean::NAME,
        Subcommand::Deplist => deplist::NAME,
        Subcommand::DistroSync => distrosync::NAME,
        Subcommand::Downgrade => downgrade::NAME,
        Subcommand::Group => group::NAME,
        Subcommand::Help => help::NAME,
        Subcommand::History => history::NAME,
        Subcommand::Info => info::NAME,
        Subcommand::Install => install::NAME,
        Subcommand::List => list::NAME,
        Subcommand::Makecache => makecache::NAME,
        Subcommand::Mark => mark::NAME,
        Subcommand::Module => module::NAME,
        Subcommand::Provides => provides::NAME,
        Subcommand::Reinstall => reinstall::NAME,
        Subcommand::Remove => remove::NAME,
        Subcommand::Repoinfo => repoinfo::NAME,
        Subcommand::Repolist => repolist::NAME,
        Subcommand::Repoquery => repoquery::NAME,
        Subcommand::RepositoryPackages => repositorypackages::NAME,
        Subcommand::Search => search::NAME,
        Subcommand::Shell => shell::NAME,
        Subcommand::Swap => swap::NAME,
        Subcommand::Updateinfo => updateinfo::NAME,
        Subcommand::Upgrade => upgrade::NAME,
        Subcommand::UpgradeMinimal => upgrademinimal::NAME,
    }
}

impl Display for Subcommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", subcommand_str(self))
    }
}
