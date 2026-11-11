//! The quick start's steps (record 0043; site.8; design Decisions 5 and 6
//! of the change boomdrives-quick-start-takes-a-stranger-to-a-working-drive):
//! each step's file is read by its path under `site/src/start/`, beside
//! the crate's manifest, as `readme.rs` reads README, and run as it stands
//! through `sh`, never a copy of its text. A step whose run would reach
//! only a stand-in, a sign-in, the state repository or the start, is held
//! to what setup names for it, setup run over `common::stand`'s stand-ins.
//! The family's tools are the runner cargo built for the test and the
//! set's Flume and sluice by the paths the devenv shell gives (core.45),
//! never stubs of them; the agent is the stand-in for Claude Code built
//! under the feature `stand-in` (design Decision 6). Every test runs on a
//! `Scratch` home, and nothing reaches the network. A test that runs
//! setup is macOS's, where setup runs and the gate runs.

mod common;
#[cfg(target_os = "macos")]
#[path = "../../boomdrive-crew/tests/common/mod.rs"]
mod records;

#[cfg(target_os = "macos")]
use common::stand::{refused, Stand};
use common::Scratch;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The quick start's answers to setup, by its placeholders (design
/// Decision 3): the account `me`'s drive `home`, keeping
/// `your-github-name/your-repository`, its state repository
/// `your-github-name/drive-state`.
#[cfg(target_os = "macos")]
const ANSWERS: [&str; 8] = [
    "--account",
    "me",
    "--drive",
    "home",
    "--state-repository",
    "your-github-name/drive-state",
    "--repository",
    "your-github-name/your-repository",
];

/// The step's file at `path` under `site/src/start/`.
fn step_file(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../site/src/start")
        .join(path)
}

/// The text of the step's file at `path`.
fn step(path: &str) -> String {
    let file = step_file(path);
    std::fs::read_to_string(&file)
        .unwrap_or_else(|error| panic!("the step's file {}: {error}", file.display()))
}

/// The lines of the step's file at `path`.
fn lines(path: &str) -> Vec<String> {
    step(path).lines().map(str::to_string).collect()
}

/// The set's program of `tool`, `flume` or `sluice`, by the path the
/// devenv shell gives it, as `tests/family.rs` reads it; where the shell
/// gives none, the test fails, saying so.
fn set_program(tool: &str) -> PathBuf {
    let variable = format!("BOOMDRIVE_SET_{}", tool.to_uppercase());
    match std::env::var_os(&variable) {
        Some(program) if !program.is_empty() => PathBuf::from(program),
        _ => panic!(
            "{variable} is not set: the devenv shell gives the set's {tool} by its path \
             (devenv.nix); run the tests in it, `devenv shell -- moon run boomdrive:test`"
        ),
    }
}

/// `program` on the scratch host alone: none of the tests' own
/// environment but the scratch one, its state, cache and `TMPDIR` the
/// scratch host's, and `path` its `PATH`.
fn on_scratch(scratch: &Scratch, program: impl AsRef<std::ffi::OsStr>, path: &str) -> Command {
    let tmp = scratch.root.join("tmp");
    std::fs::create_dir_all(&tmp).expect("the scratch TMPDIR");
    let mut command = Command::new(program);
    command
        .env_clear()
        .envs(scratch.overlay())
        .env("XDG_STATE_HOME", scratch.home.join(".local/state"))
        .env("XDG_CACHE_HOME", scratch.home.join(".cache"))
        .env("TMPDIR", tmp)
        .env("PATH", path)
        .current_dir(&scratch.home);
    command
}

/// What a run said, for a failing assertion.
fn said(output: &Output) -> String {
    format!(
        "exit: {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Writes the step's file at `path` to `to`, its folders made.
fn write_step(path: &str, to: &Path) {
    write_text(&step(path), to);
}

/// Writes `text` to `to`, its folders made.
fn write_text(text: &str, to: &Path) {
    std::fs::create_dir_all(to.parent().expect("the file's folder")).expect("its folder");
    std::fs::write(to, text).expect("the file written");
}

/// The quick start's answers to setup, then `extra`.
#[cfg(target_os = "macos")]
fn answers(extra: &[&'static str]) -> Vec<&'static str> {
    ANSWERS.iter().chain(extra).copied().collect()
}

/// The quick-start spec's "Each sign-in as setup names it": GitHub signed
/// out gives setup's refusal, which names its fix; GitHub signed in and
/// Claude Code and Pending You signed out give the answer's `missing`,
/// each with its fix. The sign-in step is those three, in that order.
#[cfg(target_os = "macos")]
#[test]
fn the_sign_ins_are_the_ones_setup_names() {
    let stand = Stand::new();
    stand.answer(
        "gh",
        Some("auth-status"),
        "",
        "You are not logged into any GitHub hosts.\n",
        1,
    );
    let reason = refused(&stand.setup(&answers(&["--json"])), 2);
    let github = reason
        .split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("setup's refusal names no fix: {reason}"))
        .to_string();

    let stand = Stand::new();
    stand.answer("claude", None, "{\"loggedIn\":false}\n", "", 1);
    stand.answer(
        "pendingyou",
        None,
        "Pending You for Claude Code\n  missing Sign-in: none\n",
        "",
        0,
    );
    let answer = stand.set_up(&answers(&["--json"]));
    let missing = answer["missing"]
        .as_array()
        .unwrap_or_else(|| panic!("no missing: {answer}"));
    let fixes: Vec<String> = missing
        .iter()
        .map(|missing| missing["fix"].as_str().expect("a fix").to_string())
        .collect();
    assert_eq!(fixes.len(), 2, "{answer}");

    let mut want = vec![github];
    want.extend(fixes);
    assert_eq!(lines("both/sign-in.sh"), want);
}

/// The Mac's Flume step's files and where the reader writes each, under
/// their home (design Decision 4).
const MAC_FLUME: [(&str, &str); 2] = [
    ("mac/flume.toml", ".config/flume/config.toml"),
    ("mac/crew-source.toml", ".config/flume/sources/crew.toml"),
];

/// crew's roles file for the drive `home` on the host `host-a`, as crew's
/// follower writes it from the teams file setup gives (crew record 0057;
/// crew's `source.text`): a standing agent's entry for each of the
/// partition's five roles, at `home/<role>`, the operator its stream's
/// answerer. No crew runs in the gate and crew has no command that writes
/// the file alone, so the tests write it in crew's stead.
const CREWS_ROLES: &str = r#"# crew's roles for Flume on host-a, written whole from crew's teams file by crew's follower and each start
# of a main-level role, and renamed over the old one: an edit by hand is lost at the next write. Each role
# that stands, of each partition whose main level runs on this host, at <label>/<role> (crew record 0057).

[roles."home/design"]
agent = "home-design"
session = "home"
answers = "the design"

[roles."home/planner"]
agent = "home-planner"
session = "home"
answers = "the plan"

[roles."home/reviewer"]
agent = "home-reviewer"
session = "home"

[roles."home/ops"]
agent = "home-ops"
session = "home"
answers = "a live system, a deploy or a landing"

[roles."home/operator"]
agent = "home-operator"
session = "home"
answerer = true
"#;

/// The Mac's Flume step's files written as the scratch home's, where the
/// reader writes them, and crew's roles file and kinds folder made in
/// crew's stead, where crew keeps them.
fn flumes_files(scratch: &Scratch) {
    for (path, writes) in MAC_FLUME {
        write_step(path, &scratch.home.join(writes));
    }
    write_text(
        CREWS_ROLES,
        &scratch.home.join(".local/state/crew/flume/roles.toml"),
    );
    std::fs::create_dir_all(scratch.home.join(".local/share/flume/kinds/crew"))
        .expect("crew's kinds folder");
}

/// The quick-start spec's "The set's Flume takes the file": the Mac's
/// Flume step's files written as the scratch home's, at Flume's own paths,
/// beside crew's roles file as crew writes it, and the set's Flume's
/// `--json status` there answers with no refusal. Its default stream and
/// the drive's stream, `home`, each give the multiplexer `herdr` and the
/// terminal app `terminal` from the host's keys; the source `crew` names
/// crew's kinds folder and roles file under the home, and reads from it
/// the five roles of the drive's stream, which a stream the host did not
/// declare would leave undeclared.
#[test]
fn flumes_file_opens_agents_in_herdr() {
    let scratch = Scratch::new();
    flumes_files(&scratch);
    let output = on_scratch(&scratch, set_program("flume"), "/usr/bin:/bin")
        .args(["--json", "status"])
        .output()
        .expect("the set's Flume runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let answer: Value = serde_json::from_slice(&output.stdout).expect("Flume's status is JSON");
    assert!(answer.get("refused").is_none(), "{answer}");
    let host = &answer["status"]["host"];
    for name in ["default", "home"] {
        let stream = host["streams"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|stream| stream["name"] == name)
            .unwrap_or_else(|| panic!("no stream {name} in Flume's status: {answer}"));
        assert_eq!(stream["refused"], Value::Null, "{stream}");
        for (part, profile) in [("multiplexer", "herdr"), ("terminal_app", "terminal")] {
            assert_eq!(stream[part]["profile"], profile, "{stream}");
            assert_eq!(stream[part]["from"], "host", "{stream}");
        }
    }
    let sources = host["sources"]
        .as_array()
        .unwrap_or_else(|| panic!("no sources in Flume's status: {answer}"));
    assert_eq!(sources.len(), 1, "{answer}");
    let crew = &sources[0];
    let home = |path: &str| Value::from(scratch.home.join(path).to_string_lossy().into_owned());
    assert_eq!(crew["name"], "crew", "{crew}");
    assert_eq!(
        crew["kinds_dir"],
        home(".local/share/flume/kinds/crew"),
        "{crew}"
    );
    assert_eq!(
        crew["roles_file"],
        home(".local/state/crew/flume/roles.toml"),
        "{crew}"
    );
    let mut roles: Vec<&str> = crew["roles"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    roles.sort_unstable();
    assert_eq!(
        roles,
        [
            "home/design",
            "home/operator",
            "home/ops",
            "home/planner",
            "home/reviewer"
        ],
        "{crew}"
    );
}

/// The step's shell file at `path`, run as `sh <file>` on the stand's
/// scratch home: its `PATH` a folder linking `boomdrive` to the binary
/// cargo built for the test, then the stand-ins, then the test's own.
#[cfg(target_os = "macos")]
fn run_on_stand(stand: &Stand, path: &str) -> Output {
    let runner = stand.scratch.root.join("runner");
    std::fs::create_dir_all(&runner).expect("the runner's folder");
    let link = runner.join("boomdrive");
    if !link.exists() {
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_boomdrive"), &link)
            .expect("the runner linked");
    }
    let mut search = runner.into_os_string();
    search.push(":");
    search.push(stand.folder());
    if let Some(own) = std::env::var_os("PATH") {
        search.push(":");
        search.push(own);
    }
    stand
        .scratch
        .command("sh")
        .arg(step_file(path))
        .env("PATH", search)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("sh runs")
}

/// The quick-start spec's "Setup as written": the Mac's setup step, run
/// through `sh` with GitHub signed in and no state repository, exits 0,
/// and the runner's file it wrote, read as the runner reads it, names the
/// account `me`'s drive `home` keeping `your-github-name/your-repository`,
/// and crew's table the partition `home` with
/// `your-github-name/drive-state`.
#[cfg(target_os = "macos")]
#[test]
fn setup_sets_up_the_drive_as_written() {
    let stand = Stand::new();
    let output = run_on_stand(&stand, "mac/setup.sh");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));

    let config = boomdrive::config::read(&stand.config(), stand.home())
        .unwrap_or_else(|error| panic!("the runner's file: {error}"));
    let drives: Vec<(String, String, Vec<String>)> = config
        .accounts
        .iter()
        .flat_map(|account| {
            account.drives.iter().map(|drive| {
                (
                    account.name.clone(),
                    drive.name.clone(),
                    drive.repositories.iter().map(ToString::to_string).collect(),
                )
            })
        })
        .collect();
    assert_eq!(
        drives,
        [(
            "me".to_string(),
            "home".to_string(),
            vec!["your-github-name/your-repository".to_string()]
        )]
    );

    let table = config.control_plane.expect("a control plane");
    assert_eq!(table.implementation, "crew");
    let crew = boomdrive_crew::config::read(&table.implementation_table, &stand.config())
        .unwrap_or_else(|error| panic!("crew's table: {error}"));
    let partitions: Vec<(String, String)> = crew
        .partitions
        .iter()
        .map(|partition| {
            let state = &partition.state_repository;
            (
                partition.label.clone(),
                format!("{}/{}", state.owner, state.name),
            )
        })
        .collect();
    assert_eq!(
        partitions,
        [(
            "home".to_string(),
            "your-github-name/drive-state".to_string()
        )]
    );
}

/// The quick-start spec's "Made as setup makes it": setup, over the
/// stand-ins where gh finds no `your-github-name/drive-state`, makes it
/// by one `gh repo create`, and NixOS's state step is that call, word for
/// word.
#[cfg(target_os = "macos")]
#[test]
fn the_state_repository_is_made_as_setup_makes_it() {
    let stand = Stand::new();
    stand.set_up(&answers(&["--json"]));
    let made: Vec<String> = stand
        .calls("gh")
        .into_iter()
        .filter(|call| call.starts_with("repo create "))
        .map(|call| format!("gh {call}"))
        .collect();
    assert_eq!(made.len(), 1, "{:?}", stand.calls("gh"));
    assert_eq!(lines("nixos/state.sh"), made);
}

/// The scratch host's `PATH` for a step's line and the set's tools: the
/// tests' own, whose git is the gate's.
fn own_path() -> String {
    std::env::var("PATH").unwrap_or_default()
}

/// The landing step's files and where the reader writes each, in its
/// clone of the drive's repository (design Decision 4).
const LANDING: [(&str, &str); 2] = [
    ("both/sluice.toml", ".config/sluice.toml"),
    ("both/flume.toml", ".config/flume.toml"),
];

/// The scratch host's `PATH` with the set's `tools` first, then the
/// tests' own.
fn set_path(tools: &[&str]) -> String {
    let mut folders: Vec<String> = tools
        .iter()
        .map(|tool| {
            set_program(tool)
                .parent()
                .expect("the program's folder")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    folders.push(own_path());
    folders.join(":")
}

/// `your-github-name/your-repository` on the scratch forge, its `main` as
/// the landing step leaves it: the step's files written in a clone of it
/// where the reader writes them, and its `land.sh` run there through `sh`.
fn landed(scratch: &Scratch) -> common::Origin<'_> {
    let origin = scratch.origin("your-github-name/your-repository");
    let first = scratch.root.join("clones/first");
    std::fs::create_dir_all(first.parent().expect("the clones' folder")).expect("the folder");
    scratch.git(
        &scratch.root,
        &["clone", "--quiet", &origin.url(), &first.to_string_lossy()],
    );
    for (path, writes) in LANDING {
        write_step(path, &first.join(writes));
    }
    let output = on_scratch(scratch, "sh", &own_path())
        .arg(step_file("both/land.sh"))
        .current_dir(&first)
        .output()
        .expect("sh runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    origin
}

/// The quick-start spec's "The policy on main": the landing step's
/// policy and Flume's binding written as `.config/sluice.toml` and
/// `.config/flume.toml` in a scratch clone of a bare origin, its
/// `land.sh` run there through `sh`, and origin's `main` holds both. In a
/// second clone the set's sluice reads `main`'s policy, as `sluice equip
/// main` reads it at the branch's revision, and refuses nothing; and the
/// set's Flume reads the binding, which it reads at the default branch
/// alone: a fix launched there, its stage held so no agent starts, is not
/// refused, lands, and takes its base from the merge queue.
#[test]
fn sluices_file_lands_on_main() {
    let scratch = Scratch::new();
    let origin = landed(&scratch);
    for (path, writes) in LANDING {
        assert_eq!(
            scratch.git(&origin.bare, &["show", &format!("main:{writes}")]) + "\n",
            step(path)
        );
    }

    let second = scratch.root.join("clones/second");
    scratch.git(
        &scratch.root,
        &["clone", "--quiet", &origin.url(), &second.to_string_lossy()],
    );
    let output = on_scratch(&scratch, set_program("sluice"), &own_path())
        .args(["equip", "main", "--json"])
        .current_dir(&second)
        .output()
        .expect("the set's sluice runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let answer: Value = serde_json::from_slice(&output.stdout).expect("sluice's answer is JSON");
    assert_eq!(answer["branch"], "main", "{answer}");
    assert_eq!(answer["revision"], origin.tip("main"), "{answer}");
    assert!(answer.get("refused").is_none(), "{answer}");

    let output = on_scratch(&scratch, set_program("flume"), &set_path(&["sluice"]))
        .args([
            "--json",
            "launch",
            "--kind",
            "fix",
            "--name",
            "binding",
            "--brief",
            "Read the binding.",
            "--recipient",
            "reader",
            "--hold",
            "code=the test reads the binding",
        ])
        .current_dir(&second)
        .output()
        .expect("the set's Flume runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let answer: Value = serde_json::from_slice(&output.stdout).expect("Flume's answer is JSON");
    let place = &answer["launched"]["place"];
    assert_eq!(place["lands"], true, "{answer}");
    assert_eq!(place["base_from"], "merge-queue", "{answer}");
}

/// The quick-start spec's "Started as setup says": setup, with `--json`
/// over the stand-ins for the drive `home`, answers the line that starts
/// the drive's agents as `start`, and the start step is that one line.
#[cfg(target_os = "macos")]
#[test]
fn the_drives_agents_start_as_setup_says() {
    let stand = Stand::new();
    let answer = stand.set_up(&answers(&["--json"]));
    let start = answer["control_plane"]["start"]
        .as_str()
        .unwrap_or_else(|| panic!("setup's answer gives no start: {answer}"));
    assert_eq!(lines("both/start.sh"), [start]);
}

/// The stand-in for Claude Code, which cargo builds under the feature
/// `stand-in` (design Decision 6).
#[cfg(feature = "stand-in")]
fn stand_in() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_boomdrive-stand-in"))
}

/// Without the feature `stand-in` there is no stand-in agent, and the
/// test that needs one fails, saying so.
#[cfg(not(feature = "stand-in"))]
fn stand_in() -> PathBuf {
    panic!(
        "the stand-in for Claude Code is built under the feature stand-in: \
         `cargo test -p boomdrive --features stand-in`, as `moon run boomdrive:test` runs it"
    )
}

/// The scratch host's local layer of Flume's file for the last step, over
/// the Mac's Flume step's files as they stand: the drive's stream prints
/// its agents' commands, where the gate runs no multiplexer's server
/// (site.8).
const PRINTED: &str = "[streams.home]\nmultiplexer = \"printed\"\n";

/// How long the last step's test waits for the change to land.
const LANDING_WITHIN: std::time::Duration = std::time::Duration::from_secs(120);

/// The fix crew writes for the drive `home`, of
/// `your-github-name/your-repository`'s main, as `crew main fix`
/// appends it: request 1, for the stand-ins' host `host-a`, its
/// submission held for the reader's send.
#[cfg(target_os = "macos")]
fn fix_of_main() -> String {
    records::launch(
        1,
        Some("host-a"),
        "fix/main/your-repository/one",
        "your-github-name/your-repository",
        "main--one",
        &[
            ("Kind", "fix"),
            ("Base", "main"),
            ("Priority", "1"),
            ("Intent", "home/main/your-repository"),
            ("Recipient", "home/ops"),
            ("Hold", "submission"),
            ("Brief", "Make the stand-in's change."),
        ],
    )
}

/// The reader's send of the fix's held submission, as crew appends it on
/// their word: request 2, a retry of launch 1 naming its submission.
#[cfg(target_os = "macos")]
fn send_of_fix() -> String {
    records::act(
        2,
        "retry",
        Some("host-a"),
        "fix/main/your-repository/one",
        1,
        &[("Submission", "yes")],
    )
}

/// The place's status that request `request`'s `Status` on the runner's
/// branch of the state repository `state` carries, as Flume's
/// `--json status <place>` gives it, where the branch holds one.
#[cfg(target_os = "macos")]
fn carried(scratch: &Scratch, state: &Path, request: u64) -> Option<Value> {
    if !scratch.holds(
        state,
        &["rev-parse", "--verify", "--quiet", "home/runner/host-a"],
    ) {
        return None;
    }
    let places = scratch.git(state, &["show", "home/runner/host-a:places.rec"]);
    let mut status = None;
    let mut ours = false;
    for line in places.lines() {
        if let Some(number) = line.strip_prefix("Request: ") {
            ours = number == request.to_string();
        } else if let Some(text) = line.strip_prefix("Status: ").filter(|_| ours) {
            status = Some(text.to_string());
        } else if let Some(more) = line.strip_prefix("+ ").filter(|_| ours) {
            if let Some(status) = status.as_mut() {
                status.push('\n');
                status.push_str(more);
            }
        }
    }
    status.map(|text| {
        let status: Value = serde_json::from_str(&text).expect("a carried status is JSON");
        status["status"]["places"][0].clone()
    })
}

/// The commit a place's status gives it landed as, its phase `over` and
/// landed, or tidied once over and landed; none for any other phase.
#[cfg(target_os = "macos")]
fn landed_commit(status: &Value) -> Option<&Value> {
    let phase = &status["phase"];
    [&phase["over"], &phase["tidied"]["over"]]
        .into_iter()
        .map(|over| &over["landed"]["commit"])
        .find(|commit| commit.is_string())
}

/// The quick-start spec's "From crew's launch to main" (design Decision
/// 6): setup writes the runner's file for the quick start's answers over
/// the stand-ins; origin's `main` is as the landing step leaves it; crew's
/// state repository holds, on `home/main`, a fix of `main` for this host.
/// The runner's passes, over the set's Flume and sluice, launch it; the
/// test runs each agent command Flume prints, whose `claude` is the
/// stand-in; once the runner's branch carries the fix's code delivered
/// and its submission held, crew's send of it is appended, as the
/// reader's approval appends it. Flume's files are the Mac's step's as
/// they stand, with crew's roles file as crew writes it, and a local
/// layer that prints the drive's stream's commands. Within two minutes
/// origin's `main` holds the stand-in's change, and the runner's branch
/// carries the place's status as the set's Flume gives it, landed. Then
/// the watch step, run through `sh` on the same host, reaches the drive's
/// stream's session, which the set's Flume refuses in its own words under
/// the printed command: a window is what the gate cannot reach (site.8).
#[cfg(target_os = "macos")]
#[test]
fn a_change_the_drive_is_given_lands() {
    let stand = Stand::new();
    stand.set_up(&answers(&["--json"]));
    let scratch = &stand.scratch;
    let origin = landed(scratch);
    let state = scratch.empty_origin("your-github-name/drive-state");
    state.commit(
        "home/main",
        "requests.rec",
        &records::file(records::REQUESTS, &[fix_of_main()]),
    );
    state.commit("home/main", "plan.rec", &records::plan(&[], &[]));
    flumes_files(scratch);
    write_text(
        PRINTED,
        &scratch.home.join(".config/flume/config.local.toml"),
    );
    let agent = scratch.root.join("agent");
    std::fs::create_dir_all(&agent).expect("the agent's folder");
    std::os::unix::fs::symlink(stand_in(), agent.join("claude")).expect("the agent linked");
    let path = format!("{}:{}", agent.display(), set_path(&["flume", "sluice"]));
    // Flume runs a place's steps, sluice's among them, under the login
    // shell's PATH, which a reader's nix profile gives the set; the
    // scratch home's login profile gives it the set's.
    write_text(
        &format!("PATH='{}'\nexport PATH\n", set_path(&["flume", "sluice"])),
        &scratch.home.join(".profile"),
    );

    let began = std::time::Instant::now();
    let mut said_last = String::new();
    let mut ran: Vec<String> = Vec::new();
    let mut sent = false;
    let landed = loop {
        let status = carried(scratch, &state.bare, 1);
        if let Some(status) = status
            .as_ref()
            .filter(|status| landed_commit(status).is_some())
        {
            break status.clone();
        }
        assert!(
            began.elapsed() < LANDING_WITHIN,
            "the fix is not carried landed within {LANDING_WITHIN:?}; the agents run: {ran:?}; \
             the last pass: {said_last}; the runner's branch carries: {}",
            status.map(|status| status.to_string()).unwrap_or_default()
        );
        if status.is_some_and(|status| status["submission"]["held"] == true) && !sent {
            state.commit(
                "home/main",
                "requests.rec",
                &records::file(records::REQUESTS, &[fix_of_main(), send_of_fix()]),
            );
            sent = true;
        }
        let output = on_scratch(scratch, env!("CARGO_BIN_EXE_boomdrive"), &path)
            .args(["run", "--once"])
            .output()
            .expect("the runner runs");
        said_last = said(&output);
        let listed = on_scratch(scratch, set_program("flume"), &path)
            .args(["--json", "status"])
            .output()
            .expect("the set's Flume runs");
        let listed: Value = serde_json::from_slice(&listed.stdout).expect("Flume's status is JSON");
        for place in listed["status"]["places"].as_array().into_iter().flatten() {
            let Some(printed) = place["agent"]["printed"].as_str() else {
                continue;
            };
            if ran.iter().any(|run| run == printed) {
                continue;
            }
            let output = on_scratch(scratch, "sh", &path)
                .args(["-c", printed])
                .output()
                .expect("sh runs");
            assert_eq!(
                output.status.code(),
                Some(0),
                "{printed}: {}",
                said(&output)
            );
            ran.push(printed.to_string());
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
    assert!(sent, "the fix landed with no send: {landed}");
    assert_eq!(ran.len(), 1, "{ran:?}");
    assert_eq!(landed["identity"], "flume/fix@1", "{landed}");
    assert_eq!(landed["mark"], "boomdrive:home/1", "{landed}");
    assert_eq!(
        landed_commit(&landed),
        Some(&Value::from(origin.tip("main"))),
        "{landed}"
    );
    assert_eq!(
        scratch.git(&origin.bare, &["show", "main:change.txt"]) + "\n",
        "the stand-in's change\n"
    );

    let output = on_scratch(scratch, "sh", &path)
        .arg(step_file("both/watch.sh"))
        .output()
        .expect("sh runs");
    assert_eq!(output.status.code(), Some(1), "{}", said(&output));
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .starts_with("refused: the printed command holds no session a person can be shown"),
        "{}",
        said(&output)
    );
}
