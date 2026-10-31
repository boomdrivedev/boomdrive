//! The work the forge lacks in a clone and in its view (design 8): what
//! the runner would lose by removing them. The forge is judged by
//! origin's refs as the last fetch left them, so a misjudgement keeps a
//! clone longer than needed and never loses work.

use crate::clone;
use crate::git::{At, Git, Words};
use crate::view;
use std::fmt;
use std::path::{Path, PathBuf};

/// Something in a clone or its view that the forge lacks, which holds
/// them where they are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Holding {
    /// A ref of the clone's own, by its full name, holding `ahead`
    /// commits that origin's refs do not reach: a branch, a tag, the
    /// stash.
    Ref { name: String, ahead: u32 },
    /// A worktree of the clone other than its view, whose folder is
    /// there: a Flume place among them.
    Worktree { path: PathBuf },
    /// Changes to tracked files and untracked files in the view, in
    /// `count` files. Files git ignores are not counted.
    ViewChanges { count: usize },
    /// The view's HEAD, holding `ahead` commits that origin's refs do
    /// not reach.
    ViewCommits { ahead: u32 },
}

impl fmt::Display for Holding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Holding::Ref { name, ahead } => write!(
                f,
                "{}, {} the forge lacks",
                ref_words(name),
                commits(*ahead)
            ),
            Holding::Worktree { path } => write!(f, "the worktree {}", path.display()),
            Holding::ViewChanges { count } => {
                write!(f, "the view's changes in {}", files(*count))
            }
            Holding::ViewCommits { ahead } => {
                write!(f, "the view's HEAD, {} the forge lacks", commits(*ahead))
            }
        }
    }
}

/// A ref, in words: a branch or a tag by its own name, the stash, any
/// other ref by its full name.
fn ref_words(name: &str) -> String {
    if let Some(branch) = name.strip_prefix("refs/heads/") {
        format!("branch {branch}")
    } else if let Some(tag) = name.strip_prefix("refs/tags/") {
        format!("tag {tag}")
    } else if name == "refs/stash" {
        "the stash".to_string()
    } else {
        name.to_string()
    }
}

/// `n` commits, in words.
fn commits(n: u32) -> String {
    match n {
        1 => "1 commit".to_string(),
        n => format!("{n} commits"),
    }
}

/// `n` files, in words.
fn files(n: usize) -> String {
    match n {
        1 => "1 file".to_string(),
        n => format!("{n} files"),
    }
}

/// What a clone and its view hold, as read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Facts {
    /// Each ref of the clone, by its full name, with how many commits it
    /// holds that origin's refs do not reach.
    pub refs: Vec<(String, u32)>,
    /// Each worktree of the clone other than its bare repository and its
    /// view, with whether its folder is there.
    pub worktrees: Vec<(PathBuf, bool)>,
    /// The view, where the folder at its path is the clone's worktree.
    pub view: Option<View>,
}

/// What a view holds, as read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    /// What `status --porcelain` says in it.
    pub status: String,
    /// How many commits its HEAD holds that origin's refs do not reach.
    pub ahead: u32,
}

/// Design 8: each ref of the clone's own holding commits origin's refs
/// do not reach, each worktree other than the view whose folder is
/// there, and what the view holds.
pub fn holdings(facts: &Facts) -> Vec<Holding> {
    let refs = facts
        .refs
        .iter()
        .filter(|(name, ahead)| counted(name) && *ahead > 0)
        .map(|(name, ahead)| Holding::Ref {
            name: name.clone(),
            ahead: *ahead,
        });
    let worktrees = facts
        .worktrees
        .iter()
        .filter(|(_, there)| *there)
        .map(|(path, _)| Holding::Worktree { path: path.clone() });
    refs.chain(worktrees)
        .chain(facts.view.iter().flat_map(view_holdings))
        .collect()
}

/// What the view holds: its changes, untracked files among them and
/// ignored files not, and its HEAD where origin's refs do not reach it.
pub fn view_holdings(view: &View) -> Vec<Holding> {
    let count = view
        .status
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("!!"))
        .count();
    let mut holdings = Vec::new();
    if count > 0 {
        holdings.push(Holding::ViewChanges { count });
    }
    if view.ahead > 0 {
        holdings.push(Holding::ViewCommits { ahead: view.ahead });
    }
    holdings
}

/// Whether the commits of the ref `name` are counted: any ref but
/// origin's own and the runner's.
fn counted(name: &str) -> bool {
    !name.starts_with("refs/remotes/origin/") && !name.starts_with("refs/flywheel/")
}

/// Reads what the clone `bare` holds, and its view at `view` where the
/// runner made one. A worktree registered at the view's path is the
/// view's, whatever stands there now.
pub fn read(git: &Git, bare: &Path, view: Option<&Path>) -> Result<Facts, Words> {
    let at = At::GitDir(bare);
    let mut refs = Vec::new();
    for name in git
        .run(at, ["for-each-ref", "--format=%(refname)"])?
        .lines()
    {
        if counted(name) {
            refs.push((name.to_string(), ahead(git, at, name)?));
        }
    }
    let worktrees = clone::worktrees(git, bare)?
        .into_iter()
        .filter(|worktree| {
            !worktree.bare && !view.is_some_and(|view| view::same_path(&worktree.path, view))
        })
        .map(|worktree| {
            let there = worktree.path.symlink_metadata().is_ok();
            (worktree.path, there)
        })
        .collect();
    let view = match view {
        Some(path) if view::is_view(git, bare, path)? => Some(read_view(git, path)?),
        _ => None,
    };
    Ok(Facts {
        refs,
        worktrees,
        view,
    })
}

/// Reads what the view at `path`, a worktree of the clone, holds.
pub fn read_view(git: &Git, path: &Path) -> Result<View, Words> {
    let at = At::Dir(path);
    let status = git.run(
        at,
        [
            "--no-optional-locks",
            "status",
            "--porcelain",
            "--untracked-files=all",
        ],
    )?;
    let born = git
        .ask(at, ["rev-parse", "--verify", "--quiet", "HEAD^{commit}"])?
        .is_some();
    let ahead = if born { ahead(git, at, "HEAD")? } else { 0 };
    Ok(View { status, ahead })
}

/// How many commits `rev` holds at `at` that origin's refs do not reach.
fn ahead(git: &Git, at: At, rev: &str) -> Result<u32, Words> {
    let counted = git.run(
        at,
        [
            "rev-list",
            "--count",
            rev,
            "--not",
            "--remotes=origin",
            "--",
        ],
    )?;
    counted
        .trim()
        .parse()
        .map_err(|_| Words(format!("rev-list counted {:?}", counted.trim())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refs(refs: &[(&str, u32)]) -> Facts {
        Facts {
            refs: refs
                .iter()
                .map(|(name, ahead)| (name.to_string(), *ahead))
                .collect(),
            ..Facts::default()
        }
    }

    fn view(status: &str, ahead: u32) -> Facts {
        Facts {
            view: Some(View {
                status: status.to_string(),
                ahead,
            }),
            ..Facts::default()
        }
    }

    #[test]
    fn nothing_held() {
        assert_eq!(holdings(&Facts::default()), []);
        assert_eq!(
            holdings(&refs(&[("refs/heads/main", 0), ("refs/tags/v1", 0)])),
            []
        );
        assert_eq!(holdings(&view("", 0)), []);
    }

    #[test]
    fn a_ref_the_forge_lacks() {
        assert_eq!(
            holdings(&refs(&[
                ("refs/heads/main", 0),
                ("refs/heads/wip", 2),
                ("refs/stash", 2),
                ("refs/tags/mine", 1),
            ])),
            [
                Holding::Ref {
                    name: "refs/heads/wip".to_string(),
                    ahead: 2
                },
                Holding::Ref {
                    name: "refs/stash".to_string(),
                    ahead: 2
                },
                Holding::Ref {
                    name: "refs/tags/mine".to_string(),
                    ahead: 1
                },
            ]
        );
    }

    #[test]
    fn origins_refs_and_the_runners_are_not_counted() {
        assert_eq!(
            holdings(&refs(&[
                ("refs/remotes/origin/main", 3),
                ("refs/remotes/origin/HEAD", 3),
                ("refs/flywheel/view", 1),
            ])),
            []
        );
        assert_eq!(
            holdings(&refs(&[("refs/remotes/upstream/main", 1)])),
            [Holding::Ref {
                name: "refs/remotes/upstream/main".to_string(),
                ahead: 1
            }]
        );
    }

    #[test]
    fn a_worktree_whose_folder_is_there() {
        let facts = Facts {
            worktrees: vec![
                (PathBuf::from("/d/r/places/change--one"), true),
                (PathBuf::from("/d/r/places/change--gone"), false),
            ],
            ..Facts::default()
        };
        assert_eq!(
            holdings(&facts),
            [Holding::Worktree {
                path: PathBuf::from("/d/r/places/change--one")
            }]
        );
    }

    #[test]
    fn changes_in_the_view() {
        assert_eq!(
            holdings(&view(" M README\n?? notes\n?? build/out\n", 0)),
            [Holding::ViewChanges { count: 3 }]
        );
    }

    #[test]
    fn ignored_files_hold_nothing() {
        assert_eq!(holdings(&view("!! build/\n!! target/out\n", 0)), []);
        assert_eq!(
            holdings(&view("!! build/\n?? notes\n", 0)),
            [Holding::ViewChanges { count: 1 }]
        );
    }

    #[test]
    fn a_commit_in_the_view() {
        assert_eq!(holdings(&view("", 1)), [Holding::ViewCommits { ahead: 1 }]);
    }

    #[test]
    fn every_holding_together() {
        let facts = Facts {
            worktrees: vec![(PathBuf::from("/d/r/places/change--one"), true)],
            ..view(" M README\n", 2)
        };
        let facts = Facts {
            refs: vec![("refs/heads/wip".to_string(), 1)],
            ..facts
        };
        assert_eq!(
            holdings(&facts),
            [
                Holding::Ref {
                    name: "refs/heads/wip".to_string(),
                    ahead: 1
                },
                Holding::Worktree {
                    path: PathBuf::from("/d/r/places/change--one")
                },
                Holding::ViewChanges { count: 1 },
                Holding::ViewCommits { ahead: 2 },
            ]
        );
    }

    #[test]
    fn each_holdings_words() {
        let said = |holding: Holding| holding.to_string();
        assert_eq!(
            said(Holding::Ref {
                name: "refs/heads/wip".to_string(),
                ahead: 2
            }),
            "branch wip, 2 commits the forge lacks"
        );
        assert_eq!(
            said(Holding::Ref {
                name: "refs/tags/v1".to_string(),
                ahead: 1
            }),
            "tag v1, 1 commit the forge lacks"
        );
        assert_eq!(
            said(Holding::Ref {
                name: "refs/stash".to_string(),
                ahead: 2
            }),
            "the stash, 2 commits the forge lacks"
        );
        assert_eq!(
            said(Holding::Ref {
                name: "refs/notes/commits".to_string(),
                ahead: 1
            }),
            "refs/notes/commits, 1 commit the forge lacks"
        );
        assert_eq!(
            said(Holding::Worktree {
                path: PathBuf::from("/d/r/places/change--one")
            }),
            "the worktree /d/r/places/change--one"
        );
        assert_eq!(
            said(Holding::ViewChanges { count: 1 }),
            "the view's changes in 1 file"
        );
        assert_eq!(
            said(Holding::ViewChanges { count: 3 }),
            "the view's changes in 3 files"
        );
        assert_eq!(
            said(Holding::ViewCommits { ahead: 1 }),
            "the view's HEAD, 1 commit the forge lacks"
        );
    }
}
