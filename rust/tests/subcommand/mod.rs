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


use dnf_scaffold::command::subcommand::Subcommand;
use std::sync::OnceLock;


pub(crate) struct TestData {
    pub(crate) subcommand: Subcommand,
    pub(crate) text: String,
}


pub(crate) fn test_data() -> &'static [&'static TestData; 30] {
    static INSTANCE: OnceLock<[&TestData; 30]> = OnceLock::new();
    INSTANCE.get_or_init(|| [
        alias::test_data(),
        autoremove::test_data(),
        check::test_data(),
        checkupdate::test_data(),
        clean::test_data(),
        deplist::test_data(),
        distrosync::test_data(),
        downgrade::test_data(),
        group::test_data(),
        help::test_data(),
        history::test_data(),
        info::test_data(),
        install::test_data(),
        list::test_data(),
        makecache::test_data(),
        mark::test_data(),
        module::test_data(),
        provides::test_data(),
        reinstall::test_data(),
        remove::test_data(),
        repoinfo::test_data(),
        repolist::test_data(),
        repoquery::test_data(),
        repositorypackages::test_data(),
        search::test_data(),
        shell::test_data(),
        swap::test_data(),
        updateinfo::test_data(),
        upgrade::test_data(),
        upgrademinimal::test_data(),
    ])
}
