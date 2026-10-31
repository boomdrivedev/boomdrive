//! One pass over the configuration (design 9): each configured
//! repository in the file's order, its clone made or fetched, its HEAD
//! settled and its own default branch levelled. Each repository is
//! independent: a failure in one is named and the next is kept. The pass
//! says each act and each thing it names as one line,
//! `<owner>/<name>: <words>`, and a pass with nothing to say writes
//! nothing.

use crate::clone::{self, LevelAct};
use crate::config::{Address, Config};
use crate::git::{Git, Words};
use std::fs;
use std::path::{Path, PathBuf};

/// Whether every git call of a pass succeeded. Something left and named
/// by rule, a folder the runner did not make among it, is not a failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Every call succeeded.
    Kept,
    /// At least one failed, and was named.
    Failed,
}

impl Outcome {
    /// Failed where either is.
    fn and(self, other: Outcome) -> Outcome {
        if self == Outcome::Failed {
            self
        } else {
            other
        }
    }
}

/// One pass over `config`, the runner's clones kept in `data`, each line
/// said to `say` as it is said.
pub fn pass(git: &Git, data: &Path, config: &Config, say: &mut dyn FnMut(String)) -> Outcome {
    let clones = match clone::find(git, data) {
        Ok(clones) => clones,
        Err(error) => {
            say(format!(
                "flywheel: {} cannot be read: {error}",
                clone::repositories(data).display()
            ));
            return Outcome::Failed;
        }
    };
    let mut outcome = Outcome::Kept;
    for address in &config.repositories {
        let found = clones
            .iter()
            .find(|(kept, _)| kept == address)
            .map(|(_, container)| container.as_path());
        let kept = keep(git, data, address, found, &mut |words| {
            say(format!("{address}: {words}"))
        });
        outcome = outcome.and(kept);
    }
    outcome.and(clear_partials(data, &clones, say))
}

/// Keeps `address`: makes its clone where it has none (design 4), else
/// fetches the one `found` (design 5), then settles its HEAD on origin's
/// default branch and levels its own (design 6).
fn keep(
    git: &Git,
    data: &Path,
    address: &Address,
    found: Option<&Path>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let container = match found {
        Some(container) => {
            if let Err(failure) = clone::fetch(git, &clone::bare(container)) {
                say(format!("not fetched: {failure}"));
                return Outcome::Failed;
            }
            container.to_path_buf()
        }
        None => {
            let container = clone::container(data, address);
            if container.symlink_metadata().is_ok() {
                say(format!(
                    "left: {} is not a clone the runner made, so none is made",
                    container.display()
                ));
                return Outcome::Kept;
            }
            match clone::make(git, data, address) {
                Ok(container) => {
                    say(format!("cloned from {}", clone::url(address)));
                    container
                }
                Err(failure) => {
                    say(format!("not cloned: {failure}"));
                    return Outcome::Failed;
                }
            }
        }
    };
    match settle(git, &clone::bare(&container), say) {
        Ok(_) => Outcome::Kept,
        Err(words) => {
            say(format!("default branch not kept: {words}"));
            Outcome::Failed
        }
    }
}

/// Settles the clone's HEAD on origin's default branch and levels the
/// clone's own (design 6). Gives the default, none where origin names
/// none.
fn settle(git: &Git, bare: &Path, say: &mut dyn FnMut(String)) -> Result<Option<String>, Words> {
    let Some(default) = clone::default_of(git, bare)? else {
        say("origin names no default branch, so it has no view".to_string());
        return Ok(None);
    };
    if clone::settle_head(git, bare, &default)? {
        say(format!("HEAD names {default}, origin's default branch"));
    }
    let facts = clone::level_facts(git, bare, &default)?;
    match clone::level(&facts) {
        LevelAct::Level => {}
        LevelAct::Move => {
            clone::level_to(git, bare, &default, &facts)?;
            say(format!(
                "{default} moved to origin/{default} at {}",
                short(&facts.origin)
            ));
        }
        LevelAct::Ahead(ahead) => say(format!(
            "{default} left: {} ahead of origin/{default}",
            commits(ahead)
        )),
        LevelAct::Held(worktree) => say(format!(
            "{default} left: checked out in the worktree {}",
            worktree.display()
        )),
    }
    Ok(Some(default))
}

/// A commit as a line gives it, at seven characters.
fn short(commit: &str) -> &str {
    &commit[..commit.len().min(7)]
}

/// `n` commits, in words.
fn commits(n: u32) -> String {
    match n {
        1 => "1 commit".to_string(),
        n => format!("{n} commits"),
    }
}

/// Removes each `.<name>.partial` folder an earlier run left in `data`,
/// other than one of `clones`, a repository's own name ending so
/// (design 9's step 5).
fn clear_partials(
    data: &Path,
    clones: &[(Address, PathBuf)],
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Ok(owners) = fs::read_dir(clone::repositories(data)) else {
        return Outcome::Kept;
    };
    let mut outcome = Outcome::Kept;
    for owner in owners.flatten() {
        let Ok(folders) = fs::read_dir(owner.path()) else {
            continue;
        };
        for folder in folders.flatten() {
            let path = folder.path();
            if !clone::is_partial(&folder.file_name())
                || clones.iter().any(|(_, container)| *container == path)
            {
                continue;
            }
            match fs::remove_dir_all(&path) {
                Ok(()) => say(format!(
                    "flywheel: removed {}, a clone left unfinished",
                    path.display()
                )),
                Err(error) => {
                    say(format!(
                        "flywheel: {} is a clone left unfinished and cannot be removed: {error}",
                        path.display()
                    ));
                    outcome = Outcome::Failed;
                }
            }
        }
    }
    outcome
}
