//! What a control pass runs over, faked for its tests (design Decision
//! 21). Flume is held in memory as the places it stands and its host
//! part, each answer scripted where a test says, and each call recorded.
//! The runner's clones and its tools' versions are what a test sets. A
//! stub `flume` program stands in where a test runs the pass over
//! Flume's own command line, a stub `sluice` stands in for sluice's, a
//! stub `herdr` for herdr's, and a stub `systemd-creds` for the host's
//! credential store. Every file lies in the test's own temporary
//! directory, and no test runs a real Flume, sluice, herdr or
//! `systemd-creds`.

#![allow(dead_code)]

use boomdrive::carry::{Carrier, Clones, Keeping, Least, Passed, Reach};
use boomdrive::flume::{self, Act, Flume, LaunchArgs, Reply, Unrun};
use boomdrive::git::Git;
use boomdrive::hello::Tools;
use boomdrive::locks::Locks;
use boomdrive::report::{self, ReportFile};
use boomdrive::run::{Event, Notices};
use boomdrive::sluice;
use boomdrive::terminal::{self, Relays};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, AgentSetting, ControlPlane, Hold, Id, Launch, Lease, Offer, Preview, Report,
    ToolVersion, Want,
};
use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::TempDir;

/// The repository every bench keeps with its clone made.
pub const KEPT: &str = "agentplot/flume";

/// A fixture of Flume 0.4.1's, as it printed it.
pub fn fixture(name: &str) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/flume-0.4.1")
        .join(name);
    fs::read_to_string(file).expect("the fixture")
}

/// A fixture of sluice 0.61.1's, as it printed it.
pub fn sluice_fixture(name: &str) -> String {
    fs::read_to_string(sluice_fixtures().join(name)).expect("the fixture")
}

/// The directory of sluice 0.61.1's fixtures.
pub fn sluice_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sluice-0.61.1")
}

/// A fixture of sluice 0.74.3's, a preview's, as it printed it.
pub fn preview_fixture(name: &str) -> String {
    fs::read_to_string(preview_fixtures().join(name)).expect("the fixture")
}

/// The directory of sluice 0.74.3's fixtures, a preview's.
pub fn preview_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sluice-0.74.3")
}

/// `value` as Flume prints JSON: on one line, ending in a newline.
pub fn printed(value: &Value) -> String {
    serde_json::to_string(value).expect("JSON") + "\n"
}

/// One call the runner made of Flume.
// A test records a handful of calls, so a launch's size costs nothing
// that boxing it would save.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    Launch(LaunchArgs),
    Act(String, Act),
    Status,
    /// `status <place>`, in the clone's container where one was named.
    StatusOf(Option<PathBuf>, String),
    /// `inbox <record> --recipient boomdrive`.
    Collect(String),
}

/// Flume in memory.
#[derive(Default)]
pub struct FakeFlume {
    state: RefCell<State>,
}

#[derive(Default)]
struct State {
    /// The entry of each place standing, in Flume's order.
    places: Vec<Value>,
    host: Value,
    /// The listing's whole answer, where a test scripts it.
    listing: Option<Result<Reply, Unrun>>,
    /// A launch's answer, by the launch's name.
    launches: BTreeMap<String, Result<Reply, Unrun>>,
    /// An act's answer, by its place.
    acts: BTreeMap<String, Result<Reply, Unrun>>,
    /// `status <place>`'s answer, by the place.
    statuses: BTreeMap<String, Result<Reply, Unrun>>,
    /// A collection's answer, by its record.
    collects: BTreeMap<String, Result<Reply, Unrun>>,
    /// The recipients each place it launched was launched with, by its
    /// place, the runner's own among them where the launch named any.
    recipients: BTreeMap<String, Vec<String>>,
    /// The time the next record of a drop or a done is tidied at.
    tidied_at: u64,
    /// Every call left unrun, with these words.
    unrun: Option<String>,
    calls: Vec<Call>,
    /// What Flume does on its own while the next launch or act runs.
    meanwhile: Option<Meanwhile>,
}

/// What Flume does on its own while a command of the runner's runs.
type Meanwhile = Box<dyn FnOnce(&FakeFlume)>;

/// A host part with no bound, its one stream `default`, and a pool whose
/// account is named.
pub fn unbounded() -> Value {
    json!({
        "running": 0, "bound": null, "room": null, "queue": [],
        "streams": [
            { "name": "default", "priority": 0, "running": 0, "bound": null, "room": null, "queue": [] }
        ],
        "pools": [{ "name": "pool-a", "accounts": ["pool-a-1"] }],
        "addresses": [], "bases": [], "terminal_kept": false
    })
}

impl FakeFlume {
    /// A Flume standing no place, on a host with no bound.
    pub fn new() -> FakeFlume {
        let flume = FakeFlume::default();
        flume.state.borrow_mut().host = unbounded();
        flume
    }

    /// Stands a place whose entry is `entry`.
    pub fn stand(&self, entry: Value) {
        self.state.borrow_mut().places.push(entry);
    }

    /// The entry of the place at `place`, where one stands, else the
    /// record Flume keeps of it tidied, `<place>@<at>`.
    pub fn entry(&self, place: &str) -> Option<Value> {
        let state = self.state.borrow();
        let tidied = format!("{place}@");
        let standing = state.places.iter().find(|entry| entry["place"] == place);
        standing
            .or_else(|| {
                state.places.iter().find(|entry| {
                    entry["place"]
                        .as_str()
                        .is_some_and(|named| named.starts_with(&tidied))
                })
            })
            .cloned()
    }

    /// Sets the place at `place`'s field `field` to `value`.
    pub fn set(&self, place: &str, field: &str, value: Value) {
        let mut state = self.state.borrow_mut();
        let entry = state
            .places
            .iter_mut()
            .find(|entry| entry["place"] == place)
            .expect("the place stands");
        entry[field] = value;
    }

    /// The place at `place` landed, at commit `c0ffee`, and over.
    pub fn land(&self, place: &str) {
        let landed =
            json!({ "over": { "over": { "landed": { "commit": "c0ffee" } }, "kept": null } });
        self.set(place, "phase", landed);
    }

    /// The place at `place`, landed at commit `c0ffee`, tidied at `at`
    /// while an end of it is pending for `scratch/planner`: Flume keeps its
    /// record as `<place>@<at>`, and answers `status <place>` with it.
    pub fn tidy_pending(&self, place: &str, at: u64) {
        self.tidy_held(place, at, &["scratch/planner"]);
    }

    /// The place at `place`, landed at commit `c0ffee`, tidied at `at`
    /// while an end of it is pending for each of `pending_for`: Flume keeps
    /// its record as `<place>@<at>`, and answers `status <place>` with it.
    pub fn tidy_held(&self, place: &str, at: u64, pending_for: &[&str]) {
        self.tidy_over(
            place,
            json!({ "landed": { "commit": "c0ffee" } }),
            at,
            pending_for,
        );
    }

    /// The place at `place`, its work over as `over`, tidied at `at` while
    /// an end of it is pending for each of `pending_for`.
    fn tidy_over(&self, place: &str, over: Value, at: u64, pending_for: &[&str]) {
        let tidied = json!({ "tidied": {
            "over": over,
            "at": at,
            "pending_for": pending_for,
        } });
        self.set(place, "phase", tidied);
        self.set(place, "place", json!(format!("{place}@{at}")));
    }

    /// The place at `place` tidied, with nothing pending: no listing holds
    /// it, and `status <place>` is refused.
    pub fn tidy(&self, place: &str) {
        self.state
            .borrow_mut()
            .places
            .retain(|entry| entry["place"] != place);
    }

    /// The host part, `status.host`.
    pub fn host(&self, host: Value) {
        self.state.borrow_mut().host = host;
    }

    /// The listing answered with `reply`, as a test scripts it whole.
    pub fn listing(&self, reply: Result<Reply, Unrun>) {
        self.state.borrow_mut().listing = Some(reply);
    }

    /// The launch named `name` answered with `reply`.
    pub fn launch_answers(&self, name: &str, reply: Result<Reply, Unrun>) {
        self.state
            .borrow_mut()
            .launches
            .insert(name.to_string(), reply);
    }

    /// An act on the place at `place` answered with `reply`.
    pub fn act_answers(&self, place: &str, reply: Result<Reply, Unrun>) {
        self.state
            .borrow_mut()
            .acts
            .insert(place.to_string(), reply);
    }

    /// `status <place>` answered with `reply`.
    pub fn status_answers(&self, place: &str, reply: Result<Reply, Unrun>) {
        self.state
            .borrow_mut()
            .statuses
            .insert(place.to_string(), reply);
    }

    /// The collection of `record` answered with `reply`, until the test
    /// says otherwise.
    pub fn collect_answers(&self, record: &str, reply: Result<Reply, Unrun>) {
        self.state
            .borrow_mut()
            .collects
            .insert(record.to_string(), reply);
    }

    /// The collection of `record` answered as Flume answers it again.
    pub fn collect_as_flume(&self, record: &str) {
        self.state.borrow_mut().collects.remove(record);
    }

    /// `then` done to Flume while the next launch or act runs, as Flume's
    /// own settling moves a place on while the runner waits on a command.
    pub fn meanwhile(&self, then: impl FnOnce(&FakeFlume) + 'static) {
        self.state.borrow_mut().meanwhile = Some(Box::new(then));
    }

    /// Every call left unrun with `words`, as when `flume` is not there.
    pub fn unrun(&self, words: &str) {
        self.state.borrow_mut().unrun = Some(words.to_string());
    }

    /// Every call answered again.
    pub fn run(&self) {
        self.state.borrow_mut().unrun = None;
    }

    /// Each call made, in order.
    pub fn calls(&self) -> Vec<Call> {
        self.state.borrow().calls.clone()
    }

    /// Each call made, in order, forgotten once given.
    pub fn take_calls(&self) -> Vec<Call> {
        std::mem::take(&mut self.state.borrow_mut().calls)
    }

    /// Each call but the listing: what acts on, launches or reads a place.
    pub fn commands(&self) -> Vec<Call> {
        let calls = self.calls();
        calls
            .into_iter()
            .filter(|call| *call != Call::Status)
            .collect()
    }

    /// Each act asked, with its place, in order.
    pub fn acts(&self) -> Vec<(String, Act)> {
        self.calls()
            .into_iter()
            .filter_map(|call| match call {
                Call::Act(place, act) => Some((place, act)),
                _ => None,
            })
            .collect()
    }

    /// Each record collected, in order.
    pub fn collects(&self) -> Vec<String> {
        self.calls()
            .into_iter()
            .filter_map(|call| match call {
                Call::Collect(record) => Some(record),
                _ => None,
            })
            .collect()
    }

    /// Each launch's arguments, in order.
    pub fn launches(&self) -> Vec<LaunchArgs> {
        self.calls()
            .into_iter()
            .filter_map(|call| match call {
                Call::Launch(args) => Some(args),
                _ => None,
            })
            .collect()
    }

    fn call(&self, call: Call) -> Option<Unrun> {
        let mut state = self.state.borrow_mut();
        state.calls.push(call);
        state.unrun.clone().map(Unrun)
    }

    /// What Flume does on its own while a launch or an act runs.
    fn settle(&self) {
        let meanwhile = self.state.borrow_mut().meanwhile.take();
        if let Some(then) = meanwhile {
            then(self);
        }
    }

    /// Whether the place at `place` was launched with the runner among
    /// its recipients.
    fn held_for_runner(&self, place: &str) -> bool {
        self.state
            .borrow()
            .recipients
            .get(place)
            .is_some_and(|named| named.iter().any(|name| name == flume::RECIPIENT))
    }

    fn listing_now(&self) -> Value {
        let state = self.state.borrow();
        json!({ "status": { "places": state.places, "host": state.host } })
    }
}

/// The place a launch named `name` stands at in a fake Flume.
pub fn place_of(name: &str) -> String {
    format!("/places/{name}")
}

impl Flume for FakeFlume {
    fn launch(&self, args: &LaunchArgs, _brief_dir: &Path) -> Result<Reply, Unrun> {
        if let Some(unrun) = self.call(Call::Launch(args.clone())) {
            return Err(unrun);
        }
        self.settle();
        if let Some(reply) = self.state.borrow().launches.get(&args.name) {
            return reply.clone();
        }
        let entry = json!({
            "place": place_of(&args.name),
            "repository": args.container_dir,
            "kind": args.kind,
            "name": args.name,
            "mark": args.mark,
            "phase": "live",
            "priority": args.priority.unwrap_or(0),
            "stream": { "name": args.stream.as_deref().unwrap_or("default"), "from": "launch" },
        });
        self.stand(entry.clone());
        self.state
            .borrow_mut()
            .recipients
            .insert(place_of(&args.name), flume::recipients(&args.recipients));
        Ok(Reply::Given(printed(
            &json!({ "launched": { "place": entry } }),
        )))
    }

    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun> {
        if let Some(unrun) = self.call(Call::Act(place.to_string(), act.clone())) {
            return Err(unrun);
        }
        self.settle();
        if let Some(reply) = self.state.borrow().acts.get(place) {
            return reply.clone();
        }
        let word = match act {
            Act::Retry { .. } => "retry",
            Act::Drop => "drop",
            Act::Priority(_) => "priority",
            Act::Done => "done",
        };
        match act {
            // A drop or a done is an end pending for the place's recipients
            // that are no role's address, the runner's among them, and Flume
            // keeps the place's record while it is (Flume record 0060).
            Act::Drop | Act::Done if self.held_for_runner(place) => {
                let over = if *act == Act::Drop { "dropped" } else { "done" };
                let at = {
                    let mut state = self.state.borrow_mut();
                    state.tidied_at += 1;
                    100 + state.tidied_at
                };
                self.tidy_over(place, json!(over), at, &[flume::RECIPIENT]);
            }
            Act::Drop | Act::Done => self.tidy(place),
            Act::Priority(priority) => self.set(place, "priority", json!(priority)),
            Act::Retry { .. } => {}
        }
        Ok(Reply::Given(printed(
            &json!({ "acted": { "act": word, "place": place } }),
        )))
    }

    fn status(&self) -> Result<Reply, Unrun> {
        if let Some(unrun) = self.call(Call::Status) {
            return Err(unrun);
        }
        if let Some(reply) = &self.state.borrow().listing {
            return reply.clone();
        }
        Ok(Reply::Given(printed(&self.listing_now())))
    }

    fn collect(&self, record: &str) -> Result<Reply, Unrun> {
        if let Some(unrun) = self.call(Call::Collect(record.to_string())) {
            return Err(unrun);
        }
        if let Some(reply) = self.state.borrow().collects.get(record) {
            return reply.clone();
        }
        let mut state = self.state.borrow_mut();
        let Some(at) = state
            .places
            .iter()
            .position(|entry| entry["place"] == record)
        else {
            return Ok(Reply::Refused(format!(
                "no place {record} stands on this host"
            )));
        };
        let entry = &mut state.places[at];
        let mut collected = Vec::new();
        if let Some(pending) = entry
            .pointer_mut("/phase/tidied/pending_for")
            .and_then(Value::as_array_mut)
        {
            let before = pending.len();
            pending.retain(|name| name != flume::RECIPIENT);
            if pending.len() < before {
                collected.push(json!({ "place": record }));
            }
            if pending.is_empty() {
                state.places.remove(at);
            }
        }
        Ok(Reply::Given(printed(&json!({ "inbox": {
            "recipient": flume::RECIPIENT,
            "ends": collected,
            "waiting": [],
        } }))))
    }

    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun> {
        let call = Call::StatusOf(container_dir.map(Path::to_path_buf), place.to_string());
        if let Some(unrun) = self.call(call) {
            return Err(unrun);
        }
        if let Some(reply) = self.state.borrow().statuses.get(place) {
            return reply.clone();
        }
        let host = self.state.borrow().host.clone();
        match self.entry(place) {
            Some(entry) => Ok(Reply::Given(printed(
                &json!({ "status": { "places": [entry], "host": host } }),
            ))),
            None => Ok(Reply::Refused(format!(
                "no place {place} stands on this host"
            ))),
        }
    }
}

/// The runner's clones, as a test sets them.
#[derive(Default)]
pub struct FakeClones {
    kept: RefCell<Vec<(String, Keeping)>>,
}

impl FakeClones {
    /// `address` named, its clone made in `container`, which is made
    /// where it is not there, so a launch finds the clone again.
    pub fn ready(&self, address: &str, container: &Path) {
        fs::create_dir_all(container).expect("the clone's container");
        self.keep(address, Keeping::Ready(container.to_path_buf()));
    }

    /// `address` named, its clone not yet made.
    pub fn not_yet(&self, address: &str) {
        self.keep(address, Keeping::NotYet);
    }

    /// `address` named no more, as a configuration that drops it: a clone
    /// made stands, dropped, in its container, and one not made is gone.
    pub fn drop_named(&self, address: &str) {
        match self.keeping(address) {
            Keeping::Ready(container) | Keeping::Dropped(container) => {
                self.keep(address, Keeping::Dropped(container))
            }
            Keeping::NotYet | Keeping::NotNamed => self.gone(address),
        }
    }

    /// `address` neither named nor with a clone, as one dropped and then
    /// removed.
    pub fn gone(&self, address: &str) {
        let mut kept = self.kept.borrow_mut();
        kept.retain(|(named, _)| !named.eq_ignore_ascii_case(address));
    }

    fn keep(&self, address: &str, keeping: Keeping) {
        let mut kept = self.kept.borrow_mut();
        kept.retain(|(named, _)| !named.eq_ignore_ascii_case(address));
        kept.push((address.to_string(), keeping));
    }
}

impl Clones for FakeClones {
    fn keeping(&self, address: &str) -> Keeping {
        let kept = self.kept.borrow();
        kept.iter()
            .find(|(named, _)| named.eq_ignore_ascii_case(address))
            .map_or(Keeping::NotNamed, |(_, keeping)| keeping.clone())
    }

    fn made(&self) -> Vec<String> {
        let kept = self.kept.borrow();
        kept.iter()
            .filter(|(_, keeping)| matches!(keeping, Keeping::Ready(_)))
            .map(|(address, _)| address.clone())
            .collect()
    }

    fn dropped(&self) -> Vec<String> {
        let kept = self.kept.borrow();
        kept.iter()
            .filter(|(_, keeping)| matches!(keeping, Keeping::Dropped(_)))
            .map(|(address, _)| address.clone())
            .collect()
    }
}

/// The tools' versions: each `<tool> 1.0.0`, but as a test names one.
#[derive(Default)]
pub struct FakeTools {
    read: RefCell<BTreeMap<String, ToolVersion>>,
}

impl FakeTools {
    /// `tool` read as missing, as when it could not run.
    pub fn missing(&self, tool: &str) {
        self.reads(
            tool,
            ToolVersion::Missing(format!("{tool} could not run: not found")),
        );
    }

    /// `tool`'s `--version` read as printing `line` first.
    pub fn prints(&self, tool: &str, line: &str) {
        self.reads(tool, ToolVersion::Printed(line.to_string()));
    }

    /// `tool`'s version read as `version`.
    pub fn reads(&self, tool: &str, version: ToolVersion) {
        self.read.borrow_mut().insert(tool.to_string(), version);
    }
}

impl Tools for FakeTools {
    fn version(&self, tool: &str) -> ToolVersion {
        match self.read.borrow().get(tool) {
            Some(version) => version.clone(),
            None => ToolVersion::Printed(format!("{tool} 1.0.0")),
        }
    }
}

/// The label of the part a bench's passes are of, but where a test names
/// another.
pub const PART: &str = "home";

/// A host for control passes: a fake Flume, a stub `sluice`, a stub
/// `herdr` and the terminals relayed through it, clones and tools, each
/// clone's lock, the one report file, and a data directory of its own,
/// which keeps [`KEPT`] with its clone's container made.
pub struct Bench {
    /// Dropped first, so every terminal's thread ends before the stubs
    /// and the data directory go.
    pub relays: Relays,
    relay_lines: Arc<Mutex<Vec<String>>>,
    pub herdr: HerdrStub,
    pub dir: TempDir,
    pub flume: FakeFlume,
    pub sluice: SluiceStub,
    /// git, for an inbox read from a clone: a test that names an inbox
    /// sets the scratch host's.
    pub git: Git,
    pub clones: FakeClones,
    pub tools: FakeTools,
    /// Each clone's lock, as the keeper and the launches take it.
    pub locks: Arc<Locks>,
    /// The one report file every part's pass keeps its share of.
    pub report_file: ReportFile,
    /// Whether the last pull was read, kept across the bench's passes as
    /// a part's thread keeps it.
    reach: Reach,
    /// What was last said of Flume's version, kept across the bench's
    /// passes as the loop keeps it.
    least: Least,
    /// How long a pass awaits sluice's ask of a preview before it
    /// interrupts it: [`sluice::PREVIEW_PATIENCE`], but where a test sets
    /// another.
    pub sluice_patience: Cell<Duration>,
    notices: Notices,
    events: Receiver<Event>,
}

impl Bench {
    pub fn new() -> Bench {
        let dir = tempfile::tempdir().expect("a temporary directory");
        let (events, received) = mpsc::channel();
        let herdr = HerdrStub::new();
        let relay_lines = Arc::new(Mutex::new(Vec::new()));
        let said = Arc::clone(&relay_lines);
        let relays = Relays::new(terminal::Cli::new(herdr.program()), move |line| {
            said.lock().expect("the relay's lines").push(line);
        });
        let report_file = ReportFile::new(&report::control_dir(&dir.path().join("data")));
        let bench = Bench {
            relays,
            relay_lines,
            herdr,
            dir,
            flume: FakeFlume::new(),
            sluice: SluiceStub::new(),
            git: Git::default(),
            clones: FakeClones::default(),
            tools: FakeTools::default(),
            locks: Arc::new(Locks::new()),
            report_file,
            reach: Reach::default(),
            least: Least::default(),
            sluice_patience: Cell::new(sluice::PREVIEW_PATIENCE),
            notices: Notices::new(events),
            events: received,
        };
        let container = bench.container(KEPT);
        fs::create_dir_all(&container).expect("the clone's container");
        bench.clones.ready(KEPT, &container);
        bench
    }

    /// The data directory.
    pub fn data(&self) -> PathBuf {
        self.dir.path().join("data")
    }

    /// The control directory, `<data>/control`.
    pub fn control_dir(&self) -> PathBuf {
        report::control_dir(&self.data())
    }

    /// The report file.
    pub fn report_file(&self) -> PathBuf {
        self.report_file.path().to_path_buf()
    }

    /// The container `address`'s clone is made in.
    pub fn container(&self, address: &str) -> PathBuf {
        self.data().join("repositories").join(address)
    }

    /// One control pass against `plane` over the fake Flume, and the lines
    /// it said.
    pub fn pass(&self, plane: &mut Fake) -> (Passed, Vec<String>) {
        self.pass_over(&self.flume, plane)
    }

    /// One control pass against `plane` over `flume`, the next pass never
    /// due, and the lines it said.
    pub fn pass_over(&self, flume: &dyn Flume, plane: &mut Fake) -> (Passed, Vec<String>) {
        self.pass_due(flume, plane, &|| false)
    }

    /// One control pass against `plane` over `flume`, the next pass due
    /// once `due` says so, and the lines it said.
    pub fn pass_due(
        &self,
        flume: &dyn Flume,
        plane: &mut Fake,
        due: &dyn Fn() -> bool,
    ) -> (Passed, Vec<String>) {
        self.pass_of(PART, flume, plane, due)
    }

    /// One control pass of the part `label`, whose control plane is
    /// `plane`, over `flume`, the next pass due once `due` says so, and
    /// the lines it said.
    pub fn pass_of(
        &self,
        label: &str,
        flume: &dyn Flume,
        plane: &mut dyn ControlPlane,
        due: &dyn Fn() -> bool,
    ) -> (Passed, Vec<String>) {
        let control_dir = self.control_dir();
        let sluice = sluice::Cli::new(self.sluice.program()).patient(self.sluice_patience.get());
        let carrier = Carrier {
            label,
            flume,
            sluice: &sluice,
            git: &self.git,
            clones: &self.clones,
            tools: &self.tools,
            notices: &self.notices,
            control_dir: &control_dir,
            report: &self.report_file,
            locks: &self.locks,
            relays: &self.relays,
            reach: &self.reach,
            least: &self.least,
            due,
        };
        let mut lines = Vec::new();
        let passed = carrier.pass(plane, &mut |line| lines.push(line));
        (passed, lines)
    }

    /// The report the file holds.
    pub fn report(&self) -> Report {
        report::read(&self.report_file())
            .expect("the report file reads")
            .expect("the report file is there")
    }

    /// Each line the terminals said, in order.
    pub fn relay_lines(&self) -> Vec<String> {
        self.relay_lines.lock().expect("the relay's lines").clone()
    }

    /// Each repository the keeper was told of a landing on since last
    /// asked.
    pub fn landings(&self) -> Vec<String> {
        self.events
            .try_iter()
            .filter_map(|event| match event {
                Event::Landed(address) => Some(address.to_string()),
                Event::Stop => None,
            })
            .collect()
    }
}

/// An offer `id` of a `change` named `name` in [`KEPT`] under lease 1 with
/// no expiry, giving nothing more.
pub fn offer(id: &str, name: &str) -> Offer {
    Offer {
        id: Id::new(id),
        lease: Lease {
            number: 1,
            expires: None,
        },
        launch: Launch {
            repository: KEPT.to_string(),
            kind: "change".to_string(),
            name: name.to_string(),
            base: None,
            brief: None,
            stream: None,
            priority: None,
            intent: None,
            recipients: Vec::new(),
            holds: Vec::new(),
            hold_submission: false,
            after: Vec::new(),
            agent_settings: Vec::new(),
        },
    }
}

/// An offer of a launch with every field, design's unit launch: `change`
/// named `a` in `example/crew` on `bolt/x`.
pub fn every_field(id: &str) -> Offer {
    let mut offer = offer(id, "a");
    offer.launch = Launch {
        repository: "example/crew".to_string(),
        kind: "change".to_string(),
        name: "a".to_string(),
        base: Some("bolt/x".to_string()),
        brief: Some("Parse the input.\n\nAs words.".to_string()),
        stream: Some("home".to_string()),
        priority: Some(4),
        intent: Some("home/x".to_string()),
        recipients: vec!["home/planner".to_string()],
        holds: vec![Hold {
            stage: "code".to_string(),
            words: "the user reads the design".to_string(),
        }],
        hold_submission: true,
        after: Vec::new(),
        agent_settings: vec![AgentSetting {
            field: "model".to_string(),
            value: "opus".to_string(),
        }],
    };
    offer
}

/// An act `id` on launch `launch`, under lease 1.
pub fn act(id: &str, launch: &str, what: ActSort) -> Want {
    Want::Act(boomdrive_control::Act {
        id: Id::new(id),
        launch: Id::new(launch),
        lease: 1,
        what,
    })
}

/// A preview `id` of `branch` at `revision` in [`KEPT`], marked `done`
/// or not, under lease 1 with no expiry.
pub fn preview(id: &str, branch: &str, revision: &str, done: bool) -> Want {
    Want::Preview(Preview {
        id: Id::new(id),
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: KEPT.to_string(),
        branch: branch.to_string(),
        revision: revision.to_string(),
        done,
    })
}

/// The stub's program. Each call writes its arguments, NUL-terminated, to
/// `call-<n>`, whole once there, so a test that reads the calls while the
/// runner runs never reads one half written. It writes whether
/// `FLUME_RECIPIENT` was set to `recipient-<n>`, and the brief its
/// `--brief-file` names to `brief-<n>`. It then prints the
/// fixture `<what>.stdout` names and exits with `<what>.exit`'s status,
/// each where the test wrote one, `<what>` being its command, or
/// `status-of` for `status` with a place.
const STUB: &str = r#"#!/bin/sh
dir=$(dirname "$0")
n=1
while [ -e "$dir/call-$n" ]; do n=$((n + 1)); done
for arg in "$@"; do printf '%s\0' "$arg"; done > "$dir/call-$n.part"
mv "$dir/call-$n.part" "$dir/call-$n"
if [ "${FLUME_RECIPIENT+set}" = set ]; then echo set; else echo unset; fi > "$dir/recipient-$n"
previous=
for arg in "$@"; do
  if [ "$previous" = --brief-file ]; then cp "$arg" "$dir/brief-$n"; fi
  previous=$arg
done
what=$2
if [ "$what" = status ] && [ $# -gt 2 ]; then what=status-of; fi
if [ -e "$dir/$what.stdout" ]; then cat "$(cat "$dir/$what.stdout")"; fi
if [ -e "$dir/$what.exit" ]; then exit "$(cat "$dir/$what.exit")"; fi
exit 0
"#;

/// A stub `flume` in a temporary directory of its own.
pub struct Stub {
    pub dir: TempDir,
}

impl Stub {
    /// A stub not yet written: its program is not there.
    pub fn absent() -> Stub {
        Stub {
            dir: tempfile::tempdir().expect("a temporary directory"),
        }
    }

    /// A stub answering `status` with `status.json`, and `settle` with
    /// Flume 0.17.1's `settled.json`.
    pub fn new() -> Stub {
        let stub = Stub::absent();
        stub.write();
        stub.answers("status", "status.json", 0);
        stub.answers_from(
            "settle",
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/flume-0.17.1/settled.json"),
            0,
        );
        stub
    }

    /// Writes the stub's program.
    pub fn write(&self) {
        let program = self.program();
        fs::write(&program, STUB).expect("the stub");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("its mode");
    }

    pub fn program(&self) -> PathBuf {
        self.dir.path().join("flume")
    }

    /// `what`, a command or `status-of`, answered with the fixture `name`
    /// and the exit `exit`.
    pub fn answers(&self, what: &str, name: &str, exit: i32) {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/flume-0.4.1")
            .join(name);
        self.answers_from(what, &fixture, exit);
    }

    /// `what`, a command or `status-of`, answered with the file `answer`
    /// and the exit `exit`.
    fn answers_from(&self, what: &str, answer: &Path, exit: i32) {
        let dir = self.dir.path();
        fs::write(
            dir.join(format!("{what}.stdout")),
            answer.to_string_lossy().as_bytes(),
        )
        .expect("the stub's answer");
        fs::write(dir.join(format!("{what}.exit")), exit.to_string()).expect("the stub's exit");
    }

    /// A file of the stub's, as it wrote it.
    pub fn read(&self, name: &str) -> Option<String> {
        fs::read_to_string(self.dir.path().join(name)).ok()
    }

    /// The arguments of every call, in order.
    pub fn calls(&self) -> Vec<Vec<String>> {
        (1..)
            .map_while(|n| self.read(&format!("call-{n}")))
            .map(|call| call.split_terminator('\0').map(str::to_string).collect())
            .collect()
    }

    /// `what`, a command or `status-of`, answered with `text` and the exit
    /// `exit`.
    pub fn answers_text(&self, what: &str, text: &str, exit: i32) {
        let dir = self.dir.path();
        let file = dir.join(format!("{what}.answer"));
        fs::write(&file, text).expect("the stub's answer");
        fs::write(
            dir.join(format!("{what}.stdout")),
            file.to_string_lossy().as_bytes(),
        )
        .expect("the stub's answer");
        fs::write(dir.join(format!("{what}.exit")), exit.to_string()).expect("the stub's exit");
    }

    /// The number of the first call whose command, after `--json`, is
    /// `what`, counting from 1.
    pub fn call_of(&self, what: &str) -> Option<usize> {
        self.calls()
            .iter()
            .position(|call| call.get(1).map(String::as_str) == Some(what))
            .map(|n| n + 1)
    }
}

/// The stub `sluice`'s program. Each call writes its arguments,
/// NUL-terminated, to `call-<n>`, whole once there, and the directory it
/// ran in to `dir-<n>`.
/// Where the test wrote `<what>.wait`, it waits, as sluice's ask of a
/// preview waits at the host's bound, until it is sent SIGINT, then says
/// it stopped on standard error and exits 130 with no document, as sluice
/// does (Task notes 4.3); its wait is a `sleep` it awaits, so its trap
/// runs at once, and which it ends. Otherwise it prints the file
/// `<what>.stdout` names, writes `<what>.stderr` on standard error, and
/// exits with `<what>.exit`'s status, each where the test wrote one,
/// `<what>` being its command.
const SLUICE: &str = r#"#!/bin/sh
dir=$(dirname "$0")
n=1
while [ -e "$dir/call-$n" ]; do n=$((n + 1)); done
for arg in "$@"; do printf '%s\0' "$arg"; done > "$dir/call-$n.part"
mv "$dir/call-$n.part" "$dir/call-$n"
pwd -P > "$dir/dir-$n"
what=$1
if [ -e "$dir/$what.wait" ]; then
  sleep 60 </dev/null >/dev/null 2>&1 &
  sleeper=$!
  trap 'kill "$sleeper" 2>/dev/null; echo "stopped: preview/$3 waits; ask for it again to start it" >&2; exit 130' INT
  echo "waiting: the host is at its bound" >&2
  wait "$sleeper"
fi
if [ -e "$dir/$what.stdout" ]; then cat "$(cat "$dir/$what.stdout")"; fi
if [ -e "$dir/$what.stderr" ]; then cat "$dir/$what.stderr" >&2; fi
if [ -e "$dir/$what.exit" ]; then exit "$(cat "$dir/$what.exit")"; fi
exit 0
"#;

/// A stub `sluice` in a temporary directory of its own. Unscripted, it
/// prints nothing.
pub struct SluiceStub {
    pub dir: TempDir,
}

impl SluiceStub {
    pub fn new() -> SluiceStub {
        let stub = SluiceStub::absent();
        let program = stub.program();
        fs::write(&program, SLUICE).expect("the stub");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("its mode");
        stub
    }

    /// A stub not written: its program is not there.
    pub fn absent() -> SluiceStub {
        SluiceStub {
            dir: tempfile::tempdir().expect("a temporary directory"),
        }
    }

    pub fn program(&self) -> PathBuf {
        self.dir.path().join("sluice")
    }

    /// `what`, a command, answered with sluice's fixture `name` and the
    /// exit `exit`.
    pub fn answers(&self, what: &str, name: &str, exit: i32) {
        self.answers_text(what, &sluice_fixture(name), exit);
    }

    /// `what`, a command, answered with sluice 0.74.3's fixture `name`,
    /// a preview's, and the exit `exit`.
    pub fn answers_preview(&self, what: &str, name: &str, exit: i32) {
        self.answers_text(what, &preview_fixture(name), exit);
    }

    /// `what`, a command, answered with `text` and the exit `exit`.
    pub fn answers_text(&self, what: &str, text: &str, exit: i32) {
        let dir = self.dir.path();
        let file = dir.join(format!("{what}.answer"));
        fs::write(&file, text).expect("the stub's answer");
        fs::write(
            dir.join(format!("{what}.stdout")),
            file.to_string_lossy().as_bytes(),
        )
        .expect("the stub's answer");
        fs::write(dir.join(format!("{what}.exit")), exit.to_string()).expect("the stub's exit");
    }

    /// `what`, a command, waiting until it is interrupted, as sluice's ask
    /// of a preview waits at the host's bound.
    pub fn waits(&self, what: &str) {
        fs::write(self.dir.path().join(format!("{what}.wait")), "").expect("its wait");
    }

    /// `what`, a command, answering at once again.
    pub fn waits_no_more(&self, what: &str) {
        let _ = fs::remove_file(self.dir.path().join(format!("{what}.wait")));
    }

    /// `what`, a command, saying `words` on standard error.
    pub fn says(&self, what: &str, words: &str) {
        fs::write(self.dir.path().join(format!("{what}.stderr")), words).expect("its words");
    }

    /// The arguments of every call, in order.
    pub fn calls(&self) -> Vec<Vec<String>> {
        (1..)
            .map_while(|n| fs::read_to_string(self.dir.path().join(format!("call-{n}"))).ok())
            .map(|call| call.split_terminator('\0').map(str::to_string).collect())
            .collect()
    }

    /// The directory the `n`th call ran in, counting from 1.
    pub fn dir_of(&self, n: usize) -> Option<PathBuf> {
        let said = fs::read_to_string(self.dir.path().join(format!("dir-{n}"))).ok()?;
        Some(PathBuf::from(said.trim_end_matches('\n')))
    }
}

/// The stub `herdr`'s program. Each call writes its `HERDR_SOCKET_PATH`,
/// or `unset`, to `socket-<n>`, then its arguments, NUL-terminated, to
/// `call-<n>`, each whole once there. A `pane read` is counted as
/// `read-<r>`. Where the test wrote `refuse`, it prints that on standard
/// error and exits 1, as herdr refuses; otherwise it prints `screen-<s>`
/// for the greatest `s` up to `r` the test wrote.
const HERDR: &str = r#"#!/bin/sh
dir=$(dirname "$0")
n=1
while [ -e "$dir/call-$n" ]; do n=$((n + 1)); done
printf '%s' "${HERDR_SOCKET_PATH-unset}" > "$dir/socket-$n"
for arg in "$@"; do printf '%s\0' "$arg"; done > "$dir/writing"
mv "$dir/writing" "$dir/call-$n"
[ "$2" = read ] || exit 0
r=1
while [ -e "$dir/read-$r" ]; do r=$((r + 1)); done
: > "$dir/read-$r"
if [ -e "$dir/refuse" ]; then cat "$dir/refuse" >&2; exit 1; fi
s=$r
while [ "$s" -gt 0 ] && [ ! -e "$dir/screen-$s" ]; do s=$((s - 1)); done
if [ "$s" -gt 0 ]; then cat "$dir/screen-$s"; fi
exit 0
"#;

/// A stub `herdr` in a temporary directory of its own. Unscripted, it
/// prints no screen.
pub struct HerdrStub {
    pub dir: TempDir,
}

impl HerdrStub {
    pub fn new() -> HerdrStub {
        let stub = HerdrStub {
            dir: tempfile::tempdir().expect("a temporary directory"),
        };
        let program = stub.program();
        fs::write(&program, HERDR).expect("the stub");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("its mode");
        stub
    }

    pub fn program(&self) -> PathBuf {
        self.dir.path().join("herdr")
    }

    /// The pane's screen from the `read`th read on, until a later one the
    /// test wrote.
    pub fn screen(&self, read: usize, text: &str) {
        fs::write(self.dir.path().join(format!("screen-{read}")), text).expect("the screen");
    }

    /// Every read refused from now on, herdr's `error` answer giving
    /// `message`.
    pub fn refuse(&self, message: &str) {
        let answer = json!({ "id": "cli:pane:read", "error": { "code": "pane_not_found", "message": message } });
        fs::write(self.dir.path().join("refuse"), printed(&answer)).expect("the refusal");
    }

    /// The arguments of every call, in order.
    pub fn calls(&self) -> Vec<Vec<String>> {
        (1..)
            .map_while(|n| fs::read_to_string(self.dir.path().join(format!("call-{n}"))).ok())
            .map(|call| call.split_terminator('\0').map(str::to_string).collect())
            .collect()
    }

    /// The `HERDR_SOCKET_PATH` of every call, in order.
    pub fn sockets(&self) -> Vec<String> {
        (1..=self.calls().len())
            .map(|n| {
                fs::read_to_string(self.dir.path().join(format!("socket-{n}"))).expect("its socket")
            })
            .collect()
    }

    /// How many reads were made.
    pub fn reads(&self) -> usize {
        (1..)
            .take_while(|r| self.dir.path().join(format!("read-{r}")).exists())
            .count()
    }
}

/// The stub `systemd-creds`'s program, which says it is systemd 256's and
/// keeps nothing: each call writes its arguments, NUL-terminated, to
/// `call-<n>`, and every call but `--version` is refused. It stands
/// first on a binary's `PATH` wherever the hosted implementation may look
/// for the host's credential store, so that on Linux no real
/// `systemd-creds` runs, and a test sees that no key was kept (design
/// Decision 14). On macOS the store is the keychain, which the scratch
/// host's own `HOME` holds none of.
const SYSTEMD_CREDS: &str = r#"#!/bin/sh
dir=$(dirname "$0")
n=1
while [ -e "$dir/call-$n" ]; do n=$((n + 1)); done
for arg in "$@"; do printf '%s\0' "$arg"; done > "$dir/call-$n.part"
mv "$dir/call-$n.part" "$dir/call-$n"
if [ "$1" = "--version" ]; then
  printf 'systemd 256 (256-stub)\n'
  exit 0
fi
echo "the stub keeps no credential" >&2
exit 1
"#;

/// A stub `systemd-creds` in a temporary directory of its own.
pub struct CredsStub {
    pub dir: TempDir,
}

impl CredsStub {
    pub fn new() -> CredsStub {
        let stub = CredsStub {
            dir: tempfile::tempdir().expect("a temporary directory"),
        };
        let program = stub.dir.path().join("systemd-creds");
        fs::write(&program, SYSTEMD_CREDS).expect("the stub");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("its mode");
        stub
    }

    /// The arguments of every call, in order.
    pub fn calls(&self) -> Vec<Vec<String>> {
        (1..)
            .map_while(|n| fs::read_to_string(self.dir.path().join(format!("call-{n}"))).ok())
            .map(|call| call.split_terminator('\0').map(str::to_string).collect())
            .collect()
    }

    /// Whether any call but `--version` was made: a key kept or read.
    pub fn kept_or_read(&self) -> bool {
        self.calls()
            .iter()
            .any(|call| call.first().map(String::as_str) != Some("--version"))
    }
}

/// A fetch of one repository held until the test lets it go: an
/// upload-pack of the test's own, set as the scratch git configuration's
/// `remote.origin.uploadpack`, which, for that repository alone, waits
/// while a file the test makes is there, and runs git's own upload-pack
/// for every repository once it is gone.
pub struct HeldFetch {
    dir: TempDir,
}

impl HeldFetch {
    /// Sets the scratch git configuration `gitconfig` to hold the fetches
    /// of the repository `address`, `<owner>/<name>`, while
    /// [`HeldFetch::hold`] holds them.
    pub fn new(gitconfig: &Path, address: &str) -> HeldFetch {
        let held = HeldFetch {
            dir: tempfile::tempdir().expect("the upload-pack's directory"),
        };
        let script = held.dir.path().join("upload-pack");
        fs::write(
            &script,
            format!(
                "#!/bin/sh\n\
                 case \"$*\" in\n\
                 *'/{address}.git'*)\n\
                 \tif [ -e '{hold}' ]; then\n\
                 \t\ttouch '{waiting}'\n\
                 \t\twhile [ -e '{hold}' ]; do sleep 0.05; done\n\
                 \tfi ;;\n\
                 esac\n\
                 exec git upload-pack \"$@\"\n",
                hold = held.hold_file().display(),
                waiting = held.waiting_file().display(),
            ),
        )
        .expect("the upload-pack written");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
            .expect("the upload-pack made runnable");
        let mut config = fs::read_to_string(gitconfig).expect("the git configuration");
        config.push_str(&format!(
            "[remote \"origin\"]\n\tuploadpack = {}\n",
            script.display()
        ));
        fs::write(gitconfig, config).expect("the git configuration written");
        held
    }

    fn hold_file(&self) -> PathBuf {
        self.dir.path().join("hold")
    }

    fn waiting_file(&self) -> PathBuf {
        self.dir.path().join("waiting")
    }

    /// Holds every fetch of the repository from now on.
    pub fn hold(&self) {
        fs::write(self.hold_file(), "").expect("the hold made");
    }

    /// Waits until a fetch of the repository is held; panics where none
    /// is within `patience`.
    pub fn wait_held(&self, patience: std::time::Duration) {
        let deadline = std::time::Instant::now() + patience;
        while !self.waiting_file().exists() {
            assert!(
                std::time::Instant::now() < deadline,
                "no fetch held in time"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    /// Lets every fetch held go on.
    pub fn let_go(&self) {
        let _ = fs::remove_file(self.hold_file());
    }
}

impl Drop for HeldFetch {
    fn drop(&mut self) {
        self.let_go();
    }
}
