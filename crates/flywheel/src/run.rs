//! `flywheel run` (design 10): the start's checks, the lock that keeps
//! one runner to a data directory, and its loop: a pass on the timer,
//! each landing it is told of kept at once, and its stop. The loop is the
//! runner's one thread of work, so no two of its git calls touch one
//! clone at once. The runner opens no listening socket: every connection
//! it makes is git's, outgoing (core.3).

use crate::config::{self, Address, Config, Env};
use crate::git::Git;
use crate::pass::{self, Outcome};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::ffi::OsString;
use std::fs::{self, File, TryLockError};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// What the loop is told between passes.
pub enum Event {
    /// A landing on a repository, which is kept at once.
    Landed(Address),
    /// The runner is to stop.
    Stop,
}

/// Tells the runner of landings, from whatever learns of them.
#[derive(Clone)]
pub struct Notices {
    events: Sender<Event>,
}

impl Notices {
    /// Tells the runner of a landing on `address`, which it keeps at once
    /// between passes, leaving its timer where it was. A runner that has
    /// stopped is told nothing.
    pub fn landed(&self, address: Address) {
        let _ = self.events.send(Event::Landed(address));
    }
}

/// Stops the runner: it finishes the git call in hand, keeps no further
/// repository and says it stopped.
#[derive(Clone)]
pub struct Stopper {
    events: Sender<Event>,
    stop: Arc<AtomicBool>,
}

impl Stopper {
    /// Stops the runner, its pass read between repositories and its wait
    /// woken.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = self.events.send(Event::Stop);
    }
}

/// When the pass after one that began at `last_start` begins:
/// `fetch_every` after it, or `now` where that is past (design 10).
pub fn next_pass(last_start: Instant, every: Duration, now: Instant) -> Instant {
    (last_start + every).max(now)
}

/// A runner whose start's checks passed, holding the lock on its data
/// directory until it is dropped.
pub struct Runner {
    git: Git,
    home: PathBuf,
    file: PathBuf,
    data: PathBuf,
    config: Config,
    events: Sender<Event>,
    waiting: Receiver<Event>,
    stop: Arc<AtomicBool>,
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
        let (events, waiting) = mpsc::channel();
        Ok(Runner {
            git,
            home: env.home,
            file,
            data,
            config,
            events,
            waiting,
            stop: Arc::new(AtomicBool::new(false)),
            _lock: lock,
        })
    }

    /// What tells the runner of landings.
    pub fn notices(&self) -> Notices {
        Notices {
            events: self.events.clone(),
        }
    }

    /// What stops the runner.
    pub fn stopper(&self) -> Stopper {
        Stopper {
            events: self.events.clone(),
            stop: Arc::clone(&self.stop),
        }
    }

    /// Stops the runner on SIGTERM or SIGINT, from a thread that waits
    /// for either.
    pub fn stop_on_signals(&self) -> Result<(), String> {
        let mut signals = Signals::new([SIGTERM, SIGINT])
            .map_err(|error| format!("SIGTERM and SIGINT cannot be caught: {error}"))?;
        let stopper = self.stopper();
        thread::spawn(move || {
            for _ in signals.forever() {
                stopper.stop();
            }
        });
        Ok(())
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
        pass::pass(&self.git, &self.data, &self.config, &self.stop, say)
    }

    /// Passes until stopped (design 10): one at once, on the
    /// configuration the runner started with, then one each `fetch_every`
    /// after the start of the one before, the configuration read again
    /// for each. A pass that cannot read it names why and keeps nothing.
    /// Between passes, each landing it is told of is kept at once.
    /// Stopped, it keeps no further repository and says so.
    pub fn keep(&self, say: &mut dyn FnMut(String)) {
        let mut read = Ok(self.config.clone());
        let mut every = self.config.fetch_every;
        'passes: loop {
            let began = Instant::now();
            match read {
                Ok(config) => {
                    every = config.fetch_every;
                    pass::pass(&self.git, &self.data, &config, &self.stop, say);
                }
                Err(why) => say(format!("flywheel: {why}: nothing kept this pass")),
            }
            let due = next_pass(began, every, Instant::now());
            loop {
                if self.stop.load(Ordering::SeqCst) {
                    break 'passes;
                }
                let wait = due.saturating_duration_since(Instant::now());
                match self.waiting.recv_timeout(wait) {
                    Ok(Event::Landed(address)) => self.landed(&address, say),
                    Ok(Event::Stop) => break 'passes,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => {
                        unreachable!("the runner holds a sender of its own")
                    }
                }
            }
            read = config::read(&self.file, &self.home);
        }
        say("flywheel: stopped".to_string());
    }

    /// Keeps `address` at once, told of a landing on it, against the
    /// configuration as it stands: read again, so a repository dropped
    /// since the last pass is never fetched.
    fn landed(&self, address: &Address, say: &mut dyn FnMut(String)) {
        match config::read(&self.file, &self.home) {
            Ok(config) => {
                pass::keep_one(&self.git, &self.data, &config, address, say);
            }
            Err(why) => say(format!(
                "flywheel: {why}: nothing kept at the landing on {address}"
            )),
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
/// pass where `once`, else passes until SIGTERM or SIGINT stops it. Exits
/// 2 where the runner does not start, 1 where the one pass failed, and 0
/// otherwise.
pub fn main(file: Option<&Path>, once: bool) -> ExitCode {
    let started =
        Runner::start(file, |name| std::env::var_os(name), Git::default()).and_then(|runner| {
            if !once {
                runner.stop_on_signals()?;
            }
            Ok(runner)
        });
    let runner = match started {
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
        return ExitCode::SUCCESS;
    }
    match runner.once(&mut say) {
        Outcome::Kept => ExitCode::SUCCESS,
        Outcome::Failed => ExitCode::from(1),
    }
}

#[cfg(test)]
mod tests {
    use super::next_pass;
    use std::time::{Duration, Instant};

    /// The running spec's "Passes on the timer": with `fetch_every` two
    /// minutes and a pass begun at 10:00:00, the next begins at 10:02:00,
    /// or at once when the pass before ends later than that.
    #[test]
    fn passes_on_the_timer() {
        let ten = Instant::now();
        let every = Duration::from_secs(120);
        let due = ten + every;
        assert_eq!(next_pass(ten, every, ten + Duration::from_secs(5)), due);
        assert_eq!(next_pass(ten, every, due), due);
        let late = ten + Duration::from_secs(150);
        assert_eq!(next_pass(ten, every, late), late);
    }
}
