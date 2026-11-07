//! One pass over the configuration (design 9): each configured
//! repository in the file's order, its clone made or fetched, its HEAD
//! settled and its view kept, then each
//! dropped repository let go, then, where the configuration asks, each
//! owner's VS Code workspace kept (core.43). Where it asks, a removed
//! view's owner folder that then holds only a workspace no kept owner has
//! goes with that workspace. Each repository is
//! independent: a failure in one is named and the next is kept. A pass
//! stopped keeps no further repository and no workspace. The pass says
//! each act and each thing it names as one line, `<owner>/<name>:
//! <words>`, and a pass with nothing to say writes nothing.

use crate::clone;
use crate::code_workspace::{self, Act, Wanted};
use crate::config::{self, Address, Config};
use crate::git::Git;
use crate::lacks;
use crate::view::{self, OldAct, ViewAct};
use std::fs;
use std::io;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// Whether every git call of a pass succeeded, and every workspace it
/// kept was read and written. Something left and named by rule, a folder
/// the runner did not make or a workspace it cannot read among it, is not
/// a failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Every call, read and write succeeded.
    Kept,
    /// At least one failed, and was named.
    Failed,
}

impl Outcome {
    /// Failed where either is.
    pub fn and(self, other: Outcome) -> Outcome {
        if self == Outcome::Failed {
            self
        } else {
            other
        }
    }
}

/// One pass over `config`, the runner's clones kept in `data`, each line
/// said to `say` as it is said. Once `stop` is set, no further
/// repository is kept or let go, no workspace is kept, and the pass ends.
pub fn pass(
    git: &Git,
    data: &Path,
    config: &Config,
    stop: &AtomicBool,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Some(clones) = find(git, data, say) else {
        return Outcome::Failed;
    };
    let workspaces = wanted_workspaces(config);
    let mut outcome = Outcome::Kept;
    for address in config.repositories() {
        if stop.load(Ordering::SeqCst) {
            return outcome;
        }
        let kept = keep_configured(
            git,
            data,
            config,
            workspaces.as_deref(),
            &clones,
            address,
            say,
        );
        outcome = outcome.and(kept);
    }
    for (address, container) in &clones {
        if stop.load(Ordering::SeqCst) {
            return outcome;
        }
        if config.repositories().any(|kept| kept == address) {
            continue;
        }
        outcome = outcome.and(let_go(git, address, container, workspaces.as_deref(), say));
    }
    if let Some(workspaces) = &workspaces {
        if !stop.load(Ordering::SeqCst) {
            outcome = outcome.and(keep_workspaces(workspaces, say));
        }
    }
    outcome.and(clear_partials(data, &clones, say))
}

/// The workspaces `config` wants, none where it does not ask for them
/// (core.43).
fn wanted_workspaces(config: &Config) -> Option<Vec<Wanted>> {
    config
        .code_workspaces
        .then(|| code_workspace::wanted(config))
}

/// Keeps `address` alone, just as a pass keeps it, against `config`
/// (design 9): what a landing told of it runs. An address `config` does
/// not name is named, and nothing is cloned or fetched for it (core.5).
pub fn keep_one(
    git: &Git,
    data: &Path,
    config: &Config,
    address: &Address,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Some(address) = config.repositories().find(|kept| *kept == address) else {
        say(format!(
            "{address}: not kept: the configuration does not name it"
        ));
        return Outcome::Kept;
    };
    let workspaces = wanted_workspaces(config);
    match find(git, data, say) {
        Some(clones) => keep_configured(
            git,
            data,
            config,
            workspaces.as_deref(),
            &clones,
            address,
            say,
        ),
        None => Outcome::Failed,
    }
}

/// The runner's clones in `data`, none where they cannot be read, which
/// is named.
fn find(git: &Git, data: &Path, say: &mut dyn FnMut(String)) -> Option<Vec<(Address, PathBuf)>> {
    match clone::find(git, data) {
        Ok(clones) => Some(clones),
        Err(error) => {
            say(format!(
                "boomdrive: {} cannot be read: {error}",
                clone::repositories(data).display()
            ));
            None
        }
    }
}

/// Keeps `address`, which `config` names, its clone among `clones` where
/// it has one, each line said with its address. `workspaces` are those
/// `config` wants, none where it does not ask for them.
fn keep_configured(
    git: &Git,
    data: &Path,
    config: &Config,
    workspaces: Option<&[Wanted]>,
    clones: &[(Address, PathBuf)],
    address: &Address,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let found = clones
        .iter()
        .find(|(kept, _)| kept == address)
        .map(|(_, container)| container.as_path());
    let view = config::view_path(config, address);
    keep(git, data, address, found, &view, workspaces, say)
}

/// Keeps each owner's VS Code workspace the configuration wants, the
/// `workspaces` (design Decisions 4 to 6; core.43): made, its folders
/// set, or left as it is and named. A workspace whose folders are the
/// wanted ones says nothing. One that cannot be read or written is named,
/// and fails the pass.
fn keep_workspaces(workspaces: &[Wanted], say: &mut dyn FnMut(String)) -> Outcome {
    let mut outcome = Outcome::Kept;
    for wanted in workspaces {
        outcome = outcome.and(keep_workspace(wanted, say));
    }
    outcome
}

/// Keeps the one workspace `wanted` names, as `keep_workspaces` does.
fn keep_workspace(wanted: &Wanted, say: &mut dyn FnMut(String)) -> Outcome {
    let file = wanted.file.display();
    let stands = match code_workspace::stands(&wanted.file) {
        Ok(stands) => stands,
        Err(error) => {
            say(format!("boomdrive: workspace not read at {file}: {error}"));
            return Outcome::Failed;
        }
    };
    match code_workspace::decide(&stands, &wanted.folders) {
        Act::Leave => Outcome::Kept,
        Act::Name(wrong) => {
            say(format!("boomdrive: workspace left at {file}: {wrong}"));
            Outcome::Kept
        }
        Act::Write(text) => match code_workspace::write(&wanted.file, &text) {
            Ok(()) => {
                let folders = match wanted.folders.as_slice() {
                    [] => "no folders".to_string(),
                    folders => folders.join(", "),
                };
                say(format!("boomdrive: workspace written at {file}: {folders}"));
                Outcome::Kept
            }
            Err(error) => {
                say(format!(
                    "boomdrive: workspace not written at {file}: {error}"
                ));
                Outcome::Failed
            }
        },
    }
}

/// Lets go of the clone in `container`, whose address the configuration
/// no longer names (design 8). It is never fetched. Once neither it nor
/// its view holds anything the forge lacks, the view is removed, then
/// the view's owner folder if empty, or if it holds only a workspace none
/// of `workspaces` is (design Decision 7), then the container, then the
/// container's owner folder if empty. Until then nothing is touched, and
/// each thing that holds them is named. A folder at the view's path that
/// is not the clone's worktree is left as it is. Each line of the clone's
/// says its address.
fn let_go(
    git: &Git,
    address: &Address,
    container: &Path,
    workspaces: Option<&[Wanted]>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let bare = clone::bare(container);
    let read = view::recorded_path(git, &bare).and_then(|view| {
        let facts = lacks::read(git, &bare, view.as_deref())?;
        Ok((view, facts))
    });
    let (view, facts) = match read {
        Ok(read) => read,
        Err(words) => {
            say(format!("{address}: not let go: {words}"));
            return Outcome::Failed;
        }
    };
    let held = lacks::holdings(&facts);
    if !held.is_empty() {
        for holding in held {
            say(format!("{address}: left: held by {holding}"));
        }
        return Outcome::Kept;
    }
    let view = view.filter(|_| facts.view.is_some());
    let mut outcome = Outcome::Kept;
    if let Some(view) = &view {
        match view::remove(git, &bare, view) {
            Ok(Some(folder)) => outcome = take_workspace(&folder, workspaces, say),
            Ok(None) => {}
            Err(words) => {
                say(format!("{address}: view not removed: {words}"));
                return Outcome::Failed;
            }
        }
    }
    if let Err(error) = fs::remove_dir_all(container) {
        say(format!(
            "{address}: clone not removed: {}: {error}",
            container.display()
        ));
        return Outcome::Failed;
    }
    if let Some(owner) = container.parent() {
        let _ = fs::remove_dir(owner);
    }
    say(match view {
        Some(_) => format!("{address}: removed: clone and view"),
        None => format!("{address}: removed: clone"),
    });
    outcome
}

/// Takes the workspace in `folder`, an owner's folder a removed view left
/// standing, and then the folder, where the folder holds nothing else but
/// the workspace's partial file and the workspace is none of `workspaces`
/// (design Decision 7; core.43). Where the configuration does not ask for
/// workspaces, `workspaces` is none and the folder is left as it is. A
/// workspace or folder that cannot be removed is named, and fails the
/// pass.
fn take_workspace(
    folder: &Path,
    workspaces: Option<&[Wanted]>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Some(workspaces) = workspaces else {
        return Outcome::Kept;
    };
    let Some(file) = code_workspace::alone_in(folder) else {
        return Outcome::Kept;
    };
    if workspaces
        .iter()
        .any(|kept| view::same_path(&kept.file, &file))
    {
        return Outcome::Kept;
    }
    let removed = fs::remove_file(&file).and_then(|()| {
        match fs::remove_file(code_workspace::partial(&file)) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            removed => removed,
        }
    });
    if let Err(error) = removed {
        say(format!(
            "boomdrive: workspace not removed from {}: {error}",
            file.display()
        ));
        return Outcome::Failed;
    }
    if let Err(error) = fs::remove_dir(folder) {
        say(format!(
            "boomdrive: workspace removed from {}, but not its folder: {error}",
            file.display()
        ));
        return Outcome::Failed;
    }
    say(format!(
        "boomdrive: workspace removed from {}, with its folder",
        file.display()
    ));
    Outcome::Kept
}

/// Keeps `address`: makes its clone where it has none (design 4), else
/// fetches the one `found` (design 5), then settles its HEAD on origin's
/// default branch (design 6), then keeps its view at `view` (design 7). A view the runner put where the repository root no
/// longer gives is let go of first, its owner's workspace with it where
/// none of `workspaces` is that one (design Decision 7). Each line of the
/// repository's says its address.
fn keep(
    git: &Git,
    data: &Path,
    address: &Address,
    found: Option<&Path>,
    view: &Path,
    workspaces: Option<&[Wanted]>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let container = match found {
        Some(container) => {
            if let Err(failure) = clone::fetch(git, &clone::bare(container)) {
                say(format!("{address}: not fetched: {failure}"));
                return Outcome::Failed;
            }
            container.to_path_buf()
        }
        None => {
            let container = clone::container(data, address);
            if container.symlink_metadata().is_ok() {
                say(format!(
                    "{address}: left: {} is not a clone the runner made, so none is made",
                    container.display()
                ));
                return Outcome::Kept;
            }
            match clone::make(git, data, address) {
                Ok(container) => {
                    say(format!("{address}: cloned from {}", clone::url(address)));
                    container
                }
                Err(failure) => {
                    say(format!("{address}: not cloned: {failure}"));
                    return Outcome::Failed;
                }
            }
        }
    };
    let bare = clone::bare(&container);
    let default = match clone::default_of(git, &bare) {
        Ok(Some(default)) => default,
        Ok(None) => {
            say(format!(
                "{address}: origin names no default branch, so it has no view"
            ));
            return Outcome::Kept;
        }
        Err(words) => {
            say(format!("{address}: default branch not read: {words}"));
            return Outcome::Failed;
        }
    };
    let settled = settle(git, &bare, &default, &mut |words| {
        say(format!("{address}: {words}"))
    });
    match let_go_of_old_view(git, address, &bare, view, workspaces, say) {
        ControlFlow::Break(outcome) => settled.and(outcome),
        ControlFlow::Continue(outcome) => {
            settled
                .and(outcome)
                .and(keep_view(git, &bare, view, &default, &mut |words| {
                    say(format!("{address}: {words}"))
                }))
        }
    }
}

/// Settles the clone's HEAD on `default`, origin's default branch, making
/// that branch where the clone lacks it (design 6). No branch the clone
/// has is moved: each is read at origin's tracking ref, and a branch
/// sluice keeps is its copy (core.14). A HEAD that cannot be settled is
/// named and fails the pass, and the view is still kept, since it reads
/// only `origin/<default>`.
fn settle(git: &Git, bare: &Path, default: &str, say: &mut dyn FnMut(String)) -> Outcome {
    match clone::settle_head(git, bare, default) {
        Ok(true) => {
            say(format!("HEAD names {default}, origin's default branch"));
            Outcome::Kept
        }
        Ok(false) => Outcome::Kept,
        Err(words) => {
            say(format!("HEAD not settled on {default}: {words}"));
            Outcome::Failed
        }
    }
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

/// Lets go of the view the clone `bare` of `address` records where
/// `path`, the view's path now, is not that one, the repository root
/// having changed (design 7). The old view is removed while it holds
/// nothing the forge lacks, its owner's workspace and folder with it
/// where none of `workspaces` is that one (design Decision 7), else left
/// and named; where it is gone or no longer the clone's worktree, it is
/// forgotten and any folder there left untouched. Breaks with the outcome
/// where no view is to be kept at `path` this pass, and else goes on with
/// the let-go's own.
fn let_go_of_old_view(
    git: &Git,
    address: &Address,
    bare: &Path,
    path: &Path,
    workspaces: Option<&[Wanted]>,
    say: &mut dyn FnMut(String),
) -> ControlFlow<Outcome, Outcome> {
    let read = view::recorded_path(git, bare).and_then(|old| match old {
        Some(old) if !view::same_path(&old, path) => {
            let facts = view::old(git, bare, &old)?;
            Ok(Some((old, facts)))
        }
        _ => Ok(None),
    });
    let (old, facts) = match read {
        Ok(Some(read)) => read,
        Ok(None) => return ControlFlow::Continue(Outcome::Kept),
        Err(words) => {
            say(format!("{address}: view not read: {words}"));
            return ControlFlow::Break(Outcome::Failed);
        }
    };
    let mut outcome = Outcome::Kept;
    match view::decide_old(&facts) {
        OldAct::Hold(held) => {
            for holding in held {
                say(format!(
                    "{address}: view left at {}, so none is made at {}: held by {holding}",
                    old.display(),
                    path.display()
                ));
            }
            return ControlFlow::Break(Outcome::Kept);
        }
        OldAct::Remove => {
            let left = match view::remove(git, bare, &old) {
                Ok(left) => left,
                Err(words) => {
                    say(format!("{address}: view not removed: {words}"));
                    return ControlFlow::Break(Outcome::Failed);
                }
            };
            say(format!("{address}: view removed from {}", old.display()));
            if let Some(folder) = left {
                outcome = take_workspace(&folder, workspaces, say);
            }
        }
        OldAct::Forget => {}
    }
    if let Err(words) = view::forget(git, bare) {
        say(format!("{address}: view not forgotten: {words}"));
        return ControlFlow::Break(Outcome::Failed);
    }
    ControlFlow::Continue(outcome)
}

/// A commit as a line gives it, at seven characters.
fn short(commit: &str) -> &str {
    &commit[..commit.len().min(7)]
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
                    "boomdrive: removed {}, a clone left unfinished",
                    path.display()
                )),
                Err(error) => {
                    say(format!(
                        "boomdrive: {} is a clone left unfinished and cannot be removed: {error}",
                        path.display()
                    ));
                    outcome = Outcome::Failed;
                }
            }
        }
    }
    outcome
}
