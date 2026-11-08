//! The ids crew's implementation gives its wants (design Decisions 17 and
//! 19): `<label>/<Request>` for a request, `<label>/<role>/<k>` for a
//! role's `k`th run, and `<label>/<role>/<k>/done` for its done.

use crate::records::whole;
use boomdrive_control::Id;

/// What an id of partition `label` names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Named {
    Request(u64),
    Run(String, u64),
    Done(String, u64),
}

/// A request's id.
pub fn request(label: &str, number: u64) -> Id {
    Id::new(format!("{label}/{number}"))
}

/// A role's `k`th run's id.
pub fn run(label: &str, role: &str, k: u64) -> Id {
    Id::new(format!("{label}/{role}/{k}"))
}

/// The id of the done of a role's `k`th run.
pub fn done(label: &str, role: &str, k: u64) -> Id {
    Id::new(format!("{label}/{role}/{k}/done"))
}

/// What `id` names in partition `label`, none where it is no id of the
/// partition's.
pub fn named(label: &str, id: &Id) -> Option<Named> {
    let rest = id.as_str().strip_prefix(label)?.strip_prefix('/')?;
    let parts: Vec<&str> = rest.split('/').collect();
    match parts.as_slice() {
        [n] => whole(n).map(Named::Request),
        [role, k] if whole(role).is_none() => Some(Named::Run(role.to_string(), whole(k)?)),
        [role, k, "done"] if whole(role).is_none() => {
            Some(Named::Done(role.to_string(), whole(k)?))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_id_names_what_it_was_given_for() {
        assert_eq!(
            named("home", &request("home", 12)),
            Some(Named::Request(12))
        );
        assert_eq!(
            named("home", &run("home", "planner", 3)),
            Some(Named::Run("planner".to_string(), 3))
        );
        assert_eq!(
            named("home", &done("home", "planner", 3)),
            Some(Named::Done("planner".to_string(), 3))
        );
        for other in ["work/12", "home-x/1", "home/x", "home/1/2", "fake/1"] {
            assert_eq!(named("home", &Id::new(other)), None, "{other}");
        }
    }
}
