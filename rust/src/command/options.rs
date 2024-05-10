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


use crate::types::{
    color::Color,
    debug_level::DebugLevel,
    error_level::ErrorLevel,
    exclude::Exclude,
};
use lazy_static::lazy_static;
use std::{
    fmt::Display,
    path::PathBuf,
};


pub enum CommandOption {
    Advisory{arg: advisory::Arg},
    Allowerasing,
    Assumeno,
    Assumeyes,
    Best,
    Bugfix,
    Bz{arg: bz::Arg},
    Cacheonly,
    Color{arg: color::Arg},
    Comment{arg: comment::Arg},
    Config{arg: config::Arg},
    Cve{arg: cve::Arg},
    Debuglevel{arg: debuglevel::Arg},
    Debugsolver,
    Disable,
    Disableexcludes{arg: disableexcludes::Arg},
    Disableplugin{arg: disableplugin::Arg},
    Disablerepo{arg: disablerepo::Arg},
    Downloaddir{arg: downloaddir::Arg},
    Downloadonly,
    Enable,
    Enableplugin{arg: enableplugin::Arg},
    Enablerepo{arg: enablerepo::Arg},
    Enhancement,
    Errorlevel{arg: errorlevel::Arg},
    Exclude{arg: exclude::Arg},
    Excludepkgs{arg: excludepkgs::Arg},
    Forcearch{arg: forcearch::Arg},
    Help,
    Installroot{arg: installroot::Arg},
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
    Randomwait{arg: randomwait::Arg},
    Refresh,
    Releasever{arg: releasever::Arg},
    Repo{arg: repo::Arg},
    Repofrompath{arg: repofrompath::Arg},
    Rpmverbosity{arg: rpmverbosity::Arg},
    Secseverity{arg: secseverity::Arg},
    Security,
    Setopt{arg: setopt::Arg},
    Showduplicates,
    Skipbroken,
    Verbose,
    Version,
}

impl CommandOption {
    const fn name(&self) -> &str {
        match self {
            Self::Advisory{..} => advisory::NAME,
            Self::Allowerasing => allowerasing::NAME,
            Self::Assumeno => assumeno::NAME,
            Self::Assumeyes => assumeyes::NAME,
            Self::Best => best::NAME,
            Self::Bugfix => bugfix::NAME,
            Self::Bz{..} => bz::NAME,
            Self::Cacheonly => cacheonly::NAME,
            Self::Color{..} => color::NAME,
            Self::Comment{..} => comment::NAME,
            Self::Config{..} => config::NAME,
            Self::Cve{..} => cve::NAME,
            Self::Debuglevel{..} => debuglevel::NAME,
            Self::Debugsolver => debugsolver::NAME,
            Self::Disable => disable::NAME,
            Self::Disableexcludes{..} => disableexcludes::NAME,
            Self::Disableplugin{..} => disableplugin::NAME,
            Self::Disablerepo{..} => disablerepo::NAME,
            Self::Downloaddir{..} => downloaddir::NAME,
            Self::Downloadonly => downloadonly::NAME,
            Self::Enable => enable::NAME,
            Self::Enableplugin{..} => enableplugin::NAME,
            Self::Enablerepo{..} => enablerepo::NAME,
            Self::Enhancement => enhancement::NAME,
            Self::Errorlevel{..} => errorlevel::NAME,
            Self::Exclude{..} => exclude::NAME,
            Self::Excludepkgs{..} => excludepkgs::NAME,
            Self::Forcearch{..} => forcearch::NAME,
            Self::Help => help::NAME,
            Self::Installroot{..} => installroot::NAME,
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
            Self::Randomwait{..} => randomwait::NAME,
            Self::Refresh => refresh::NAME,
            Self::Releasever{..} => releasever::NAME,
            Self::Repo{..} => repo::NAME,
            Self::Repofrompath{..} => repofrompath::NAME,
            Self::Rpmverbosity{..} => rpmverbosity::NAME,
            Self::Secseverity{..} => secseverity::NAME,
            Self::Security => security::NAME,
            Self::Setopt{..} => setopt::NAME,
            Self::Showduplicates => showduplicates::NAME,
            Self::Skipbroken => skipbroken::NAME,
            Self::Verbose => verbose::NAME,
            Self::Version => version::NAME,
        }
    }

    fn arg(&self) -> Option<String> {
        match self {
            Self::Advisory{arg} => Some(arg.to_string()),
            Self::Color{arg} => Some(arg.to_string()),
            Self::Comment{arg} => Some(arg.to_string()),
            Self::Config{arg} => Some(arg.to_string_lossy().to_string()),
            Self::Cve{arg} => Some(arg.to_string()),
            Self::Debuglevel{arg} => Some(arg.to_string()),
            Self::Disableexcludes{arg} => Some(arg.to_string()),
            Self::Disableplugin{arg} => Some(arg.to_string()),
            Self::Disablerepo{arg} => Some(arg.to_string()),
            Self::Downloaddir{arg} => Some(arg.to_string_lossy().to_string()),
            Self::Enableplugin{arg} => Some(arg.to_string()),
            Self::Enablerepo{arg} => Some(arg.to_string()),
            Self::Errorlevel{arg} => Some(arg.to_string()),
            Self::Exclude{arg} => Some(arg.to_string()),
            Self::Excludepkgs{arg} => Some(arg.to_string()),
            Self::Forcearch{arg} => Some(arg.to_string()),
            Self::Installroot{arg} => Some(arg.to_string_lossy().to_string()),
            Self::Randomwait{arg} => Some(arg.to_string()),
            Self::Releasever{arg} => Some(arg.to_string()),
            Self::Repo{arg} => Some(arg.to_string()),
            //Self::Repofrompath(){arg} => Some(arg.to_string()),
            Self::Rpmverbosity{arg} => Some(arg.to_string()),
            Self::Secseverity{arg} => Some(arg.to_string()),
            Self::Setopt{arg} => Some(arg.to_string()),
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


lazy_static! {
    pub static ref COMMAND_OPTIONS: [CommandOption; 53] = [
        CommandOption::Advisory{arg: advisory::Arg::default()},
        CommandOption::Allowerasing,
        CommandOption::Assumeno,
        CommandOption::Assumeyes,
        CommandOption::Best,
        CommandOption::Bugfix,
        CommandOption::Bz{arg: bz::Arg::default()},
        CommandOption::Cacheonly,
        CommandOption::Color{arg: color::Arg::default()},
        CommandOption::Comment{arg: comment::Arg::default()},
        CommandOption::Config{arg: config::Arg::default()},
        CommandOption::Cve{arg: cve::Arg::default()},
        CommandOption::Debuglevel{arg: debuglevel::Arg::default()},
        CommandOption::Debugsolver,
        CommandOption::Disable,
        CommandOption::Disableexcludes{arg: disableexcludes::Arg::default()},
        CommandOption::Disableplugin{arg: disableplugin::Arg::default()},
        CommandOption::Disablerepo{arg: disablerepo::Arg::default()},
        CommandOption::Downloaddir{arg: downloaddir::Arg::default()},
        CommandOption::Downloadonly,
        CommandOption::Enable,
        CommandOption::Enableplugin{arg: enableplugin::Arg::default()},
        CommandOption::Enablerepo{arg: enablerepo::Arg::default()},
        CommandOption::Enhancement,
        CommandOption::Errorlevel{arg: errorlevel::Arg::default()},
        CommandOption::Exclude{arg: exclude::Arg::default()},
        CommandOption::Excludepkgs{arg: excludepkgs::Arg::default()},
        CommandOption::Forcearch{arg: forcearch::Arg::default()},
        CommandOption::Help,
        CommandOption::Installroot{arg: installroot::Arg::default()},
        CommandOption::Ipv4,
        CommandOption::Ipv6,
        CommandOption::Newpackage,
        CommandOption::Noautoremove,
        CommandOption::Nobest,
        CommandOption::Nodocs,
        CommandOption::Nogpgcheck,
        CommandOption::Noplugins,
        CommandOption::Obsoletes,
        CommandOption::Quiet,
        CommandOption::Randomwait{arg: randomwait::Arg::default()},
        CommandOption::Refresh,
        CommandOption::Releasever{arg: releasever::Arg::default()},
        CommandOption::Repo{arg: repo::Arg::default()},
        CommandOption::Repofrompath{arg: repofrompath::Arg::default()},
        CommandOption::Rpmverbosity{arg: rpmverbosity::Arg::default()},
        CommandOption::Secseverity{arg: secseverity::Arg::default()},
        CommandOption::Security,
        CommandOption::Setopt{arg: setopt::Arg::default()},
        CommandOption::Showduplicates,
        CommandOption::Skipbroken,
        CommandOption::Verbose,
        CommandOption::Version,
    ];
}


#[derive(Default)]
pub struct CommandOptions {
    advisory: Option<String>,
    allowerasing: bool,
    assumeno: bool,
    assumeyes: bool,
    best: bool,
    bugfix: bool,
    bz: Option<String>,
    cacheonly: bool,
    color: Option<Color>,
    comment: Option<String>,
    config: Option<PathBuf>,
    cve: Option<String>,
    debuglevel: Option<DebugLevel>,
    debugsolver: bool,
    disable: bool,
    disableexcludes: Option<Exclude>,
    disableplugin: Option<String>,
    disablerepo: Option<String>,
    downloaddir: Option<PathBuf>,
    downloadonly: bool,
    enable: bool,
    enableplugin: Option<String>,
    enablerepo: Option<String>,
    enhancement: bool,
    errorlevel: Option<ErrorLevel>,
    exclude: Option<String>,
    excludepkgs: Option<String>,
    forcearch: Option<String>,
    help: bool,
    installroot: Option<PathBuf>,
    ipv4: bool,
    ipv6: bool,
    newpackage: bool,
    noautoremove: bool,
    nobest: bool,
    nodocs: bool,
    nogpgcheck: bool,
    noplugins: bool,
    obsoletes: bool,
    quiet: bool,
    randomwait: Option<u8>,
    refresh: bool,
    releasever: Option<String>,
    repo: Option<String>,
    repofrompath: Option<(String, PathBuf)>,
    rpmverbosity: Option<String>,
    secseverity: Option<String>,
    security: bool,
    setopt: Option<String>,
    showduplicates: bool,
    skipbroken: bool,
    verbose: bool,
    version: bool,
}

impl CommandOptions {
    fn get(&self, command_option: &CommandOption) -> Option<CommandOption> {
        match command_option {
            CommandOption::Advisory{..} =>
                self
                    .advisory
                    .as_ref()
                    .map(|arg| CommandOption::Advisory{arg: arg.to_string()}),
            CommandOption::Allowerasing => {
                if self.allowerasing {
                    Some(CommandOption::Allowerasing)
                } else {
                    None
                }
            },
            CommandOption::Assumeno => {
                if self.assumeno {
                    Some(CommandOption::Assumeno)
                } else {
                    None
                }
            },
            CommandOption::Assumeyes => {
                if self.assumeyes {
                    Some(CommandOption::Assumeyes)
                } else {
                    None
                }
            },
            CommandOption::Best => {
                if self.best {
                    Some(CommandOption::Best)
                } else {
                    None
                }
            },
            CommandOption::Bugfix => {
                if self.bugfix {
                    Some(CommandOption::Bugfix)
                } else {
                    None
                }
            },
            CommandOption::Bz{..} =>
                self
                    .bz
                    .as_ref()
                    .map(|arg| CommandOption::Bz{arg: arg.to_string()}),
            CommandOption::Cacheonly => {
                if self.cacheonly {
                    Some(CommandOption::Cacheonly)
                } else {
                    None
                }
            },
            CommandOption::Color{..} =>
                self
                    .color
                    .as_ref()
                    .map(|arg| CommandOption::Color{arg: arg.clone()}),
            CommandOption::Comment{..} =>
                self
                    .comment
                    .as_ref()
                    .map(|arg| CommandOption::Comment{arg: arg.to_string()}),
            CommandOption::Config{..} =>
                self
                    .config
                    .as_ref()
                    .map(|arg| CommandOption::Config{arg: arg.to_path_buf()}),
            CommandOption::Cve{..} =>
                self
                    .cve
                    .as_ref()
                    .map(|arg| CommandOption::Cve{arg: arg.to_string()}),
            CommandOption::Debuglevel{..} =>
                self
                    .debuglevel
                    .as_ref()
                    .map(|arg| CommandOption::Debuglevel{arg: arg.clone()}),
            CommandOption::Debugsolver => {
                if self.debugsolver {
                    Some(CommandOption::Debugsolver)
                } else {
                    None
                }
            },
            CommandOption::Disable => {
                if self.disable {
                    Some(CommandOption::Disable)
                } else {
                    None
                }
            },
            CommandOption::Disableexcludes{..} =>
                self
                    .disableexcludes
                    .as_ref()
                    .map(|arg| CommandOption::Disableexcludes{arg: arg.clone()}),
            CommandOption::Disableplugin{..} =>
                self
                    .disableplugin
                    .as_ref()
                    .map(|arg| CommandOption::Disableplugin{arg: arg.to_string()}),
            CommandOption::Disablerepo{..} =>
                self
                    .disablerepo
                    .as_ref()
                    .map(|arg| CommandOption::Disablerepo{arg: arg.to_string()}),
            CommandOption::Downloaddir{..} =>
                self
                    .downloaddir
                    .as_ref()
                    .map(|arg| CommandOption::Downloaddir{arg: arg.to_path_buf()}),
            CommandOption::Downloadonly => {
                if self.downloadonly {
                    Some(CommandOption::Downloadonly)
                } else {
                    None
                }
            },
            CommandOption::Enable => {
                if self.enable {
                    Some(CommandOption::Enable)
                } else {
                    None
                }
            },
            CommandOption::Enableplugin{..} =>
                self
                    .enableplugin
                    .as_ref()
                    .map(|arg| CommandOption::Enableplugin{arg: arg.to_string()}),
            CommandOption::Enablerepo{..} =>
                self
                    .enablerepo
                    .as_ref()
                    .map(|arg| CommandOption::Enablerepo{arg: arg.to_string()}),
            CommandOption::Enhancement => {
                if self.enhancement {
                    Some(CommandOption::Enhancement)
                } else {
                    None
                }
            },
            CommandOption::Errorlevel{..} =>
                self
                    .errorlevel
                    .as_ref()
                    .map(|arg| CommandOption::Errorlevel{arg: arg.clone()}),
            CommandOption::Exclude{..} =>
                self
                    .exclude
                    .as_ref()
                    .map(|arg| CommandOption::Exclude{arg: arg.to_string()}),
            CommandOption::Excludepkgs{..} =>
                self
                    .excludepkgs
                    .as_ref()
                    .map(|arg| CommandOption::Excludepkgs{arg: arg.to_string()}),
            CommandOption::Forcearch{..} =>
                self
                    .forcearch
                    .as_ref()
                    .map(|arg| CommandOption::Forcearch{arg: arg.to_string()}),
            CommandOption::Help => {
                if self.help {
                    Some(CommandOption::Help)
                } else {
                    None
                }
            },
            CommandOption::Installroot{..} =>
                self
                    .installroot
                    .as_ref()
                    .map(|arg| CommandOption::Installroot{arg: arg.to_path_buf()}),
            CommandOption::Ipv4 => {
                if self.ipv4 {
                    Some(CommandOption::Ipv4)
                } else {
                    None
                }
            },
            CommandOption::Ipv6 => {
                if self.ipv6 {
                    Some(CommandOption::Ipv6)
                } else {
                    None
                }
            },
            CommandOption::Newpackage => {
                if self.newpackage {
                    Some(CommandOption::Newpackage)
                } else {
                    None
                }
            },
            CommandOption::Noautoremove => {
                if self.noautoremove {
                    Some(CommandOption::Noautoremove)
                } else {
                    None
                }
            },
            CommandOption::Nobest => {
                if self.nobest {
                    Some(CommandOption::Nobest)
                } else {
                    None
                }
            },
            CommandOption::Nodocs => {
                if self.nodocs {
                    Some(CommandOption::Nodocs)
                } else {
                    None
                }
            },
            CommandOption::Nogpgcheck => {
                if self.nogpgcheck {
                    Some(CommandOption::Nogpgcheck)
                } else {
                    None
                }
            },
            CommandOption::Noplugins => {
                if self.noplugins {
                    Some(CommandOption::Noplugins)
                } else {
                    None
                }
            },
            CommandOption::Obsoletes => {
                if self.obsoletes {
                    Some(CommandOption::Obsoletes)
                } else {
                    None
                }
            },
            CommandOption::Quiet => {
                if self.quiet {
                    Some(CommandOption::Quiet)
                } else {
                    None
                }
            },
            CommandOption::Randomwait{..} =>
                self
                    .randomwait
                    .map(|arg| CommandOption::Randomwait{arg}),
            CommandOption::Refresh => {
                if self.refresh {
                    Some(CommandOption::Refresh)
                } else {
                    None
                }
            },
            CommandOption::Releasever{..} =>
                self
                    .releasever
                    .as_ref()
                    .map(|arg| CommandOption::Releasever{arg: arg.to_string()}),
            CommandOption::Repo{..} =>
                self
                    .repo
                    .as_ref()
                    .map(|arg| CommandOption::Repo{arg: arg.to_string()}),
            CommandOption::Repofrompath{..} =>
                self
                    .repofrompath
                    .as_ref()
                    .map(|arg| CommandOption::Repofrompath{arg: arg.clone()}),
            CommandOption::Rpmverbosity{..} =>
                self
                    .rpmverbosity
                    .as_ref()
                    .map(|arg| CommandOption::Rpmverbosity{arg: arg.to_string()}),
            CommandOption::Secseverity{..} =>
                self
                    .secseverity
                    .as_ref()
                    .map(|arg| CommandOption::Secseverity{arg: arg.to_string()}),
            CommandOption::Security => {
                if self.security {
                    Some(CommandOption::Security)
                } else {
                    None
                }
            },
            CommandOption::Setopt{..} =>
                self
                    .setopt
                    .as_ref()
                    .map(|arg| CommandOption::Setopt{arg: arg.to_string()}),
            CommandOption::Showduplicates => {
                if self.showduplicates {
                    Some(CommandOption::Showduplicates)
                } else {
                    None
                }
            },
            CommandOption::Skipbroken => {
                if self.skipbroken {
                    Some(CommandOption::Skipbroken)
                } else {
                    None
                }
            },
            CommandOption::Verbose => {
                if self.verbose {
                    Some(CommandOption::Verbose)
                } else {
                    None
                }
            },
            CommandOption::Version => {
                if self.version {
                    Some(CommandOption::Version)
                } else {
                    None
                }
            },
        }
    }

    pub fn set(&mut self, command_option: &CommandOption) {
        match command_option {
            CommandOption::Advisory{arg} => self.advisory = Some(arg.to_string()),
            CommandOption::Allowerasing => self.allowerasing = true,
            CommandOption::Assumeno => self.assumeno = true,
            CommandOption::Assumeyes => self.assumeyes = true,
            CommandOption::Best => self.best = true,
            CommandOption::Bugfix => self.bugfix = true,
            CommandOption::Bz{arg} => self.bz = Some(arg.to_string()),
            CommandOption::Cacheonly => self.cacheonly = true,
            CommandOption::Color{arg} => self.color = Some(arg.clone()),
            CommandOption::Comment{arg} => self.comment = Some(arg.to_string()),
            CommandOption::Config{arg} => self.config = Some(arg.to_path_buf()),
            CommandOption::Cve{arg} => self.cve = Some(arg.to_string()),
            CommandOption::Debuglevel{arg} => self.debuglevel = Some(arg.clone()),
            CommandOption::Debugsolver => self.debugsolver = true,
            CommandOption::Disable => self.disable = true,
            CommandOption::Disableexcludes{arg} => self.disableexcludes = Some(arg.clone()),
            CommandOption::Disableplugin{arg} => self.disableplugin = Some(arg.to_string()),
            CommandOption::Disablerepo{arg} => self.disablerepo = Some(arg.to_string()),
            CommandOption::Downloaddir{arg} => self.downloaddir = Some(arg.to_path_buf()),
            CommandOption::Downloadonly => self.downloadonly = true,
            CommandOption::Enable => self.enable = true,
            CommandOption::Enableplugin{arg} => self.enableplugin = Some(arg.to_string()),
            CommandOption::Enablerepo{arg} => self.enablerepo = Some(arg.to_string()),
            CommandOption::Enhancement => self.enhancement = true,
            CommandOption::Errorlevel{arg} => self.errorlevel = Some(arg.clone()),
            CommandOption::Exclude{arg} => self.exclude = Some(arg.to_string()),
            CommandOption::Excludepkgs{arg} => self.excludepkgs = Some(arg.to_string()),
            CommandOption::Forcearch{arg} => self.forcearch = Some(arg.to_string()),
            CommandOption::Help => self.help = true,
            CommandOption::Installroot{arg} => self.installroot = Some(arg.to_path_buf()),
            CommandOption::Ipv4 => self.ipv4 = true,
            CommandOption::Ipv6 => self.ipv6 = true,
            CommandOption::Newpackage => self.newpackage = true,
            CommandOption::Noautoremove => self.noautoremove = true,
            CommandOption::Nobest => self.nobest = true,
            CommandOption::Nodocs => self.nodocs = true,
            CommandOption::Nogpgcheck => self.nogpgcheck = true,
            CommandOption::Noplugins => self.noplugins = true,
            CommandOption::Obsoletes => self.obsoletes = true,
            CommandOption::Quiet => self.quiet = true,
            CommandOption::Randomwait{arg} => self.randomwait = Some(*arg),
            CommandOption::Refresh => self.refresh = true,
            CommandOption::Releasever{arg} => self.releasever = Some(arg.to_string()),
            CommandOption::Repo{arg} => self.repo = Some(arg.to_string()),
            CommandOption::Repofrompath{arg} => self.repofrompath = Some(arg.clone()),
            CommandOption::Rpmverbosity{arg} => self.rpmverbosity = Some(arg.to_string()),
            CommandOption::Secseverity{arg} => self.secseverity = Some(arg.to_string()),
            CommandOption::Security => self.security = true,
            CommandOption::Setopt{arg} => self.setopt = Some(arg.to_string()),
            CommandOption::Showduplicates => self.showduplicates = true,
            CommandOption::Skipbroken => self.skipbroken = true,
            CommandOption::Verbose => self.verbose = true,
            CommandOption::Version => self.version = true,
        }
    }

    pub fn unset(&mut self, command_option: &CommandOption) {
        match command_option {
            CommandOption::Advisory{..} => self.advisory = None,
            CommandOption::Allowerasing => self.allowerasing = false,
            CommandOption::Assumeno => self.assumeno = false,
            CommandOption::Assumeyes => self.assumeyes = false,
            CommandOption::Best => self.best = false,
            CommandOption::Bugfix => self.bugfix = false,
            CommandOption::Bz{..} => self.bz = None,
            CommandOption::Cacheonly => self.cacheonly = false,
            CommandOption::Color{..} => self.color = None,
            CommandOption::Comment{..} => self.comment = None,
            CommandOption::Config{..} => self.config = None,
            CommandOption::Cve{..} => self.cve = None,
            CommandOption::Debuglevel{..} => self.debuglevel = None,
            CommandOption::Debugsolver => self.debugsolver = false,
            CommandOption::Disable => self.disable = false,
            CommandOption::Disableexcludes{..} => self.disableexcludes = None,
            CommandOption::Disableplugin{..} => self.disableplugin = None,
            CommandOption::Disablerepo{..} => self.disablerepo = None,
            CommandOption::Downloaddir{..} => self.downloaddir = None,
            CommandOption::Downloadonly => self.downloadonly = false,
            CommandOption::Enable => self.enable = false,
            CommandOption::Enableplugin{..} => self.enableplugin = None,
            CommandOption::Enablerepo{..} => self.enablerepo = None,
            CommandOption::Enhancement => self.enhancement = false,
            CommandOption::Errorlevel{..} => self.errorlevel = None,
            CommandOption::Exclude{..} => self.exclude = None,
            CommandOption::Excludepkgs{..} => self.excludepkgs = None,
            CommandOption::Forcearch{..} => self.forcearch = None,
            CommandOption::Help => self.help = false,
            CommandOption::Installroot{..} => self.installroot = None,
            CommandOption::Ipv4 => self.ipv4 = false,
            CommandOption::Ipv6 => self.ipv6 = false,
            CommandOption::Newpackage => self.newpackage = false,
            CommandOption::Noautoremove => self.noautoremove = false,
            CommandOption::Nobest => self.nobest = false,
            CommandOption::Nodocs => self.nodocs = false,
            CommandOption::Nogpgcheck => self.nogpgcheck = false,
            CommandOption::Noplugins => self.noplugins = false,
            CommandOption::Obsoletes => self.obsoletes = false,
            CommandOption::Quiet => self.quiet = false,
            CommandOption::Randomwait{..} => self.randomwait = None,
            CommandOption::Refresh => self.refresh = false,
            CommandOption::Releasever{..} => self.releasever = None,
            CommandOption::Repo{..} => self.repo = None,
            CommandOption::Repofrompath{..} => self.repofrompath = None,
            CommandOption::Rpmverbosity{..} => self.rpmverbosity = None,
            CommandOption::Secseverity{..} => self.secseverity = None,
            CommandOption::Security => self.security = false,
            CommandOption::Setopt{..} => self.setopt = None,
            CommandOption::Showduplicates => self.showduplicates = false,
            CommandOption::Skipbroken => self.skipbroken = false,
            CommandOption::Verbose => self.verbose = false,
            CommandOption::Version => self.version = false,
        }
    }
}

impl Display for CommandOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for command_option in COMMAND_OPTIONS.iter() {
            if let Some(value) = self.get(command_option) {
                write!(f, " {value}")?;
            }
        }

        Ok(())
    }
}
