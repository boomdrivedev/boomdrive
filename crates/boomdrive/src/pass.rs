//! One pass over the configuration (design 9): each configured
//! repository in the file's order, its clone made or fetched, its HEAD
//! settled and its view kept, then each dropped repository let go, kept
//! while a place the runner launched there has not ended (record 0021),
//! then, where the configuration asks, each drive's VS Code workspace
//! kept at the repository root and each departed drive's taken, once no
//! clone the runner holds records the drive (core.43; design Decision 8).
//! A removed view's owner folder goes only where it is then empty, and
//! the pass touches no workspace in an owner's folder. While a local
//! layer stands that the configuration could not take, no repository is
//! let go and no workspace is set or taken, since that layer may name
//! them (record 0009 as amended 2026-11-07). Each repository is
//! independent: a failure in one is named and the next is kept. Each is
//! kept or let go under its clone's lock, which a launch into the clone
//! takes shared, and after each the pass runs the loop's hook, which keeps
//! the landings told since, so a landing waits for no more than the
//! repository in hand (record 0026; design Decisions 3 and 4). A pass
//! stopped keeps no further repository and no workspace. The pass says
//! each act and each thing it names as one line, `<owner>/<name>:
//! <words>`, and a pass with nothing to say writes nothing.

use crate::clone::{self, Found};
use crate::code_workspace::{self, Act, Wanted};
use crate::config::{self, Address, Config, LocalLayer};
use crate::git::Git;
use crate::lacks;
use crate::locks::Locks;
use crate::report;
use crate::view::{self, OldAct, ViewAct};
use boomdrive_control::Id;
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::ops::ControlFlow;
use std::path::Path;
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

/// What a pass runs after each repository it keeps or lets go of, and
/// whether it changed a clone, so the pass reads its clones again: the
/// loop's keeping of the landings told since (design Decision 3).
pub type Between<'a> = &'a mut dyn FnMut() -> bool;

/// One pass over `config`, the runner's clones kept in `data`, each
/// repository under its lock among `locks`, `between` run after each,
/// each line said to `say` as it is said. Where `config`'s local layer
/// was not taken, its managed layer's repositories are kept and none is
/// let go. Once `stop` is set, no further repository is kept or let go,
/// no workspace is kept, and the pass ends.
pub fn pass(
    git: &Git,
    data: &Path,
    config: &Config,
    locks: &Locks,
    stop: &AtomicBool,
    between: Between<'_>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Some(mut clones) = find(git, data, say) else {
        return Outcome::Failed;
    };
    let workspaces = wanted_workspaces(config);
    // The drives the clones standing record, once kept or let go.
    let mut standing = BTreeSet::new();
    let mut outcome = Outcome::Kept;
    for address in config.repositories() {
        if stop.load(Ordering::SeqCst) {
            return outcome;
        }
        let held = locks.keep(address);
        let kept = keep_configured(git, data, config, &clones, address, &mut standing, say);
        drop(held);
        outcome = outcome.and(kept);
        if between() {
            let Some(found) = find(git, data, say) else {
                return Outcome::Failed;
            };
            clones = found;
        }
    }
    let dropped: Vec<&Found> = clones
        .iter()
        .filter(|found| {
            !holds_back(config) && !config.repositories().any(|kept| *kept == found.address)
        })
        .collect();
    for found in dropped {
        if stop.load(Ordering::SeqCst) {
            return outcome;
        }
        // The runner's report is read under the clone's lock, after any
        // launch into the clone has ended, so what holds it is read as it
        // stands (record 0021; record 0026).
        let address = &found.address;
        let lock = locks.keep(address);
        let launched = report::read(&report::file(&report::control_dir(data)));
        let held = match &launched {
            Ok(report) => report
                .as_ref()
                .map_or_else(Vec::new, |report| report::holding(report, address)),
            Err(why) => {
                say(format!("{address}: not let go: {why}"));
                outcome = Outcome::Failed;
                standing.extend(found.drive.clone());
                drop(lock);
                between();
                continue;
            }
        };
        outcome = outcome.and(let_go(git, found, &held, &mut standing, say));
        drop(lock);
        between();
    }
    if let Some(workspaces) = &workspaces {
        if !stop.load(Ordering::SeqCst) {
            outcome = outcome.and(keep_workspaces(workspaces, say));
            outcome = outcome.and(take_departed(config, &clones, &standing, say));
        }
    }
    outcome.and(clear_partials(data, &clones, say))
}

/// The workspaces `config` wants, none where it does not ask for them
/// (core.43), or where its local layer was not taken (design Decision 5).
fn wanted_workspaces(config: &Config) -> Option<Vec<Wanted>> {
    (config.code_workspaces && !holds_back(config)).then(|| code_workspace::wanted(config))
}

/// Whether `config` stands on its managed layer alone because a local
/// layer there was not taken: the repositories and folders that layer may
/// name are then neither let go nor set (design Decision 5).
fn holds_back(config: &Config) -> bool {
    matches!(config.local, LocalLayer::NotTaken { .. })
}

/// Keeps `address` alone, just as a pass keeps it, against `config`,
/// under its lock among `locks` (design 9): what a landing told of it
/// runs. An address `config` does not name is named, and nothing is
/// cloned or fetched for it (core.5).
pub fn keep_one(
    git: &Git,
    data: &Path,
    config: &Config,
    locks: &Locks,
    address: &Address,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let Some(address) = config.repositories().find(|kept| *kept == address) else {
        say(format!(
            "{address}: not kept: the configuration does not name it"
        ));
        return Outcome::Kept;
    };
    let _held = locks.keep(address);
    match find(git, data, say) {
        Some(clones) => keep_configured(
            git,
            data,
            config,
            &clones,
            address,
            &mut BTreeSet::new(),
            say,
        ),
        None => Outcome::Failed,
    }
}

/// The runner's clones in `data`, none where they cannot be read, which
/// is named.
fn find(git: &Git, data: &Path, say: &mut dyn FnMut(String)) -> Option<Vec<Found>> {
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
/// it has one, each line said with its address. The clone records the
/// drive `config` names it under (design Decision 7), and the drive it
/// records once kept joins `standing`.
fn keep_configured(
    git: &Git,
    data: &Path,
    config: &Config,
    clones: &[Found],
    address: &Address,
    standing: &mut BTreeSet<String>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let found = clones.iter().find(|found| found.address == *address);
    let drive = config
        .drive_of(address)
        .map_or("", |drive| drive.name.as_str());
    let view = config::view_path(config, address);
    keep(git, data, address, found, drive, &view, standing, say)
}

/// Keeps each drive's VS Code workspace the configuration wants, the
/// `workspaces` (design Decision 1; core.43): made, its folders
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

/// Lets go of the clone `found`, whose address the configuration no
/// longer names (design 8). It is never fetched. While a launch of the
/// runner's report holds it at a place that has not ended, each such
/// place with its launch among the `launched`, it is drained: kept as it
/// is, neither fetched nor moved, and named held by each (record 0021;
/// core.5). Once nothing holds it and neither it nor its view holds
/// anything the forge lacks, the view is removed, then the view's owner
/// folder if empty, then the container, then the container's owner folder
/// if empty (design Decision 10). Until then nothing is touched, and each
/// thing that holds them is named. A folder at the view's path that is
/// not the clone's worktree is left as it is. A clone left standing has
/// the drive it records join `standing`. Each line of the clone's says
/// its address.
fn let_go(
    git: &Git,
    found: &Found,
    launched: &[(Id, String)],
    standing: &mut BTreeSet<String>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let removed = remove_dropped(git, &found.address, &found.container, launched, say);
    if removed != Removed::Gone {
        standing.extend(found.drive.clone());
    }
    match removed {
        Removed::Failed => Outcome::Failed,
        Removed::Gone | Removed::Held => Outcome::Kept,
    }
}

/// What became of a dropped repository's clone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Removed {
    /// It and its view are gone.
    Gone,
    /// It is left, held by what the forge lacks or by a place the
    /// runner launched there that has not ended.
    Held,
    /// A step failed, and was named.
    Failed,
}

/// `let_go`'s steps for the clone of `address` in `container`.
fn remove_dropped(
    git: &Git,
    address: &Address,
    container: &Path,
    launched: &[(Id, String)],
    say: &mut dyn FnMut(String),
) -> Removed {
    let bare = clone::bare(container);
    let read = view::recorded_path(git, &bare).and_then(|view| {
        let facts = lacks::read(git, &bare, view.as_deref())?;
        Ok((view, facts))
    });
    let (view, facts) = match read {
        Ok(read) => read,
        Err(words) => {
            say(format!("{address}: not let go: {words}"));
            return Removed::Failed;
        }
    };
    let held = lacks::holdings(&facts);
    if !held.is_empty() || !launched.is_empty() {
        for holding in held {
            say(format!("{address}: left: held by {holding}"));
        }
        for (id, place) in launched {
            say(format!(
                "{address}: left: held by the place {place} of {id}, which has not ended"
            ));
        }
        return Removed::Held;
    }
    let view = view.filter(|_| facts.view.is_some());
    if let Some(view) = &view {
        if let Err(words) = view::remove(git, &bare, view) {
            say(format!("{address}: view not removed: {words}"));
            return Removed::Failed;
        }
    }
    if let Err(error) = fs::remove_dir_all(container) {
        say(format!(
            "{address}: clone not removed: {}: {error}",
            container.display()
        ));
        return Removed::Failed;
    }
    if let Some(owner) = container.parent() {
        let _ = fs::remove_dir(owner);
    }
    say(match view {
        Some(_) => format!("{address}: removed: clone and view"),
        None => format!("{address}: removed: clone"),
    });
    Removed::Gone
}

/// Takes the workspace of each drive that has gone with its last clone
/// (design Decision 8; core.43): each drive one of `clones` recorded when
/// the pass found it, that `config` does not name and that no clone in
/// `standing` records once the pass has kept and let go. Its workspace at
/// `config`'s repository root is removed where it is a file the runner
/// reads as a JSON object, its partial file with it, and one line names
/// it; anything else there is left as it is and named. A drive's record
/// that is no drive's name names no workspace. A workspace that cannot
/// be read or removed is named, and fails the pass.
fn take_departed(
    config: &Config,
    clones: &[Found],
    standing: &BTreeSet<String>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let mut departed: Vec<&str> = Vec::new();
    for drive in clones.iter().filter_map(|found| found.drive.as_deref()) {
        if config::in_form(drive)
            && !config.drives().any(|kept| kept.name == drive)
            && !standing.contains(drive)
            && !departed.contains(&drive)
        {
            departed.push(drive);
        }
    }
    let mut outcome = Outcome::Kept;
    for drive in departed {
        outcome = outcome.and(take_workspace(
            &code_workspace::file(config, drive),
            drive,
            say,
        ));
    }
    outcome
}

/// Takes the workspace at `file` of the drive `drive`, which has gone, as
/// `take_departed` does.
fn take_workspace(file: &Path, drive: &str, say: &mut dyn FnMut(String)) -> Outcome {
    let shown = file.display();
    let stands = match code_workspace::stands(file) {
        Ok(stands) => stands,
        Err(error) => {
            say(format!("boomdrive: workspace not read at {shown}: {error}"));
            return Outcome::Failed;
        }
    };
    match code_workspace::object(&stands) {
        Ok(None) => return Outcome::Kept,
        Ok(Some(_)) => {}
        Err(wrong) => {
            say(format!("boomdrive: workspace left at {shown}: {wrong}"));
            return Outcome::Kept;
        }
    }
    let removed =
        fs::remove_file(file).and_then(|()| match fs::remove_file(code_workspace::partial(file)) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            removed => removed,
        });
    match removed {
        Ok(()) => {
            say(format!(
                "boomdrive: workspace removed at {shown}: its drive {drive} has left the \
                 configuration"
            ));
            Outcome::Kept
        }
        Err(error) => {
            say(format!(
                "boomdrive: workspace not removed at {shown}: {error}"
            ));
            Outcome::Failed
        }
    }
}

/// Keeps `address`: makes its clone where it has none (design 4), else
/// fetches the one `found` (design 5), the clone recording `drive`, the
/// drive the configuration names it under, where it records another or
/// none (design Decision 7); then settles its HEAD on origin's default
/// branch (design 6), then keeps its view at `view` (design 7). A view
/// the runner put where the repository root no longer gives is let go of
/// first. The drive the clone records once kept joins `standing`. Each
/// line of the repository's says its address.
#[allow(clippy::too_many_arguments)]
fn keep(
    git: &Git,
    data: &Path,
    address: &Address,
    found: Option<&Found>,
    drive: &str,
    view: &Path,
    standing: &mut BTreeSet<String>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let mut recorded = Outcome::Kept;
    let container = match found {
        Some(found) => {
            let bare = clone::bare(&found.container);
            if found.drive.as_deref() != Some(drive) {
                recorded = record_drive(git, address, &bare, drive, say);
            }
            let records = match recorded {
                Outcome::Kept => Some(drive.to_string()),
                Outcome::Failed => found.drive.clone(),
            };
            standing.extend(records);
            if let Err(failure) = clone::fetch(git, &bare) {
                say(format!("{address}: not fetched: {failure}"));
                return Outcome::Failed;
            }
            found.container.clone()
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
                    recorded = record_drive(git, address, &clone::bare(&container), drive, say);
                    if recorded == Outcome::Kept {
                        standing.insert(drive.to_string());
                    }
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
            return recorded;
        }
        Err(words) => {
            say(format!("{address}: default branch not read: {words}"));
            return Outcome::Failed;
        }
    };
    let settled = recorded.and(settle(git, &bare, &default, &mut |words| {
        say(format!("{address}: {words}"))
    }));
    match let_go_of_old_view(git, address, &bare, view, say) {
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

/// Records in the clone `bare` of `address` that the configuration names
/// it under `drive` (design Decision 7). A record that cannot be written
/// is named, and fails the pass.
fn record_drive(
    git: &Git,
    address: &Address,
    bare: &Path,
    drive: &str,
    say: &mut dyn FnMut(String),
) -> Outcome {
    match clone::record_drive(git, bare, drive) {
        Ok(()) => Outcome::Kept,
        Err(words) => {
            say(format!("{address}: drive {drive} not recorded: {words}"));
            Outcome::Failed
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
/// nothing the forge lacks, its owner's folder with it where that is then
/// empty (design Decision 10), else left and named; where it is gone or
/// no longer the clone's worktree, it is forgotten and any folder there
/// left untouched. Breaks with the outcome where no view is to be kept at
/// `path` this pass, and else goes on with the let-go's own.
fn let_go_of_old_view(
    git: &Git,
    address: &Address,
    bare: &Path,
    path: &Path,
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
            if let Err(words) = view::remove(git, bare, &old) {
                say(format!("{address}: view not removed: {words}"));
                return ControlFlow::Break(Outcome::Failed);
            }
            say(format!("{address}: view removed from {}", old.display()));
        }
        OldAct::Forget => {}
    }
    if let Err(words) = view::forget(git, bare) {
        say(format!("{address}: view not forgotten: {words}"));
        return ControlFlow::Break(Outcome::Failed);
    }
    ControlFlow::Continue(Outcome::Kept)
}

/// A commit as a line gives it, at seven characters.
fn short(commit: &str) -> &str {
    &commit[..commit.len().min(7)]
}

/// Removes each `.<name>.partial` folder an earlier run left in `data`,
/// other than one of `clones`, a repository's own name ending so
/// (design 9's step 5).
fn clear_partials(data: &Path, clones: &[Found], say: &mut dyn FnMut(String)) -> Outcome {
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
                || clones.iter().any(|found| found.container == path)
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
