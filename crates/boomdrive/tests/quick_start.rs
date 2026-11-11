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
use std::path::{Path, PathBuf};

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
