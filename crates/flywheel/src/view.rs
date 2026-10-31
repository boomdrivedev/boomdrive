//! A repository's view (design 7): a worktree of its clone at
//! `<views>/<owner>/<name>`, detached at `origin/<default>` and never on
//! a branch. It is made where its folder is missing and never over a
//! folder that is there, and moved only while it stands where the runner
//! last put it, clean of tracked changes, so a view the user moved or is
//! working in is left as they have it (`herdr.14`, carried whole). When
//! the views root changes, the old view is removed once it holds nothing
//! the forge lacks, and the view made at the new path.

use crate::clone;
use crate::git::{At, Git, Words};
use crate::lacks::{self, Holding};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// The key in a clone's own git config that records the path its view
/// was last made or moved at.
const PATH: &str = "flywheel.view";

/// The ref in a clone that records the commit the runner last put its
/// view at, which keeps that commit reachable.
const COMMIT: &str = "refs/flywheel/view";

/// What stands at a view's path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stands {
    /// Nothing. `registered` where git still registers a worktree of the
    /// clone there, as after the user deleted the view.
    Missing { registered: bool },
    /// A folder that is not a worktree of the clone.
    Other,
    /// A worktree of the clone, as it has things checked out.
    Worktree(Checkout),
}

/// What a worktree of the clone has checked out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkout {
    /// Whether its HEAD is detached, naming no branch.
    pub detached: bool,
    /// The commit its HEAD is at.
    pub head: String,
    /// Whether it holds a change to a tracked file. Untracked files are
    /// not counted: they do not hold a view.
    pub changed: bool,
}

/// What deciding a view reads (design 7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewFacts {
    /// What stands at the view's path.
    pub stands: Stands,
    /// The commit the runner last put the view at, where it has.
    pub recorded: Option<String>,
    /// The commit `origin/<default>` is at, none where origin names no
    /// default branch.
    pub target: Option<String>,
}

/// What becomes of a view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewAct {
    /// Nothing is done or said: origin names no default, the view is at
    /// origin's already, or the user has moved it or is working in it.
    Leave,
    /// The view is made, detached at this commit, `force` over a
    /// registration whose folder is gone.
    Make { at: String, force: bool },
    /// The view is moved, detached, to this commit.
    Move { to: String },
    /// The folder at the view's path is not the clone's worktree: it is
    /// left untouched and named, and no view is made elsewhere.
    Occupied,
}

/// Design 7, for a view whose path is the one the configuration gives:
/// made where it is missing, never over a folder that is there, and moved
/// only while it is detached, at the commit the runner last put it at and
/// clean of tracked changes.
pub fn decide(facts: &ViewFacts) -> ViewAct {
    let Some(target) = &facts.target else {
        return ViewAct::Leave;
    };
    match &facts.stands {
        Stands::Missing { registered } => ViewAct::Make {
            at: target.clone(),
            force: *registered,
        },
        Stands::Other => ViewAct::Occupied,
        Stands::Worktree(checkout) => {
            let placed = checkout.detached
                && facts.recorded.as_ref() == Some(&checkout.head)
                && !checkout.changed;
            if placed && checkout.head != *target {
                ViewAct::Move { to: target.clone() }
            } else {
                ViewAct::Leave
            }
        }
    }
}

/// What stands where the runner last made or moved the view, once the
/// views root no longer gives that path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Old {
    /// Nothing, or a folder that is not the clone's worktree.
    Gone,
    /// The clone's worktree, holding these, which the forge lacks.
    View(Vec<Holding>),
}

/// What becomes of the old view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OldAct {
    /// It is forgotten, any folder there left untouched, and the view is
    /// kept at the new path.
    Forget,
    /// It is removed and forgotten, and the view is kept at the new path.
    Remove,
    /// It is left and named with what holds it, and no view is made at
    /// the new path.
    Hold(Vec<Holding>),
}

/// Design 7, for a view whose recorded path the views root no longer
/// gives: removed once it holds nothing the forge lacks (design 8's
/// removal), left while it holds anything, and forgotten where it is no
/// longer the clone's worktree.
pub fn decide_old(old: &Old) -> OldAct {
    match old {
        Old::Gone => OldAct::Forget,
        Old::View(held) if held.is_empty() => OldAct::Remove,
        Old::View(held) => OldAct::Hold(held.clone()),
    }
}

/// Reads what stands at `path`, the old view of the clone `bare`.
pub fn old(git: &Git, bare: &Path, path: &Path) -> Result<Old, Words> {
    if !is_worktree_of(git, bare, path)? {
        return Ok(Old::Gone);
    }
    Ok(Old::View(lacks::view_holdings(&lacks::read_view(
        git, path,
    )?)))
}

/// Reads the facts of the view at `path` of the clone `bare`, whose
/// origin's default branch is `default`.
pub fn facts(git: &Git, bare: &Path, path: &Path, default: &str) -> Result<ViewFacts, Words> {
    let at = At::GitDir(bare);
    let commit = |name: &str| -> Result<Option<String>, Words> {
        let said = git.ask(
            at,
            [
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("{name}^{{commit}}"),
            ],
        )?;
        Ok(said.map(|said| said.trim().to_string()))
    };
    Ok(ViewFacts {
        stands: stands(git, bare, path)?,
        recorded: commit(COMMIT)?,
        target: commit(&format!("refs/remotes/origin/{default}"))?,
    })
}

/// What stands at `path`. A link there is a folder that is not the
/// clone's worktree: it is not followed.
fn stands(git: &Git, bare: &Path, path: &Path) -> Result<Stands, Words> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let real = resolved(path);
            let registered = clone::worktrees(git, bare)?
                .iter()
                .any(|worktree| !worktree.bare && (worktree.path == path || worktree.path == real));
            return Ok(Stands::Missing { registered });
        }
        Ok(metadata) if metadata.is_dir() => {}
        _ => return Ok(Stands::Other),
    }
    if !is_worktree(git, bare, path) {
        return Ok(Stands::Other);
    }
    let said = git.run(
        At::Dir(path),
        [
            "--no-optional-locks",
            "status",
            "--porcelain=v2",
            "--branch",
            "--untracked-files=no",
        ],
    )?;
    read_status(&said)
        .map(Stands::Worktree)
        .ok_or_else(|| Words(format!("git status names no HEAD in {}", path.display())))
}

/// Whether the folder at `path` is a worktree of the clone `bare`, a
/// link there not followed.
pub fn is_worktree_of(git: &Git, bare: &Path, path: &Path) -> Result<bool, Words> {
    Ok(matches!(stands(git, bare, path)?, Stands::Worktree(_)))
}

/// Whether `a` and `b` name one folder: as written, as git records a
/// worktree's folder, or on disk.
pub fn same_path(a: &Path, b: &Path) -> bool {
    a == b || resolved(a) == resolved(b) || same(a, b)
}

/// `path` as git records a worktree's folder: its nearest ancestor that
/// exists resolved, so a folder that is gone still matches its
/// registration.
fn resolved(path: &Path) -> PathBuf {
    for ancestor in path.ancestors() {
        if let (Ok(real), Ok(rest)) = (ancestor.canonicalize(), path.strip_prefix(ancestor)) {
            return real.join(rest);
        }
    }
    path.to_path_buf()
}

/// Whether the folder `path` is a worktree of the clone `bare`: git
/// there names `bare` as its common repository (swancloud's own test)
/// and `path` itself as its top level, each the same folder on disk.
fn is_worktree(git: &Git, bare: &Path, path: &Path) -> bool {
    let Ok(said) = git.run(
        At::Dir(path),
        [
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
            "--show-toplevel",
        ],
    ) else {
        return false;
    };
    let mut lines = said.lines();
    match (lines.next(), lines.next()) {
        (Some(common), Some(top)) => same(Path::new(common), bare) && same(Path::new(top), path),
        _ => false,
    }
}

/// Whether `a` and `b` are one folder on disk, however each is spelled.
fn same(a: &Path, b: &Path) -> bool {
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(a), Ok(b)) => a.dev() == b.dev() && a.ino() == b.ino(),
        _ => false,
    }
}

/// What `status --porcelain=v2 --branch` says in `said`: the HEAD from
/// its `# branch.` headers, and a change where any other line is.
fn read_status(said: &str) -> Option<Checkout> {
    let mut head = None;
    let mut detached = None;
    let mut changed = false;
    for line in said.lines() {
        if let Some(oid) = line.strip_prefix("# branch.oid ") {
            head = Some(oid.to_string());
        } else if let Some(branch) = line.strip_prefix("# branch.head ") {
            detached = Some(branch == "(detached)");
        } else if !line.starts_with('#') && !line.is_empty() {
            changed = true;
        }
    }
    Some(Checkout {
        detached: detached?,
        head: head?,
        changed,
    })
}

/// Makes the view at `path`, a worktree of `bare` detached at `commit`,
/// git making the folders above it. `force` overrides only git's refusal
/// of a path it still registers as a worktree whose folder is gone,
/// since no branch is named.
pub fn make(git: &Git, bare: &Path, path: &Path, commit: &str, force: bool) -> Result<(), Words> {
    let mut args = vec![
        OsStr::new("worktree"),
        OsStr::new("add"),
        OsStr::new("--quiet"),
        OsStr::new("--detach"),
    ];
    if force {
        args.push(OsStr::new("--force"));
    }
    args.extend([path.as_os_str(), OsStr::new(commit)]);
    git.run(At::GitDir(bare), args)?;
    Ok(())
}

/// Moves the view at `path` to `commit`, detached. Untracked files are
/// carried across, and git refuses where the commit would overwrite one.
pub fn move_to(git: &Git, path: &Path, commit: &str) -> Result<(), Words> {
    git.run(At::Dir(path), ["switch", "--quiet", "--detach", commit])?;
    Ok(())
}

/// The path the runner last made or moved the view of the clone `bare`
/// at, where it has.
pub fn recorded_path(git: &Git, bare: &Path) -> Result<Option<PathBuf>, Words> {
    let said = git.ask(At::GitDir(bare), ["config", "--local", "--get", PATH])?;
    Ok(said.map(|said| PathBuf::from(said.strip_suffix('\n').unwrap_or(&said))))
}

/// Removes the view at `path`, a worktree of the clone `bare` that holds
/// nothing the forge lacks, then its owner's folder where that leaves it
/// empty (design 7). git refuses a view that has changed since.
pub fn remove(git: &Git, bare: &Path, path: &Path) -> Result<(), Words> {
    git.run(
        At::GitDir(bare),
        [
            OsStr::new("worktree"),
            OsStr::new("remove"),
            path.as_os_str(),
        ],
    )?;
    if let Some(owner) = path.parent() {
        let _ = fs::remove_dir(owner);
    }
    Ok(())
}

/// Forgets, in `bare`, the path the runner last put its view at.
pub fn forget(git: &Git, bare: &Path) -> Result<(), Words> {
    git.run(At::GitDir(bare), ["config", "--local", "--unset", PATH])?;
    Ok(())
}

/// Records in `bare` that the runner put its view at `path`, at `commit`.
pub fn record(git: &Git, bare: &Path, path: &Path, commit: &str) -> Result<(), Words> {
    let at = At::GitDir(bare);
    git.run(at, ["update-ref", COMMIT, commit])?;
    git.run(
        at,
        [OsStr::new("config"), OsStr::new(PATH), path.as_os_str()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkout(detached: bool, head: &str, changed: bool) -> Stands {
        Stands::Worktree(Checkout {
            detached,
            head: head.to_string(),
            changed,
        })
    }

    /// Facts whose target is `t`, the view recorded at `c`.
    fn facts(stands: Stands) -> ViewFacts {
        ViewFacts {
            stands,
            recorded: Some("c".to_string()),
            target: Some("t".to_string()),
        }
    }

    #[test]
    fn no_target_is_left() {
        for stands in [
            Stands::Missing { registered: false },
            Stands::Other,
            checkout(true, "c", false),
        ] {
            let facts = ViewFacts {
                target: None,
                ..facts(stands)
            };
            assert_eq!(decide(&facts), ViewAct::Leave, "{facts:?}");
        }
    }

    #[test]
    fn missing_is_made() {
        assert_eq!(
            decide(&facts(Stands::Missing { registered: false })),
            ViewAct::Make {
                at: "t".to_string(),
                force: false
            }
        );
        let never = ViewFacts {
            recorded: None,
            ..facts(Stands::Missing { registered: false })
        };
        assert_eq!(
            decide(&never),
            ViewAct::Make {
                at: "t".to_string(),
                force: false
            }
        );
    }

    #[test]
    fn missing_but_registered_is_made_by_force() {
        assert_eq!(
            decide(&facts(Stands::Missing { registered: true })),
            ViewAct::Make {
                at: "t".to_string(),
                force: true
            }
        );
    }

    #[test]
    fn a_folder_not_the_clones_worktree_is_named() {
        assert_eq!(decide(&facts(Stands::Other)), ViewAct::Occupied);
    }

    #[test]
    fn where_the_runner_put_it_is_moved() {
        assert_eq!(
            decide(&facts(checkout(true, "c", false))),
            ViewAct::Move {
                to: "t".to_string()
            }
        );
    }

    #[test]
    fn at_origin_already_is_left() {
        let level = ViewFacts {
            recorded: Some("t".to_string()),
            ..facts(checkout(true, "t", false))
        };
        assert_eq!(decide(&level), ViewAct::Leave);
    }

    #[test]
    fn moved_by_the_user_is_left() {
        assert_eq!(decide(&facts(checkout(true, "u", false))), ViewAct::Leave);
        assert_eq!(decide(&facts(checkout(true, "t", false))), ViewAct::Leave);
    }

    #[test]
    fn never_recorded_is_left() {
        let never = ViewFacts {
            recorded: None,
            ..facts(checkout(true, "c", false))
        };
        assert_eq!(decide(&never), ViewAct::Leave);
    }

    #[test]
    fn on_a_branch_is_left() {
        assert_eq!(decide(&facts(checkout(false, "c", false))), ViewAct::Leave);
    }

    #[test]
    fn a_tracked_change_is_left() {
        assert_eq!(decide(&facts(checkout(true, "c", true))), ViewAct::Leave);
    }

    #[test]
    fn status_read() {
        let clean = "# branch.oid 1a2b\n# branch.head (detached)\n";
        assert_eq!(
            read_status(clean),
            Some(Checkout {
                detached: true,
                head: "1a2b".to_string(),
                changed: false,
            })
        );
        let changed = "# branch.oid 1a2b\n# branch.head mine\n\
                       1 .M N... 100644 100644 100644 3c4d 3c4d README\n";
        assert_eq!(
            read_status(changed),
            Some(Checkout {
                detached: false,
                head: "1a2b".to_string(),
                changed: true,
            })
        );
        assert_eq!(read_status(""), None);
    }

    #[test]
    fn an_old_view_that_is_gone_is_forgotten() {
        assert_eq!(decide_old(&Old::Gone), OldAct::Forget);
    }

    #[test]
    fn an_old_view_holding_nothing_is_removed() {
        assert_eq!(decide_old(&Old::View(Vec::new())), OldAct::Remove);
    }

    #[test]
    fn an_old_view_holding_work_is_left() {
        let held = vec![
            Holding::ViewChanges { count: 1 },
            Holding::ViewCommits { ahead: 2 },
        ];
        assert_eq!(decide_old(&Old::View(held.clone())), OldAct::Hold(held));
    }
}
