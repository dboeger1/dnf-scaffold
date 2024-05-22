pub mod command_option;


use command_option::{
    CommandOption,
    CommandOptionDiscriminants,
};
use std::fmt::Display;
use strum::IntoEnumIterator;


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

    pub fn get(
        &self,
        discriminant: &CommandOptionDiscriminants,
    ) -> Option<CommandOption> {
        match discriminant {
            CommandOptionDiscriminants::Advisory =>
                self.advisory.clone(),
            CommandOptionDiscriminants::Allowerasing =>
                self.allowerasing.clone(),
            CommandOptionDiscriminants::Assumeno =>
                self.assumeno.clone(),
            CommandOptionDiscriminants::Assumeyes =>
                self.assumeyes.clone(),
            CommandOptionDiscriminants::Best =>
                self.best.clone(),
            CommandOptionDiscriminants::Bugfix =>
                self.bugfix.clone(),
            CommandOptionDiscriminants::Bz =>
                self.bz.clone(),
            CommandOptionDiscriminants::Cacheonly =>
                self.cacheonly.clone(),
            CommandOptionDiscriminants::Color =>
                self.color.clone(),
            CommandOptionDiscriminants::Comment =>
                self.comment.clone(),
            CommandOptionDiscriminants::Config =>
                self.config.clone(),
            CommandOptionDiscriminants::Cve =>
                self.cve.clone(),
            CommandOptionDiscriminants::Debuglevel =>
                self.debuglevel.clone(),
            CommandOptionDiscriminants::Debugsolver =>
                self.debugsolver.clone(),
            CommandOptionDiscriminants::Disable =>
                self.disable.clone(),
            CommandOptionDiscriminants::Disableexcludes =>
                self.disableexcludes.clone(),
            CommandOptionDiscriminants::Disableplugin =>
                self.disableplugin.clone(),
            CommandOptionDiscriminants::Disablerepo =>
                self.disablerepo.clone(),
            CommandOptionDiscriminants::Downloaddir =>
                self.downloaddir.clone(),
            CommandOptionDiscriminants::Downloadonly =>
                self.downloadonly.clone(),
            CommandOptionDiscriminants::Enable =>
                self.enable.clone(),
            CommandOptionDiscriminants::Enableplugin =>
                self.enableplugin.clone(),
            CommandOptionDiscriminants::Enablerepo =>
                self.enablerepo.clone(),
            CommandOptionDiscriminants::Enhancement =>
                self.enhancement.clone(),
            CommandOptionDiscriminants::Errorlevel =>
                self.errorlevel.clone(),
            CommandOptionDiscriminants::Exclude =>
                self.exclude.clone(),
            CommandOptionDiscriminants::Excludepkgs =>
                self.excludepkgs.clone(),
            CommandOptionDiscriminants::Forcearch =>
                self.forcearch.clone(),
            CommandOptionDiscriminants::Help =>
                self.help.clone(),
            CommandOptionDiscriminants::Installroot =>
                self.installroot.clone(),
            CommandOptionDiscriminants::Ipv4 =>
                self.ipv4.clone(),
            CommandOptionDiscriminants::Ipv6 =>
                self.ipv6.clone(),
            CommandOptionDiscriminants::Newpackage =>
                self.newpackage.clone(),
            CommandOptionDiscriminants::Noautoremove =>
                self.noautoremove.clone(),
            CommandOptionDiscriminants::Nobest =>
                self.nobest.clone(),
            CommandOptionDiscriminants::Nodocs =>
                self.nodocs.clone(),
            CommandOptionDiscriminants::Nogpgcheck =>
                self.nogpgcheck.clone(),
            CommandOptionDiscriminants::Noplugins =>
                self.noplugins.clone(),
            CommandOptionDiscriminants::Obsoletes =>
                self.obsoletes.clone(),
            CommandOptionDiscriminants::Quiet =>
                self.quiet.clone(),
            CommandOptionDiscriminants::Randomwait =>
                self.randomwait.clone(),
            CommandOptionDiscriminants::Refresh =>
                self.refresh.clone(),
            CommandOptionDiscriminants::Releasever =>
                self.releasever.clone(),
            CommandOptionDiscriminants::Repo =>
                self.repo.clone(),
            CommandOptionDiscriminants::Repofrompath =>
                self.repofrompath.clone(),
            CommandOptionDiscriminants::Rpmverbosity =>
                self.rpmverbosity.clone(),
            CommandOptionDiscriminants::Secseverity =>
                self.secseverity.clone(),
            CommandOptionDiscriminants::Security =>
                self.security.clone(),
            CommandOptionDiscriminants::Setopt =>
                self.setopt.clone(),
            CommandOptionDiscriminants::Showduplicates =>
                self.showduplicates.clone(),
            CommandOptionDiscriminants::Skipbroken =>
                self.skipbroken.clone(),
            CommandOptionDiscriminants::Verbose =>
                self.verbose.clone(),
            CommandOptionDiscriminants::Version =>
                self.version.clone(),
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

    pub fn unset(&mut self, discriminant: &CommandOptionDiscriminants) {
        match discriminant {
            CommandOptionDiscriminants::Advisory =>
                self.advisory = None,
            CommandOptionDiscriminants::Allowerasing =>
                self.allowerasing = None,
            CommandOptionDiscriminants::Assumeno =>
                self.assumeno = None,
            CommandOptionDiscriminants::Assumeyes =>
                self.assumeyes = None,
            CommandOptionDiscriminants::Best =>
                self.best = None,
            CommandOptionDiscriminants::Bugfix =>
                self.bugfix = None,
            CommandOptionDiscriminants::Bz =>
                self.bz = None,
            CommandOptionDiscriminants::Cacheonly =>
                self.cacheonly = None,
            CommandOptionDiscriminants::Color =>
                self.color = None,
            CommandOptionDiscriminants::Comment =>
                self.comment = None,
            CommandOptionDiscriminants::Config =>
                self.config = None,
            CommandOptionDiscriminants::Cve =>
                self.cve = None,
            CommandOptionDiscriminants::Debuglevel =>
                self.debuglevel = None,
            CommandOptionDiscriminants::Debugsolver =>
                self.debugsolver = None,
            CommandOptionDiscriminants::Disable =>
                self.disable = None,
            CommandOptionDiscriminants::Disableexcludes =>
                self.disableexcludes = None,
            CommandOptionDiscriminants::Disableplugin =>
                self.disableplugin = None,
            CommandOptionDiscriminants::Disablerepo =>
                self.disablerepo = None,
            CommandOptionDiscriminants::Downloaddir =>
                self.downloaddir = None,
            CommandOptionDiscriminants::Downloadonly =>
                self.downloadonly = None,
            CommandOptionDiscriminants::Enable =>
                self.enable = None,
            CommandOptionDiscriminants::Enableplugin =>
                self.enableplugin = None,
            CommandOptionDiscriminants::Enablerepo =>
                self.enablerepo = None,
            CommandOptionDiscriminants::Enhancement =>
                self.enhancement = None,
            CommandOptionDiscriminants::Errorlevel =>
                self.errorlevel = None,
            CommandOptionDiscriminants::Exclude =>
                self.exclude = None,
            CommandOptionDiscriminants::Excludepkgs =>
                self.excludepkgs = None,
            CommandOptionDiscriminants::Forcearch =>
                self.forcearch = None,
            CommandOptionDiscriminants::Help =>
                self.help = None,
            CommandOptionDiscriminants::Installroot =>
                self.installroot = None,
            CommandOptionDiscriminants::Ipv4 =>
                self.ipv4 = None,
            CommandOptionDiscriminants::Ipv6 =>
                self.ipv6 = None,
            CommandOptionDiscriminants::Newpackage =>
                self.newpackage = None,
            CommandOptionDiscriminants::Noautoremove =>
                self.noautoremove = None,
            CommandOptionDiscriminants::Nobest =>
                self.nobest = None,
            CommandOptionDiscriminants::Nodocs =>
                self.nodocs = None,
            CommandOptionDiscriminants::Nogpgcheck =>
                self.nogpgcheck = None,
            CommandOptionDiscriminants::Noplugins =>
                self.noplugins = None,
            CommandOptionDiscriminants::Obsoletes =>
                self.obsoletes = None,
            CommandOptionDiscriminants::Quiet =>
                self.quiet = None,
            CommandOptionDiscriminants::Randomwait =>
                self.randomwait = None,
            CommandOptionDiscriminants::Refresh =>
                self.refresh = None,
            CommandOptionDiscriminants::Releasever =>
                self.releasever = None,
            CommandOptionDiscriminants::Repo =>
                self.repo = None,
            CommandOptionDiscriminants::Repofrompath =>
                self.repofrompath = None,
            CommandOptionDiscriminants::Rpmverbosity =>
                self.rpmverbosity = None,
            CommandOptionDiscriminants::Secseverity =>
                self.secseverity = None,
            CommandOptionDiscriminants::Security =>
                self.security = None,
            CommandOptionDiscriminants::Setopt =>
                self.setopt = None,
            CommandOptionDiscriminants::Showduplicates =>
                self.showduplicates = None,
            CommandOptionDiscriminants::Skipbroken =>
                self.skipbroken = None,
            CommandOptionDiscriminants::Verbose =>
                self.verbose = None,
            CommandOptionDiscriminants::Version =>
                self.version = None,
        }
    }
}

impl Display for CommandOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for discriminant in CommandOptionDiscriminants::iter() {
            if let Some(value) = self.get(&discriminant) {
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
