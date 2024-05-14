mod advisory;
mod allowerasing;
mod assumeno;
mod assumeyes;
mod best;
mod bugfix;
mod bz;
mod cacheonly;
mod color;
mod comment;
mod config;
mod cve;
mod debuglevel;
mod debugsolver;
mod disable;
mod disableexcludes;
mod disableplugin;
mod disablerepo;
mod downloaddir;
mod downloadonly;
mod enable;
mod enableplugin;
mod enablerepo;
mod enhancement;
mod errorlevel;
mod exclude;
mod excludepkgs;
mod forcearch;
mod help;
mod installroot;
mod ipv4;
mod ipv6;
mod newpackage;
mod noautoremove;
mod nobest;
mod nodocs;
mod nogpgcheck;
mod noplugins;
mod obsoletes;
mod quiet;
mod randomwait;
mod refresh;
mod releasever;
mod repo;
mod repofrompath;
mod rpmverbosity;
mod secseverity;
mod security;
mod setopt;
mod showduplicates;
mod skipbroken;
mod verbose;
mod version;


use dnf_scaffold::command::options::CommandOption;
use std::sync::OnceLock;


pub(crate) struct TestData {
    pub(crate) command_option: CommandOption,
    pub(crate) text: String,
}


pub(crate) fn test_data() -> &'static [&'static TestData; 53] {
    static INSTANCE: OnceLock<[&TestData; 53]> = OnceLock::new();
    INSTANCE.get_or_init(|| [
        advisory::test_data(),
        allowerasing::test_data(),
        assumeno::test_data(),
        assumeyes::test_data(),
        best::test_data(),
        bugfix::test_data(),
        bz::test_data(),
        cacheonly::test_data(),
        color::test_data(),
        comment::test_data(),
        config::test_data(),
        cve::test_data(),
        debuglevel::test_data(),
        debugsolver::test_data(),
        disable::test_data(),
        disableexcludes::test_data(),
        disableplugin::test_data(),
        disablerepo::test_data(),
        downloaddir::test_data(),
        downloadonly::test_data(),
        enable::test_data(),
        enableplugin::test_data(),
        enablerepo::test_data(),
        enhancement::test_data(),
        errorlevel::test_data(),
        exclude::test_data(),
        excludepkgs::test_data(),
        forcearch::test_data(),
        help::test_data(),
        installroot::test_data(),
        ipv4::test_data(),
        ipv6::test_data(),
        newpackage::test_data(),
        noautoremove::test_data(),
        nobest::test_data(),
        nodocs::test_data(),
        nogpgcheck::test_data(),
        noplugins::test_data(),
        obsoletes::test_data(),
        quiet::test_data(),
        randomwait::test_data(),
        refresh::test_data(),
        releasever::test_data(),
        repo::test_data(),
        repofrompath::test_data(),
        rpmverbosity::test_data(),
        secseverity::test_data(),
        security::test_data(),
        setopt::test_data(),
        showduplicates::test_data(),
        skipbroken::test_data(),
        verbose::test_data(),
        version::test_data(),
    ])
}
