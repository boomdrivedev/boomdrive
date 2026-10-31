//! A repository's view (design 7): a worktree of its clone at
//! `<views>/<owner>/<name>`, detached at `origin/<default>` and never on
//! a branch. It is made where its folder is missing and never over a
//! folder that is there, and moved only while it stands where the runner
//! last put it, clean of tracked changes, so a view the user moved or is
//! working in is left as they have it (`herdr.14`, carried whole).

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
}
