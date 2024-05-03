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
mod opt;
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


use crate::types::{
    color::Color,
    debug_level::DebugLevel,
    error_level::ErrorLevel,
    exclude::Exclude,
};
use opt::{
    Opt,
    OPTS,
};
use std::{
    fmt::Display,
    path::PathBuf,
};


#[derive(Default)]
pub struct Options {
    pub advisory: Option<String>,
    pub allowerasing: bool,
    pub assumeno: bool,
    pub assumeyes: bool,
    pub best: bool,
    pub bugfix: bool,
    pub bz: Option<String>,
    pub cacheonly: bool,
    pub color: Option<Color>,
    pub comment: Option<String>,
    pub config: Option<PathBuf>,
    pub cve: Option<String>,
    pub debuglevel: Option<DebugLevel>,
    pub debugsolver: bool,
    pub disable: bool,
    pub disableexcludes: Option<Exclude>,
    pub disableplugin: Option<String>,
    pub disablerepo: Option<String>,
    pub downloaddir: Option<PathBuf>,
    pub downloadonly: bool,
    pub enable: bool,
    pub enableplugin: Option<String>,
    pub enablerepo: Option<String>,
    pub enhancement: bool,
    pub errorlevel: Option<ErrorLevel>,
    pub exclude: Option<String>,
    pub excludepkgs: Option<String>,
    pub forcearch: Option<String>,
    pub help: bool,
    pub installroot: Option<PathBuf>,
    pub ipv4: bool,
    pub ipv6: bool,
    pub newpackage: bool,
    pub noautoremove: bool,
    pub nobest: bool,
    pub nodocs: bool,
    pub nogpgcheck: bool,
    pub noplugins: bool,
    pub obsoletes: bool,
    pub quiet: bool,
    pub randomwait: Option<u8>,
    pub refresh: bool,
    pub releasever: Option<String>,
    pub repo: Option<String>,
    pub repofrompath: Option<(String, PathBuf)>,
    pub rpmverbosity: Option<String>,
    pub secseverity: Option<String>,
    pub security: bool,
    pub setopt: Option<String>,
    pub showduplicates: bool,
    pub skipbroken: bool,
    pub verbose: bool,
    pub version: bool,
}

impl Options {
    fn extract(&self, opt: &Opt) -> Option<Opt> {
        match opt {
            Opt::Advisory(_) =>
                self
                    .advisory
                    .as_ref()
                    .map(|arg| Opt::Advisory(arg.to_string())),
            Opt::Allowerasing => {
                if self.allowerasing {
                    Some(Opt::Allowerasing)
                } else {
                    None
                }
            },
            Opt::Assumeno => {
                if self.assumeno {
                    Some(Opt::Assumeno)
                } else {
                    None
                }
            },
            Opt::Assumeyes => {
                if self.assumeyes {
                    Some(Opt::Assumeyes)
                } else {
                    None
                }
            },
            Opt::Best => {
                if self.best {
                    Some(Opt::Best)
                } else {
                    None
                }
            },
            Opt::Bugfix => {
                if self.bugfix {
                    Some(Opt::Bugfix)
                } else {
                    None
                }
            },
            Opt::Bz(_) =>
                self
                    .bz
                    .as_ref()
                    .map(|arg| Opt::Bz(arg.to_string())),
            Opt::Cacheonly => {
                if self.cacheonly {
                    Some(Opt::Cacheonly)
                } else {
                    None
                }
            },
            Opt::Color(_) =>
                self
                    .color
                    .as_ref()
                    .map(|arg| Opt::Color(arg.clone())),
            Opt::Comment(_) =>
                self
                    .comment
                    .as_ref()
                    .map(|arg| Opt::Comment(arg.to_string())),
            Opt::Config(_) =>
                self
                    .config
                    .as_ref()
                    .map(|arg| Opt::Config(arg.to_path_buf())),
            Opt::Cve(_) =>
                self
                    .cve
                    .as_ref()
                    .map(|arg| Opt::Cve(arg.to_string())),
            Opt::Debuglevel(_) =>
                self
                    .debuglevel
                    .as_ref()
                    .map(|arg| Opt::Debuglevel(arg.clone())),
            Opt::Debugsolver => {
                if self.debugsolver {
                    Some(Opt::Debugsolver)
                } else {
                    None
                }
            },
            Opt::Disable => {
                if self.disable {
                    Some(Opt::Disable)
                } else {
                    None
                }
            },
            Opt::Disableexcludes(_) =>
                self
                    .disableexcludes
                    .as_ref()
                    .map(|arg| Opt::Disableexcludes(arg.clone())),
            Opt::Disableplugin(_) =>
                self
                    .disableplugin
                    .as_ref()
                    .map(|arg| Opt::Disableplugin(arg.to_string())),
            Opt::Disablerepo(_) =>
                self
                    .disablerepo
                    .as_ref()
                    .map(|arg| Opt::Disablerepo(arg.to_string())),
            Opt::Downloaddir(_) =>
                self
                    .downloaddir
                    .as_ref()
                    .map(|arg| Opt::Downloaddir(arg.to_path_buf())),
            Opt::Downloadonly => {
                if self.downloadonly {
                    Some(Opt::Downloadonly)
                } else {
                    None
                }
            },
            Opt::Enable => {
                if self.enable {
                    Some(Opt::Enable)
                } else {
                    None
                }
            },
            Opt::Enableplugin(_) =>
                self
                    .enableplugin
                    .as_ref()
                    .map(|arg| Opt::Enableplugin(arg.to_string())),
            Opt::Enablerepo(_) =>
                self
                    .enablerepo
                    .as_ref()
                    .map(|arg| Opt::Enablerepo(arg.to_string())),
            Opt::Enhancement => {
                if self.enhancement {
                    Some(Opt::Enhancement)
                } else {
                    None
                }
            },
            Opt::Errorlevel(_) =>
                self
                    .errorlevel
                    .as_ref()
                    .map(|arg| Opt::Errorlevel(arg.clone())),
            Opt::Exclude(_) =>
                self
                    .exclude
                    .as_ref()
                    .map(|arg| Opt::Exclude(arg.to_string())),
            Opt::Excludepkgs(_) =>
                self
                    .excludepkgs
                    .as_ref()
                    .map(|arg| Opt::Excludepkgs(arg.to_string())),
            Opt::Forcearch(_) =>
                self
                    .forcearch
                    .as_ref()
                    .map(|arg| Opt::Forcearch(arg.to_string())),
            Opt::Help => {
                if self.help {
                    Some(Opt::Help)
                } else {
                    None
                }
            },
            Opt::Installroot(_) =>
                self
                    .installroot
                    .as_ref()
                    .map(|arg| Opt::Installroot(arg.to_path_buf())),
            Opt::Ipv4 => {
                if self.ipv4 {
                    Some(Opt::Ipv4)
                } else {
                    None
                }
            },
            Opt::Ipv6 => {
                if self.ipv6 {
                    Some(Opt::Ipv6)
                } else {
                    None
                }
            },
            Opt::Newpackage => {
                if self.newpackage {
                    Some(Opt::Newpackage)
                } else {
                    None
                }
            },
            Opt::Noautoremove => {
                if self.noautoremove {
                    Some(Opt::Noautoremove)
                } else {
                    None
                }
            },
            Opt::Nobest => {
                if self.nobest {
                    Some(Opt::Nobest)
                } else {
                    None
                }
            },
            Opt::Nodocs => {
                if self.nodocs {
                    Some(Opt::Nodocs)
                } else {
                    None
                }
            },
            Opt::Nogpgcheck => {
                if self.nogpgcheck {
                    Some(Opt::Nogpgcheck)
                } else {
                    None
                }
            },
            Opt::Noplugins => {
                if self.noplugins {
                    Some(Opt::Noplugins)
                } else {
                    None
                }
            },
            Opt::Obsoletes => {
                if self.obsoletes {
                    Some(Opt::Obsoletes)
                } else {
                    None
                }
            },
            Opt::Quiet => {
                if self.quiet {
                    Some(Opt::Quiet)
                } else {
                    None
                }
            },
            Opt::Randomwait(_) =>
                self
                    .randomwait
                    .map(|arg| Opt::Randomwait(arg)),
            Opt::Refresh => {
                if self.refresh {
                    Some(Opt::Refresh)
                } else {
                    None
                }
            },
            Opt::Releasever(_) =>
                self
                    .releasever
                    .as_ref()
                    .map(|arg| Opt::Releasever(arg.to_string())),
            Opt::Repo(_) =>
                self
                    .repo
                    .as_ref()
                    .map(|arg| Opt::Repo(arg.to_string())),
            Opt::Repofrompath(_) =>
                self
                    .repofrompath
                    .as_ref()
                    .map(|arg| Opt::Repofrompath(arg.clone())),
            Opt::Rpmverbosity(_) =>
                self
                    .rpmverbosity
                    .as_ref()
                    .map(|arg| Opt::Rpmverbosity(arg.to_string())),
            Opt::Secseverity(_) =>
                self
                    .secseverity
                    .as_ref()
                    .map(|arg| Opt::Secseverity(arg.to_string())),
            Opt::Security => {
                if self.security {
                    Some(Opt::Security)
                } else {
                    None
                }
            },
            Opt::Setopt(_) =>
                self
                    .setopt
                    .as_ref()
                    .map(|arg| Opt::Setopt(arg.to_string())),
            Opt::Showduplicates => {
                if self.showduplicates {
                    Some(Opt::Showduplicates)
                } else {
                    None
                }
            },
            Opt::Skipbroken => {
                if self.skipbroken {
                    Some(Opt::Skipbroken)
                } else {
                    None
                }
            },
            Opt::Verbose => {
                if self.verbose {
                    Some(Opt::Verbose)
                } else {
                    None
                }
            },
            Opt::Version => {
                if self.version {
                    Some(Opt::Version)
                } else {
                    None
                }
            },
        }
    }
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for opt in OPTS.iter() {
            if let Some(value) = self.extract(opt) {
                write!(f, " {value}")?;
            }
        }

        Ok(())
    }
}
