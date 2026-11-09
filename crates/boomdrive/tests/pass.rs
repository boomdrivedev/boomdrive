//! One control pass end to end (design Decisions 5, 7, 9, 17 and 18;
//! Task notes 9.3): crew's implementation over a state repository on the
//! scratch forge, a stub `flume` answering as Flume 0.4.1 answered, and
//! the sibling's clones. crew's start of `a` and `b`, `b` after `a`,
//! writes two launch requests of a kit's work naming no host or stream.
//! The first pass claims both, launches `a` and holds `b`; once Flume's
//! status says `a` landed, the next pass launches `b`.
//!
//! Flume's own answers line up: `launch.json` launched `fix--parser`, and
//! `status.json` lists `fix--parser`'s record once it landed and was
//! tidied. Before the launch, Flume lists no such place, so the first
//! pass's listing is `status.json` without it, and the status of the
//! place just launched is its entry in `launch.json`'s answer.

mod common;
mod fakes;
#[path = "../../boomdrive-crew/tests/common/mod.rs"]
mod records;

use boomdrive::carry::{Carrier, Configured, Least, Passed, Reach};
use boomdrive::run::Notices;
use boomdrive::terminal::{self, Relays};
use boomdrive::{clone, config, facts, flume, report, sluice};
use boomdrive_control::ToolVersion;
use boomdrive_crew::config::{Config, Partition, StateRepository};
use boomdrive_crew::Crew;
use common::{keeping, Scratch};
use fakes::{fixture, printed, FakeTools, HerdrStub, SluiceStub, Stub};
use serde_json::{json, Value};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Once};

/// The repository the configuration keeps, and the kit's launches name.
const KEPT: &str = "agentplot/flume";

/// The state repository, on the scratch forge.
const STATE: &str = "example/home-state";

/// The runner's branch, as crew reads it.
const BRANCH: &str = "home/runner/host-a";

/// The claim's words, as crew's implementation writes them.
const CLAIMED: &str = "claimed by host-a's runner; launching once no other runner has claimed it";

static ENVIRONMENT: Once = Once::new();

/// The host: the scratch forge with the kept repository and the state
/// repository on it, the clone made, and the stub tools.
struct Host {
    scratch: Scratch,
    config: config::Config,
    flume: Stub,
    sluice: SluiceStub,
    herdr: HerdrStub,
    tools: FakeTools,
}

impl Host {
    fn new() -> Host {
        // crew's git runs with the process's environment, never the
        // sibling's overlay: no system or global configuration of the
        // user's is read (crew's own tests do the same).
        ENVIRONMENT.call_once(|| {
            std::env::set_var("GIT_CONFIG_NOSYSTEM", "1");
            std::env::set_var("GIT_CONFIG_GLOBAL", "/dev/null");
            std::env::set_var("GIT_TERMINAL_PROMPT", "0");
        });
        let scratch = Scratch::new();
        scratch.origin(KEPT);
        let text = keeping(&[KEPT]);
        let (_, said) = scratch.pass(&text);
        assert!(scratch.clone_of(KEPT).exists(), "{said:#?}");
        let config =
            config::parse(&text, &scratch.config_file(), &scratch.home).expect("a configuration");
        let flume = Stub::absent();
        flume.write();
        let host = Host {
            scratch,
            config,
            flume,
            sluice: SluiceStub::new(),
            herdr: HerdrStub::new(),
            tools: FakeTools::default(),
        };
        // The stub `flume` prints nothing for `--version`, which the hello
        // reads as missing: its launches are tried as before (design
        // Decision 5).
        host.tools.reads(
            "flume",
            ToolVersion::Missing("flume --version printed nothing".to_string()),
        );
        host
    }

    /// crew's start of `a` and `b`, `b` after `a`: two launch requests of
    /// a kit's work in [`KEPT`], naming no host or stream, and the plan
    /// holding both units.
    fn start_a_and_b(&self) -> common::Origin<'_> {
        let state = self.scratch.empty_origin(STATE);
        let request = |n: u64, unit: &str, more: &[(&str, &str)]| {
            let work = format!("unit/{unit}");
            let mut fields = vec![
                ("Base", "bolt/x"),
                ("Intent", "home/x"),
                ("Brief", "Parse the input.\n\nAs words."),
            ];
            fields.extend_from_slice(more);
            records::launch(n, None, &work, KEPT, unit, &fields)
        };
        let requests = [
            request(1, "a", &[("Priority", "3")]),
            request(2, "b", &[("Priority", "4"), ("After", "1")]),
        ];
        state.commit(
            "home/main",
            "requests.rec",
            &records::file(records::REQUESTS, &requests),
        );
        state.commit("home/main", "plan.rec", &records::plan(&["a", "b"], &[]));
        state
    }

    /// crew's implementation on `host-a`, serving `home`, its state
    /// repository reached on the scratch forge, and the lines it says.
    fn crew(&self) -> (Crew, Arc<Mutex<Vec<String>>>) {
        let said = Arc::new(Mutex::new(Vec::new()));
        let lines = Arc::clone(&said);
        let config = Config {
            host: "host-a".to_string(),
            partitions: vec![Partition {
                label: "home".to_string(),
                state_repository: StateRepository::parse(STATE).unwrap(),
            }],
        };
        let crew = Crew::new(
            config,
            &format!("file://{}/", self.scratch.forge.display()),
            &self.scratch.data(),
            Box::new(move |line| lines.lock().unwrap().push(line)),
        );
        (crew, said)
    }

    /// One control pass of `crew`, as the runner makes it: its outcome,
    /// and the lines it said.
    fn pass(&self, crew: &mut Crew) -> (Passed, Vec<String>) {
        let git = self.scratch.runner_git();
        let data = self.scratch.data();
        let clones = Configured::new(
            &self.config,
            clone::find(&git, &data).expect("the clones read"),
        );
        let flume = flume::Cli::new(self.flume.program());
        let sluice = sluice::Cli::new(self.sluice.program());
        let (events, _told) = mpsc::channel();
        let notices = Notices::new(events);
        let control_dir = report::control_dir(&data);
        let relays = Relays::new(terminal::Cli::new(self.herdr.program()), |_| {});
        let reach = Reach::default();
        let least = Least::default();
        let carrier = Carrier {
            flume: &flume,
            sluice: &sluice,
            git: &git,
            clones: &clones,
            tools: &self.tools,
            notices: &notices,
            control_dir: &control_dir,
            relays: &relays,
            reach: &reach,
            least: &least,
            due: &|| false,
        };
        let mut lines = Vec::new();
        let passed = carrier.pass(crew, &mut |line| lines.push(line));
        (passed, lines)
    }

    /// The text of `file` on the runner's branch, `at` its tip or a
    /// commit before it.
    fn branch_file(&self, at: &str, file: &str) -> String {
        let bare = self.scratch.forge.join(format!("{STATE}.git"));
        self.scratch.git(&bare, &["show", &format!("{at}:{file}")])
    }

    /// Each launch the stub was asked, its arguments after `--json launch`.
    fn launches(&self) -> Vec<Vec<String>> {
        self.flume
            .calls()
            .into_iter()
            .filter(|call| call.get(1).map(String::as_str) == Some("launch"))
            .map(|call| call[2..].to_vec())
            .collect()
    }
}

/// The value of `flag` in `args`, where it is given.
fn flag<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    let at = args.iter().position(|arg| arg == flag)?;
    args.get(at + 1).map(String::as_str)
}

/// The records of `text`, a crew file, each as its lines.
fn records_of(text: &str) -> Vec<String> {
    text.split("\n\n")
        .filter(|block| !block.trim_start().starts_with(['#', '%']))
        .map(str::to_string)
        .collect()
}

/// The record of request `n` in `text`, a crew file.
fn request(text: &str, n: u64) -> String {
    records_of(text)
        .into_iter()
        .find(|record| record.lines().any(|line| line == format!("Request: {n}")))
        .unwrap_or_else(|| panic!("no record of request {n} in {text}"))
}

#[test]
fn a_kits_two_launches_claimed_launched_held_then_launched_once_the_first_lands() {
    let host = Host::new();
    host.start_a_and_b();
    let (mut crew, crew_said) = host.crew();

    // Flume's listing before the launch holds no fix--parser.
    let mut before: Value = serde_json::from_str(&fixture("status.json")).unwrap();
    let launched: Value = serde_json::from_str(&fixture("launch.json")).unwrap();
    let place = facts::launched(&launched)
        .and_then(facts::location)
        .expect("launch.json names its place")
        .to_string();
    before["status"]["places"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| !facts::location(entry).is_some_and(|at| at.starts_with(&place)));
    assert!(facts::entry(&before, &place).is_none());
    let status_of_launched = printed(&json!({
        "status": {
            "places": [launched["launched"]["place"]],
            "host": before["status"]["host"],
        }
    }));
    host.flume.answers_text("status", &printed(&before), 0);
    host.flume.answers("launch", "launch.json", 0);
    host.flume.answers_text("status-of", &status_of_launched, 0);

    let (passed, lines) = host.pass(&mut crew);

    assert!(
        passed.reported,
        "{lines:#?}\n{:#?}",
        crew_said.lock().unwrap()
    );
    let launches = host.launches();
    assert_eq!(launches.len(), 1, "{launches:?}");
    assert_eq!(flag(&launches[0], "--name"), Some("a"));
    assert_eq!(
        lines,
        [
            format!("home/1: launched at {place}"),
            format!("home/2: held: waits on home/1 (launched at {place}, not landed)"),
        ]
    );
    // The claim of both, in one write, before either was given.
    let claims = host.branch_file(&format!("{BRANCH}~1"), "answers.rec");
    for n in [1, 2] {
        let claim = request(&claims, n);
        assert!(claim.contains("Outcome: held"), "{claim}");
        assert!(claim.contains(&format!("Words: {CLAIMED}")), "{claim}");
    }
    let answers = host.branch_file(BRANCH, "answers.rec");
    let a = request(&answers, 1);
    assert!(a.contains("Outcome: launched"), "{a}");
    assert!(a.contains(&format!("Place: {place}")), "{a}");
    let b = request(&answers, 2);
    assert!(b.contains("Outcome: held"), "{b}");
    assert!(b.contains("Words: waits on home/1"), "{b}");
    let places = host.branch_file(BRANCH, "places.rec");
    let a_place = request(&places, 1);
    assert!(a_place.contains(&format!("Place: {place}")), "{a_place}");
    assert!(
        a_place.contains(&format!(
            "Status: {}",
            status_of_launched.trim_end_matches('\n')
        )),
        "{a_place}"
    );
    assert!(
        records_of(&places)
            .iter()
            .all(|record| !record.contains("Request: 2")),
        "{places}"
    );

    // Flume's status then lists a's place landed, and tidied with an end
    // pending, as status.json holds it.
    host.flume.answers("status", "status.json", 0);
    host.flume
        .answers("status-of", "status-landed-kept.json", 0);

    let (passed, lines) = host.pass(&mut crew);

    assert!(
        passed.reported,
        "{lines:#?}\n{:#?}",
        crew_said.lock().unwrap()
    );
    let launches = host.launches();
    assert_eq!(launches.len(), 2, "{launches:?}");
    let b = &launches[1];
    assert_eq!(flag(b, "--name"), Some("b"));
    assert_eq!(flag(b, "--kind"), Some("change"));
    assert_eq!(flag(b, "--base"), Some("bolt/x"));
    assert_eq!(flag(b, "--priority"), Some("4"));
    assert_eq!(flag(b, "--intent"), Some("home/x"));
    assert_eq!(flag(b, "--mark"), Some("boomdrive:home/2"));
    for launch in &launches {
        assert_eq!(flag(launch, "--stream"), None, "{launch:?}");
        assert!(!launch.iter().any(|arg| arg == "--kinds"), "{launch:?}");
    }
    // The stub answers every launch as Flume answered fix--parser's, so b
    // too stands at its place.
    assert!(
        lines.contains(&format!("home/2: launched at {place}")),
        "{lines:#?}"
    );
    let answers = host.branch_file(BRANCH, "answers.rec");
    let b = request(&answers, 2);
    assert!(b.contains("Outcome: launched"), "{b}");
}
