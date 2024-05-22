pub mod advisory;
pub mod allowerasing;
pub mod assumeno;
pub mod assumeyes;
pub mod best;
pub mod bugfix;
pub mod bz;
pub mod cacheonly;
pub mod color;
pub mod comment;
pub mod config;
pub mod cve;
pub mod debuglevel;
pub mod debugsolver;
pub mod disable;
pub mod disableexcludes;
pub mod disableplugin;
pub mod disablerepo;
pub mod downloaddir;
pub mod downloadonly;
pub mod enable;
pub mod enableplugin;
pub mod enablerepo;
pub mod enhancement;
pub mod errorlevel;
pub mod exclude;
pub mod excludepkgs;
pub mod forcearch;
pub mod help;
pub mod installroot;
pub mod ipv4;
pub mod ipv6;
pub mod newpackage;
pub mod noautoremove;
pub mod nobest;
pub mod nodocs;
pub mod nogpgcheck;
pub mod noplugins;
pub mod obsoletes;
pub mod quiet;
pub mod randomwait;
pub mod refresh;
pub mod releasever;
pub mod repo;
pub mod repofrompath;
pub mod rpmverbosity;
pub mod secseverity;
pub mod security;
pub mod setopt;
pub mod showduplicates;
pub mod skipbroken;
pub mod verbose;
pub mod version;


use crate::{Error, Validate};
use std::fmt::Display;
use strum_macros::{
    EnumDiscriminants,
    EnumIter,
};


#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum CommandOption {
    Advisory(advisory::Arg),
    Allowerasing,
    Assumeno,
    Assumeyes,
    Best,
    Bugfix,
    Bz(bz::Arg),
    Cacheonly,
    Color(color::Arg),
    Comment(comment::Arg),
    Config(config::Arg),
    Cve(cve::Arg),
    Debuglevel(debuglevel::Arg),
    Debugsolver,
    Disable,
    Disableexcludes(disableexcludes::Arg),
    Disableplugin(disableplugin::Arg),
    Disablerepo(disablerepo::Arg),
    Downloaddir(downloaddir::Arg),
    Downloadonly,
    Enable,
    Enableplugin(enableplugin::Arg),
    Enablerepo(enablerepo::Arg),
    Enhancement,
    Errorlevel(errorlevel::Arg),
    Exclude(exclude::Arg),
    Excludepkgs(excludepkgs::Arg),
    Forcearch(forcearch::Arg),
    Help,
    Installroot(installroot::Arg),
    Ipv4,
    Ipv6,
    Newpackage,
    Noautoremove,
    Nobest,
    Nodocs,
    Nogpgcheck,
    Noplugins,
    Obsoletes,
    Quiet,
    Randomwait(randomwait::Arg),
    Refresh,
    Releasever(releasever::Arg),
    Repo(repo::Arg),
    Repofrompath(repofrompath::Arg),
    Rpmverbosity(rpmverbosity::Arg),
    Secseverity(secseverity::Arg),
    Security,
    Setopt(setopt::Arg),
    Showduplicates,
    Skipbroken,
    Verbose,
    Version,
}

impl CommandOption {
    pub fn name(&self) -> &'static str {
        CommandOptionDiscriminants::from(self).name()
    }

    fn arg(&self) -> Option<String> {
        match self {
            Self::Advisory(arg) => Some(arg.to_string()),
            Self::Bz(arg) => Some(arg.to_string()),
            Self::Color(arg) => Some(arg.to_string()),
            Self::Comment(arg) => Some(arg.to_string()),
            Self::Config(arg) => Some(arg.to_string()),
            Self::Cve(arg) => Some(arg.to_string()),
            Self::Debuglevel(arg) => Some(arg.to_string()),
            Self::Disableexcludes(arg) => Some(arg.to_string()),
            Self::Disableplugin(arg) => Some(arg.to_string()),
            Self::Disablerepo(arg) => Some(arg.to_string()),
            Self::Downloaddir(arg) => Some(arg.to_string()),
            Self::Enableplugin(arg) => Some(arg.to_string()),
            Self::Enablerepo(arg) => Some(arg.to_string()),
            Self::Errorlevel(arg) => Some(arg.to_string()),
            Self::Exclude(arg) => Some(arg.to_string()),
            Self::Excludepkgs(arg) => Some(arg.to_string()),
            Self::Forcearch(arg) => Some(arg.to_string()),
            Self::Installroot(arg) => Some(arg.to_string()),
            Self::Randomwait(arg) => Some(arg.to_string()),
            Self::Releasever(arg) => Some(arg.to_string()),
            Self::Repo(arg) => Some(arg.to_string()),
            Self::Repofrompath(arg) => Some(arg.to_string()),
            Self::Rpmverbosity(arg) => Some(arg.to_string()),
            Self::Secseverity(arg) => Some(arg.to_string()),
            Self::Setopt(arg) => Some(arg.to_string()),
            _ => None,
        }
    }

    pub fn new_advisory(value: advisory::ArgType) -> Result<Self, Error> {
        let arg = advisory::Arg::new(value)?;
        Ok(Self::Advisory(arg))
    }

    pub fn new_bz(value: bz::ArgType) -> Result<Self, Error> {
        let arg = bz::Arg::new(value)?;
        Ok(Self::Bz(arg))
    }

    pub fn new_color(value: color::ArgType) -> Result<Self, Error> {
        let arg = color::Arg::new(value)?;
        Ok(Self::Color(arg))
    }

    pub fn new_comment(value: comment::ArgType) -> Result<Self, Error> {
        let arg = comment::Arg::new(value)?;
        Ok(Self::Comment(arg))
    }

    pub fn new_config(value: config::ArgType) -> Result<Self, Error> {
        let arg = config::Arg::new(value)?;
        Ok(Self::Config(arg))
    }

    pub fn new_cve(value: cve::ArgType) -> Result<Self, Error> {
        let arg = cve::Arg::new(value)?;
        Ok(Self::Cve(arg))
    }

    pub fn new_debuglevel(value: debuglevel::ArgType) -> Result<Self, Error> {
        let arg = debuglevel::Arg::new(value)?;
        Ok(Self::Debuglevel(arg))
    }

    pub fn new_disableexcludes(value: disableexcludes::ArgType) -> Result<Self, Error> {
        let arg = disableexcludes::Arg::new(value)?;
        Ok(Self::Disableexcludes(arg))
    }

    pub fn new_disableplugin(value: disableplugin::ArgType) -> Result<Self, Error> {
        let arg = disableplugin::Arg::new(value)?;
        Ok(Self::Disableplugin(arg))
    }

    pub fn new_disablerepo(value: disablerepo::ArgType) -> Result<Self, Error> {
        let arg = disablerepo::Arg::new(value)?;
        Ok(Self::Disablerepo(arg))
    }

    pub fn new_downloaddir(value: downloaddir::ArgType) -> Result<Self, Error> {
        let arg = downloaddir::Arg::new(value)?;
        Ok(Self::Downloaddir(arg))
    }

    pub fn new_enableplugin(value: enableplugin::ArgType) -> Result<Self, Error> {
        let arg = enableplugin::Arg::new(value)?;
        Ok(Self::Enableplugin(arg))
    }

    pub fn new_enablerepo(value: enablerepo::ArgType) -> Result<Self, Error> {
        let arg = enablerepo::Arg::new(value)?;
        Ok(Self::Enablerepo(arg))
    }

    pub fn new_errorlevel(value: errorlevel::ArgType) -> Result<Self, Error> {
        let arg = errorlevel::Arg::new(value)?;
        Ok(Self::Errorlevel(arg))
    }

    pub fn new_exclude(value: exclude::ArgType) -> Result<Self, Error> {
        let arg = exclude::Arg::new(value)?;
        Ok(Self::Exclude(arg))
    }

    pub fn new_excludepkgs(value: excludepkgs::ArgType) -> Result<Self, Error> {
        let arg = excludepkgs::Arg::new(value)?;
        Ok(Self::Excludepkgs(arg))
    }

    pub fn new_forcearch(value: forcearch::ArgType) -> Result<Self, Error> {
        let arg = forcearch::Arg::new(value)?;
        Ok(Self::Forcearch(arg))
    }

    pub fn new_installroot(value: installroot::ArgType) -> Result<Self, Error> {
        let arg = installroot::Arg::new(value)?;
        Ok(Self::Installroot(arg))
    }

    pub fn new_randomwait(value: randomwait::ArgType) -> Result<Self, Error> {
        let arg = randomwait::Arg::new(value)?;
        Ok(Self::Randomwait(arg))
    }

    pub fn new_releasever(value: releasever::ArgType) -> Result<Self, Error> {
        let arg = releasever::Arg::new(value)?;
        Ok(Self::Releasever(arg))
    }

    pub fn new_repo(value: repo::ArgType) -> Result<Self, Error> {
        let arg = repo::Arg::new(value)?;
        Ok(Self::Repo(arg))
    }

    pub fn new_repofrompath(value: repofrompath::ArgType) -> Result<Self, Error> {
        let arg = repofrompath::Arg::new(value)?;
        Ok(Self::Repofrompath(arg))
    }

    pub fn new_rpmverbosity(value: rpmverbosity::ArgType) -> Result<Self, Error> {
        let arg = rpmverbosity::Arg::new(value)?;
        Ok(Self::Rpmverbosity(arg))
    }

    pub fn new_secseverity(value: secseverity::ArgType) -> Result<Self, Error> {
        let arg = secseverity::Arg::new(value)?;
        Ok(Self::Secseverity(arg))
    }

    pub fn new_setopt(value: setopt::ArgType) -> Result<Self, Error> {
        let arg = setopt::Arg::new(value)?;
        Ok(Self::Setopt(arg))
    }
}


impl Display for CommandOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--{}", self.name())?;
        if let Some(arg) = self.arg() {
            write!(f, " {arg}")?;
        }

        Ok(())
    }
}


impl CommandOptionDiscriminants {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Advisory => advisory::NAME,
            Self::Allowerasing => allowerasing::NAME,
            Self::Assumeno => assumeno::NAME,
            Self::Assumeyes => assumeyes::NAME,
            Self::Best => best::NAME,
            Self::Bugfix => bugfix::NAME,
            Self::Bz => bz::NAME,
            Self::Cacheonly => cacheonly::NAME,
            Self::Color => color::NAME,
            Self::Comment => comment::NAME,
            Self::Config => config::NAME,
            Self::Cve => cve::NAME,
            Self::Debuglevel => debuglevel::NAME,
            Self::Debugsolver => debugsolver::NAME,
            Self::Disable => disable::NAME,
            Self::Disableexcludes => disableexcludes::NAME,
            Self::Disableplugin => disableplugin::NAME,
            Self::Disablerepo => disablerepo::NAME,
            Self::Downloaddir => downloaddir::NAME,
            Self::Downloadonly => downloadonly::NAME,
            Self::Enable => enable::NAME,
            Self::Enableplugin => enableplugin::NAME,
            Self::Enablerepo => enablerepo::NAME,
            Self::Enhancement => enhancement::NAME,
            Self::Errorlevel => errorlevel::NAME,
            Self::Exclude => exclude::NAME,
            Self::Excludepkgs => excludepkgs::NAME,
            Self::Forcearch => forcearch::NAME,
            Self::Help => help::NAME,
            Self::Installroot => installroot::NAME,
            Self::Ipv4 => ipv4::NAME,
            Self::Ipv6 => ipv6::NAME,
            Self::Newpackage => newpackage::NAME,
            Self::Noautoremove => noautoremove::NAME,
            Self::Nobest => nobest::NAME,
            Self::Nodocs => nodocs::NAME,
            Self::Nogpgcheck => nogpgcheck::NAME,
            Self::Noplugins => noplugins::NAME,
            Self::Obsoletes => obsoletes::NAME,
            Self::Quiet => quiet::NAME,
            Self::Randomwait => randomwait::NAME,
            Self::Refresh => refresh::NAME,
            Self::Releasever => releasever::NAME,
            Self::Repo => repo::NAME,
            Self::Repofrompath => repofrompath::NAME,
            Self::Rpmverbosity => rpmverbosity::NAME,
            Self::Secseverity => secseverity::NAME,
            Self::Security => security::NAME,
            Self::Setopt => setopt::NAME,
            Self::Showduplicates => showduplicates::NAME,
            Self::Skipbroken => skipbroken::NAME,
            Self::Verbose => verbose::NAME,
            Self::Version => version::NAME,
        }
    }
}
