//! A role's run, read as crew's `asks.py` at `9913f249` reads it (design
//! Decision 19; crew's `roles-run-as-flume-kinds` R1 to R4). A role whose
//! record names this host is wanted while anything is pending for it: an
//! ask, or a place of any host's that waits on it, that no ended turn
//! took. Its run is offered while it is wanted and none stands, its kind
//! the identity `<Kinds>/<Kind>`, and declared done once its turn has
//! delivered. Wanted, standing and done are crew's rules, read here from
//! crew's files (core.1 "Not ruled out"; core.2).

use crate::ids;
use crate::read::{Main, Request, LEASE};
use crate::records::whole;
use flywheel_control::{
    Act, ActSort, Acted, AgentSetting, Launch, Launched, Offer, Outcome, Report, Want,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use weir_capture::rec::Record;

/// Why a place waits on its caller, as Flume's status names it, that makes
/// it wait on the role its launch names as its recipient (crew's
/// `asks.WAITS`).
const WAITS: [&str; 3] = ["answer", "lost", "past_bound"];

/// A place of a host's `places.rec`, or of the runner's report: its
/// request's number, its place and Flume's status of it as carried.
pub struct Placed<'a> {
    pub request: u64,
    pub place: Option<&'a str>,
    pub status: &'a str,
}

/// The place's own facts in Flume's status as the runner carried it: the
/// one place it holds (crew's `asks.status_of`).
fn status_of(text: &str) -> Option<Value> {
    let mut status: Value = serde_json::from_str(text).ok()?;
    if status.get("status").is_some_and(Value::is_object) {
        status = status["status"].take();
    }
    if let Some(places) = status.get("places") {
        status = match places.as_array().map(Vec::as_slice) {
            Some([one]) => one.clone(),
            _ => return None,
        };
    }
    status.is_object().then_some(status)
}

/// Whether a place waits on its caller as the role's (crew's
/// `asks.waiting`): its since, where Flume gives one.
fn waiting(place: &Value) -> Option<Option<i64>> {
    let wait = place.get("waiting_on")?.as_object()?;
    let why = wait.get("why").and_then(Value::as_str);
    let held = why == Some("on_hold") && wait.get("stage").is_none_or(Value::is_null);
    let stalled = place.get("stalled") == Some(&Value::Bool(true));
    (why.is_some_and(|why| WAITS.contains(&why)) || held || stalled)
        .then(|| wait.get("since").and_then(Value::as_i64))
}

/// Whether a place's phase reads it landed (crew's `asks.landed`).
fn landed(place: &Value) -> bool {
    let Some(phase) = place.get("phase").and_then(Value::as_object) else {
        return false;
    };
    let mut bodies = phase.values();
    match (bodies.next(), bodies.next()) {
        (Some(body), None) => body
            .get("over")
            .and_then(|over| over.get("landed"))
            .is_some_and(Value::is_object),
        _ => false,
    }
}

/// The keys of the places that wait on the role at `address` (crew's
/// `asks.waits_on`): `place/<n>@<since>`, or `place/<n>`, for a place
/// whose launch names the role as its recipient and waits on its caller;
/// `landed/<n>` for a role's work that has landed.
fn waits_on(address: &str, requests: &[Request], places: &[Placed]) -> Vec<String> {
    let launches: BTreeMap<u64, &Request> = requests
        .iter()
        .filter(|r| r.is_launch() && r.get("Recipient") == Some(address))
        .map(|r| (r.number, r))
        .collect();
    let mut out: Vec<(u64, String)> = Vec::new();
    for placed in places {
        let Some(request) = launches.get(&placed.request) else {
            continue;
        };
        let Some(status) = status_of(placed.status) else {
            continue;
        };
        let n = placed.request;
        if let Some(since) = waiting(&status) {
            out.push((
                n,
                since.map_or(format!("place/{n}"), |since| format!("place/{n}@{since}")),
            ));
        } else if request.work().unwrap_or_default().starts_with("role/") && landed(&status) {
            out.push((n, format!("landed/{n}")));
        }
    }
    out.sort_by_key(|(n, _)| *n);
    out.into_iter().map(|(_, key)| key).collect()
}

/// Whether anything is pending for `role` (crew's `asks.pending` and
/// `asks.wanted`): an ask of it, or a wait on it, that no ended turn took.
/// A superseded or open turn answers nothing.
fn wanted(main: &Main, role: &str, waits: &[String]) -> bool {
    let answered: BTreeSet<&str> = main
        .turns
        .iter()
        .filter(|turn| {
            turn.get("Turn").and_then(whole).is_some()
                && turn.get("Run").is_some()
                && turn.get("Role") == Some(role)
                && turn.get("Ended").is_some_and(|ended| !ended.is_empty())
        })
        .flat_map(|turn| turn.get_all("Took"))
        .collect();
    let asks = main.asks.iter().any(|ask| {
        ask.get("Role") == Some(role)
            && ask
                .get("Id")
                .and_then(whole)
                .is_some_and(|n| !answered.contains(format!("ask/{n}").as_str()))
    });
    asks || waits.iter().any(|key| !answered.contains(key.as_str()))
}

/// What Flume's status says of the place at `place`: its phase, as
/// serde's tag names it, and why it waits on its caller (crew's
/// `runner.flume_facts`, as far as a run's standing reads it). The
/// place's entry is the one at its path or, tidied, at `<path>@<when>`.
fn facts(status: &str, place: &str) -> Option<(Option<String>, Option<String>)> {
    let status: Value = serde_json::from_str(status).ok()?;
    let places = status.get("status")?.get("places")?.as_array()?;
    let entry = places.iter().rfind(|entry| {
        entry
            .get("place")
            .and_then(Value::as_str)
            .is_some_and(|at| {
                at == place
                    || at
                        .strip_prefix(place)
                        .is_some_and(|rest| rest.starts_with('@'))
            })
    })?;
    let phase = match entry.get("phase") {
        Some(Value::String(phase)) => Some(phase.clone()),
        Some(Value::Object(tagged)) if tagged.len() == 1 => tagged.keys().next().cloned(),
        _ => None,
    };
    let why = entry
        .get("waiting_on")
        .and_then(|wait| wait.get("why"))
        .and_then(Value::as_str)
        .map(str::to_string);
    Some((phase, why))
}

/// Whether a place reads live or making in Flume's status (crew's
/// `role_run`).
fn live(status: &str, place: &str) -> bool {
    matches!(facts(status, place), Some((Some(phase), _)) if phase == "live" || phase == "making")
}

/// A role's run as the runner's report holds it.
pub struct Run<'a> {
    pub k: u64,
    pub launched: &'a Launched,
    /// Its done, where the report holds one.
    pub done: Option<&'a Acted>,
}

impl Run<'_> {
    /// Whether the run stands: launched, not yet done, and its place, as
    /// last carried, listed and live or making. A run launched whose
    /// status is not yet carried stands.
    pub fn stands(&self) -> bool {
        let answer = &self.launched.answer;
        if answer.outcome != Outcome::Launched
            || self.done.is_some_and(|done| done.outcome == Outcome::Done)
        {
            return false;
        }
        match (&self.launched.status, &answer.place_dir) {
            (None, _) => true,
            (Some(status), Some(place)) => status.listed && live(&status.text, place),
            (Some(_), None) => false,
        }
    }

    /// Whether the run's turn has delivered: it stands, and its place
    /// waits on its caller with `why` `delivered`.
    fn delivered(&self) -> bool {
        let (Some(status), Some(place)) = (&self.launched.status, &self.launched.answer.place_dir)
        else {
            return false;
        };
        self.stands()
            && matches!(facts(&status.text, place), Some((Some(phase), Some(why)))
                if phase == "live" && why == "delivered")
    }
}

/// Each role's runs the report holds of partition `label`, by role, in
/// order of `k`.
pub fn runs<'a>(label: &str, report: &'a Report) -> BTreeMap<String, Vec<Run<'a>>> {
    let mut out: BTreeMap<String, Vec<Run<'a>>> = BTreeMap::new();
    for (id, launched) in &report.launches {
        if let Some(ids::Named::Run(role, k)) = ids::named(label, id) {
            let done = report.acts.get(&ids::done(label, &role, k));
            out.entry(role).or_default().push(Run { k, launched, done });
        }
    }
    for runs in out.values_mut() {
        runs.sort_by_key(|run| run.k);
    }
    out
}

/// The role records whose `Host` is `host`, by role.
pub fn of_host<'a>(main: &'a Main, host: &str) -> BTreeMap<&'a str, &'a Record> {
    main.roles
        .iter()
        .filter(|role| role.get("Host") == Some(host))
        .filter_map(|role| Some((role.get("Role")?, role)))
        .collect()
}

/// A role's run's launch, from its role record (R1): its repository, its
/// kind's identity `<Kinds>/<Kind>`, its name, stream, priority and
/// intent, and the agent setting `model=<Model>`; no base, brief or
/// recipient. Or the field it lacks or can't be read.
fn launch(role: &Record) -> Result<Launch, String> {
    let named = |field: &str| -> Result<&str, String> {
        role.get(field)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("names no {field}"))
    };
    let priority = match role.get("Priority") {
        None => None,
        Some(text) => Some(
            text.parse::<i64>()
                .map_err(|_| format!("its Priority {text} is no whole number"))?,
        ),
    };
    Ok(Launch {
        repository: named("Repository")?.to_string(),
        kind: format!("{}/{}", named("Kinds")?, named("Kind")?),
        name: named("Name")?.to_string(),
        base: None,
        brief: None,
        stream: role.get("Stream").map(str::to_string),
        priority,
        intent: role.get("Intent").map(str::to_string),
        recipients: Vec::new(),
        holds: Vec::new(),
        hold_submission: false,
        after: Vec::new(),
        agent_settings: role
            .get("Model")
            .map(|model| AgentSetting {
                field: "model".to_string(),
                value: model.to_string(),
            })
            .into_iter()
            .collect(),
    })
}

/// The wants of partition `label`'s roles on `host`: each run the report
/// holds, offered again so its place's status is carried; a done of the
/// latest where its turn has delivered; and the next run where the role
/// is wanted and none stands. `places` are every host's places, this
/// host's from the report; `branch_runs` this host's `Run` records, read
/// where the report holds no run of a role. Gives the lines to say of a
/// role record it can't read.
pub fn wants(
    label: &str,
    host: &str,
    main: &Main,
    report: &Report,
    places: &[Placed],
    branch_runs: &BTreeMap<String, Record>,
) -> (Vec<Want>, Vec<String>) {
    let mut wants = Vec::new();
    let mut lines = Vec::new();
    let held = runs(label, report);
    for (role, record) in of_host(main, host) {
        let Some(address) = record.get("Address") else {
            lines.push(format!(
                "{label}'s role {role} names no Address, and is left"
            ));
            continue;
        };
        let launch = match launch(record) {
            Ok(launch) => launch,
            Err(why) => {
                lines.push(format!("{label}'s role {role} {why}, and is left"));
                continue;
            }
        };
        let runs = held.get(role).map(Vec::as_slice).unwrap_or_default();
        for run in runs {
            wants.push(Want::Offer(Offer {
                id: ids::run(label, role, run.k),
                lease: LEASE,
                launch: launch.clone(),
            }));
        }
        let latest = runs.last();
        let stands = match latest {
            Some(run) => run.stands(),
            None => branch_runs.get(address).is_some_and(|run| {
                matches!((run.get("Status"), run.get("Place")),
                    (Some(status), Some(place)) if live(status, place))
            }),
        };
        if let Some(run) = latest.filter(|run| run.delivered()) {
            wants.push(Want::Act(Act {
                id: ids::done(label, role, run.k),
                launch: ids::run(label, role, run.k),
                lease: LEASE.number,
                what: ActSort::Done,
            }));
        }
        let waits = waits_on(address, &main.requests, places);
        if !stands && wanted(main, role, &waits) {
            let k = latest.map_or(0, |run| run.k);
            wants.push(Want::Offer(Offer {
                id: ids::run(label, role, k + 1),
                lease: LEASE,
                launch,
            }));
        }
    }
    (wants, lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(place: &str) -> String {
        serde_json::json!({"status": {"places": [serde_json::from_str::<Value>(place).unwrap()], "host": {}}})
            .to_string()
    }

    #[test]
    fn a_place_waits_on_its_caller_as_crew_reads_it() {
        let wait = |why: &str, stage: Option<&str>, stalled: bool| {
            let place = serde_json::json!({"place": "/p", "phase": "live", "stalled": stalled,
                "waiting_on": {"act": "retry", "stage": stage, "why": why, "since": 1793440000}});
            waiting(&status_of(&status(&place.to_string())).unwrap())
        };
        for why in WAITS {
            assert_eq!(
                wait(why, Some("code"), false),
                Some(Some(1793440000)),
                "{why}"
            );
        }
        assert!(wait("on_hold", None, false).is_some());
        assert!(wait("on_hold", Some("code"), false).is_none());
        assert!(wait("delivered", None, false).is_none());
        assert!(wait("never_begun", Some("code"), true).is_some());
    }

    #[test]
    fn a_status_that_holds_no_one_place_is_none() {
        assert!(status_of("not json").is_none());
        assert!(status_of(r#"{"status":{"places":[]}}"#).is_none());
        assert!(status_of(r#"{"place":"/p","waiting_on":{"why":"answer"}}"#).is_some());
    }

    #[test]
    fn a_place_tidied_is_found_by_its_path_and_reads_its_phase() {
        let tidied = status(r#"{"place":"/p@1793474400","phase":{"tidied":{"over":"done"}}}"#);
        assert_eq!(
            facts(&tidied, "/p"),
            Some((Some("tidied".to_string()), None))
        );
        assert!(!live(&tidied, "/p"));
        assert!(live(&status(r#"{"place":"/p","phase":"making"}"#), "/p"));
        assert!(!live(&status(r#"{"place":"/q","phase":"live"}"#), "/p"));
    }
}
