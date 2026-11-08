//! crew's records, read at one tip of a partition's drive branch, and
//! each runner's branch beside it (design Decision 17): `requests.rec`,
//! `roles.rec`, `asks.rec` and `plan.rec` at `<label>/main`, through a
//! bare cache by weir-branch's read; every `<label>/runner/*` the state
//! repository lists; and each request for this host given as a want under
//! the lease number 1.

use crate::ids;
use crate::records::{self, whole, Sets};
use boomdrive_control::{
    Act, ActSort, Answer, At, Carried, Id, Launch, Launched, Lease, Offer, Outcome, Preview, Want,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use weir_branch::{Branch, Tip};
use weir_capture::rec::Record;

/// Every want crew's implementation gives is under this lease: crew never
/// moves work once a host's runner holds it (design Decision 17).
pub const LEASE: Lease = Lease {
    number: 1,
    expires: None,
};

/// One state repository, reached at its address through its bare cache.
pub struct Store {
    pub cache: PathBuf,
    pub url: String,
}

/// A partition's drive branch at one tip.
#[derive(Clone, Debug)]
pub struct Main {
    pub tip: String,
    /// Each request with a whole `Request` number, in `Request` order.
    pub requests: Vec<Request>,
    /// How many records of `requests.rec` hold no whole `Request` number.
    pub unnumbered: usize,
    pub roles: Vec<Record>,
    pub asks: Vec<Record>,
    pub turns: Vec<Record>,
    /// The plan's `Unit` and `Bolt` records, by name.
    pub units: BTreeSet<String>,
    pub bolts: BTreeSet<String>,
    /// The launches a drop names: each request's whole `Of` whose `Act`
    /// is `drop`, whatever its `Host`, its `Work` or its place in the file,
    /// as crew's `standing` reads them.
    pub dropped: BTreeSet<u64>,
}

/// A request of `requests.rec`, with its number.
#[derive(Clone, Debug)]
pub struct Request {
    pub number: u64,
    pub record: Record,
}

impl Request {
    pub fn get(&self, field: &str) -> Option<&str> {
        self.record.get(field)
    }

    pub fn act(&self) -> Option<&str> {
        self.get("Act")
    }

    pub fn host(&self) -> Option<&str> {
        self.get("Host")
    }

    pub fn work(&self) -> Option<&str> {
        self.get("Work")
    }

    pub fn is_launch(&self) -> bool {
        self.act() == Some("launch")
    }

    /// The launch a retry, drop or priority names, where `Of` is whole.
    pub fn of(&self) -> Option<u64> {
        self.get("Of").and_then(whole)
    }
}

impl Main {
    pub fn request(&self, number: u64) -> Option<&Request> {
        self.requests.iter().find(|r| r.number == number)
    }

    /// Whether a drop names launch `number`: such a launch no longer
    /// stands, as crew reads it.
    pub fn dropped(&self, number: u64) -> bool {
        self.dropped.contains(&number)
    }

    /// Whether the work a request serves is still in crew's plan:
    /// `unit/<u>` and `chore/<c>` while the plan holds `Unit: <u>`,
    /// `fix/<bolt>/<name>` while it holds `Bolt: <bolt>`. A fix of main, a
    /// role's work and any other the plan never holds always are.
    pub fn plans(&self, work: Option<&str>) -> bool {
        let parts: Vec<&str> = work.unwrap_or_default().split('/').collect();
        match parts.as_slice() {
            ["unit" | "chore", unit] => self.units.contains(*unit),
            ["fix", bolt, _] if *bolt != "main" => self.bolts.contains(*bolt),
            _ => true,
        }
    }
}

/// A runner's branch as crew's `host_read` reads it: the host's one
/// `Runner`, each answer and place by its request's number, each role's
/// run by its address, and each preview by its request's number.
#[derive(Clone, Debug, Default)]
pub struct Runner {
    pub since: Option<String>,
    pub answers: BTreeMap<u64, Record>,
    pub places: BTreeMap<u64, Record>,
    pub runs: BTreeMap<String, Record>,
    pub previews: BTreeMap<u64, Record>,
}

/// A runner's files at a tip, by name: `runner.rec`, `answers.rec` and
/// `places.rec`, each none where the tip lacks it.
pub type RunnerFiles = BTreeMap<&'static str, Option<String>>;

/// The runner's files a branch holds.
pub const FILES: [&str; 3] = ["runner.rec", "answers.rec", "places.rec"];

impl Store {
    /// The branch `name` of the state repository.
    pub fn branch(&self, name: &str) -> Branch {
        Branch::remote(name, &self.cache, &self.url)
    }

    /// `<label>/main` at its tip, or why it could not be read.
    pub fn main(&self, label: &str) -> Result<Main, String> {
        let name = format!("{label}/main");
        let tip = weir_branch::read(&self.branch(&name))
            .map_err(|failure| one_line(&failure.to_string()))?
            .ok_or_else(|| format!("the state repository holds no {name}"))?;
        let file = |path: &str| -> Result<Sets, String> {
            let text = tip
                .file(path)
                .map_err(|error| format!("{name} at {}: {error}", short(tip.commit())))?;
            records::read(text.as_deref().unwrap_or_default()).map_err(|error| {
                format!(
                    "{name} at {}: {path} can't be read: {error}",
                    short(tip.commit())
                )
            })
        };
        let requests = file("requests.rec")?;
        let mut numbered = Vec::new();
        let mut unnumbered = 0;
        for record in records::of(&requests, "Request") {
            match record.get("Request").and_then(whole) {
                Some(number) => numbered.push(Request {
                    number,
                    record: record.clone(),
                }),
                None => unnumbered += 1,
            }
        }
        numbered.sort_by_key(|r| r.number);
        let dropped = records::of(&requests, "Request")
            .filter(|record| record.get("Act") == Some("drop"))
            .filter_map(|record| record.get("Of").and_then(whole))
            .collect();
        let roles = file("roles.rec")?;
        let asks = file("asks.rec")?;
        let plan = file("plan.rec")?;
        let names = |set: &str| -> BTreeSet<String> {
            records::of(&plan, set)
                .filter_map(|record| record.get(set).map(str::to_string))
                .collect()
        };
        Ok(Main {
            tip: tip.commit().to_string(),
            requests: numbered,
            unnumbered,
            roles: records::of(&roles, "Role").cloned().collect(),
            asks: records::of(&asks, "Ask").cloned().collect(),
            turns: records::of(&asks, "Turn").cloned().collect(),
            units: names("Unit"),
            bolts: names("Bolt"),
            dropped,
        })
    }

    /// The hosts whose runner's branch, `<label>/runner/<host>`, the state
    /// repository lists, in name order, by `git ls-remote` through the
    /// cache.
    pub fn hosts(&self, label: &str) -> Result<Vec<String>, String> {
        let prefix = format!("refs/heads/{label}/runner/");
        let mut command = Command::new("git");
        command.arg("--git-dir").arg(&self.cache);
        for name in REDIRECTS {
            command.env_remove(name);
        }
        let output = command
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("LC_ALL", "C")
            .args(["ls-remote", "--", &self.url, &format!("{prefix}*")])
            .stdin(Stdio::null())
            .output()
            .map_err(|error| format!("git could not be run: {error}"))?;
        if !output.status.success() {
            let said = String::from_utf8_lossy(&output.stderr);
            let last = said.lines().rfind(|line| !line.trim().is_empty());
            return Err(format!(
                "git ls-remote failed: {}",
                last.unwrap_or("it said nothing")
            ));
        }
        let mut hosts: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| line.split('\t').nth(1)?.strip_prefix(&prefix))
            .filter(|host| crate::config::is_name(host))
            .map(str::to_string)
            .collect();
        hosts.sort();
        hosts.dedup();
        Ok(hosts)
    }

    /// `<label>/runner/<host>` at its tip and its files, none where it
    /// does not exist.
    pub fn runner_files(
        &self,
        label: &str,
        host: &str,
    ) -> Result<Option<(Tip, RunnerFiles)>, String> {
        let name = runner_branch(label, host);
        let Some(tip) = weir_branch::read(&self.branch(&name))
            .map_err(|failure| one_line(&failure.to_string()))?
        else {
            return Ok(None);
        };
        let mut files = RunnerFiles::new();
        for path in FILES {
            let text = tip
                .file(path)
                .map_err(|error| format!("{name} at {}: {error}", short(tip.commit())))?;
            files.insert(path, text);
        }
        Ok(Some((tip, files)))
    }

    /// `<label>/runner/<host>` as crew reads it: none where it does not
    /// exist, else its reading or why it can't be read.
    pub fn runner(&self, label: &str, host: &str) -> Result<Option<Runner>, String> {
        let Some((tip, files)) = self.runner_files(label, host)? else {
            return Ok(None);
        };
        let at = format!("{} at {}", runner_branch(label, host), short(tip.commit()));
        host_read(host, &files)
            .map(Some)
            .map_err(|why| format!("{host}'s runner can't be read: {at}: {why}"))
    }
}

/// The variables that would send git to another repository than the one
/// named, removed as weir-branch removes them.
const REDIRECTS: [&str; 7] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
    "GIT_QUARANTINE_PATH",
];

/// The branch a host's runner answers a partition's requests on.
pub fn runner_branch(label: &str, host: &str) -> String {
    format!("{label}/runner/{host}")
}

/// A commit's first seven characters, as crew names a tip.
pub fn short(commit: &str) -> &str {
    &commit[..commit.len().min(7)]
}

/// What weir said of a read or a write, git's words among it, on one
/// line: each of its lines that holds anything, trimmed, joined by `; `.
/// So every line crew's implementation says, and every refusal it gives
/// the runner to say, is one line.
pub fn one_line(said: &str) -> String {
    said.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("; ")
}

/// A runner's files read as crew's `runner.host_read` reads them: one
/// `Runner` naming `host`, no record before its `%rec`, no request
/// answered or placed twice and no run twice; or why not.
pub fn host_read(host: &str, files: &RunnerFiles) -> Result<Runner, String> {
    let mut sets = BTreeMap::new();
    for (path, set) in [
        ("runner.rec", "Runner"),
        ("answers.rec", "Answer"),
        ("places.rec", "Place"),
    ] {
        let text = files.get(path).cloned().flatten();
        let read = match &text {
            Some(text) => Some(records::read(text).map_err(|error| format!("{path}: {error}"))?),
            None => None,
        };
        if read
            .as_ref()
            .is_some_and(|read| read.iter().any(|(s, _)| s.is_none()))
        {
            return Err(format!("{path} holds a record before its %rec: {set}"));
        }
        sets.insert(path, read);
    }
    let runners: Vec<&Record> = match &sets["runner.rec"] {
        Some(read) => records::of(read, "Runner").collect(),
        None => return Err("it holds no runner.rec".to_string()),
    };
    let runner = match runners.as_slice() {
        [one] if one.get("Host") == Some(host) => *one,
        [one] => {
            return Err(format!(
                "runner.rec names host {}",
                one.get("Host").unwrap_or("none")
            ))
        }
        many => {
            return Err(format!(
                "runner.rec holds {} Runner records, and names one",
                many.len()
            ))
        }
    };
    let mut out = Runner {
        since: runner.get("Since").map(str::to_string),
        ..Runner::default()
    };
    for (path, set) in [("answers.rec", "Answer"), ("places.rec", "Place")] {
        let Some(read) = &sets[path] else { continue };
        for record in records::of(read, set) {
            let number = record
                .get("Request")
                .and_then(whole)
                .ok_or_else(|| format!("{path} holds a record with no request's number"))?;
            let into = match set {
                "Answer" => &mut out.answers,
                _ => &mut out.places,
            };
            if into.insert(number, record.clone()).is_some() {
                return Err(format!("{path} holds request {number} twice"));
            }
        }
    }
    if let Some(read) = &sets["places.rec"] {
        for record in records::of(read, "Run") {
            let address = record
                .get("Address")
                .filter(|address| !address.is_empty())
                .ok_or_else(|| "places.rec holds a run with no role's address".to_string())?;
            if out
                .runs
                .insert(address.to_string(), record.clone())
                .is_some()
            {
                return Err(format!("places.rec holds the run of {address} twice"));
            }
        }
    }
    Ok(out)
}

/// What of a request crew's implementation cannot read.
pub type Unreadable = String;

/// Request `request` of partition `label` as a want; or what of it can't
/// be read. A launch's or a preview's repository is `repository` where one
/// is given, the address its `Repo` names among those the runner keeps,
/// else its `Repo` as written. A `preview` is a preview of its `Branch` at
/// its `Revision`, done where its `Done` says so (design Decision 10;
/// core.55).
pub fn want(label: &str, request: &Request, repository: Option<&str>) -> Result<Want, Unreadable> {
    let id = ids::request(label, request.number);
    let of = || -> Result<Id, Unreadable> {
        let text = request.get("Of").ok_or("it names no Of")?;
        whole(text)
            .map(|n| ids::request(label, n))
            .ok_or_else(|| format!("its Of {text}, which is no whole number"))
    };
    let act = |what: ActSort| -> Result<Want, Unreadable> {
        Ok(Want::Act(Act {
            id: id.clone(),
            launch: of()?,
            lease: LEASE.number,
            what,
        }))
    };
    match request.act() {
        Some("launch") => Ok(Want::Offer(Offer {
            id: id.clone(),
            lease: LEASE,
            launch: launch(label, request, repository)?,
        })),
        Some("retry") => act(ActSort::Retry {
            stage: request.get("Stage").map(str::to_string),
            words: request.get("Words").map(str::to_string),
            submission: yes_or_no(request, "Submission")?,
        }),
        Some("preview") => {
            let named = |field: &str| -> Result<String, Unreadable> {
                request
                    .get(field)
                    .map(str::to_string)
                    .ok_or_else(|| format!("it names no {field}"))
            };
            let repository = match repository {
                Some(repository) => repository.to_string(),
                None => named("Repo")?,
            };
            Ok(Want::Preview(Preview {
                id: id.clone(),
                lease: LEASE,
                repository,
                branch: named("Branch")?,
                revision: named("Revision")?,
                done: yes_or_no(request, "Done")?,
            }))
        }
        Some("drop") => act(ActSort::Drop),
        Some("priority") => act(ActSort::Priority(
            number(request, "Priority")?.ok_or("it names no Priority".to_string())?,
        )),
        Some(other) => Err(format!(
            "its Act {other}, which crew's implementation does not read"
        )),
        None => Err("it names no Act".to_string()),
    }
}

/// A request's want the runner refuses, its words naming what of it can't
/// be read: an act on a launch that is no launch of the runner's, whose
/// id says why (design Decision 17).
pub fn refused(label: &str, request: &Request, why: &str) -> Want {
    Want::Act(Act {
        id: ids::request(label, request.number),
        launch: Id::new(format!("request {} ({why})", request.number)),
        lease: LEASE.number,
        what: ActSort::Drop,
    })
}

/// A request's field read as crew writes a bool: `yes`, `true` or `1`, or
/// `no`, `false` or `0`, and no where it is not there.
fn yes_or_no(request: &Request, field: &str) -> Result<bool, Unreadable> {
    match request.get(field) {
        None => Ok(false),
        Some("yes" | "true" | "1") => Ok(true),
        Some("no" | "false" | "0") => Ok(false),
        Some(other) => Err(format!("its {field} {other}, which is no yes or no")),
    }
}

/// A request's field as a number, none where it is not there.
fn number(request: &Request, field: &str) -> Result<Option<i64>, Unreadable> {
    match request.get(field) {
        None => Ok(None),
        Some(text) => text
            .parse::<i64>()
            .map(Some)
            .map_err(|_| format!("its {field} {text}, which is no whole number")),
    }
}

/// A launch request's launch: its repository, `Kind`, `Name`, `Base`,
/// `Stream` where named, `Priority`, `Intent`, each `Recipient`, `Brief`,
/// each `After` as `<label>/<n>`, and `Hold: submission`.
fn launch(label: &str, request: &Request, repository: Option<&str>) -> Result<Launch, Unreadable> {
    let named = |field: &str| -> Result<String, Unreadable> {
        request
            .get(field)
            .map(str::to_string)
            .ok_or_else(|| format!("it names no {field}"))
    };
    let mut after = Vec::new();
    for text in request.record.get_all("After") {
        let n = whole(text).ok_or_else(|| format!("its After {text}, which is no whole number"))?;
        after.push(ids::request(label, n));
    }
    let mut hold_submission = false;
    for hold in request.record.get_all("Hold") {
        match hold {
            "submission" => hold_submission = true,
            other => {
                return Err(format!(
                    "its Hold {other}, which crew's implementation does not read"
                ))
            }
        }
    }
    let repository = match repository {
        Some(repository) => repository.to_string(),
        None => named("Repo")?,
    };
    Ok(Launch {
        repository,
        kind: named("Kind")?,
        name: named("Name")?,
        base: request.get("Base").map(str::to_string),
        brief: request.get("Brief").map(str::to_string),
        stream: request.get("Stream").map(str::to_string),
        priority: number(request, "Priority")?,
        intent: request.get("Intent").map(str::to_string),
        recipients: request
            .record
            .get_all("Recipient")
            .into_iter()
            .map(str::to_string)
            .collect(),
        holds: Vec::new(),
        hold_submission,
        after,
        agent_settings: Vec::new(),
    })
}

/// A repository a launch's `Repo` names, against the addresses the runner
/// keeps (design Decision 18).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kept {
    /// The address it is, as the runner names it.
    Address(String),
    /// A name alone that two or more kept addresses have.
    Shared(Vec<String>),
    /// A repository the runner does not keep.
    No,
}

/// What `repo` names among `kept`: an address, compared without case as
/// GitHub compares it; or a name alone that exactly one kept address has.
pub fn kept(repo: &str, kept: &[String]) -> Kept {
    if repo.contains('/') {
        return kept
            .iter()
            .find(|address| address.eq_ignore_ascii_case(repo))
            .map_or(Kept::No, |address| Kept::Address(address.clone()));
    }
    let named: Vec<String> = kept
        .iter()
        .filter(|address| {
            address
                .rsplit_once('/')
                .is_some_and(|(_, name)| name.eq_ignore_ascii_case(repo))
        })
        .cloned()
        .collect();
    match named.as_slice() {
        [] => Kept::No,
        [one] => Kept::Address(one.clone()),
        _ => Kept::Shared(named),
    }
}

/// An answer record of a runner's branch as the runner's answer, none
/// where its outcome or time can't be read.
pub fn answer(record: &Record) -> Option<Answer> {
    let outcome = match record.get("Outcome")? {
        "held" => Outcome::Held,
        "launched" => Outcome::Launched,
        "done" => Outcome::Done,
        "refused" => Outcome::Refused,
        _ => return None,
    };
    Some(Answer {
        outcome,
        place_dir: record.get("Place").map(str::to_string),
        words: record.get("Words").map(str::to_string),
        at: At::parse(record.get("At")?).ok()?,
    })
}

/// A launch's report as the runner's branch holds it: its answer, and its
/// place's status where it has one.
pub fn launched(answer: Answer, place: Option<&Record>) -> Launched {
    let status = place.and_then(|place| {
        Some(Carried {
            text: place.get("Status")?.to_string(),
            at: At::parse(place.get("At")?).ok()?,
            listed: true,
        })
    });
    Launched {
        lease: LEASE,
        repository: None,
        answer,
        status,
        entry: None,
    }
}
