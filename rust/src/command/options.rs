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


use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;



#[derive(Clone, EnumIter)]
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
    const fn name(&self) -> &str {
        match self {
            Self::Advisory(_) => advisory::NAME,
            Self::Allowerasing => allowerasing::NAME,
            Self::Assumeno => assumeno::NAME,
            Self::Assumeyes => assumeyes::NAME,
            Self::Best => best::NAME,
            Self::Bugfix => bugfix::NAME,
            Self::Bz(_) => bz::NAME,
            Self::Cacheonly => cacheonly::NAME,
            Self::Color(_) => color::NAME,
            Self::Comment(_) => comment::NAME,
            Self::Config(_) => config::NAME,
            Self::Cve(_) => cve::NAME,
            Self::Debuglevel(_) => debuglevel::NAME,
            Self::Debugsolver => debugsolver::NAME,
            Self::Disable => disable::NAME,
            Self::Disableexcludes(_) => disableexcludes::NAME,
            Self::Disableplugin(_) => disableplugin::NAME,
            Self::Disablerepo(_) => disablerepo::NAME,
            Self::Downloaddir(_) => downloaddir::NAME,
            Self::Downloadonly => downloadonly::NAME,
            Self::Enable => enable::NAME,
            Self::Enableplugin(_) => enableplugin::NAME,
            Self::Enablerepo(_) => enablerepo::NAME,
            Self::Enhancement => enhancement::NAME,
            Self::Errorlevel(_) => errorlevel::NAME,
            Self::Exclude(_) => exclude::NAME,
            Self::Excludepkgs(_) => excludepkgs::NAME,
            Self::Forcearch(_) => forcearch::NAME,
            Self::Help => help::NAME,
            Self::Installroot(_) => installroot::NAME,
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
            Self::Randomwait(_) => randomwait::NAME,
            Self::Refresh => refresh::NAME,
            Self::Releasever(_) => releasever::NAME,
            Self::Repo(_) => repo::NAME,
            Self::Repofrompath(_) => repofrompath::NAME,
            Self::Rpmverbosity(_) => rpmverbosity::NAME,
            Self::Secseverity(_) => secseverity::NAME,
            Self::Security => security::NAME,
            Self::Setopt(_) => setopt::NAME,
            Self::Showduplicates => showduplicates::NAME,
            Self::Skipbroken => skipbroken::NAME,
            Self::Verbose => verbose::NAME,
            Self::Version => version::NAME,
        }
    }

    fn arg(&self) -> Option<String> {
        match self {
            Self::Advisory(arg) => Some(arg.to_string()),
            Self::Color(arg) => Some(arg.to_string()),
            Self::Comment(arg) => Some(arg.to_string()),
            Self::Config(arg) => Some(arg.to_string_lossy().to_string()),
            Self::Cve(arg) => Some(arg.to_string()),
            Self::Debuglevel(arg) => Some(arg.to_string()),
            Self::Disableexcludes(arg) => Some(arg.to_string()),
            Self::Disableplugin(arg) => Some(arg.to_string()),
            Self::Disablerepo(arg) => Some(arg.to_string()),
            Self::Downloaddir(arg) => Some(arg.to_string_lossy().to_string()),
            Self::Enableplugin(arg) => Some(arg.to_string()),
            Self::Enablerepo(arg) => Some(arg.to_string()),
            Self::Errorlevel(arg) => Some(arg.to_string()),
            Self::Exclude(arg) => Some(arg.to_string()),
            Self::Excludepkgs(arg) => Some(arg.to_string()),
            Self::Forcearch(arg) => Some(arg.to_string()),
            Self::Installroot(arg) => Some(arg.to_string_lossy().to_string()),
            Self::Randomwait(arg) => Some(arg.to_string()),
            Self::Releasever(arg) => Some(arg.to_string()),
            Self::Repo(arg) => Some(arg.to_string()),
            //Self::Repofrompath()(arg) => Some(arg.to_string()),
            Self::Rpmverbosity(arg) => Some(arg.to_string()),
            Self::Secseverity(arg) => Some(arg.to_string()),
            Self::Setopt(arg) => Some(arg.to_string()),
            _ => None,
        }
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


#[derive(Clone, Default)]
pub struct CommandOptions {
    advisory: Option<CommandOption>,
    allowerasing: Option<CommandOption>,
    assumeno: Option<CommandOption>,
    assumeyes: Option<CommandOption>,
    best: Option<CommandOption>,
    bugfix: Option<CommandOption>,
    bz: Option<CommandOption>,
    cacheonly: Option<CommandOption>,
    color: Option<CommandOption>,
    comment: Option<CommandOption>,
    config: Option<CommandOption>,
    cve: Option<CommandOption>,
    debuglevel: Option<CommandOption>,
    debugsolver: Option<CommandOption>,
    disable: Option<CommandOption>,
    disableexcludes: Option<CommandOption>,
    disableplugin: Option<CommandOption>,
    disablerepo: Option<CommandOption>,
    downloaddir: Option<CommandOption>,
    downloadonly: Option<CommandOption>,
    enable: Option<CommandOption>,
    enableplugin: Option<CommandOption>,
    enablerepo: Option<CommandOption>,
    enhancement: Option<CommandOption>,
    errorlevel: Option<CommandOption>,
    exclude: Option<CommandOption>,
    excludepkgs: Option<CommandOption>,
    forcearch: Option<CommandOption>,
    help: Option<CommandOption>,
    installroot: Option<CommandOption>,
    ipv4: Option<CommandOption>,
    ipv6: Option<CommandOption>,
    newpackage: Option<CommandOption>,
    noautoremove: Option<CommandOption>,
    nobest: Option<CommandOption>,
    nodocs: Option<CommandOption>,
    nogpgcheck: Option<CommandOption>,
    noplugins: Option<CommandOption>,
    obsoletes: Option<CommandOption>,
    quiet: Option<CommandOption>,
    randomwait: Option<CommandOption>,
    refresh: Option<CommandOption>,
    releasever: Option<CommandOption>,
    repo: Option<CommandOption>,
    repofrompath: Option<CommandOption>,
    rpmverbosity: Option<CommandOption>,
    secseverity: Option<CommandOption>,
    security: Option<CommandOption>,
    setopt: Option<CommandOption>,
    showduplicates: Option<CommandOption>,
    skipbroken: Option<CommandOption>,
    verbose: Option<CommandOption>,
    version: Option<CommandOption>,
}

impl CommandOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, command_option: &CommandOption) -> Option<CommandOption> {
        match command_option {
            CommandOption::Advisory(_) => self.advisory.clone(),
            CommandOption::Allowerasing => self.allowerasing.clone(),
            CommandOption::Assumeno => self.assumeno.clone(),
            CommandOption::Assumeyes => self.assumeyes.clone(),
            CommandOption::Best => self.best.clone(),
            CommandOption::Bugfix => self.bugfix.clone(),
            CommandOption::Bz(_) => self.bz.clone(),
            CommandOption::Cacheonly => self.cacheonly.clone(),
            CommandOption::Color(_) => self.color.clone(),
            CommandOption::Comment(_) => self.comment.clone(),
            CommandOption::Config(_) => self.config.clone(),
            CommandOption::Cve(_) => self.cve.clone(),
            CommandOption::Debuglevel(_) => self.debuglevel.clone(),
            CommandOption::Debugsolver => self.debugsolver.clone(),
            CommandOption::Disable => self.disable.clone(),
            CommandOption::Disableexcludes(_) => self.disableexcludes.clone(),
            CommandOption::Disableplugin(_) => self.disableplugin.clone(),
            CommandOption::Disablerepo(_) => self.disablerepo.clone(),
            CommandOption::Downloaddir(_) => self.downloaddir.clone(),
            CommandOption::Downloadonly => self.downloadonly.clone(),
            CommandOption::Enable => self.enable.clone(),
            CommandOption::Enableplugin(_) => self.enableplugin.clone(),
            CommandOption::Enablerepo(_) => self.enablerepo.clone(),
            CommandOption::Enhancement => self.enhancement.clone(),
            CommandOption::Errorlevel(_) => self.errorlevel.clone(),
            CommandOption::Exclude(_) => self.exclude.clone(),
            CommandOption::Excludepkgs(_) => self.excludepkgs.clone(),
            CommandOption::Forcearch(_) => self.forcearch.clone(),
            CommandOption::Help => self.help.clone(),
            CommandOption::Installroot(_) => self.installroot.clone(),
            CommandOption::Ipv4 => self.ipv4.clone(),
            CommandOption::Ipv6 => self.ipv6.clone(),
            CommandOption::Newpackage => self.newpackage.clone(),
            CommandOption::Noautoremove => self.noautoremove.clone(),
            CommandOption::Nobest => self.nobest.clone(),
            CommandOption::Nodocs => self.nodocs.clone(),
            CommandOption::Nogpgcheck => self.nogpgcheck.clone(),
            CommandOption::Noplugins => self.noplugins.clone(),
            CommandOption::Obsoletes => self.obsoletes.clone(),
            CommandOption::Quiet => self.quiet.clone(),
            CommandOption::Randomwait(_) => self.randomwait.clone(),
            CommandOption::Refresh => self.refresh.clone(),
            CommandOption::Releasever(_) => self.releasever.clone(),
            CommandOption::Repo(_) => self.repo.clone(),
            CommandOption::Repofrompath(_) => self.repofrompath.clone(),
            CommandOption::Rpmverbosity(_) => self.rpmverbosity.clone(),
            CommandOption::Secseverity(_) => self.secseverity.clone(),
            CommandOption::Security => self.security.clone(),
            CommandOption::Setopt(_) => self.setopt.clone(),
            CommandOption::Showduplicates => self.showduplicates.clone(),
            CommandOption::Skipbroken => self.skipbroken.clone(),
            CommandOption::Verbose => self.verbose.clone(),
            CommandOption::Version => self.version.clone(),
        }
    }

    pub fn set(&mut self, command_option: &CommandOption) {
        match command_option {
            CommandOption::Advisory(_) =>
                self.advisory = Some(command_option.clone()),
            CommandOption::Allowerasing =>
                self.allowerasing = Some(command_option.clone()),
            CommandOption::Assumeno =>
                self.assumeno = Some(command_option.clone()),
            CommandOption::Assumeyes =>
                self.assumeyes = Some(command_option.clone()),
            CommandOption::Best =>
                self.best = Some(command_option.clone()),
            CommandOption::Bugfix =>
                self.bugfix = Some(command_option.clone()),
            CommandOption::Bz(_) =>
                self.bz = Some(command_option.clone()),
            CommandOption::Cacheonly =>
                self.cacheonly = Some(command_option.clone()),
            CommandOption::Color(_) =>
                self.color = Some(command_option.clone()),
            CommandOption::Comment(_) =>
                self.comment = Some(command_option.clone()),
            CommandOption::Config(_) =>
                self.config = Some(command_option.clone()),
            CommandOption::Cve(_) =>
                self.cve = Some(command_option.clone()),
            CommandOption::Debuglevel(_) =>
                self.debuglevel = Some(command_option.clone()),
            CommandOption::Debugsolver =>
                self.debugsolver = Some(command_option.clone()),
            CommandOption::Disable =>
                self.disable = Some(command_option.clone()),
            CommandOption::Disableexcludes(_) =>
                self.disableexcludes = Some(command_option.clone()),
            CommandOption::Disableplugin(_) =>
                self.disableplugin = Some(command_option.clone()),
            CommandOption::Disablerepo(_) =>
                self.disablerepo = Some(command_option.clone()),
            CommandOption::Downloaddir(_) =>
                self.downloaddir = Some(command_option.clone()),
            CommandOption::Downloadonly =>
                self.downloadonly = Some(command_option.clone()),
            CommandOption::Enable =>
                self.enable = Some(command_option.clone()),
            CommandOption::Enableplugin(_) =>
                self.enableplugin = Some(command_option.clone()),
            CommandOption::Enablerepo(_) =>
                self.enablerepo = Some(command_option.clone()),
            CommandOption::Enhancement =>
                self.enhancement = Some(command_option.clone()),
            CommandOption::Errorlevel(_) =>
                self.errorlevel = Some(command_option.clone()),
            CommandOption::Exclude(_) =>
                self.exclude = Some(command_option.clone()),
            CommandOption::Excludepkgs(_) =>
                self.excludepkgs = Some(command_option.clone()),
            CommandOption::Forcearch(_) =>
                self.forcearch = Some(command_option.clone()),
            CommandOption::Help =>
                self.help = Some(command_option.clone()),
            CommandOption::Installroot(_) =>
                self.installroot = Some(command_option.clone()),
            CommandOption::Ipv4 =>
                self.ipv4 = Some(command_option.clone()),
            CommandOption::Ipv6 =>
                self.ipv6 = Some(command_option.clone()),
            CommandOption::Newpackage =>
                self.newpackage = Some(command_option.clone()),
            CommandOption::Noautoremove =>
                self.noautoremove = Some(command_option.clone()),
            CommandOption::Nobest =>
                self.nobest = Some(command_option.clone()),
            CommandOption::Nodocs =>
                self.nodocs = Some(command_option.clone()),
            CommandOption::Nogpgcheck =>
                self.nogpgcheck = Some(command_option.clone()),
            CommandOption::Noplugins =>
                self.noplugins = Some(command_option.clone()),
            CommandOption::Obsoletes =>
                self.obsoletes = Some(command_option.clone()),
            CommandOption::Quiet =>
                self.quiet = Some(command_option.clone()),
            CommandOption::Randomwait(_) =>
                self.randomwait = Some(command_option.clone()),
            CommandOption::Refresh =>
                self.refresh = Some(command_option.clone()),
            CommandOption::Releasever(_) =>
                self.releasever = Some(command_option.clone()),
            CommandOption::Repo(_) =>
                self.repo = Some(command_option.clone()),
            CommandOption::Repofrompath(_) =>
                self.repofrompath = Some(command_option.clone()),
            CommandOption::Rpmverbosity(_) =>
                self.rpmverbosity = Some(command_option.clone()),
            CommandOption::Secseverity(_) =>
                self.secseverity = Some(command_option.clone()),
            CommandOption::Security =>
                self.security = Some(command_option.clone()),
            CommandOption::Setopt(_) =>
                self.setopt = Some(command_option.clone()),
            CommandOption::Showduplicates =>
                self.showduplicates = Some(command_option.clone()),
            CommandOption::Skipbroken =>
                self.skipbroken = Some(command_option.clone()),
            CommandOption::Verbose =>
                self.verbose = Some(command_option.clone()),
            CommandOption::Version =>
                self.version = Some(command_option.clone()),
        };
    }

    pub fn unset(&mut self, command_option: &CommandOption) {
        match command_option {
            CommandOption::Advisory(_) => self.advisory = None,
            CommandOption::Allowerasing => self.allowerasing = None,
            CommandOption::Assumeno => self.assumeno = None,
            CommandOption::Assumeyes => self.assumeyes = None,
            CommandOption::Best => self.best = None,
            CommandOption::Bugfix => self.bugfix = None,
            CommandOption::Bz(_) => self.bz = None,
            CommandOption::Cacheonly => self.cacheonly = None,
            CommandOption::Color(_) => self.color = None,
            CommandOption::Comment(_) => self.comment = None,
            CommandOption::Config(_) => self.config = None,
            CommandOption::Cve(_) => self.cve = None,
            CommandOption::Debuglevel(_) => self.debuglevel = None,
            CommandOption::Debugsolver => self.debugsolver = None,
            CommandOption::Disable => self.disable = None,
            CommandOption::Disableexcludes(_) => self.disableexcludes = None,
            CommandOption::Disableplugin(_) => self.disableplugin = None,
            CommandOption::Disablerepo(_) => self.disablerepo = None,
            CommandOption::Downloaddir(_) => self.downloaddir = None,
            CommandOption::Downloadonly => self.downloadonly = None,
            CommandOption::Enable => self.enable = None,
            CommandOption::Enableplugin(_) => self.enableplugin = None,
            CommandOption::Enablerepo(_) => self.enablerepo = None,
            CommandOption::Enhancement => self.enhancement = None,
            CommandOption::Errorlevel(_) => self.errorlevel = None,
            CommandOption::Exclude(_) => self.exclude = None,
            CommandOption::Excludepkgs(_) => self.excludepkgs = None,
            CommandOption::Forcearch(_) => self.forcearch = None,
            CommandOption::Help => self.help = None,
            CommandOption::Installroot(_) => self.installroot = None,
            CommandOption::Ipv4 => self.ipv4 = None,
            CommandOption::Ipv6 => self.ipv6 = None,
            CommandOption::Newpackage => self.newpackage = None,
            CommandOption::Noautoremove => self.noautoremove = None,
            CommandOption::Nobest => self.nobest = None,
            CommandOption::Nodocs => self.nodocs = None,
            CommandOption::Nogpgcheck => self.nogpgcheck = None,
            CommandOption::Noplugins => self.noplugins = None,
            CommandOption::Obsoletes => self.obsoletes = None,
            CommandOption::Quiet => self.quiet = None,
            CommandOption::Randomwait(_) => self.randomwait = None,
            CommandOption::Refresh => self.refresh = None,
            CommandOption::Releasever(_) => self.releasever = None,
            CommandOption::Repo(_) => self.repo = None,
            CommandOption::Repofrompath(_) => self.repofrompath = None,
            CommandOption::Rpmverbosity(_) => self.rpmverbosity = None,
            CommandOption::Secseverity(_) => self.secseverity = None,
            CommandOption::Security => self.security = None,
            CommandOption::Setopt(_) => self.setopt = None,
            CommandOption::Showduplicates => self.showduplicates = None,
            CommandOption::Skipbroken => self.skipbroken = None,
            CommandOption::Verbose => self.verbose = None,
            CommandOption::Version => self.version = None,
        }
    }
}

impl Display for CommandOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for command_option in CommandOption::iter() {
            if let Some(value) = self.get(&command_option) {
                if first {
                    write!(f, "{value}")?;
                    first = false;
                } else {
                    write!(f, " {value}")?;
                }
            }
        }

        Ok(())
    }
}
