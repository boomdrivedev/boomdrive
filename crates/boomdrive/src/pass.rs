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
//! stopped keeps no further repository and no workspace. Each fetch, a new
//! clone's first among them, is given what the forge gives for its
//! repository: the App's token alone where the configuration names
//! `[forge]`, no credential where the App is not installed there or no
//! token could be minted, and the host's own helpers where it names no
//! `[forge]`; a fetch that fails drops the repository's kept token (record
//! 0007 as amended on 2026-11-11; design Decision 5). The pass says
//! each act and each thing it names as one line, `<owner>/<name>:
//! <words>`, and a pass with nothing to say writes nothing. git's own
//! collection is off in every clone the pass keeps, and once it has kept
//! and let go of every repository, kept the workspaces and cleared partial
//! clones, the pass packs each kept clone past git's own bounds, holding
//! no lock, reading its stop before each and running the loop's hook
//! after each it packs (record 0051; core.14).

use crate::clone::{self, Found};
use crate::code_workspace::{self, Act, Wanted};
use crate::config::{self, Address, Config, LocalLayer};
use crate::forge::{Asked, Forge};
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

/// What a pass runs after each repository it keeps or lets go of, and
/// whether it changed a clone, so the pass reads its clones again: the
/// loop's keeping of the landings told since (design Decision 3).
pub type Between<'a> = &'a mut dyn FnMut() -> bool;

/// The runner's seam to the keeping of its clones: a keeping pass over a
/// configuration, and the keeping of one repository alone at a landing
/// told of it, each repository under its lock among the clones' `locks`
/// (design 9; design Decision 3). [`Keeping`] keeps the host's clones by
/// git; the runner's loop and `run --once` reach them through it alone.
pub trait Keeper: Send + Sync {
    /// One keeping pass over `config`, as [`pass`] makes it.
    fn pass(
        &self,
        config: &Config,
        locks: &Locks,
        stop: &AtomicBool,
        between: Between<'_>,
        say: &mut dyn FnMut(String),
    ) -> Outcome;

    /// `address` kept alone, as [`keep_one`] keeps it.
    fn keep_one(
        &self,
        config: &Config,
        locks: &Locks,
        address: &Address,
        say: &mut dyn FnMut(String),
    ) -> Outcome;
}

/// The keeper of the host's clones in `data`, by `git`, each fetch given
/// what `forge` gives.
pub struct Keeping {
    pub git: Git,
    pub forge: Forge,
    pub data: PathBuf,
}

impl Keeper for Keeping {
    fn pass(
        &self,
        config: &Config,
        locks: &Locks,
        stop: &AtomicBool,
        between: Between<'_>,
        say: &mut dyn FnMut(String),
    ) -> Outcome {
        pass(
            &self.git,
            &self.forge,
            &self.data,
            config,
            locks,
            stop,
            between,
            say,
        )
    }

    fn keep_one(
        &self,
        config: &Config,
        locks: &Locks,
        address: &Address,
        say: &mut dyn FnMut(String),
    ) -> Outcome {
        keep_one(
            &self.git,
            &self.forge,
            &self.data,
            config,
            locks,
            address,
            say,
        )
    }
}

/// One pass over `config`, the runner's clones kept in `data`, each
/// fetch given what `forge` gives for its repository, each repository
/// under its lock among `locks`, `between` run after each, each line said
/// to `say` as it is said. Where `config`'s local layer was not taken, its
/// managed layer's repositories are kept and none is let go. Once every
/// repository is kept and let go of, the workspaces kept and partial
/// clones cleared, each kept clone past git's own bounds is packed, in the
/// configuration's order, under no lock, `between` run after each it
/// packs (record 0051). Once `stop` is set, no further repository is kept
/// or let go, no workspace is kept, no clone is packed, and the pass ends.
#[allow(clippy::too_many_arguments)]
pub fn pass(
    git: &Git,
    forge: &Forge,
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
        let kept = keep_configured(
            git,
            forge,
            data,
            config,
            &clones,
            address,
            &mut standing,
            say,
        );
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
    outcome = outcome.and(clear_partials(data, &clones, say));
    outcome.and(pack_kept(git, data, config, clones, stop, between, say))
}

/// Packs each clone of a repository `config` keeps, in the
/// configuration's order, that `clones`, the pass's clones, holds and
/// whose occasion has come, more than git's own bound of loose objects or
/// packs (record 0051; core.14). A dropped repository's clone is not
/// packed, as it is not fetched. No clone's lock is held, so no launch
/// into a clone waits on its pack. `stop` is read before each clone, and
/// none is packed once it is set; a pack in flight runs to its end. After
/// each clone it packs, or whose pack fails, `between` keeps the landings
/// told since, and the clones are found again where it changed one. A clone packed says
/// `<address>: packed <n> loose objects and <m> packs`, its counts before
/// the pack, and one below both bounds says nothing. A count or a pack
/// that fails is named, `<address>: not packed: <git's words>`, and fails
/// the pass, and every other clone is still packed.
fn pack_kept(
    git: &Git,
    data: &Path,
    config: &Config,
    mut clones: Vec<Found>,
    stop: &AtomicBool,
    between: Between<'_>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let mut outcome = Outcome::Kept;
    for address in config.repositories() {
        if stop.load(Ordering::SeqCst) {
            return outcome;
        }
        let Some(found) = clones.iter().find(|found| found.address == *address) else {
            continue;
        };
        let bare = clone::bare(&found.container);
        let counts = match clone::counts(git, &bare) {
            Ok(counts) => counts,
            Err(words) => {
                say(format!("{address}: not packed: {words}"));
                outcome = Outcome::Failed;
                continue;
            }
        };
        if !clone::is_occasion(counts) {
            continue;
        }
        match clone::pack(git, &bare) {
            Ok(()) => say(format!(
                "{address}: packed {} and {}",
                counted(counts.loose, "loose object"),
                counted(counts.packs, "pack")
            )),
            Err(words) => {
                say(format!("{address}: not packed: {words}"));
                outcome = Outcome::Failed;
            }
        }
        if between() {
            let Some(found) = find(git, data, say) else {
                return Outcome::Failed;
            };
            clones = found;
        }
    }
    outcome
}

/// `count` of `thing`, `1 <thing>` for one and `<count> <thing>s` for any
/// other.
fn counted(count: u64, thing: &str) -> String {
    match count {
        1 => format!("1 {thing}"),
        count => format!("{count} {thing}s"),
    }
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

/// Keeps `address` alone, just as a pass keeps it, against `config`, its
/// fetch given what `forge` gives, under its lock among `locks` (design
/// 9): what a landing told of it runs. An address `config` does not name
/// is named, and nothing is cloned or fetched for it (core.5).
pub fn keep_one(
    git: &Git,
    forge: &Forge,
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
            forge,
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
/// it has one, its fetch given what `forge` gives for it under `config`'s
/// `[forge]`, each line said with its address. The clone records the
/// drive `config` names it under (design Decision 7), and the drive it
/// records once kept joins `standing`.
#[allow(clippy::too_many_arguments)]
fn keep_configured(
    git: &Git,
    forge: &Forge,
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
    let asked = forge.credential(config.forge.as_ref(), address);
    keep(
        git, forge, &asked, data, address, found, drive, &view, standing, say,
    )
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
/// none (design Decision 7), and git's own collection turned off in it
/// before its fetch where its config says otherwise (record 0051); then
/// settles its HEAD on origin's default branch (design 6), then keeps its
/// view at `view` (design 7). A view the runner put where the repository
/// root no longer gives is let go of first. The drive the clone records once kept joins `standing`. Each
/// line of the repository's says its address. The fetch is given what
/// `forge` gave, `asked`; one that fails drops the repository's kept
/// token, and its line gives git's words, then the mint's where a mint was
/// refused.
#[allow(clippy::too_many_arguments)]
fn keep(
    git: &Git,
    forge: &Forge,
    asked: &Asked,
    data: &Path,
    address: &Address,
    found: Option<&Found>,
    drive: &str,
    view: &Path,
    standing: &mut BTreeSet<String>,
    say: &mut dyn FnMut(String),
) -> Outcome {
    let mut so_far = Outcome::Kept;
    let container = match found {
        Some(found) => {
            let bare = clone::bare(&found.container);
            if found.drive.as_deref() != Some(drive) {
                so_far = record_drive(git, address, &bare, drive, say);
            }
            let records = match so_far {
                Outcome::Kept => Some(drive.to_string()),
                Outcome::Failed => found.drive.clone(),
            };
            standing.extend(records);
            so_far = so_far.and(turn_collection_off(git, address, &bare, say));
            if let Err(failure) = clone::fetch(git, &bare, &asked.credential) {
                forge.drop(address);
                say(format!(
                    "{address}: not fetched: {failure}{}",
                    no_token(asked)
                ));
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
            match clone::make(git, data, address, &asked.credential) {
                Ok(container) => {
                    say(format!("{address}: cloned from {}", clone::url(address)));
                    so_far = record_drive(git, address, &clone::bare(&container), drive, say);
                    if so_far == Outcome::Kept {
                        standing.insert(drive.to_string());
                    }
                    container
                }
                Err(failure) => {
                    forge.drop(address);
                    say(format!(
                        "{address}: not cloned: {failure}{}",
                        no_token(asked)
                    ));
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
            return so_far;
        }
        Err(words) => {
            say(format!("{address}: default branch not read: {words}"));
            return Outcome::Failed;
        }
    };
    let settled = so_far.and(settle(git, &bare, &default, &mut |words| {
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

/// What a failed fetch's line adds after git's words: the mint's, where a
/// mint was refused, and nothing otherwise.
fn no_token(asked: &Asked) -> String {
    match &asked.refused {
        Some(words) => format!("; no forge token: {words}"),
        None => String::new(),
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

/// Turns git's own collection off in the clone `bare` of `address`, before
/// its fetch, the first command of the pass that would start a collection
/// (record 0051; core.14). A key that cannot be written is named, and
/// fails the pass, and the keep goes on to the fetch, the HEAD and the
/// view, as every clone was kept before its collection was off.
fn turn_collection_off(
    git: &Git,
    address: &Address,
    bare: &Path,
    say: &mut dyn FnMut(String),
) -> Outcome {
    match clone::turn_collection_off(git, bare) {
        Ok(()) => Outcome::Kept,
        Err(words) => {
            say(format!(
                "{address}: git's own collection not turned off: {words}"
            ));
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

/// Each drive's VS Code workspace at the repository root, where the
/// configuration asks: its folders the drive's kept repositories, set at
/// every pass, the rest of the file kept, and a file the runner cannot
/// read left as it is and named (the views spec; core.43; record 0017 as
/// revised; design Decision 1). These read the configuration and the
/// files at the repository root, and a pass over drives that keep no
/// repository, so none runs git (logic-is-tested-as-units-and-few-tests-
/// run-the-binary); `tests/code_workspaces.rs` and
/// `tests/drive_workspaces.rs` keep those whose clones record their
/// drives in git. A departed drive's workspace is taken here from the
/// clones a pass found, as `clone::find` gives them.
#[cfg(test)]
mod workspaces {
    use super::{keep_workspaces, pass, take_departed, wanted_workspaces, Outcome};
    use crate::clone::Found;
    use crate::config::{self, Address, Config};
    use crate::forge::Forge;
    use crate::git::Git;
    use crate::locks::Locks;
    use serde_json::{json, Value};
    use std::collections::BTreeSet;
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::AtomicBool;
    use tempfile::TempDir;

    /// A home of the test's own, its repository root `~/Code` by default.
    struct Home(TempDir);

    impl Home {
        fn new() -> Home {
            Home(tempfile::tempdir().expect("a home"))
        }

        fn path(&self) -> &Path {
            self.0.path()
        }

        /// The configuration `text`, read as the default file.
        fn config(&self, text: &str) -> Config {
            let file = self.path().join(".config/boomdrive/config.toml");
            config::parse(text, &file, self.path()).expect("a configuration")
        }

        /// `drive`'s workspace at the default repository root.
        fn workspace(&self, drive: &str) -> PathBuf {
            self.path()
                .join("Code")
                .join(format!("{drive}.code-workspace"))
        }

        /// One pass over `config`, stopped from its start where `stop`.
        /// It keeps no repository, so it runs no git.
        fn pass(&self, config: &Config, stop: bool) -> (Outcome, Vec<String>) {
            let mut lines = Vec::new();
            let outcome = pass(
                &Git::default(),
                &Forge::new(),
                &self.path().join(".local/share/boomdrive"),
                config,
                &Locks::new(),
                &AtomicBool::new(stop),
                &mut || false,
                &mut |line| lines.push(line),
            );
            (outcome, lines)
        }
    }

    /// A configuration keeping each of `kept`'s drives of the account
    /// `own` with its addresses, in order, and none where `kept` names
    /// none, asking for the workspaces where `asks`.
    fn drives(asks: bool, kept: &[(&str, &[&str])]) -> String {
        let mut text = if asks {
            "code_workspaces = true\n".to_string()
        } else {
            String::new()
        };
        if kept.is_empty() {
            text.push_str("accounts.own.drives = {}\n");
        }
        for (drive, addresses) in kept {
            let entries: Vec<String> = addresses.iter().map(|a| format!("{a:?}")).collect();
            text.push_str(&format!(
                "accounts.own.drives.{drive}.repositories = [{}]\n",
                entries.join(", ")
            ));
        }
        text
    }

    /// A configuration keeping `addresses` in the drive `home`, asking
    /// for the workspaces.
    fn asking(addresses: &[&str]) -> String {
        drives(true, &[("home", addresses)])
    }

    /// The workspaces `config` wants kept, as a pass's end keeps them.
    fn keep(config: &Config) -> (Outcome, Vec<String>) {
        let wanted = wanted_workspaces(config).expect("the workspaces asked for");
        let mut lines = Vec::new();
        let outcome = keep_workspaces(&wanted, &mut |line| lines.push(line));
        (outcome, lines)
    }

    /// The JSON the file at `path` holds.
    fn json(path: &Path) -> Value {
        serde_json::from_slice(&fs::read(path).expect("the workspace")).expect("JSON")
    }

    /// The names of `value`'s members, in its order.
    fn members(value: &Value) -> Vec<String> {
        value
            .as_object()
            .expect("an object")
            .keys()
            .cloned()
            .collect()
    }

    /// The line a pass says when it writes the workspace `file` with
    /// `folders`.
    fn written(file: &Path, folders: &str) -> String {
        format!(
            "boomdrive: workspace written at {}: {folders}",
            file.display()
        )
    }

    /// The beginning of the line a pass says when it leaves `file`.
    fn left(file: &Path) -> String {
        format!("boomdrive: workspace left at {}: ", file.display())
    }

    /// Every file under `dir` whose name ends in `.code-workspace`, sorted.
    fn workspaces_under(dir: &Path) -> Vec<PathBuf> {
        let mut found = Vec::new();
        let Ok(read) = fs::read_dir(dir) else {
            return found;
        };
        for entry in read {
            let path = entry.expect("an entry").path();
            let kind = fs::symlink_metadata(&path).expect("the entry").file_type();
            if kind.is_dir() {
                found.extend(workspaces_under(&path));
            } else if path.extension() == Some(OsStr::new("code-workspace")) {
                found.push(path);
            }
        }
        found.sort();
        found
    }

    /// A folder's mode, set for a test and set back to 0755 when dropped,
    /// so the scratch folder can be removed.
    struct Mode(PathBuf);

    impl Mode {
        fn set(folder: &Path, mode: u32) -> Mode {
            fs::set_permissions(folder, fs::Permissions::from_mode(mode))
                .expect("the folder's mode");
            Mode(folder.to_path_buf())
        }
    }

    impl Drop for Mode {
        fn drop(&mut self) {
            let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
        }
    }

    /// The views spec's "The first workspaces": each drive's, at the
    /// repository root alone, its folders its repositories in the file's
    /// order, and `settings` after them.
    #[test]
    fn the_first_workspaces() {
        let home = Home::new();
        let config = home.config(&drives(
            true,
            &[
                ("home", &["agentplot/flume", "example/crew"]),
                ("work", &["agentplot/sluice"]),
            ],
        ));

        let (outcome, lines) = keep(&config);

        let (mine, work) = (home.workspace("home"), home.workspace("work"));
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(
            json(&mine),
            json!({
                "folders": [{"path": "agentplot/flume"}, {"path": "example/crew"}],
                "settings": {},
            })
        );
        assert_eq!(members(&json(&mine)), ["folders", "settings"]);
        assert_eq!(
            json(&work),
            json!({"folders": [{"path": "agentplot/sluice"}], "settings": {}})
        );
        assert_eq!(
            workspaces_under(&home.path().join("Code")),
            [mine.clone(), work.clone()],
            "no owner's folder holds a workspace"
        );
        assert_eq!(
            lines,
            [
                written(&mine, "agentplot/flume, example/crew"),
                written(&work, "agentplot/sluice"),
            ]
        );
    }

    /// The views spec's "No workspace asked for": a pass writes none.
    #[test]
    fn no_workspace_asked_for() {
        let home = Home::new();
        let config = home.config(&drives(false, &[("home", &[])]));

        let (outcome, lines) = home.pass(&config, false);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(workspaces_under(home.path()), [] as [PathBuf; 0]);
        assert_eq!(lines, [] as [String; 0]);
    }

    /// The views spec's "A kept repository with no view": its folder is
    /// in the workspace all the same.
    #[test]
    fn a_kept_repository_with_no_view() {
        let home = Home::new();
        let config = home.config(&asking(&["agentplot/flume", "agentplot/empty"]));

        let (outcome, lines) = keep(&config);

        let file = home.workspace("home");
        assert_eq!(outcome, Outcome::Kept);
        assert!(!home.path().join("Code/agentplot/empty").exists());
        assert_eq!(
            json(&file)["folders"],
            json!([{"path": "agentplot/flume"}, {"path": "agentplot/empty"}])
        );
        assert_eq!(lines, [written(&file, "agentplot/flume, agentplot/empty")]);
    }

    /// The views spec's "A drive keeping none": a pass, at its end,
    /// writes the drive's workspace with no folders.
    #[test]
    fn a_drive_keeping_none() {
        let home = Home::new();
        let config = home.config(&drives(true, &[("garden", &[])]));

        let (outcome, lines) = home.pass(&config, false);

        let file = home.workspace("garden");
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(json(&file), json!({"folders": [], "settings": {}}));
        assert_eq!(lines, [written(&file, "no folders")]);
    }

    /// The views spec's "A new root": the drive's workspace is made afresh
    /// at the new root, and the one at the old root is left as it was.
    #[test]
    fn a_new_root() {
        let home = Home::new();
        let under = |root: &str| {
            home.config(&format!(
                "repository_root = {root:?}\n{}",
                asking(&["agentplot/flume"])
            ))
        };
        keep(&under("~/Code"));
        let old = home.workspace("home");
        let before = fs::read(&old).expect("the old root's workspace");

        let (outcome, lines) = keep(&under("~/src"));

        let new = home.path().join("src/home.code-workspace");
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(
            json(&new),
            json!({"folders": [{"path": "agentplot/flume"}], "settings": {}})
        );
        assert_eq!(fs::read(&old).expect("the old root's workspace"), before);
        assert_eq!(lines, [written(&new, "agentplot/flume")]);
    }

    /// The views spec's "The user's settings kept".
    #[test]
    fn the_users_settings_kept() {
        let home = Home::new();
        keep(&home.config(&asking(&["agentplot/flume"])));
        let file = home.workspace("home");
        fs::write(
            &file,
            "{\n\
             \t\"folders\": [\n\
             \t\t{\n\
             \t\t\t\"path\": \"agentplot/flume\"\n\
             \t\t}\n\
             \t],\n\
             \t\"settings\": {\n\
             \t\t\"editor.tabSize\": 2\n\
             \t},\n\
             \t\"extensions\": {\n\
             \t\t\"recommendations\": [\"rust-lang.rust-analyzer\"]\n\
             \t}\n\
             }\n",
        )
        .expect("the user's settings");

        let (outcome, lines) =
            keep(&home.config(&asking(&["agentplot/flume", "agentplot/sluice"])));

        let value = json(&file);
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(members(&value), ["folders", "settings", "extensions"]);
        assert_eq!(
            value,
            json!({
                "folders": [{"path": "agentplot/flume"}, {"path": "agentplot/sluice"}],
                "settings": {"editor.tabSize": 2},
                "extensions": {"recommendations": ["rust-lang.rust-analyzer"]},
            })
        );
        assert_eq!(lines, [written(&file, "agentplot/flume, agentplot/sluice")]);
    }

    /// The views spec's "Folders edited by hand": set back at the next
    /// pass.
    #[test]
    fn folders_edited_by_hand() {
        let home = Home::new();
        let config = home.config(&asking(&["agentplot/flume", "agentplot/sluice"]));
        keep(&config);
        let file = home.workspace("home");
        let mut edited = json(&file);
        edited["folders"] = json!([{"path": "agentplot/sluice"}]);
        fs::write(
            &file,
            serde_json::to_string_pretty(&edited).expect("the edit"),
        )
        .expect("the folders edited");

        let (outcome, lines) = keep(&config);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [written(&file, "agentplot/flume, agentplot/sluice")]);
        assert_eq!(
            json(&file),
            json!({
                "folders": [{"path": "agentplot/flume"}, {"path": "agentplot/sluice"}],
                "settings": {},
            })
        );
    }

    /// The views spec's "A quiet pass leaves the workspaces": each file is
    /// neither written nor said.
    #[test]
    fn a_quiet_pass_leaves_the_workspaces() {
        let home = Home::new();
        let config = home.config(&drives(
            true,
            &[("home", &["agentplot/flume"]), ("work", &["example/crew"])],
        ));
        keep(&config);
        let files = [home.workspace("home"), home.workspace("work")];
        let before: Vec<(Vec<u8>, u64)> = files
            .iter()
            .map(|file| {
                let ino = fs::metadata(file).expect("the workspace").ino();
                (fs::read(file).expect("the workspace"), ino)
            })
            .collect();

        let (outcome, lines) = keep(&config);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [] as [String; 0]);
        for (file, (bytes, ino)) in files.iter().zip(before) {
            assert_eq!(fs::read(file).expect("the workspace"), bytes);
            assert_eq!(
                fs::metadata(file).expect("the workspace").ino(),
                ino,
                "{} was written",
                file.display()
            );
        }
    }

    /// The views spec's "A comment in the workspace": the file is left as
    /// it is and named at every pass, its words never quoted.
    #[test]
    fn a_comment_in_the_workspace() {
        let home = Home::new();
        keep(&home.config(&asking(&["agentplot/flume"])));
        let file = home.workspace("home");
        let mine = "{\n\
                    \t// mine\n\
                    \t\"folders\": [\n\
                    \t\t{\n\
                    \t\t\t\"path\": \"agentplot/flume\"\n\
                    \t\t}\n\
                    \t]\n\
                    }\n";
        fs::write(&file, mine).expect("a comment of the user's");
        let config = home.config(&asking(&["agentplot/flume", "agentplot/sluice"]));

        let (outcome, lines) = keep(&config);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
        assert_eq!(lines.len(), 1, "{lines:#?}");
        let prefix = format!("{}it is not JSON the runner reads: ", left(&file));
        assert!(lines[0].starts_with(&prefix), "{lines:#?}");
        assert!(!lines[0].contains("mine"), "{lines:#?}");

        let (outcome, again) = keep(&config);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(again, lines, "named at every pass");
        assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
    }

    /// The views spec's "A link at the workspace's path": the link and its
    /// target are left as they are, and named.
    #[test]
    fn a_link_at_the_workspaces_path() {
        let home = Home::new();
        let elsewhere = home.path().join("elsewhere/mine.code-workspace");
        let theirs = "{\"folders\": [{\"path\": \"..\"}]}\n";
        fs::create_dir_all(elsewhere.parent().expect("its folder")).expect("a folder elsewhere");
        fs::write(&elsewhere, theirs).expect("a workspace elsewhere");
        let file = home.workspace("home");
        fs::create_dir_all(file.parent().expect("the root")).expect("the repository root");
        std::os::unix::fs::symlink(&elsewhere, &file).expect("a link at the workspace's path");

        let (outcome, lines) = keep(&home.config(&asking(&["agentplot/flume"])));

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [format!("{}it is a link", left(&file))]);
        assert!(fs::symlink_metadata(&file)
            .expect("the link")
            .file_type()
            .is_symlink());
        assert_eq!(fs::read_link(&file).expect("the link"), elsewhere);
        assert_eq!(fs::read_to_string(&elsewhere).expect("its target"), theirs);
    }

    /// The views spec's "A workspace that cannot be written": no file can
    /// be made at the repository root, so the file is named and the pass
    /// fails, the file as it was and no partial file left.
    #[test]
    fn a_workspace_that_cannot_be_written() {
        let home = Home::new();
        let config = home.config(&asking(&["agentplot/flume"]));
        keep(&config);
        let file = home.workspace("home");
        let edited = "{\"folders\": [], \"settings\": {}}\n";
        fs::write(&file, edited).expect("the folders edited");
        let root = file.parent().expect("the repository root");
        let _mode = Mode::set(root, 0o555);

        let (outcome, lines) = keep(&config);

        assert_eq!(outcome, Outcome::Failed);
        assert_eq!(lines.len(), 1, "{lines:#?}");
        let prefix = format!("boomdrive: workspace not written at {}: ", file.display());
        assert!(lines[0].starts_with(&prefix), "{lines:#?}");
        assert_eq!(fs::read_to_string(&file).expect("the workspace"), edited);
        let held: Vec<String> = fs::read_dir(root)
            .expect("the repository root")
            .map(|entry| {
                entry
                    .expect("an entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        assert_eq!(held, ["home.code-workspace"]);
    }

    /// A pass that is stopped writes no workspace.
    #[test]
    fn a_stopped_pass_writes_no_workspace() {
        let home = Home::new();
        let config = home.config(&asking(&["agentplot/flume"]));

        let (_, lines) = home.pass(&config, true);

        assert_eq!(lines, [] as [String; 0]);
        assert_eq!(workspaces_under(home.path()), [] as [PathBuf; 0]);
    }

    /// The clone of `agentplot/flume` as a pass found it, recording the
    /// drive `drive`.
    fn recording(home: &Home, drive: &str) -> Found {
        Found {
            address: Address::parse("agentplot/flume").expect("an address"),
            container: home
                .path()
                .join(".local/share/boomdrive/repositories/agentplot/flume"),
            drive: Some(drive.to_string()),
        }
    }

    /// The departed drives' workspaces taken, as a pass's end takes them,
    /// of the clones `clones` found and the drives `standing` their
    /// keeping recorded.
    fn take(config: &Config, clones: &[Found], standing: &[&str]) -> (Outcome, Vec<String>) {
        let standing: BTreeSet<String> = standing.iter().map(|drive| drive.to_string()).collect();
        let mut lines = Vec::new();
        let outcome = take_departed(config, clones, &standing, &mut |line| lines.push(line));
        (outcome, lines)
    }

    /// The line a pass says when it removes the workspace `file` of the
    /// departed drive `drive`.
    fn taken(file: &Path, drive: &str) -> String {
        format!(
            "boomdrive: workspace removed at {}: its drive {drive} has left the configuration",
            file.display()
        )
    }

    /// The views spec's "A workspace of the person's own": the departed
    /// drive's workspace goes, and one no clone's drive names is left.
    #[test]
    fn a_workspace_of_the_persons_own() {
        let home = Home::new();
        keep(&home.config(&asking(&["agentplot/flume"])));
        let notes = home.workspace("notes");
        let theirs = "{\"folders\": [{\"path\": \"agentplot/flume\"}], \"settings\": {}}\n";
        fs::write(&notes, theirs).expect("a workspace of the person's");
        let config = home.config(&drives(true, &[]));

        let (outcome, lines) = take(&config, &[recording(&home, "home")], &[]);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [taken(&home.workspace("home"), "home")]);
        assert!(!home.workspace("home").exists(), "the workspace is left");
        assert_eq!(fs::read_to_string(&notes).expect("the person's"), theirs);
    }

    /// The views spec's "A departed drive's workspace the runner cannot
    /// read": left as it is and named, its words never quoted.
    #[test]
    fn a_departed_drives_workspace_the_runner_cannot_read() {
        let home = Home::new();
        let file = home.workspace("home");
        fs::create_dir_all(file.parent().expect("the root")).expect("the repository root");
        let mine = "{\n\t// mine\n\t\"folders\": []\n}\n";
        fs::write(&file, mine).expect("a comment of the person's");
        let config = home.config(&drives(true, &[]));

        let (outcome, lines) = take(&config, &[recording(&home, "home")], &[]);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines.len(), 1, "{lines:#?}");
        let prefix = format!("{}it is not JSON the runner reads: ", left(&file));
        assert!(lines[0].starts_with(&prefix), "{lines:#?}");
        assert!(!lines[0].contains("mine"), "{lines:#?}");
        assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
    }

    /// The views spec's "Its repositories moved": the drive's one
    /// repository is named by another drive, so its clone, found
    /// recording the departed drive, records the other once kept, and the
    /// departed drive's workspace goes.
    #[test]
    fn its_repositories_moved() {
        let home = Home::new();
        keep(&home.config(&asking(&["agentplot/flume"])));
        let config = home.config(&drives(true, &[("work", &["agentplot/flume"])]));

        let (kept, written_lines) = keep(&config);
        let (outcome, lines) = take(&config, &[recording(&home, "home")], &["work"]);

        let work = home.workspace("work");
        assert_eq!((kept, outcome), (Outcome::Kept, Outcome::Kept));
        assert_eq!(written_lines, [written(&work, "agentplot/flume")]);
        assert_eq!(lines, [taken(&home.workspace("home"), "home")]);
        assert!(!home.workspace("home").exists(), "the workspace is left");
        assert_eq!(
            json(&work),
            json!({"folders": [{"path": "agentplot/flume"}], "settings": {}})
        );
    }
}
