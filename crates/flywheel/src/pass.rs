//! One pass over the configuration (design 9): each configured
//! repository in the file's order, its clone made or fetched, its HEAD
//! settled, its own default branch levelled and its view kept, then each
//! dropped repository let go. Each repository is independent: a failure
//! in one is named and the next is kept. The pass says each act and each thing it names as one line,
//! `<owner>/<name>: <words>`, and a pass with nothing to say writes
//! nothing.

use crate::clone::{self, LevelAct};
use crate::config::{self, Address, Config};
use crate::git::{Git, Words};
use crate::lacks;
use crate::view::{self, ViewAct};
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
        let view = config::view_path(config, address);
        let kept = keep(git, data, address, found, &view, &mut |words| {
            say(format!("{address}: {words}"))
        });
        outcome = outcome.and(kept);
    }
    for (address, container) in &clones {
        if config.repositories.contains(address) {
            continue;
        }
        let dropped = let_go(git, container, &mut |words| {
            say(format!("{address}: {words}"))
        });
        outcome = outcome.and(dropped);
    }
    outcome.and(clear_partials(data, &clones, say))
}

/// Lets go of the clone in `container`, whose address the configuration
/// no longer names (design 8). It is never fetched. Once neither it nor
/// its view holds anything the forge lacks, the view is removed, then
/// the view's owner folder if empty, then the container, then the
/// container's owner folder if empty. Until then nothing is touched, and
/// each thing that holds them is named. A folder at the view's path that
/// is not the clone's worktree is left as it is.
fn let_go(git: &Git, container: &Path, say: &mut dyn FnMut(String)) -> Outcome {
    let bare = clone::bare(container);
    let read = view::recorded_path(git, &bare).and_then(|view| {
        let facts = lacks::read(git, &bare, view.as_deref())?;
        Ok((view, facts))
    });
    let (view, facts) = match read {
        Ok(read) => read,
        Err(words) => {
            say(format!("not let go: {words}"));
            return Outcome::Failed;
        }
    };
    let held = lacks::holdings(&facts);
    if !held.is_empty() {
        for holding in held {
            say(format!("left: held by {holding}"));
        }
        return Outcome::Kept;
    }
    let view = view.filter(|_| facts.view.is_some());
    if let Some(view) = &view {
        if let Err(words) = view::remove(git, &bare, view) {
            say(format!("view not removed: {words}"));
            return Outcome::Failed;
        }
    }
    if let Err(error) = fs::remove_dir_all(container) {
        say(format!(
            "clone not removed: {}: {error}",
            container.display()
        ));
        return Outcome::Failed;
    }
    if let Some(owner) = container.parent() {
        let _ = fs::remove_dir(owner);
    }
    say(match view {
        Some(_) => "removed: clone and view".to_string(),
        None => "removed: clone".to_string(),
    });
    Outcome::Kept
}

/// Keeps `address`: makes its clone where it has none (design 4), else
/// fetches the one `found` (design 5), then settles its HEAD on origin's
/// default branch and levels its own (design 6), then keeps its view at
/// `view` (design 7).
fn keep(
    git: &Git,
    data: &Path,
    address: &Address,
    found: Option<&Path>,
    view: &Path,
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
    let bare = clone::bare(&container);
    match settle(git, &bare, say) {
        Ok(Some(default)) => keep_view(git, &bare, view, &default, say),
        Ok(None) => Outcome::Kept,
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

/// Keeps the view at `path` of the clone `bare`, whose origin's default
/// branch is `default` (design 7): makes it where it is missing, names a
/// folder there that is not the clone's worktree, and moves it while it
/// stands where the runner last put it. A view made or moved is recorded
/// in the clone.
fn keep_view(
    git: &Git,
    bare: &Path,
    path: &Path,
    default: &str,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let facts = match view::facts(git, bare, path, default) {
        Ok(facts) => facts,
        Err(words) => {
            say(format!("view not read: {words}"));
            return Outcome::Failed;
        }
    };
    let (commit, done) = match view::decide(&facts) {
        ViewAct::Leave => return Outcome::Kept,
        ViewAct::Occupied => {
            say(format!(
                "left: {} is not a worktree of the clone, so no view is made",
                path.display()
            ));
            return Outcome::Kept;
        }
        ViewAct::Make { at, force } => {
            if let Err(words) = view::make(git, bare, path, &at, force) {
                say(format!("view not made: {words}"));
                return Outcome::Failed;
            }
            let made = format!(
                "view made at {}, at origin/{default} {}",
                path.display(),
                short(&at)
            );
            (at, made)
        }
        ViewAct::Move { to } => {
            if let Err(words) = view::move_to(git, path, &to) {
                say(format!("view not moved: {words}"));
                return Outcome::Failed;
            }
            let moved = format!("view moved to origin/{default} at {}", short(&to));
            (to, moved)
        }
    };
    say(done);
    match view::record(git, bare, path, &commit) {
        Ok(()) => Outcome::Kept,
        Err(words) => {
            say(format!("view not recorded: {words}"));
            Outcome::Failed
        }
    }
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
