//! The runner's branch, `<label>/runner/<host>`, written in crew's formats
//! by weir-branch's write (design Decision 17): `runner.rec`,
//! `answers.rec` and `places.rec`, each opening with crew's own
//! descriptor as `runner.py` holds it at `9913f249`, byte for byte, and,
//! where a preview is or was answered, `previews.rec`, under the
//! descriptor this repository proposes for crew to hold (design Decision
//! 10; core.55); each formed from the runner's report and the claims
//! standing alone. Nothing is written when the files' text equals the
//! tip's, and no ref but the runner's branch is ever pushed, never with
//! force.

use crate::ids::{self, Named};
use crate::read::{self, runner_branch, Main, Store};
use crate::records::{self, under, whole};
use crate::roles;
use boomdrive_control::{Answer, Id, Outcome, Previewed, Report};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use time::OffsetDateTime;
use weir_branch::{Ask, Files, Tip, Written};
use weir_capture::rec::Record;

/// crew's descriptor of `runner.rec`.
pub const RUNNER: &str = "\
%rec: Runner
%key: Host
%type: Since date
%mandatory: Host Version Since
%allowed: Host Version Since
";

/// crew's descriptor of `answers.rec`.
pub const ANSWERS: &str = "\
%rec: Answer
%key: Request
%type: Request int
%type: Outcome enum held launched done refused
%type: At date
%mandatory: Request Outcome At
%allowed: Request Outcome Place Words At
";

/// crew's descriptor of the places of `places.rec`.
pub const PLACES: &str = "\
%rec: Place
%key: Request
%type: Request int
%type: At date
%mandatory: Request Place At Status
%allowed: Request Place At Status
";

/// crew's descriptor of the role runs of `places.rec`.
pub const RUNS: &str = "\
%rec: Run
%key: Address
%type: At date
%mandatory: Address At
%allowed: Address Place At Status Words
";

/// crew's descriptor of `previews.rec`, as this repository proposes it
/// for crew's unit that moves its previews onto the runner to hold in
/// `runner.py` byte for byte (design Decision 10).
pub const PREVIEWS: &str = "\
%rec: Preview
%key: Request
%type: Request int
%type: Outcome enum held done refused
%type: Listed enum yes no
%type: At date
%mandatory: Request Outcome At
%allowed: Request Outcome Text Status Listed Words At
";

/// The age past which another writer's lock on the cache is stale.
const STALE_AFTER: Duration = Duration::from_secs(5 * 60);

/// The runner's files' text: its three, and `previews.rec` where the
/// branch is to hold one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Texts {
    pub runner: String,
    pub answers: String,
    pub places: String,
    pub previews: Option<String>,
}

impl Texts {
    fn by_path(&self) -> Vec<(&'static str, &String)> {
        let mut paths = vec![
            ("runner.rec", &self.runner),
            ("answers.rec", &self.answers),
            ("places.rec", &self.places),
        ];
        if let Some(previews) = &self.previews {
            paths.push(("previews.rec", previews));
        }
        paths
    }
}

/// The words a claim's answer holds (design Decision 18).
pub fn claim_words(host: &str) -> String {
    format!("claimed by {host}'s runner; launching once no other runner has claimed it")
}

/// Whether `answer` is a claim of `host`'s runner.
pub fn is_claim(answer: &Record, host: &str) -> bool {
    answer.get("Outcome") == Some("held") && answer.get("Words") == Some(claim_words(host).as_str())
}

/// A claim of request `number`, made at `at`.
pub fn claim(number: u64, host: &str, at: &str) -> Record {
    let mut record = Record::new();
    record.push("Request", number.to_string());
    record.push("Outcome", "held");
    record.push("Words", claim_words(host));
    record.push("At", at);
    record
}

fn outcome(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Held => "held",
        Outcome::Launched => "launched",
        Outcome::Done => "done",
        Outcome::Refused => "refused",
    }
}

fn answer_record(number: u64, answer: &Answer) -> Record {
    let mut record = Record::new();
    record.push("Request", number.to_string());
    record.push("Outcome", outcome(answer.outcome));
    record.push_optional("Place", answer.place_dir.as_deref());
    record.push_optional("Words", answer.words.as_deref());
    record.push("At", answer.at.to_string());
    record
}

/// `runner.rec`: one `Runner` naming the host, the runner's version and
/// when it first wrote.
pub fn runner_text(host: &str, version: &str, since: &str) -> String {
    let mut record = Record::new();
    record.push("Host", host);
    record.push("Version", version);
    record.push("Since", since);
    under(RUNNER, &[record])
}

/// `answers.rec` of `answers`, by request.
pub fn answers_text(answers: &BTreeMap<u64, Record>) -> String {
    under(ANSWERS, &answers.values().cloned().collect::<Vec<_>>())
}

/// `places.rec`: each place by request, then each run by address.
pub fn places_text(places: &BTreeMap<u64, Record>, runs: &BTreeMap<String, Record>) -> String {
    let places = under(PLACES, &places.values().cloned().collect::<Vec<_>>());
    let runs = under(RUNS, &runs.values().cloned().collect::<Vec<_>>());
    format!("{places}\n{runs}")
}

/// `previews.rec` of `previews`, by request.
pub fn previews_text(previews: &BTreeMap<u64, Record>) -> String {
    under(PREVIEWS, &previews.values().cloned().collect::<Vec<_>>())
}

/// A preview's record: its request, its outcome, sluice's answer as
/// carried as its `Text`, sluice's listing as carried as its `Status`
/// with `Listed`, its `Words`, and its `At`, the status's time where a
/// status is carried, else the answer's (design Decision 10).
fn preview_record(number: u64, previewed: &Previewed) -> Record {
    let mut record = Record::new();
    record.push("Request", number.to_string());
    record.push("Outcome", outcome(previewed.outcome));
    record.push_optional("Text", previewed.text.as_deref());
    if let Some(status) = &previewed.status {
        record.push("Status", status.text.as_str());
        record.push("Listed", if status.listed { "yes" } else { "no" });
    }
    record.push_optional("Words", previewed.words.as_deref());
    let at = previewed
        .status
        .as_ref()
        .map_or(previewed.at, |status| status.at);
    record.push("At", at.to_string());
    record
}

/// The files of partition `label` on `host`'s branch, formed from
/// `report` and the claims standing: one `Answer` per request the report
/// answers and per claim whose launch it does not, one `Place` per
/// request launched with a status, and one `Run` per role for its latest
/// run, by its address in `addresses`; and `previews.rec`, one `Preview`
/// per preview request the report answers, where the report holds one,
/// or the descriptor alone where it holds none and the tip, as
/// `previews_held` says, holds the file.
#[allow(clippy::too_many_arguments)]
pub fn texts(
    label: &str,
    host: &str,
    version: &str,
    since: &str,
    report: &Report,
    claims: &[Record],
    addresses: &BTreeMap<String, String>,
    previews_held: bool,
) -> Texts {
    let mut answers: BTreeMap<u64, Record> = claims
        .iter()
        .filter_map(|claim| Some((claim.get("Request").and_then(whole)?, claim.clone())))
        .collect();
    let mut places = BTreeMap::new();
    for (id, launched) in &report.launches {
        if let Some(Named::Request(n)) = ids::named(label, id) {
            answers.insert(n, answer_record(n, &launched.answer));
            if let (Some(place), Some(status)) = (&launched.answer.place_dir, &launched.status) {
                let mut record = Record::new();
                record.push("Request", n.to_string());
                record.push("Place", place.as_str());
                record.push("At", status.at.to_string());
                record.push("Status", status.text.as_str());
                places.insert(n, record);
            }
        }
    }
    for (id, acted) in &report.acts {
        if let Some(Named::Request(n)) = ids::named(label, id) {
            let mut record = Record::new();
            record.push("Request", n.to_string());
            record.push("Outcome", outcome(acted.outcome));
            record.push_optional("Words", acted.words.as_deref());
            record.push("At", acted.at.to_string());
            answers.insert(n, record);
        }
    }
    let mut previews = BTreeMap::new();
    for (id, previewed) in &report.previews {
        if let Some(Named::Request(n)) = ids::named(label, id) {
            previews.insert(n, preview_record(n, previewed));
        }
    }
    let mut runs = BTreeMap::new();
    for (role, held) in roles::runs(label, report) {
        let Some(run) = held.last() else { continue };
        let address = addresses
            .get(&role)
            .cloned()
            .unwrap_or_else(|| format!("{label}/{role}"));
        let answer = &run.launched.answer;
        let status = &run.launched.status;
        let refused = run
            .done
            .filter(|done| done.outcome == Outcome::Refused)
            .and_then(|done| done.words.as_deref());
        let mut record = Record::new();
        record.push("Address", address.as_str());
        record.push_optional("Place", answer.place_dir.as_deref());
        record.push(
            "At",
            status
                .as_ref()
                .map_or(answer.at, |status| status.at)
                .to_string(),
        );
        record.push_optional("Status", status.as_ref().map(|status| status.text.as_str()));
        record.push_optional("Words", refused.or(answer.words.as_deref()));
        runs.insert(address, record);
    }
    Texts {
        runner: runner_text(host, version, since),
        answers: answers_text(&answers),
        places: places_text(&places, &runs),
        previews: (!previews.is_empty() || previews_held).then(|| previews_text(&previews)),
    }
}

/// The runner's files at `tip`, none for a file it lacks.
pub fn files_at(tip: Option<&Tip>) -> Result<read::RunnerFiles, String> {
    let mut files = read::RunnerFiles::new();
    for path in read::FILES {
        let text = match tip {
            Some(tip) => tip.file(path).map_err(|error| error.to_string())?,
            None => None,
        };
        files.insert(path, text);
    }
    Ok(files)
}

/// What a write changes, as its commit names it: `runner`, then
/// `answers <n>…`, `places <n>…`, `runs <address>…` and `previews <n>…`;
/// none where every file's text is the tip's.
pub fn what(old: &read::RunnerFiles, new: &Texts) -> Option<String> {
    let changed = new
        .by_path()
        .into_iter()
        .any(|(path, text)| old.get(path).cloned().flatten().as_ref() != Some(text));
    if !changed {
        return None;
    }
    let keyed = |set: &str, key: &str, text: Option<&String>| {
        let sets = text
            .and_then(|text| records::read(text).ok())
            .unwrap_or_default();
        records::of(&sets, set)
            .filter_map(|record| Some((record.get(key)?.to_string(), record.clone())))
            .collect::<BTreeMap<String, Record>>()
    };
    let mut parts = Vec::new();
    if old.get("runner.rec").cloned().flatten().as_ref() != Some(&new.runner) {
        parts.push("runner".to_string());
    }
    for (path, set, key, name) in [
        ("answers.rec", "Answer", "Request", "answers"),
        ("places.rec", "Place", "Request", "places"),
        ("places.rec", "Run", "Address", "runs"),
        ("previews.rec", "Preview", "Request", "previews"),
    ] {
        let before = keyed(set, key, old.get(path).and_then(Option::as_ref));
        let after = keyed(
            set,
            key,
            match path {
                "answers.rec" => Some(&new.answers),
                "previews.rec" => new.previews.as_ref(),
                _ => Some(&new.places),
            },
        );
        let mut keys: Vec<&String> = before
            .keys()
            .chain(after.keys())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|key| before.get(*key) != after.get(*key))
            .collect();
        keys.sort_by_key(|key| (whole(key), (*key).clone()));
        if !keys.is_empty() {
            let keys: Vec<&str> = keys.iter().map(|key| key.as_str()).collect();
            parts.push(format!("{name} {}", keys.join(" ")));
        }
    }
    Some(match parts.is_empty() {
        true => "files".to_string(),
        false => parts.join(", "),
    })
}

/// Writes `host`'s branch of partition `label` by weir-branch's write,
/// its files `form` gives from the tip's: one commit, `who`
/// `boomdrive@<host>`, its `what` saying what changed, or `told` where one
/// is given. Gives what was written, none where nothing changed.
pub fn write<F>(
    store: &Store,
    label: &str,
    host: &str,
    told: Option<&str>,
    mut form: F,
) -> Result<Option<String>, String>
where
    F: FnMut(&read::RunnerFiles) -> Result<Texts, String>,
{
    let branch = store.branch(&runner_branch(label, host));
    let tip = weir_branch::read(&branch).map_err(|failure| read::one_line(&failure.to_string()))?;
    let old = files_at(tip.as_ref())?;
    let Some(changed) = what(&old, &form(&old)?) else {
        return Ok(None);
    };
    let what = told.map_or(changed, str::to_string);
    let who = format!("boomdrive@{host}");
    let clock = OffsetDateTime::now_utc;
    let ask = Ask {
        what: &what,
        who: &who,
        bound: None,
        stale_after: STALE_AFTER,
        clock: &clock,
    };
    let written = weir_branch::write(&branch, &ask, |tip| {
        let old = files_at(tip)?;
        let new = form(&old)?;
        Ok(new
            .by_path()
            .into_iter()
            .filter(|(path, text)| old.get(path).cloned().flatten().as_ref() != Some(*text))
            .map(|(path, text)| (path.to_string(), text.clone()))
            .collect::<Files>())
    })
    .map_err(|error| read::one_line(&error.to_string()))?;
    Ok(match written {
        Written::Landed(_) => Some(what),
        Written::Nothing(_) => None,
    })
}

/// The answers on a branch's tip, by request; none where they can't be
/// read.
pub fn answers_at(files: &read::RunnerFiles) -> BTreeMap<u64, Record> {
    let text = files
        .get("answers.rec")
        .cloned()
        .flatten()
        .unwrap_or_default();
    let sets = records::read(&text).unwrap_or_default();
    records::of(&sets, "Answer")
        .filter_map(|record| Some((record.get("Request").and_then(whole)?, record.clone())))
        .collect()
}

/// When the branch's runner first wrote, as its `runner.rec` holds it.
pub fn since_at(files: &read::RunnerFiles) -> Option<String> {
    let text = files.get("runner.rec").cloned().flatten()?;
    let sets = records::read(&text).ok()?;
    let since = records::of(&sets, "Runner").next()?.get("Since")?;
    Some(since.to_string())
}

/// What of partition `label`'s report is let go (design Decision 17):
/// each request launch whose place Flume no longer lists and whose work
/// has left crew's plan at `main`'s tip, and each a drop names that the
/// runner holds unlaunched, whatever the plan holds, with its acts; each
/// role's earlier run once a later one is recorded and its place is over;
/// and a role's last run once its record has gone from `roles.rec` and
/// its place is over, each run with its done.
pub fn let_go(label: &str, host: &str, main: &Main, report: &Report) -> BTreeSet<Id> {
    let mut gone = BTreeSet::new();
    let mut launches = BTreeSet::new();
    for (id, launched) in &report.launches {
        let Some(Named::Request(n)) = ids::named(label, id) else {
            continue;
        };
        let Some(request) = main.request(n) else {
            continue;
        };
        let over = launched.answer.outcome != Outcome::Launched
            || launched
                .status
                .as_ref()
                .is_some_and(|status| !status.listed);
        let dropped = main.dropped(n) && launched.answer.outcome != Outcome::Launched;
        if dropped || (over && !main.plans(request.work())) {
            gone.insert(id.clone());
            launches.insert(n);
        }
    }
    for id in report.acts.keys() {
        let Some(Named::Request(n)) = ids::named(label, id) else {
            continue;
        };
        let Some(request) = main.request(n) else {
            continue;
        };
        let of = request.of();
        let launch_gone = of.is_some_and(|of| {
            launches.contains(&of) || !report.launches.contains_key(&ids::request(label, of))
        });
        if launch_gone && !main.plans(request.work()) {
            gone.insert(id.clone());
        }
    }
    let kept = roles::of_host(main, host);
    for (role, runs) in roles::runs(label, report) {
        let Some(latest) = runs.last().map(|run| run.k) else {
            continue;
        };
        for run in &runs {
            let after_it = run.k < latest || !kept.contains_key(role.as_str());
            if after_it && !run.stands() {
                gone.insert(ids::run(label, &role, run.k));
                gone.insert(ids::done(label, &role, run.k));
            }
        }
    }
    gone.retain(|id| report.launches.contains_key(id) || report.acts.contains_key(id));
    gone
}
