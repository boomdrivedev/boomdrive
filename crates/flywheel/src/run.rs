//! `flywheel run` (design 10): the start's checks, the lock that keeps
//! one runner to a data directory, and its passes. The runner opens no
//! listening socket: every connection it makes is git's, outgoing
//! (core.3).

use crate::config::{self, Config, Env};
use crate::git::Git;
use crate::pass::{self, Outcome};
use std::ffi::OsString;
use std::fs::{self, File, TryLockError};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

/// A runner whose start's checks passed, holding the lock on its data
/// directory until it is dropped.
pub struct Runner {
    git: Git,
    home: PathBuf,
    file: PathBuf,
    data: PathBuf,
    config: Config,
    _lock: File,
}

impl Runner {
    /// Starts a runner on the configuration `file` names, else the
    /// default one, in the environment `var` gives. The start's checks
    /// run in design 10's order, and the first that fails refuses with
    /// why, having touched nothing: `HOME` is set, the configuration
    /// reads, git is 2.48 or later, the data directory is there or can be
    /// made, and no other runner holds it.
    pub fn start(
        file: Option<&Path>,
        var: impl Fn(&str) -> Option<OsString>,
        git: Git,
    ) -> Result<Runner, String> {
        let env = Env::read(var)?;
        let file = config::config_path(file, &env);
        let config = config::read(&file, &env.home)?;
        git.check_version()?;
        let data = config::data_dir(&env);
        fs::create_dir_all(&data).map_err(|error| {
            format!(
                "the data directory {} cannot be made: {error}",
                data.display()
            )
        })?;
        let lock = lock(&data)?;
        Ok(Runner {
            git,
            home: env.home,
            file,
            data,
            config,
            _lock: lock,
        })
    }

    /// The line the runner starts with: its version, configuration file,
    /// data directory and views root.
    pub fn started(&self) -> String {
        let kept = match self.config.repositories.len() {
            1 => "1 repository".to_string(),
            n => format!("{n} repositories"),
        };
        format!(
            "flywheel {}: keeping {kept} from {}; data {}; views {}",
            env!("CARGO_PKG_VERSION"),
            self.file.display(),
            self.data.display(),
            self.config.views.display()
        )
    }

    /// One pass over the configuration the runner started with.
    pub fn once(&self, say: &mut dyn FnMut(String)) -> Outcome {
        pass::pass(&self.git, &self.data, &self.config, say)
    }

    /// Passes until the process is stopped, each `fetch_every` after the
    /// start of the one before, the configuration read again for every
    /// pass after the first. A pass that cannot read it names why and
    /// keeps nothing.
    pub fn keep(&self, say: &mut dyn FnMut(String)) -> ! {
        let mut read = Ok(self.config.clone());
        let mut every = self.config.fetch_every;
        loop {
            let began = Instant::now();
            match read {
                Ok(config) => {
                    every = config.fetch_every;
                    pass::pass(&self.git, &self.data, &config, say);
                }
                Err(why) => say(format!("flywheel: {why}: nothing kept this pass")),
            }
            std::thread::sleep(every.saturating_sub(began.elapsed()));
            read = config::read(&self.file, &self.home);
        }
    }
}

/// Holds `<data>/lock`, refused where another runner holds it.
fn lock(data: &Path) -> Result<File, String> {
    let path = data.join("lock");
    let file = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(format!(
            "the data directory {} is held by a running runner",
            data.display()
        )),
        Err(TryLockError::Error(error)) => {
            Err(format!("{} cannot be locked: {error}", path.display()))
        }
    }
}

/// `flywheel run`, on the configuration `file` names, else the default
/// one: starts the runner and says so on standard error, then makes one
/// pass where `once`, else passes until it is stopped. Exits 2 where the
/// runner does not start, and 1 where the one pass failed.
pub fn main(file: Option<&Path>, once: bool) -> ExitCode {
    let runner = match Runner::start(file, |name| std::env::var_os(name), Git::default()) {
        Ok(runner) => runner,
        Err(why) => {
            eprintln!("flywheel: {why}");
            return ExitCode::from(2);
        }
    };
    eprintln!("{}", runner.started());
    let mut say = |line: String| eprintln!("{line}");
    if !once {
        runner.keep(&mut say);
    }
    match runner.once(&mut say) {
        Outcome::Kept => ExitCode::SUCCESS,
        Outcome::Failed => ExitCode::from(1),
    }
}
