//! `boomdrive-control`'s shared cases, run against crew's implementation
//! over a bare state repository holding crew's records (the
//! control-plane-interface spec's "crew's implementation passes"; Task
//! notes 8.8).

mod common;

use boomdrive_control::shared::{self, Harness};
use boomdrive_control::{ActSort, Id, Want};
use boomdrive_crew::Crew;
use common::{file, plan, rec, State, REQUESTS, ROLES};
use std::collections::BTreeMap;

/// The shared cases' harness: each control plane is deneb's crew's
/// implementation over a state repository of its own, whose
/// `madswan/main` holds the wants as crew writes them, and what the
/// harness sees of it is every ref it holds.
#[derive(Default)]
struct Crews {
    states: Vec<State>,
}

/// The request number of `id`, `madswan/<n>`.
fn number(id: &Id) -> String {
    id.as_str()
        .strip_prefix("madswan/")
        .expect("a madswan id")
        .to_string()
}

/// `wants` as crew writes them in `requests.rec`: each a request naming
/// deneb, an offer a launch of its unit and an act on its launch; and the
/// plan holding each unit.
fn requests(wants: &[Want]) -> (String, Vec<String>) {
    let mut records = Vec::new();
    let mut works: BTreeMap<Id, String> = BTreeMap::new();
    for want in wants {
        let mut fields: Vec<(&str, String)> = Vec::new();
        match want {
            Want::Offer(offer) => {
                let launch = &offer.launch;
                let work = format!("unit/{}", launch.name);
                works.insert(offer.id.clone(), launch.name.clone());
                fields.extend([
                    ("Request", number(&offer.id)),
                    ("Act", "launch".to_string()),
                    ("Host", "deneb".to_string()),
                    ("Work", work),
                    ("Repo", launch.repository.clone()),
                    ("Kind", launch.kind.clone()),
                    ("Name", launch.name.clone()),
                ]);
                let optional = [
                    ("Base", launch.base.clone()),
                    ("Stream", launch.stream.clone()),
                    ("Priority", launch.priority.map(|p| p.to_string())),
                    ("Intent", launch.intent.clone()),
                ];
                fields.extend(optional.into_iter().filter_map(|(f, v)| Some((f, v?))));
                fields.extend(launch.recipients.iter().map(|r| ("Recipient", r.clone())));
                if launch.hold_submission {
                    fields.push(("Hold", "submission".to_string()));
                }
                fields.extend(launch.after.iter().map(|a| ("After", number(a))));
                if let Some(brief) = &launch.brief {
                    fields.push(("Brief", brief.clone()));
                }
            }
            Want::Act(act) => {
                let (name, extra) = match &act.what {
                    ActSort::Priority(p) => ("priority", Some(("Priority", p.to_string()))),
                    ActSort::Drop => ("drop", None),
                    other => panic!("no shared case gives {other:?}"),
                };
                fields.extend([
                    ("Request", number(&act.id)),
                    ("Act", name.to_string()),
                    ("Host", "deneb".to_string()),
                    ("Work", format!("unit/{}", works[&act.launch])),
                    ("Of", number(&act.launch)),
                ]);
                fields.extend(extra);
            }
            other => panic!("no shared case gives {other:?}"),
        }
        fields.push(("Date", "2026-11-01T09:00:00Z".to_string()));
        fields.push(("By", "crew@deneb".to_string()));
        let fields: Vec<(&str, &str)> = fields.iter().map(|(f, v)| (*f, v.as_str())).collect();
        records.push(rec(&fields));
    }
    (file(REQUESTS, &records), works.into_values().collect())
}

impl Harness for Crews {
    type Plane = Crew;
    type Seen = String;

    fn id(&self, n: u32) -> Id {
        Id::new(format!("madswan/{n}"))
    }

    fn plane(&mut self, wants: &[Want]) -> Crew {
        let state = State::new();
        let (requests, units) = requests(wants);
        let units: Vec<&str> = units.iter().map(String::as_str).collect();
        state.main(&[
            ("requests.rec", &requests),
            ("roles.rec", ROLES),
            ("plan.rec", &plan(&units, &[])),
        ]);
        let (crew, _) = state.crew("deneb");
        self.states.push(state);
        crew
    }

    fn seen(&mut self, _plane: &Crew) -> String {
        self.states.last().expect("a state repository").refs()
    }
}

#[test]
fn shared_wants_in_order() {
    shared::wants_in_order(&mut Crews::default());
}

#[test]
fn shared_a_report_comes_back() {
    shared::a_report_comes_back(&mut Crews::default());
}

#[test]
fn shared_a_report_twice() {
    shared::a_report_twice(&mut Crews::default());
}

#[test]
fn shared_a_settled_want_is_given() {
    shared::a_settled_want_is_given(&mut Crews::default());
}
