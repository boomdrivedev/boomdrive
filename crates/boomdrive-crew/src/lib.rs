//! `boomdrive-crew`, crew's implementation of the control-plane interface
//! (design Decisions 1 and 17 to 19; core.2), and the one crate that names
//! crew, its branches or its records. It reads crew's records at one tip
//! of each partition's drive branch, `<label>/main`, and every runner's
//! branch beside it, and gives this host's requests and role runs as
//! wants under the lease number 1. It writes the runner's own branch,
//! `<label>/runner/<host>`, in crew's formats, and pushes no other.

pub mod config;
pub mod ids;
pub mod read;
pub mod records;
pub mod roles;
pub mod write;

use boomdrive_control::{
    Acted, Capacity, ControlPlane, Hello, Id, Launched, Outcome, Part, Pulled, Report, Reported,
    Unread, Unsent, Want,
};
use config::{Config, Partition, StateRepository};
use read::{runner_branch, Kept, Main, Request, Runner, Store};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use weir_capture::rec::Record;

/// Where every state repository is reached: GitHub, over https, through
/// the host's own git credential helper (core.4).
pub const GITHUB: &str = "https://github.com/";

/// Where crew's implementation says what it did and could not do, one
/// line each.
pub type Say = Box<dyn FnMut(String) + Send>;

/// A test's hook, run before a claim's write or between it and its
/// check.
pub type Hook = Box<dyn FnMut() + Send>;

/// crew's implementation, serving each partition of its table in turn.
/// The runner opens one for each partition, by [`parts`].
pub struct Crew {
    config: Config,
    /// The forge state repositories are reached on, ending in `/`.
    forge: String,
    data_dir: PathBuf,
    say: Say,
    /// The runner's version, as the last hello gave it.
    version: Option<String>,
    /// Each partition's `<label>/main` as the last pull that could read
    /// it read it, by label.
    mains: BTreeMap<String, Main>,
    /// The pulls made, counting from 1.
    pulls: u64,
    /// Each launch withdrawn, by label and request, with the pull from
    /// which it may be claimed again (design Decision 18).
    again: BTreeMap<(String, u64), u64>,
    /// The lines the last pull said, so a pull says only what is new.
    said: BTreeSet<String>,
    before_claim: Option<Hook>,
    between_claim_and_check: Option<Hook>,
}

/// Opens crew's implementation from its table, `table`, of the
/// configuration file `file`, keeping its caches under `data_dir`, as its
/// [`parts`], each saying through the say `say_for` makes for its label.
pub fn open(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    say_for: &dyn Fn(&str) -> Say,
) -> Result<Vec<Part>, String> {
    Ok(parts(config::read(table, file)?, GITHUB, data_dir, say_for))
}

/// crew's implementation of `config` as parts, its state repositories
/// reached on `forge`: one [`Crew`] of one partition for each partition
/// of the table, in its order, each labelled by its partition and saying
/// through the say `say_for` makes for that label. A partition is a
/// drive, and each is pulled, answered and reported on its own, so one
/// that cannot be read or written hides only its own work (record 0026;
/// design Decision 7).
pub fn parts(
    config: Config,
    forge: &str,
    data_dir: &Path,
    say_for: &dyn Fn(&str) -> Say,
) -> Vec<Part> {
    config
        .partitions
        .iter()
        .map(|partition| {
            let one = Config {
                host: config.host.clone(),
                partitions: vec![partition.clone()],
            };
            let say = say_for(&partition.label);
            Part {
                label: partition.label.clone(),
                plane: Box::new(Crew::new(one, forge, data_dir, say)),
            }
        })
        .collect()
}

impl Crew {
    /// crew's implementation of `config`, its state repositories reached
    /// on `forge`, a base address ending in `/`. The cache a release
    /// before shared among every partition of a state repository is
    /// removed where it is there, and said (design Decision 7).
    pub fn new(config: Config, forge: &str, data_dir: &Path, mut say: Say) -> Crew {
        for partition in &config.partitions {
            remove_shared_cache(data_dir, &partition.state_repository, &mut say);
        }
        Crew {
            config,
            forge: forge.to_string(),
            data_dir: data_dir.to_path_buf(),
            say,
            version: None,
            mains: BTreeMap::new(),
            pulls: 0,
            again: BTreeMap::new(),
            said: BTreeSet::new(),
            before_claim: None,
            between_claim_and_check: None,
        }
    }

    /// Runs `hook` at every pull that claims, between the claim's write
    /// and its check: a test's way to have another runner claim then.
    pub fn between_claim_and_check(&mut self, hook: impl FnMut() + Send + 'static) {
        self.between_claim_and_check = Some(Box::new(hook));
    }

    /// Runs `hook` at every pull that claims, after it has read the other
    /// runners' branches and before its claim's write: with
    /// [`Crew::between_claim_and_check`], a test's way to have two runners
    /// claim before either checks.
    pub fn before_claim(&mut self, hook: impl FnMut() + Send + 'static) {
        self.before_claim = Some(Box::new(hook));
    }

    /// The state repository of `partition`, through the partition's own
    /// bare cache, `<data>/control/crew/<owner>/<name>/<label>.git`,
    /// outside `repositories/` so the keeper never takes it for a clone.
    /// Two partitions of one state repository share no git directory and
    /// so no weir lock, and one's fetch or push holds no other's pull or
    /// report (record 0026; design Decision 7). Its listing is stopped at
    /// thirty seconds (record 0042).
    fn store(&self, partition: &Partition) -> Store {
        let repository = &partition.state_repository;
        Store::new(
            caches(&self.data_dir)
                .join(&repository.owner)
                .join(&repository.name)
                .join(format!("{}.git", partition.label)),
            repository.url(&self.forge),
        )
    }
}

/// Where crew's implementation keeps its caches, `<data>/control/crew`.
fn caches(data_dir: &Path) -> PathBuf {
    data_dir.join("control/crew")
}

/// Removes the cache a release before kept for `repository`, shared
/// among every partition of it, `<data>/control/crew/<owner>/<name>.git`,
/// where it is there, saying one line naming it. A cache is made again
/// from the state repository (core.35).
fn remove_shared_cache(data_dir: &Path, repository: &StateRepository, say: &mut Say) {
    let shared = caches(data_dir)
        .join(&repository.owner)
        .join(format!("{}.git", repository.name));
    if shared.symlink_metadata().is_err() {
        return;
    }
    match std::fs::remove_dir_all(&shared) {
        Ok(()) => say(format!(
            "crew: removed {}, the cache of {repository} a release before shared among its \
             partitions: each partition reads through a cache of its own",
            shared.display()
        )),
        Err(error) => say(format!(
            "crew: {}, the cache of {repository} a release before shared among its partitions, \
             cannot be removed: {error}",
            shared.display()
        )),
    }
}

/// The host's room, as the hello's capacity gives it: `null`, unbounded,
/// or a number above zero.
fn has_room(hello: &Hello) -> bool {
    let Capacity::Printed(host) = &hello.capacity else {
        return false;
    };
    match serde_json::from_str::<Value>(&host.room) {
        Ok(Value::Null) => true,
        Ok(Value::Number(room)) => room.as_f64().is_some_and(|room| room > 0.0),
        _ => false,
    }
}

/// The lines a pull says.
#[derive(Default)]
struct Lines(Vec<String>);

impl Lines {
    fn say(&mut self, line: String) {
        self.0.push(format!("crew: {line}"));
    }
}

/// One partition's part of a pull: what it read, and what it gives.
struct Pulling<'a> {
    label: &'a str,
    host: &'a str,
    main: &'a Main,
    report: &'a Report,
    /// This host's branch as read, none where it has none.
    own: Option<Runner>,
    /// Every other host's branch, read or why it can't be.
    others: BTreeMap<String, Result<Runner, String>>,
}

impl Pulling<'_> {
    fn id(&self, number: u64) -> Id {
        ids::request(self.label, number)
    }

    /// Whether the runner's report answers request `number`: a launch, an
    /// act or a preview.
    fn reported(&self, number: u64) -> bool {
        let id = self.id(number);
        self.report.launches.contains_key(&id)
            || self.report.acts.contains_key(&id)
            || self.report.previews.contains_key(&id)
    }

    /// Whether this host's branch answers request `number`, a claim
    /// counting or not, in `answers.rec` or, a preview, in `previews.rec`
    /// (design Decision 10).
    fn branch_answers(&self, number: u64, claims: bool) -> bool {
        let Some(own) = self.own.as_ref() else {
            return false;
        };
        own.previews.contains_key(&number)
            || own
                .answers
                .get(&number)
                .is_some_and(|answer| claims || !write::is_claim(answer, self.host))
    }

    /// Whether the runner launched request `number`: its report answers
    /// it `launched`, or, where the report holds no answer to it, this
    /// host's branch does. A claim is never launched.
    fn launched(&self, number: u64) -> bool {
        let id = self.id(number);
        if let Some(launched) = self.report.launches.get(&id) {
            return launched.answer.outcome == Outcome::Launched;
        }
        if self.report.acts.contains_key(&id) {
            return false;
        }
        self.own
            .as_ref()
            .and_then(|own| own.answers.get(&number))
            .is_some_and(|answer| answer.get("Outcome") == Some("launched"))
    }

    /// Whether this host's branch holds its claim of request `number`.
    fn claimed(&self, number: u64) -> bool {
        self.own
            .as_ref()
            .and_then(|own| own.answers.get(&number))
            .is_some_and(|answer| write::is_claim(answer, self.host))
    }

    /// The other hosts whose branch answers request `number`.
    fn answering(&self, number: u64) -> Vec<String> {
        self.others
            .iter()
            .filter(|(_, read)| {
                read.as_ref()
                    .is_ok_and(|runner| runner.answers.contains_key(&number))
            })
            .map(|(host, _)| host.clone())
            .collect()
    }

    /// Why another host's branch can't be read, where one can't: whether
    /// it answers a launch can't then be told.
    fn unread(&self) -> Option<&String> {
        self.others.values().find_map(|read| read.as_ref().err())
    }

    /// Whether the runner holds its answer to request `number`, in its
    /// report or on this host's branch, a claim apart.
    fn holds(&self, number: u64) -> bool {
        self.reported(number) || self.branch_answers(number, false)
    }

    /// Whether request `number`'s work is still crew's, or the runner has
    /// answered it: such a request is given, so its place's status is
    /// carried until it is let go. A drop is current, whatever the plan
    /// holds, while the runner answers the launch it names, since crew
    /// writes a drop as its work leaves the plan. A preview request that
    /// reads is current by its own rule, [`Pulling::preview_current`]
    /// (record 0031 as amended on 2026-11-09).
    fn current(&self, request: &Request) -> bool {
        if let Some(done) = request.preview_done() {
            return self.preview_current(request, done);
        }
        self.main.plans(request.work())
            || self.holds(request.number)
            || (request.act() == Some("drop") && request.of().is_some_and(|of| self.holds(of)))
    }

    /// Whether a preview request that reads, marked `done` or not, is
    /// given: where the runner holds it; else only while it is crew's
    /// newest preview request of its `Host`, `Repo` and `Revision`, as
    /// crew keys its asks, and then a bring-up while its work is in the
    /// plan, and a done, whatever the plan holds, while the runner holds a
    /// bring-up of those three. So a bring-up and its done, once let go,
    /// are given no more, and a bring-up crew put away before the runner
    /// answered it is never brought up (record 0031 as amended on
    /// 2026-11-09; design Decision 2).
    fn preview_current(&self, request: &Request, done: bool) -> bool {
        if self.holds(request.number) {
            return true;
        }
        if !self.main.newest_preview(request) {
            return false;
        }
        if !done {
            return self.main.plans(request.work());
        }
        let key = request.preview_key();
        self.main.requests.iter().any(|earlier| {
            earlier.number < request.number
                && earlier.preview_key() == key
                && earlier.preview_done() == Some(false)
                && self.holds(earlier.number)
        })
    }
}

impl Crew {
    fn say_lines(&mut self, lines: Lines) {
        let said: BTreeSet<String> = lines.0.into_iter().collect();
        for line in &said {
            if !self.said.contains(line) {
                (self.say)(line.clone());
            }
        }
        self.said = said;
    }

    /// Every other runner's branch of `label` the state repository lists.
    fn others(
        &self,
        store: &Store,
        label: &str,
    ) -> Result<BTreeMap<String, Result<Runner, String>>, String> {
        let mut others = BTreeMap::new();
        for host in store.hosts(label)? {
            if host == self.config.host {
                continue;
            }
            let read = store.runner(label, &host).and_then(|runner| {
                runner.ok_or_else(|| format!("{} is not there", runner_branch(label, &host)))
            });
            others.insert(host, read);
        }
        Ok(others)
    }

    /// One partition's part of a pull: its wants, in `Request` order and
    /// then its roles', and the runner's report as its branch holds it.
    fn pull_partition(
        &mut self,
        partition: &Partition,
        hello: &Hello,
        report: &Report,
        lines: &mut Lines,
    ) -> Result<(Vec<Want>, Option<Report>), String> {
        let label = partition.label.clone();
        let host = self.config.host.clone();
        let store = self.store(partition);
        let main = store.main(&label)?;
        if main.unnumbered > 0 {
            lines.say(format!(
                "{label}/main's requests.rec holds {} record(s) with no whole Request number, \
                 left out: each can be given no id",
                main.unnumbered
            ));
        }
        let mut others = self.others(&store, &label)?;
        let own = match store.runner(&label, &host) {
            Ok(own) => own,
            Err(why) => {
                lines.say(why);
                None
            }
        };
        for why in others.values().filter_map(|read| read.as_ref().err()) {
            lines.say(why.clone());
        }

        // Which requests are this host's, before any claim.
        let mut given: BTreeMap<u64, Want> = BTreeMap::new();
        let mut kits: Vec<(&Request, String)> = Vec::new();
        let mut acts: Vec<&Request> = Vec::new();
        {
            let pulling = Pulling {
                label: &label,
                host: &host,
                main: &main,
                report,
                own: own.clone(),
                others: std::mem::take(&mut others),
            };
            // A request is given while its work is in the plan or the
            // runner holds it, a preview request while crew still asks for
            // it or the runner holds it (record 0031 as amended on
            // 2026-11-09).
            for request in &main.requests {
                if !pulling.current(request) {
                    continue;
                }
                // A launch a drop names no longer stands, as crew reads it:
                // it is given only where the runner launched it, so its
                // place is followed to its end, and is claimed by no one.
                if request.is_launch()
                    && main.dropped(request.number)
                    && !pulling.launched(request.number)
                {
                    continue;
                }
                match (request.host(), request.act()) {
                    (Some(named), _) if named == host => {
                        // Its repository among those the runner keeps, else
                        // those it drains, else its Repo as written (record
                        // 0021).
                        let repository = read::resolved(request, hello);
                        let want = read::want(&label, request, repository.as_deref())
                            .unwrap_or_else(|why| read::refused(&label, request, &why));
                        given.insert(request.number, want);
                    }
                    (Some(_), _) => {}
                    (None, Some("launch")) => {
                        let Some(repo) = request.get("Repo") else {
                            lines.say(format!(
                                "{label}/{}: names no Host and no Repo, so no runner takes it",
                                request.number
                            ));
                            continue;
                        };
                        let answered = pulling.reported(request.number)
                            || pulling.branch_answers(request.number, false);
                        let shared = |addresses: Vec<String>, how: &str, lines: &mut Lines| {
                            lines.say(format!(
                                "{label}/{}: Repo {repo} is the name of {}, which this \
                                 host {how}, so it is given nothing",
                                request.number,
                                addresses.join(" and ")
                            ));
                        };
                        let address = match read::kept(repo, &hello.repositories) {
                            Kept::Address(address) => address,
                            Kept::Shared(addresses) => {
                                shared(addresses, "keeps", lines);
                                continue;
                            }
                            // A launch the runner answered is given while
                            // the runner drains its repository, at its
                            // address, so its place is followed to its end
                            // through the clone the runner drains; where
                            // the runner neither keeps nor drains it, it is
                            // given nothing, and no old name is matched to
                            // a new one (record 0021).
                            Kept::No if answered => {
                                match read::kept(repo, &hello.draining) {
                                    Kept::Address(address) => {
                                        if let Ok(want) =
                                            read::want(&label, request, Some(&address))
                                        {
                                            given.insert(request.number, want);
                                        }
                                    }
                                    Kept::Shared(addresses) => shared(addresses, "drains", lines),
                                    Kept::No => {}
                                }
                                continue;
                            }
                            Kept::No => continue,
                        };
                        if let Err(why) = read::want(&label, request, Some(&address)) {
                            lines.say(format!(
                                "{label}/{}: names no Host, and {why}, so no runner's answer \
                                 to it is the one: it is given nothing",
                                request.number
                            ));
                            continue;
                        }
                        kits.push((request, address));
                    }
                    // An act naming no host is the runner's that holds its
                    // launch, a confirm among them (core.56).
                    (None, Some("retry" | "land" | "confirm" | "drop" | "priority")) => {
                        acts.push(request)
                    }
                    // A preview is the ask of the one host its Host names:
                    // two runners keeping its repository would each bring
                    // it up (design Decision 10).
                    (None, Some("preview")) => lines.say(format!(
                        "{label}/{}: a preview names no Host, so no runner brings it up: \
                         it is given nothing",
                        request.number
                    )),
                    (None, _) => lines.say(format!(
                        "{label}/{}: names no Host and an Act crew's implementation does not \
                         read, so no runner's answer to it is the one: it is given nothing",
                        request.number
                    )),
                }
            }
            others = pulling.others;
        }

        // A launch naming no host, claimed before it is given (design
        // Decision 18).
        let pulling = Pulling {
            label: &label,
            host: &host,
            main: &main,
            report,
            own: own.clone(),
            others,
        };
        let room = has_room(hello);
        let mut claims: Vec<u64> = Vec::new();
        let mut withdraws: BTreeMap<u64, Vec<String>> = BTreeMap::new();
        let mut alone: BTreeSet<u64> = BTreeSet::new();
        for (request, _) in &kits {
            let n = request.number;
            let answering = pulling.answering(n);
            if pulling.reported(n) || pulling.branch_answers(n, false) {
                alone.insert(n);
            } else if pulling.claimed(n) {
                if !answering.is_empty() {
                    withdraws.insert(n, answering);
                } else if pulling.unread().is_none() {
                    alone.insert(n);
                }
            } else if answering.is_empty()
                && pulling.unread().is_none()
                && room
                && self
                    .again
                    .get(&(label.clone(), n))
                    .is_none_or(|again| self.pulls >= *again)
            {
                claims.push(n);
            }
        }
        let mut others = pulling.others;
        // A claim of a launch a drop names is given nothing, so no act
        // goes with it.
        let mut own_claims: BTreeSet<u64> = own
            .as_ref()
            .map(|own| {
                own.answers
                    .iter()
                    .filter(|(n, answer)| write::is_claim(answer, &host) && !main.dropped(**n))
                    .map(|(n, _)| *n)
                    .collect()
            })
            .unwrap_or_default();
        if !claims.is_empty() {
            if let Some(hook) = &mut self.before_claim {
                hook();
            }
        }
        if !claims.is_empty() || !withdraws.is_empty() {
            let mut parts = Vec::new();
            if !claims.is_empty() {
                parts.push(format!("claims {}", numbers(&claims)));
            }
            if !withdraws.is_empty() {
                let numbers_withdrawn: Vec<u64> = withdraws.keys().copied().collect();
                parts.push(format!("withdraws {}", numbers(&numbers_withdrawn)));
            }
            let what = parts.join(", ");
            let at = boomdrive_control::At::now().to_string();
            let version = hello.runner_version.clone();
            let written = write::write(&store, &label, &host, Some(&what), |old| {
                Ok(claim_texts(
                    &host,
                    &version,
                    &at,
                    old,
                    &claims,
                    &withdraws.keys().copied().collect::<Vec<_>>(),
                ))
            });
            match written {
                Ok(_) => {
                    lines.say(format!("{}: {what}", runner_branch(&label, &host)));
                    own_claims.extend(&claims);
                    for n in withdraws.keys() {
                        own_claims.remove(n);
                    }
                }
                Err(why) => {
                    lines.say(format!(
                        "{} could not be written: {why}",
                        runner_branch(&label, &host)
                    ));
                    claims.clear();
                    for n in withdraws.keys() {
                        alone.remove(n);
                    }
                }
            }
            for (n, answering) in std::mem::take(&mut withdraws) {
                self.wait(&label, &host, n, answering);
            }
        }
        if !claims.is_empty() {
            if let Some(hook) = &mut self.between_claim_and_check {
                hook();
            }
            others = self.others(&store, &label)?;
            let mut contested: BTreeMap<u64, Vec<String>> = BTreeMap::new();
            for n in &claims {
                let answering: Vec<String> = others
                    .iter()
                    .filter(|(_, read)| {
                        read.as_ref()
                            .map_or(true, |runner| runner.answers.contains_key(n))
                    })
                    .map(|(host, _)| host.clone())
                    .collect();
                match answering.is_empty() {
                    true => {
                        alone.insert(*n);
                    }
                    false => {
                        contested.insert(*n, answering);
                    }
                }
            }
            if !contested.is_empty() {
                let withdrawn: Vec<u64> = contested.keys().copied().collect();
                let what = format!("withdraws {}", numbers(&withdrawn));
                let version = hello.runner_version.clone();
                let at = boomdrive_control::At::now().to_string();
                match write::write(&store, &label, &host, Some(&what), |old| {
                    Ok(claim_texts(&host, &version, &at, old, &[], &withdrawn))
                }) {
                    Ok(_) => {
                        lines.say(format!("{}: {what}", runner_branch(&label, &host)));
                        for n in &withdrawn {
                            own_claims.remove(n);
                        }
                    }
                    Err(why) => lines.say(format!(
                        "{} could not be written: {why}",
                        runner_branch(&label, &host)
                    )),
                }
                for (n, answering) in contested {
                    self.wait(&label, &host, n, answering);
                }
            }
        }
        if let Some(why) = others.values().find_map(|read| read.as_ref().err()) {
            if !kits.is_empty() {
                lines.say(format!(
                    "{label}: no launch naming no host is claimed while another runner's \
                     branch can't be read: {why}"
                ));
            }
        }
        for (request, address) in &kits {
            if alone.contains(&request.number) {
                if let Ok(want) = read::want(&label, request, Some(address)) {
                    given.insert(request.number, want);
                }
            }
        }

        // An act naming no host goes to the runner that holds its launch.
        let pulling = Pulling {
            label: &label,
            host: &host,
            main: &main,
            report,
            own: own.clone(),
            others,
        };
        for request in acts {
            let holds = request.of().is_some_and(|of| {
                pulling.reported(of)
                    || pulling.branch_answers(of, false)
                    || own_claims.contains(&of)
            });
            if !holds {
                continue;
            }
            match read::want(&label, request, None) {
                Ok(want) => {
                    given.insert(request.number, want);
                }
                Err(why) => lines.say(format!(
                    "{label}/{}: names no Host, and {why}, so it is given nothing",
                    request.number
                )),
            }
        }

        // Each role of this host's (design Decision 19).
        let mut places: Vec<roles::Placed> = Vec::new();
        for runner in pulling
            .others
            .values()
            .filter_map(|read| read.as_ref().ok())
        {
            places.extend(placed(&runner.places));
        }
        let mut own_places: BTreeMap<u64, roles::Placed> = pulling
            .own
            .as_ref()
            .map(|own| placed(&own.places).map(|p| (p.request, p)).collect())
            .unwrap_or_default();
        for (id, launched) in &report.launches {
            if let (Some(ids::Named::Request(n)), Some(status)) =
                (ids::named(&label, id), &launched.status)
            {
                own_places.insert(
                    n,
                    roles::Placed {
                        request: n,
                        place: launched.answer.place_dir.as_deref(),
                        status: &status.text,
                    },
                );
            }
        }
        places.extend(own_places.into_values());
        let branch_runs = pulling
            .own
            .as_ref()
            .map(|own| own.runs.clone())
            .unwrap_or_default();
        let (role_wants, role_lines) =
            roles::wants(&label, &host, &main, report, &places, &branch_runs);
        for line in role_lines {
            lines.say(line);
        }

        let back = own
            .as_ref()
            .map(|own| read_back(&label, &host, &main, own, hello));
        let mut wants: Vec<Want> = given.into_values().collect();
        wants.extend(role_wants);
        drop(places);
        self.mains.insert(label, main);
        Ok((wants, back))
    }

    /// Records that `host` withdrew request `n` of `label`, having seen
    /// the claims of `answering`: it is claimed again no sooner than as
    /// many pulls later as the host's place in their name order.
    fn wait(&mut self, label: &str, host: &str, n: u64, answering: Vec<String>) {
        let mut claimants: BTreeSet<String> = answering.into_iter().collect();
        claimants.insert(host.to_string());
        let place = claimants
            .iter()
            .position(|named| named == host)
            .unwrap_or(0) as u64
            + 1;
        self.again
            .insert((label.to_string(), n), self.pulls + place);
    }
}

/// Numbers, as a commit's message names them.
fn numbers(numbers: &[u64]) -> String {
    numbers
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

/// A branch's `Place` records as the places that may wait on a role.
fn placed(places: &BTreeMap<u64, Record>) -> impl Iterator<Item = roles::Placed<'_>> {
    places.iter().filter_map(|(n, record)| {
        Some(roles::Placed {
            request: *n,
            place: record.get("Place"),
            status: record.get("Status")?,
        })
    })
}

/// The files of a write that claims `claims` and withdraws `withdraws`,
/// formed from the tip's `old`: its answers with each claim added and
/// each withdrawn claim removed, and its other files as they stand, or
/// made where the branch has none.
fn claim_texts(
    host: &str,
    version: &str,
    at: &str,
    old: &read::RunnerFiles,
    claims: &[u64],
    withdraws: &[u64],
) -> write::Texts {
    let mut answers = write::answers_at(old);
    for n in claims {
        answers
            .entry(*n)
            .or_insert_with(|| write::claim(*n, host, at));
    }
    for n in withdraws {
        if answers
            .get(n)
            .is_some_and(|answer| write::is_claim(answer, host))
        {
            answers.remove(n);
        }
    }
    let since = write::since_at(old).unwrap_or_else(|| at.to_string());
    write::Texts {
        runner: old
            .get("runner.rec")
            .cloned()
            .flatten()
            .unwrap_or_else(|| write::runner_text(host, version, &since)),
        answers: write::answers_text(&answers),
        places: old
            .get("places.rec")
            .cloned()
            .flatten()
            .unwrap_or_else(|| write::places_text(&BTreeMap::new(), &BTreeMap::new())),
        previews: old.get("previews.rec").cloned().flatten(),
    }
}

/// The runner's report as its branch of partition `label` holds it: each
/// answer, as the request it answers would be given, its claims left
/// out; a launch with its place's status; and each preview of
/// `previews.rec` as the preview its request names, its repository
/// resolved against `hello` as at a pull (design Decision 10).
fn read_back(label: &str, host: &str, main: &Main, own: &Runner, hello: &Hello) -> Report {
    let mut report = Report::default();
    for (n, record) in &own.previews {
        let Some(request) = main.request(*n) else {
            continue;
        };
        let repository = read::resolved(request, hello);
        let Ok(Want::Preview(preview)) = read::want(label, request, repository.as_deref()) else {
            continue;
        };
        if let Some(previewed) = read::previewed(preview, record) {
            report.previews.insert(ids::request(label, *n), previewed);
        }
    }
    for (n, record) in &own.answers {
        if write::is_claim(record, host) {
            continue;
        }
        let (Some(request), Some(answer)) = (main.request(*n), read::answer(record)) else {
            continue;
        };
        let id = ids::request(label, *n);
        let launch = match read::want(label, request, None) {
            Ok(Want::Offer(_)) => {
                let launched: Launched = read::launched(answer, own.places.get(n));
                report.launches.insert(id, launched);
                continue;
            }
            Ok(Want::Act(act)) => act.launch,
            Ok(_) => continue,
            Err(why) => match read::refused(label, request, &why) {
                Want::Act(act) => act.launch,
                _ => continue,
            },
        };
        report.acts.insert(
            id,
            Acted {
                launch,
                lease: read::LEASE.number,
                outcome: answer.outcome,
                words: answer.words,
                at: answer.at,
            },
        );
    }
    report
}

impl ControlPlane for Crew {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        self.pulls += 1;
        self.version = Some(hello.runner_version.clone());
        let mut lines = Lines::default();
        let mut wants = Vec::new();
        let mut back: Option<Report> = None;
        let mut unread = Vec::new();
        let partitions = self.config.partitions.clone();
        for partition in &partitions {
            match self.pull_partition(partition, hello, report, &mut lines) {
                Ok((given, held)) => {
                    wants.extend(given);
                    if let Some(held) = held {
                        let all = back.get_or_insert_with(Report::default);
                        all.launches.extend(held.launches);
                        all.acts.extend(held.acts);
                        all.previews.extend(held.previews);
                    }
                }
                Err(why) => {
                    let words = format!(
                        "{}/main of {} could not be read: {why}",
                        partition.label, partition.state_repository
                    );
                    lines.say(words.clone());
                    unread.push(words);
                }
            }
        }
        self.say_lines(lines);
        if unread.len() == partitions.len() {
            return Err(Unread(unread.join("; ")));
        }
        Ok(Pulled {
            wants,
            terminals: Vec::new(),
            report: back,
        })
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let host = self.config.host.clone();
        let mut reported = Reported::default();
        let mut unsent = Vec::new();
        let mut branches_written = 0;
        let partitions = self.config.partitions.clone();
        for partition in &partitions {
            let label = &partition.label;
            let (Some(main), Some(version)) = (self.mains.get(label), self.version.clone()) else {
                let words = format!(
                    "{label}/main has not been read, so {} is not written",
                    runner_branch(label, &host)
                );
                (self.say)(format!("crew: {words}"));
                unsent.push(words);
                continue;
            };
            let gone = write::let_go(label, &host, main, report);
            let mut kept = report.clone();
            kept.launches.retain(|id, _| !gone.contains(id));
            kept.acts.retain(|id, _| !gone.contains(id));
            kept.previews.retain(|id, _| !gone.contains(id));
            let addresses: BTreeMap<String, String> = main
                .roles
                .iter()
                .filter_map(|role| {
                    Some((
                        role.get("Role")?.to_string(),
                        role.get("Address")?.to_string(),
                    ))
                })
                .collect();
            let at = boomdrive_control::At::now().to_string();
            let store = self.store(partition);
            let main = main.clone();
            let written = write::write(&store, label, &host, None, |old| {
                let claims: Vec<Record> = write::answers_at(old)
                    .into_iter()
                    .filter(|(n, answer)| {
                        write::is_claim(answer, &host)
                            && !kept.launches.contains_key(&ids::request(label, *n))
                            && !main.dropped(*n)
                            && main
                                .request(*n)
                                .is_some_and(|request| main.plans(request.work()))
                    })
                    .map(|(_, claim)| claim)
                    .collect();
                let since = write::since_at(old).unwrap_or_else(|| at.clone());
                let previews_held = old.get("previews.rec").is_some_and(Option::is_some);
                Ok(write::texts(
                    label,
                    &host,
                    &version,
                    &since,
                    &kept,
                    &claims,
                    &addresses,
                    previews_held,
                ))
            });
            match written {
                Ok(what) => {
                    if let Some(what) = what {
                        (self.say)(format!("crew: {}: {what}", runner_branch(label, &host)));
                    }
                    reported.let_go.extend(gone);
                    reported
                        .statuses_taken
                        .extend(write::statuses_held(label, &kept));
                    branches_written += 1;
                }
                Err(why) => {
                    let words = format!(
                        "{} could not be written: {why}",
                        runner_branch(label, &host)
                    );
                    (self.say)(format!("crew: {words}"));
                    unsent.push(words);
                }
            }
        }
        // A partition not written hides only its own work, which the
        // runner's next report carries again: the others' let-go stands.
        match branches_written == 0 && !unsent.is_empty() {
            true => Err(Unsent(unsent.join("; "))),
            false => Ok(reported),
        }
    }

    /// An id `ids::named` reads as one of its partitions'.
    fn owns(&self, id: &Id) -> bool {
        self.config
            .partitions
            .iter()
            .any(|partition| ids::named(&partition.label, id).is_some())
    }
}
