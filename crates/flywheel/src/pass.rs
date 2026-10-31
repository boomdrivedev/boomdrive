//! One pass over the configuration (design 9): each configured
//! repository in the file's order, its clone made where it has none. Each
//! repository is independent: a failure in one is named and the next is
//! kept. The pass says each act and each thing it names as one line,
//! `<owner>/<name>: <words>`, and a pass with nothing to say writes
//! nothing.

use crate::clone;
use crate::config::{Address, Config};
use crate::git::Git;
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

/// Keeps `address`: makes its clone where it has none (design 4).
fn keep(
    git: &Git,
    data: &Path,
    address: &Address,
    found: Option<&Path>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    if found.is_some() {
        return Outcome::Kept;
    }
    let container = clone::container(data, address);
    if container.symlink_metadata().is_ok() {
        say(format!(
            "left: {} is not a clone the runner made, so none is made",
            container.display()
        ));
        return Outcome::Kept;
    }
    match clone::make(git, data, address) {
        Ok(_) => {
            say(format!("cloned from {}", clone::url(address)));
            Outcome::Kept
        }
        Err(failure) => {
            say(format!("not cloned: {failure}"));
            Outcome::Failed
        }
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
