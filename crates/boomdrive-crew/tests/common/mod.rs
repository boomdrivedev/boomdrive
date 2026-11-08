//! The scratch state repository every test of crew's implementation runs
//! over (design Decision 21): a bare repository in a temporary directory,
//! its file URL standing in for the state repository, crew's files
//! committed to it as crew writes them, and each runner's branch read
//! back. git runs with no system or global configuration of the user's,
//! and reaches no network.

#![allow(dead_code)]

use boomdrive_control::{Capacity, Hello, HostPart, Pulled, Report, Want, INTERFACE_VERSION};
use boomdrive_crew::config::{Config, Partition, StateRepository};
use boomdrive_crew::Crew;
use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, Once};
use tempfile::TempDir;

/// crew's header of `requests.rec`, as `runner.REQUESTS` holds it at
/// `the-team-machinery-leaves` (`2b97ee57`), where a launch of a kit's
/// work names no `Host`.
pub const REQUESTS: &str = "\
# crew's requests of the runners: a launch of a Flume place for a started
# bolt's work, a fix or a role's work that lands, or a retry, drop or priority
# of a place launched.
# Written only by crew, each in the commit of the plan write that occasions it;
# numbered once, and never changed or removed. The runner whose configuration
# names a launch's Repo takes it, or for a role's the runner on its Host, and
# answers on <label>/runner/<host> of this repository, which crew reads and
# never writes.

%rec: Request
%doc: A request for Work: a launch of a Flume place of Kind named Name, cut from Base of Repo, taken by a runner whose configuration names Repo, in the stream its host maps it to, or by Host's runner in Stream for a role's work; or a retry, drop or priority of the place the launch Of names launched, taken by the runner that took it.
%key: Request
%type: Request,Of,After,Priority int
%type: Act enum launch retry land drop priority
%type: Kind enum change chore fix
%type: Hold enum submission
%type: Submission bool
%type: Host regexp /^[a-z0-9][a-z0-9-]*$/
%type: Date date
%mandatory: Request Act Work Date By
%allowed: Request Act Host Work Repo Kind Name Base Stream Priority Intent Recipient Hold After Brief Of Stage Submission Words Date By
";

/// crew's header of `plan.rec`, as `plan.HEADER` holds it at `2b97ee57`.
pub const PLAN: &str = "\
# The partition's bolts and their units and chores: what is to be built, in
# which bolt, in what order. Nothing here says how far anything has got: `crew
# bolts` reads that from the kits. Written only by `crew bolt`, `crew unit` and
# `crew chore`.
# A Source is a path in the boomdrive's first blueprints repo, or
# <owner>/<name>:<path> in another.

%rec: Bolt
%doc: Work on branch bolt/<Bolt> of Repo, marked Path: flume once crew bolt start has started it, whose Flume places build it.
%key: Bolt
%type: Bolt regexp /^[a-z0-9][a-z0-9-]*$/
%type: Path enum flume
%mandatory: Repo Goal
%allowed: Bolt Repo Goal Source Path
%unique: Repo Goal

%rec: Unit
%doc: One OpenSpec change named Unit, built in a Flume place of its bolt's; or, marked Kind: chore, a chore: planned maintenance with no change, its Intent its words, built in a Flume place of the chore kind.
%key: Unit
%type: Unit regexp /^[a-z0-9][a-z0-9-]*$/
%type: Bolt rec Bolt
%type: Kind enum chore
%mandatory: Repo Intent
%allowed: Unit Repo Bolt Kind Intent After Source Amended
%unique: Repo Bolt Kind Intent
";

/// crew's header of `asks.rec`, as `plan.ASKS` holds it at `9913f249`.
pub const ASKS: &str = "\
# The asks of the partition's main-level roles that run as their kinds, and the turns their runs took them in
# (record 0047 move 6). An ask names a role, whose next run takes it at its start, crew role start, and answers it at
# its end, crew role end. Written only through crew. An ask is never removed, and its number is never reused.

%rec: Ask
%doc: One ask of a role: who or what asks (From), what kind of ask it is (What), the record it names (Names), the key it is written once by (Key), its words and when.
%key: Id
%type: Id int
%type: Role enum design planner reviewer ops operator
%type: What enum tell send capture landed card open batch delivered
%type: At date
%mandatory: Id Role From What At
%allowed: Id Role From What Names Key Words At
%unique: Role From What Names Key Words At
";

/// The `Turn` descriptor `plan.ASKS` holds after its `Ask`'s.
pub const TURNS: &str = "\
%rec: Turn
%doc: A run of a role taking what was pending for it (Took: ask/<id>, place/<request>@<since>, landed/<request>), from its start (Began) to its end (Ended, with what it Said it did), or Superseded by the turn that took its asks again.
%key: Turn
%type: Turn,Superseded int
%type: Role enum design planner reviewer ops operator
%type: Began,Ended date
%mandatory: Turn Role Run Place Host Began
%allowed: Turn Role Run Place Host Took Began Ended Said Superseded
%unique: Role Run Place Host Began Ended Said Superseded
";

/// crew's header of `roles.rec`, as `plan.ROLES` holds it at `9913f249`.
pub const ROLES: &str = "\
# The runs of the partition's main-level roles that run as their kinds: one record for each, the launch the host's
# runner makes while anything is pending for the role (asks.rec). Written only through crew, to match the teams file,
# at crew main up and with an ask whose role's record is missing or differs.

%rec: Role
%doc: A role's run: the place <Kind>--<Name> in Repository on Host, launched from crew's kinds in Stream at Priority for Intent, with Model and Effort.
%key: Role
%type: Role enum design planner reviewer ops operator
%type: Priority int
%type: Effort enum low medium high xhigh max
%mandatory: Role Address Run Host Repository Kinds Kind Name Stream Priority Intent Model Effort
%allowed: Role Address Run Host Repository Kinds Kind Name Stream Priority Intent Model Effort
%unique: Role Address Run Host Repository Kinds Kind Name Stream Priority Intent Model Effort
";

/// One record's text: a line per field, a value's later lines as `+ `
/// continuation lines.
pub fn rec(fields: &[(&str, &str)]) -> String {
    let mut text = String::new();
    for (name, value) in fields {
        for (n, line) in value.split('\n').enumerate() {
            match n {
                0 => text.push_str(&format!("{name}: {line}\n")),
                _ => text.push_str(&format!("+ {line}\n")),
            }
        }
    }
    text
}

/// A file under `header`: the header, then each record after a blank
/// line.
pub fn file(header: &str, records: &[String]) -> String {
    let mut text = header.to_string();
    for record in records {
        text.push('\n');
        text.push_str(record);
    }
    text
}

/// A launch request as crew writes one, numbered `n`, of `work` in
/// `repo`, named `name`, for `host` where one is named; `more` holds the
/// fields beside them, put in crew's order.
pub fn launch(
    n: u64,
    host: Option<&str>,
    work: &str,
    repo: &str,
    name: &str,
    more: &[(&str, &str)],
) -> String {
    let number = n.to_string();
    let mut fields: Vec<(&str, &str)> = vec![("Request", &number), ("Act", "launch")];
    if let Some(host) = host {
        fields.push(("Host", host));
    }
    fields.extend([
        ("Work", work),
        ("Repo", repo),
        ("Kind", "change"),
        ("Name", name),
        ("Base", "bolt/b"),
    ]);
    order(&mut fields, more);
    rec(&fields)
}

/// An act request as crew writes one: a `retry`, `drop` or `priority` of
/// launch `of`, serving `work`, for `host` where one is named.
pub fn act(
    n: u64,
    act: &str,
    host: Option<&str>,
    work: &str,
    of: u64,
    more: &[(&str, &str)],
) -> String {
    let number = n.to_string();
    let of = of.to_string();
    let mut fields: Vec<(&str, &str)> = vec![("Request", &number), ("Act", act)];
    if let Some(host) = host {
        fields.push(("Host", host));
    }
    fields.extend([("Work", work), ("Of", &of)]);
    order(&mut fields, more);
    rec(&fields)
}

/// The fields of crew's `runner.FIELDS`, in its order.
const FIELDS: [&str; 21] = [
    "Request",
    "Act",
    "Host",
    "Work",
    "Repo",
    "Kind",
    "Name",
    "Base",
    "Stream",
    "Priority",
    "Intent",
    "Recipient",
    "Hold",
    "After",
    "Brief",
    "Of",
    "Stage",
    "Submission",
    "Words",
    "Date",
    "By",
];

/// `fields` with `more` and the date and author every request carries,
/// in crew's order; a field of `more` replaces one of `fields`.
fn order<'a>(fields: &mut Vec<(&'a str, &'a str)>, more: &[(&'a str, &'a str)]) {
    fields.retain(|(name, _)| !more.iter().any(|(other, _)| other == name));
    fields.extend(more.iter().copied());
    if !fields.iter().any(|(name, _)| *name == "Date") {
        fields.push(("Date", "2026-11-01T09:00:00Z"));
    }
    if !fields.iter().any(|(name, _)| *name == "By") {
        fields.push(("By", "crew@host-a"));
    }
    let rank = |name: &str| {
        FIELDS
            .iter()
            .position(|f| *f == name)
            .unwrap_or(FIELDS.len())
    };
    fields.sort_by_key(|(name, _)| rank(name));
}

/// `plan.rec` holding the units `units` and the bolts `bolts`, each after
/// its own descriptor, as crew writes it.
pub fn plan(units: &[&str], bolts: &[&str]) -> String {
    let (head, unit) = PLAN.split_at(PLAN.find("%rec: Unit").expect("a Unit descriptor"));
    let bolts: Vec<String> = bolts
        .iter()
        .map(|bolt| {
            rec(&[
                ("Bolt", bolt),
                ("Repo", "example/crew"),
                ("Goal", &format!("{bolt}'s goal")),
            ])
        })
        .collect();
    let units: Vec<String> = units
        .iter()
        .map(|name| {
            rec(&[
                ("Unit", name),
                ("Repo", "example/crew"),
                ("Intent", &format!("{name}'s intent")),
            ])
        })
        .collect();
    let head = format!("{}\n", head.trim_end_matches('\n'));
    format!("{}\n{}", file(&head, &bolts), file(unit, &units))
}

/// The scratch state repository `example/home-state`, removed
/// when dropped.
pub struct State {
    _dir: TempDir,
    pub root: PathBuf,
    /// The forge, `<owner>/<name>.git` for each repository on it.
    pub forge: PathBuf,
}

static ENVIRONMENT: Once = Once::new();

impl State {
    pub fn new() -> State {
        // git runs, here and in the implementation, with no system or
        // global configuration: the variables are the process's, set once
        // before any test runs git.
        ENVIRONMENT.call_once(|| {
            std::env::set_var("GIT_CONFIG_NOSYSTEM", "1");
            std::env::set_var("GIT_CONFIG_GLOBAL", "/dev/null");
            std::env::set_var("GIT_TERMINAL_PROMPT", "0");
        });
        let dir = tempfile::Builder::new()
            .prefix("boomdrive-crew-")
            .tempdir()
            .expect("a scratch directory");
        let root = dir.path().canonicalize().expect("its path");
        let forge = root.join("forge");
        let state = State {
            _dir: dir,
            root,
            forge,
        };
        state.make("example/home-state");
        state
    }

    /// Makes the bare repository `address` on the forge.
    pub fn make(&self, address: &str) -> PathBuf {
        let bare = self.forge.join(format!("{address}.git"));
        std::fs::create_dir_all(&bare).expect("the repository's folder");
        git(&bare, &["init", "--quiet", "--bare"], None);
        bare
    }

    /// The bare repository `example/home-state`.
    pub fn bare(&self) -> PathBuf {
        self.forge.join("example/home-state.git")
    }

    /// The forge's base address, as crew's implementation is handed it.
    pub fn url(&self) -> String {
        format!("file://{}/", self.forge.display())
    }

    /// crew's implementation on `host`, serving `partitions` (label and
    /// state repository), its data under the scratch directory, and the
    /// lines it says.
    pub fn crew_serving(&self, host: &str, partitions: &[(&str, &str)]) -> (Crew, Said) {
        let said = Said::default();
        let lines = said.0.clone();
        let config = Config {
            host: host.to_string(),
            partitions: partitions
                .iter()
                .map(|(label, repository)| Partition {
                    label: label.to_string(),
                    state_repository: StateRepository::parse(repository).unwrap(),
                })
                .collect(),
        };
        let crew = Crew::new(
            config,
            &self.url(),
            &self.root.join(format!("data-{host}")),
            Box::new(move |line| lines.lock().unwrap().push(line)),
        );
        (crew, said)
    }

    /// crew's implementation on `host`, serving `home`.
    pub fn crew(&self, host: &str) -> (Crew, Said) {
        self.crew_serving(host, &[("home", "example/home-state")])
    }

    /// Commits `files` onto `branch` of the state repository as one
    /// commit on its tip, keeping the tip's other files: crew's write, or
    /// another host's runner's.
    pub fn commit(&self, branch: &str, files: &[(&str, &str)]) -> String {
        commit_to(&self.bare(), branch, files)
    }

    /// `home/main` holding `files`.
    pub fn main(&self, files: &[(&str, &str)]) -> String {
        self.commit("home/main", files)
    }

    /// The commit `branch` stands at, none where it does not exist.
    pub fn tip(&self, branch: &str) -> Option<String> {
        tip_of(&self.bare(), branch)
    }

    /// The text of `path` at `branch`'s tip.
    pub fn file(&self, branch: &str, path: &str) -> Option<String> {
        let tip = self.tip(branch)?;
        let out = Command::new("git")
            .arg("--git-dir")
            .arg(self.bare())
            .args(["show", &format!("{tip}:{path}")])
            .output()
            .expect("git runs");
        out.status
            .success()
            .then(|| String::from_utf8(out.stdout).expect("UTF-8"))
    }

    /// Every ref of the state repository with its commit.
    pub fn refs(&self) -> String {
        git(
            &self.bare(),
            &["for-each-ref", "--format=%(objectname) %(refname)"],
            None,
        )
    }

    /// How many commits `branch` holds.
    pub fn commits(&self, branch: &str) -> usize {
        git(&self.bare(), &["rev-list", "--count", branch], None)
            .trim()
            .parse()
            .expect("a count")
    }
}

/// git at the bare repository `bare`, `input` on its standard input, its
/// standard output; panics with its words where it fails.
pub fn git(bare: &Path, args: &[&str], input: Option<&str>) -> String {
    let mut child = Command::new("git")
        .arg("--git-dir")
        .arg(bare)
        .args(args)
        .env("GIT_AUTHOR_NAME", "crew")
        .env("GIT_AUTHOR_EMAIL", "crew@host-a.invalid")
        .env("GIT_COMMITTER_NAME", "crew")
        .env("GIT_COMMITTER_EMAIL", "crew@host-a.invalid")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("git runs");
    if let Some(input) = input {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
    }
    drop(child.stdin.take());
    let out = child.wait_with_output().expect("git ends");
    assert!(
        out.status.success(),
        "git {}: {}",
        args.join(" "),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("UTF-8")
}

/// The commit `branch` of `bare` stands at.
pub fn tip_of(bare: &Path, branch: &str) -> Option<String> {
    let out = Command::new("git")
        .arg("--git-dir")
        .arg(bare)
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .output()
        .expect("git runs");
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Commits `files` onto `branch` of `bare`, keeping its tip's other files,
/// by git's plumbing: crew's own writes are checked by crew, so a test
/// may write what crew's rules would refuse.
pub fn commit_to(bare: &Path, branch: &str, files: &[(&str, &str)]) -> String {
    let parent = tip_of(bare, branch);
    let mut entries: BTreeMap<String, String> = BTreeMap::new();
    if let Some(parent) = &parent {
        for line in git(bare, &["ls-tree", parent], None).lines() {
            let (head, path) = line.split_once('\t').expect("an entry");
            entries.insert(path.to_string(), head.to_string());
        }
    }
    for (path, text) in files {
        let blob = git(bare, &["hash-object", "-w", "--stdin"], Some(text));
        entries.insert(path.to_string(), format!("100644 blob {}", blob.trim()));
    }
    let listing: String = entries
        .iter()
        .map(|(path, head)| format!("{head}\t{path}\n"))
        .collect();
    let tree = git(bare, &["mktree"], Some(&listing));
    let mut args = vec!["commit-tree", tree.trim(), "-m", "crew's write"];
    if let Some(parent) = &parent {
        args.extend(["-p", parent]);
    }
    let commit = git(bare, &args, None).trim().to_string();
    git(
        bare,
        &["update-ref", &format!("refs/heads/{branch}"), &commit],
        None,
    );
    commit
}

/// The lines crew's implementation said.
#[derive(Clone, Default)]
pub struct Said(pub Arc<Mutex<Vec<String>>>);

impl Said {
    pub fn lines(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }

    /// Whether a line holds every one of `words`.
    pub fn holds(&self, words: &[&str]) -> bool {
        self.lines()
            .iter()
            .any(|line| words.iter().all(|word| line.contains(word)))
    }
}

/// A hello keeping `repositories`, the host's room as Flume printed it.
pub fn hello(repositories: &[&str], room: &str) -> Hello {
    Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: "boomdrive 0.0.0".to_string(),
        tool_versions: BTreeMap::new(),
        repositories: repositories.iter().map(|r| r.to_string()).collect(),
        draining: vec![],
        capacity: Capacity::Printed(HostPart {
            running: "0".to_string(),
            bound: "null".to_string(),
            room: room.to_string(),
            streams: "[]".to_string(),
        }),
        leases: BTreeMap::new(),
    }
}

/// The ids of a pull's wants, in order.
pub fn ids(pulled: &Pulled) -> Vec<String> {
    pulled.wants.iter().map(|w| w.id().to_string()).collect()
}

/// The want of `pulled` whose id is `id`.
pub fn want<'a>(pulled: &'a Pulled, id: &str) -> &'a Want {
    pulled
        .wants
        .iter()
        .find(|w| w.id().as_str() == id)
        .unwrap_or_else(|| panic!("no want {id} in {:?}", ids(pulled)))
}

/// A pull of `crew` with `hello` and an empty report.
pub fn pull(crew: &mut Crew, hello: &Hello) -> Pulled {
    use boomdrive_control::ControlPlane;
    crew.pull(hello, &Report::default()).expect("a pull")
}

/// Flume's own answer to `flume --json status <place>` of a change in
/// code, as 0.4.1 printed it, its last newline left off as the runner
/// carries it.
pub fn status_code() -> String {
    include_str!("../../../boomdrive/tests/fixtures/flume-0.4.1/status-code.json")
        .trim_end_matches('\n')
        .to_string()
}

/// Flume's own answer of a role's run whose one stage has delivered.
pub fn status_delivered() -> String {
    include_str!("../../../boomdrive/tests/fixtures/flume-0.4.1/status-delivered.json")
        .trim_end_matches('\n')
        .to_string()
}

/// `role`'s record of partition `home` on `host`, as crew's
/// `asks.role_record` writes it.
pub fn role(role: &str, host: &str) -> String {
    rec(&[
        ("Role", role),
        ("Address", &format!("home/{role}")),
        ("Run", &format!("home-{role}")),
        ("Host", host),
        ("Repository", "example/blueprints"),
        ("Kinds", "crew"),
        ("Kind", &format!("crew-{role}")),
        ("Name", "home"),
        ("Stream", "home-main"),
        ("Priority", "0"),
        ("Intent", &format!("home-{role}")),
        ("Model", "claude-fable-5-1"),
        ("Effort", "xhigh"),
    ])
}
