//! The quick start's steps (record 0043; site.8; design Decisions 5 and 6
//! of the change boomdrives-quick-start-takes-a-stranger-to-a-working-drive):
//! each step's file is read by its path under `site/src/start/`, beside
//! the crate's manifest, as `readme.rs` reads README, and run as it stands
//! through `sh`, never a copy of its text. A step whose run would reach
//! only a stand-in, a sign-in, the state repository or the start, is held
//! to what setup names for it, setup run over `common::stand`'s stand-ins.
//! The family's tools are the runner cargo built for the test and the
//! set's Flume and sluice by the paths the devenv shell gives (core.45),
//! never stubs of them. Every test runs on a `Scratch` home, and nothing
//! reaches the network. A test that runs setup is macOS's, where setup
//! runs and the gate runs.

mod common;

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
    std::fs::create_dir_all(to.parent().expect("the file's folder")).expect("its folder");
    std::fs::write(to, step(path)).expect("the step's file written");
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

/// The quick-start spec's "The set's Flume takes the file": the Mac's
/// Flume file written as the scratch home's, at Flume's own path, and the
/// set's Flume's `--json status` there answers with no refusal, its
/// default stream's part giving the multiplexer `herdr` and the terminal
/// app `terminal` from the host's keys.
#[test]
fn flumes_file_opens_agents_in_herdr() {
    let scratch = Scratch::new();
    write_step(
        "mac/flume.toml",
        &scratch.home.join(".config/flume/config.toml"),
    );
    let output = on_scratch(&scratch, set_program("flume"), "/usr/bin:/bin")
        .args(["--json", "status"])
        .output()
        .expect("the set's Flume runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let answer: Value = serde_json::from_slice(&output.stdout).expect("Flume's status is JSON");
    assert!(answer.get("refused").is_none(), "{answer}");
    let streams = answer
        .pointer("/status/host/streams")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("no streams in Flume's status: {answer}"));
    let default = streams
        .iter()
        .find(|stream| stream["name"] == "default")
        .unwrap_or_else(|| panic!("no default stream: {answer}"));
    assert_eq!(default["refused"], Value::Null, "{default}");
    for (part, profile) in [("multiplexer", "herdr"), ("terminal_app", "terminal")] {
        assert_eq!(default[part]["profile"], profile, "{default}");
        assert_eq!(default[part]["from"], "host", "{default}");
    }
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

/// The quick-start spec's "The policy on main": the landing step's policy
/// written as `.config/sluice.toml` in a scratch clone of a bare origin,
/// its `land.sh` run there through `sh`, and in a second clone the set's
/// sluice reads `main`'s policy, as `sluice equip main` reads it at the
/// branch's revision, and refuses nothing.
#[test]
fn sluices_file_lands_on_main() {
    let scratch = Scratch::new();
    let origin = scratch.origin("your-github-name/your-repository");
    let first = scratch.root.join("clones/first");
    std::fs::create_dir_all(first.parent().expect("the clones' folder")).expect("the folder");
    scratch.git(
        &scratch.root,
        &["clone", "--quiet", &origin.url(), &first.to_string_lossy()],
    );
    write_step("both/sluice.toml", &first.join(".config/sluice.toml"));
    let output = on_scratch(&scratch, "sh", &own_path())
        .arg(step_file("both/land.sh"))
        .current_dir(&first)
        .output()
        .expect("sh runs");
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    assert_eq!(
        scratch.git(&origin.bare, &["show", "main:.config/sluice.toml"]) + "\n",
        step("both/sluice.toml")
    );

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
}
