//! The scratch host stands in for GitHub on disk, and reads nothing of
//! the user's (design 11, core.4).

mod common;

use common::Scratch;

#[test]
fn a_clone_from_github_reads_the_forge() {
    let scratch = Scratch::new();
    let origin = scratch.origin("o/r");
    scratch.git(
        &scratch.home,
        &["clone", "--quiet", "https://github.com/o/r.git"],
    );
    let clone = scratch.home.join("r");
    assert_eq!(
        scratch.git(&clone, &["rev-parse", "HEAD"]),
        origin.tip("main")
    );
    assert_eq!(
        scratch.git(&clone, &["ls-remote", "--get-url", "origin"]),
        format!("file://{}", origin.bare.display())
    );
}

#[test]
fn the_scratch_host_reaches_no_network() {
    let scratch = Scratch::new();
    let output = scratch
        .command("git")
        .args(["ls-remote", "https://example.com/o/r.git"])
        .output()
        .expect("git runs");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("transport 'https' not allowed"),
        "git tried the network: {stderr}"
    );
}

#[test]
fn the_scratch_host_reads_only_its_own_configuration() {
    let scratch = Scratch::new();
    let ours = format!("file:{}\t", scratch.gitconfig.display());
    let listed = scratch.git(&scratch.home, &["config", "--list", "--show-origin"]);
    assert!(!listed.is_empty());
    for line in listed.lines() {
        assert!(
            line.starts_with(&ours),
            "configuration from elsewhere: {line}"
        );
    }
}

#[test]
fn the_forge_gains_commits_loses_branches_and_moves_its_head() {
    let scratch = Scratch::new();
    let origin = scratch.empty_origin("o/r");
    assert_eq!(
        scratch.git(&origin.bare, &["for-each-ref"]),
        "",
        "an empty origin holds a ref"
    );
    let first = origin.commit("main", "README", "one\n");
    let topic = origin.commit("topic", "topic", "two\n");
    assert_eq!(
        scratch.git(&origin.bare, &["rev-parse", "topic^"]),
        first,
        "a new branch is not from main"
    );
    assert_eq!(origin.tip("topic"), topic);
    let second = origin.commit("main", "README", "three\n");
    assert_eq!(scratch.git(&origin.bare, &["rev-parse", "main^"]), first);
    assert_eq!(origin.tip("main"), second);

    origin.point_head("topic");
    assert_eq!(
        scratch.git(&origin.bare, &["symbolic-ref", "HEAD"]),
        "refs/heads/topic"
    );
    origin.point_head("main");
    origin.delete("topic");
    assert!(!scratch.holds(
        &origin.bare,
        &["rev-parse", "--verify", "--quiet", "refs/heads/topic"]
    ));
    assert_eq!(origin.url(), "https://github.com/o/r.git");
}
