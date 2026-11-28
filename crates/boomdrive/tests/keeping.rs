//! The runner's loop keeping its repositories, on its edge's paused
//! clock: a landing it is told of between passes kept at once and alone,
//! a keeping pass that fails stopping nothing, the configuration read
//! again at every pass, a file broken while running, no settle and no
//! tick where no inbox is named, each inbox ticked after each pass, and a
//! refused tick said once, and the local layer's fault, pinned keys and
//! removal each said once (the running spec, the clones spec's landings
//! and the configuration spec's reading while running and its local
//! layer; design 10; record 0033; core.13, core.17). The keeper, Flume and millpond are held in memory, so
//! no test starts a process, git or a socket
//! (logic-is-tested-as-units-and-few-tests-run-the-binary). `loop.rs`
//! keeps the landing told during a keeping pass, which the host's own
//! keeping pass hears between two repositories.

mod fakes;
mod held;

use held::{patiently, Clock, Running, SECOND};

/// A configuration keeping `addresses`, in order, in the drive `home`,
/// each `interval`, naming no control plane.
fn every(interval: &str, addresses: &[&str]) -> String {
    let entries: Vec<String> = addresses.iter().map(|a| format!("{a:?}")).collect();
    format!(
        "fetch_interval = {interval:?}\naccounts.own.drives.home.repositories = [{}]\n",
        entries.join(", ")
    )
}

/// The lines `running` says until it stops, the stop's among them.
fn stopped(running: Running) -> Vec<String> {
    running.stop().into_iter().map(|(_, line)| line).collect()
}

/// The clones spec's "Told between passes": told of a landing on
/// `agentplot/flume` while it waits, the runner keeps that repository at
/// once and alone, and its timer stays where it was: no keeping pass but
/// the first.
#[test]
fn told_between_passes() {
    let running = Running::keeping(
        Clock::paused(),
        &every("1h", &["agentplot/flume", "agentplot/sluice"]),
    );
    assert!(patiently(|| running.kept().len() == 1), "the first pass");

    running.landed("agentplot/flume");
    let (_, before) = running.until("agentplot/flume: kept at its landing");

    assert_eq!(before, [] as [String; 0]);
    assert_eq!(running.landings(), ["agentplot/flume"]);
    assert_eq!(running.kept().len(), 1, "no keeping pass but the first");
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// A keeping pass that fails stops nothing: the next comes on its timer,
/// a second on, and the passes after it go on as before.
#[test]
fn a_failure_does_not_stop_it() {
    let running = Running::keeping(
        Clock::paused(),
        &every("1s", &["agentplot/flume", "agentplot/sluice"]),
    );
    assert!(patiently(|| running.kept().len() == 1), "the first pass");
    running.failing(Some("agentplot/flume: not fetched: origin is gone"));

    running.advance(SECOND);
    let (failed, _) = running.until("agentplot/flume: not fetched: origin is gone");
    running.failing(None);
    running.advance(SECOND);
    let passed = patiently(|| running.kept().len() == 3);

    let kept = running.kept();
    assert!(passed, "the passes go on after one that failed");
    assert_eq!(failed, kept[1]);
    assert_eq!(kept[2] - kept[1], SECOND);
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// The configuration spec's "A repository added while running": the
/// next pass keeps the repositories the file names then.
#[test]
fn a_repository_added_while_running() {
    let running = Running::keeping(Clock::paused(), &every("1s", &["agentplot/flume"]));
    assert!(patiently(|| running.kept().len() == 1), "the first pass");

    running.rewrite(&every("1s", &["agentplot/flume", "agentplot/sluice"]));
    running.advance(SECOND);
    let passed = patiently(|| running.kept().len() == 2);

    assert!(passed, "the next pass");
    assert_eq!(
        running.kept_repositories(),
        [
            vec!["agentplot/flume".to_string()],
            vec![
                "agentplot/flume".to_string(),
                "agentplot/sluice".to_string()
            ],
        ]
    );
    running.stop();
}

/// The configuration spec's "A file broken while running": each pass
/// names the file and keeps nothing, ticking all the same the inboxes of
/// the last reading (the running spec's "A pass on a broken configuration
/// ticks"), and runs no Flume; once the file reads again, the passes keep
/// as before.
#[test]
fn a_file_broken_while_running() {
    let valid = format!(
        "inboxes = [\"home\"]\n{}",
        every("1s", &["agentplot/flume"])
    );
    let running = Running::keeping(Clock::paused(), &valid);
    assert!(
        patiently(|| running.kept().len() == 1 && running.ticks().len() == 1),
        "the first pass and its tick"
    );

    running
        .rewrite("fetch_interval = \"1s\"\naccounts.own.drives.home.repositories = [\"flume\"]\n");
    let broken = format!(
        "boomdrive: {}:2: `accounts.own.drives.home.repositories` entry \"flume\": an \
         address is `owner/name`: nothing kept this pass",
        running.config_file().display()
    );
    running.advance(SECOND);
    let (_, first) = running.until(&broken);
    running.advance(SECOND);
    let (_, second) = running.until(&broken);
    let ticked = patiently(|| running.ticks().len() == 3);
    let ticks = running.ticks();
    let kept_while_broken = running.kept().len();
    running.rewrite(&valid);
    running.advance(SECOND);
    let kept_again = patiently(|| running.kept().len() == 2);

    assert_eq!(first, [] as [String; 0]);
    assert_eq!(second, [] as [String; 0]);
    assert!(ticked, "each pass ticks: {ticks:?}");
    assert_eq!(ticks, ["home", "home", "home"]);
    assert_eq!(
        kept_while_broken, 1,
        "nothing kept while the file is broken"
    );
    assert!(
        running.flume_calls().is_empty(),
        "{:?}",
        running.flume_calls()
    );
    assert!(kept_again, "kept once the file reads again");
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// The running spec's "No pass runs a settle": naming no control plane
/// and no inbox, and keeping each second, the runner makes three keeping
/// passes and asks Flume and millpond nothing, since `flume host` keeps
/// the host's watch and a runner that carries no part reads no status
/// (core.13). It holds no subscription to flume host, which only a part
/// it carries reads.
#[test]
fn no_pass_runs_a_settle() {
    let running = Running::keeping(Clock::paused(), &every("1s", &["agentplot/flume"]));
    assert!(patiently(|| running.kept().len() == 1), "the first pass");
    running.advance(SECOND);
    assert!(patiently(|| running.kept().len() == 2), "the second pass");
    running.advance(SECOND);
    assert!(patiently(|| running.kept().len() == 3), "the third pass");

    assert!(
        running.flume_calls().is_empty(),
        "{:?}",
        running.flume_calls()
    );
    assert_eq!(
        running.ticks(),
        [] as [String; 0],
        "no inbox named, so no tick"
    );
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// The running spec's "Each pass ticks each inbox": the configuration
/// names the inboxes `home` and `work`, and each keeping pass ends with
/// one tick of each, in that order, before the next pass; an answered
/// tick says nothing (record 0033; core.13).
#[test]
fn each_pass_ticks_each_inbox() {
    let running = Running::keeping(
        Clock::paused(),
        &format!(
            "inboxes = [\"home\", \"work\"]\n{}",
            every("1s", &["agentplot/flume"])
        ),
    );
    assert!(patiently(|| running.ticks().len() == 2), "the first ticks");
    running.advance(SECOND);
    assert!(patiently(|| running.ticks().len() == 4), "the second ticks");

    assert_eq!(running.ticks(), ["home", "work", "home", "work"]);
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// The running spec's "A refused tick said once until millpond ticks
/// again": millpond refuses `home`'s ticks, and one line says so however
/// many passes follow; no pass stops for it.
#[test]
fn a_refused_tick_is_said_once() {
    let running = Running::keeping(
        Clock::paused(),
        &format!(
            "inboxes = [\"home\"]\n{}",
            every("1s", &["agentplot/flume"])
        ),
    );
    assert!(patiently(|| running.ticks().len() == 1), "the first tick");
    running.refuse_ticks_of("home");
    for ticks in 2..=4 {
        running.advance(SECOND);
        assert!(patiently(|| running.ticks().len() == ticks), "tick {ticks}");
    }

    let said = stopped(running);
    let refused = "boomdrive: millpond refused the tick of the inbox home: millpond exited 1";
    assert_eq!(
        said.iter().filter(|line| *line == refused).count(),
        1,
        "{said:#?}"
    );
    assert_eq!(said.last().map(String::as_str), Some("boomdrive: stopped"));
}

/// A local layer keeping `addresses`, in order, in the drive `home`.
fn local(addresses: &[&str]) -> String {
    let entries: Vec<String> = addresses.iter().map(|a| format!("{a:?}")).collect();
    format!(
        "accounts.own.drives.home.repositories = [{}]\n",
        entries.join(", ")
    )
}

/// The line a pass says of the local layer `running` reads taken.
fn taken(running: &Running) -> String {
    format!("boomdrive: {}: taken", running.local_file().display())
}

/// The configuration spec's "A fault said once": a local layer that is
/// not TOML for three passes is named once, the managed layer's
/// repositories kept alone, and once mended is said taken once, both
/// layers' repositories kept.
#[test]
fn a_fault_said_once() {
    let managed = every("1s", &["agentplot/flume"]);
    let broken = "accounts.own.drives.home.repositories = [\n";
    let running = Running::layered(Clock::paused(), &managed, broken);
    let fault = match boomdrive::config::parse_layers(
        &managed,
        &running.config_file(),
        Some(broken),
        &running.local_file(),
        running.home(),
    )
    .expect("a managed layer")
    .local
    {
        boomdrive::config::LocalLayer::NotTaken {
            fault: Some(fault), ..
        } => fault,
        other => panic!("not taken for a fault: {other:?}"),
    };
    let named = format!(
        "boomdrive: {}: not taken, the managed layer read alone and nothing let go: {fault}",
        running.local_file().display()
    );
    let (_, before) = running.until(&named);
    for passes in 2..=3 {
        running.advance(SECOND);
        assert!(
            patiently(|| running.kept().len() == passes),
            "pass {passes}"
        );
    }

    running.rewrite_local(&local(&["agentplot/sluice"]));
    running.advance(SECOND);
    let (_, between) = running.until(&taken(&running));
    assert!(patiently(|| running.kept().len() == 4), "the mended pass");
    let kept = running.kept_repositories();

    assert_eq!(before, [] as [String; 0]);
    assert_eq!(between, [] as [String; 0], "the fault is said once");
    assert_eq!(kept[2], ["agentplot/flume"]);
    assert_eq!(kept[3], ["agentplot/flume", "agentplot/sluice"]);
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// A running runner whose pinned `accounts` the local layer sets a drive's
/// repositories in says them passed over once, at its first pass, and
/// not again, keeping the managed layer's alone.
#[test]
fn pinned_accounts_passed_over_said_once() {
    let running = Running::layered(
        Clock::paused(),
        &format!(
            "pin = [\"accounts\"]\n{}",
            every("1s", &["agentplot/flume"])
        ),
        &local(&["agentplot/sluice"]),
    );
    let passed_over = format!(
        "{}; `accounts.own.drives.home.repositories` passed over, pinned by `accounts`",
        taken(&running)
    );
    let (_, before) = running.until(&passed_over);
    for passes in 2..=3 {
        running.advance(SECOND);
        assert!(
            patiently(|| running.kept().len() == passes),
            "pass {passes}"
        );
    }

    assert_eq!(before, [] as [String; 0]);
    assert!(
        running
            .kept_repositories()
            .iter()
            .all(|kept| kept == &["agentplot/flume"]),
        "{:?}",
        running.kept_repositories()
    );
    assert_eq!(stopped(running), ["boomdrive: stopped"]);
}

/// The configuration spec's "A local file removed": the gone line is said
/// once, and the passes after keep the managed layer's repositories
/// alone, so the next lets go of what only the local layer named.
#[test]
fn a_local_file_removed() {
    let running = Running::layered(
        Clock::paused(),
        &every("1s", &["agentplot/flume"]),
        &local(&["agentplot/sluice"]),
    );
    running.until(&taken(&running));

    std::fs::remove_file(running.local_file()).expect("the local layer removed");
    let gone = format!(
        "boomdrive: {}: gone, the managed layer read alone",
        running.local_file().display()
    );
    running.advance(SECOND);
    let (_, before) = running.until(&gone);
    running.advance(SECOND);
    assert!(patiently(|| running.kept().len() == 3), "the third pass");
    let kept = running.kept_repositories();

    assert_eq!(before, [] as [String; 0]);
    assert_eq!(kept[0], ["agentplot/flume", "agentplot/sluice"]);
    assert_eq!(kept[1], ["agentplot/flume"]);
    assert_eq!(kept[2], ["agentplot/flume"]);
    assert_eq!(stopped(running), ["boomdrive: stopped"], "gone said once");
}
