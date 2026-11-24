//! The configuration chooses the implementations (design Decisions 1 and
//! 4; record 0054; the control-plane-interface spec's "The configuration
//! chooses the implementation"): `planes.rs` opens each one an account
//! names, handing each the table of its name and the `poll_interval`,
//! refuses a label two of them give, and enrolls a partition of the one
//! that enrolls. The hosted implementation is opened
//! here only with a table it refuses, before it looks for the host's
//! credential store; `control.rs` opens it in the binary, under a
//! scratch host (design Decision 14).

use boomdrive::config;
use boomdrive::planes::{self, PLANES};
use boomdrive_control::Part;
use std::path::Path;

const FILE: &str = "/c/config.toml";

/// Design Decision 4's example.
const EXAMPLE: &str = r#"accounts.own.drives.home.repositories = ["example/crew", "agentplot/flume"]
accounts.own.control_plane = "crew"

[control_plane]
poll_interval = "1m"

[control_plane.crew]
host = "host-a"

[[control_plane.crew.partitions]]
label = "home"
state_repository = "example/home-state"
"#;

/// Opens the control plane `text` names, keeping nothing anywhere: no
/// implementation touches its data directory until its first pull.
fn open(text: &str) -> Result<Vec<Part>, String> {
    let config = config::parse(text, Path::new(FILE), Path::new("/h")).expect("a configuration");
    let table = config.control_plane.expect("a control plane named");
    planes::open(
        &table,
        Path::new(FILE),
        Path::new("/nowhere/boomdrive"),
        &|_| Box::new(|line| panic!("said {line:?} at its opening")),
        &|_| Box::new(|| panic!("woken at its opening")),
        &weir_bounded::Halt::default(),
    )
}

/// The configuration spec's "crew named": the example of Decision 4
/// opens crew's implementation with its table, a part for its one
/// partition.
#[test]
fn crew_named() {
    let parts = open(EXAMPLE).expect("opened");
    let labels: Vec<&str> = parts.iter().map(|part| part.label.as_str()).collect();
    assert_eq!(labels, ["home"]);
}

/// Each part an implementation opens is handed a say made for its own
/// label, in its table's order (the control-plane-interface spec's "The
/// configuration chooses the implementation").
#[test]
fn each_part_is_handed_its_own_say() {
    let text = format!(
        "{EXAMPLE}\n[[control_plane.crew.partitions]]\nlabel = \"work\"\n\
         state_repository = \"example/work-state\"\n"
    );
    let config = config::parse(&text, Path::new(FILE), Path::new("/h")).expect("a configuration");
    let table = config.control_plane.expect("a control plane named");
    let asked = std::cell::RefCell::new(Vec::new());
    let parts = planes::open(
        &table,
        Path::new(FILE),
        Path::new("/nowhere/boomdrive"),
        &|label| {
            asked.borrow_mut().push(label.to_string());
            Box::new(|line| panic!("said {line:?} at its opening"))
        },
        &|_| Box::new(|| panic!("woken at its opening")),
        &weir_bounded::Halt::default(),
    )
    .expect("opened");
    let labels: Vec<&str> = parts.iter().map(|part| part.label.as_str()).collect();
    assert_eq!(labels, ["home", "work"]);
    assert_eq!(asked.into_inner(), ["home", "work"]);
}

/// The table an account names is handed to the implementation whole, so its
/// keys are the implementation's own to refuse.
#[test]
fn the_table_is_the_implementations() {
    let refusal = open(&EXAMPLE.replace("host = \"host-a\"\n", ""))
        .err()
        .unwrap();
    assert!(
        refusal.starts_with(&format!("{FILE}: [control_plane.crew]: names no `host`")),
        "{refusal}"
    );
    let refusal = open(&EXAMPLE.replace(
        "host = \"host-a\"\n",
        "host = \"host-a\"\nkinds = \"/k/kinds.toml\"\n",
    ))
    .err()
    .unwrap();
    assert!(
        refusal.contains("kinds are Flume's host configuration"),
        "{refusal}"
    );
}

/// The hosted table is the hosted implementation's own to refuse: a
/// token in it is named as a key it does not read, its value in no
/// output (the hosted-control-plane spec's "A token in the table").
#[test]
fn the_hosted_table_is_the_hosted_implementations() {
    let refusal = open(
        "accounts.own.drives = {}\naccounts.own.control_plane = \"hosted\"\n[control_plane]\n\n[control_plane.hosted]\n\
         url = \"https://control.example\"\ntoken = \"n0t-a-r34l-t0k3n\"\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = []\n",
    )
    .err()
    .unwrap();
    assert!(refusal.contains("`token`"), "{refusal}");
    assert!(refusal.contains("boomdrive enroll"), "{refusal}");
    assert!(!refusal.contains("n0t-a-r34l-t0k3n"), "{refusal}");
}

/// The running spec's "One label in two planes": crew's partition and
/// the hosted partition of one label are refused at the opening, naming
/// both implementations, before the hosted one looks for the host's store
/// (record 0054).
#[test]
fn one_label_in_two_planes() {
    let text = format!(
        "accounts.cloud.drives = {{}}\naccounts.cloud.control_plane = \"hosted\"\n{EXAMPLE}\n\
         [control_plane.hosted]\nurl = \"https://control.example\"\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = []\n"
    );
    let refusal = open(&text).err().unwrap();
    assert_eq!(
        refusal,
        format!(
            "{FILE}: the partition `home` is named by `[control_plane.crew]` and \
             `[control_plane.hosted]`: a part's label is given once on the host"
        )
    );
}

/// The list holds each implementation's word once, `crew` and `hosted`,
/// and an enrollment for `hosted` alone.
#[test]
fn the_list() {
    let words: Vec<(&str, bool)> = PLANES
        .iter()
        .map(|entry| (entry.word, entry.enroll.is_some()))
        .collect();
    assert_eq!(words, [("crew", false), ("hosted", true)]);
}

/// The hosted-control-plane spec's "crew's control plane": where every
/// account names crew, nothing enrolls, and it is refused as the host's to
/// fix before the token is asked for, naming `hosted`, which enrolls.
#[test]
fn crews_control_plane_enrolls_nothing() {
    let config = config::parse(EXAMPLE, Path::new(FILE), Path::new("/h")).expect("a configuration");
    let table = config.control_plane.expect("a control plane named");
    let refusal = planes::enroll(&table, Path::new(FILE), "home", || {
        panic!("the token asked for")
    })
    .unwrap_err();
    assert_eq!(
        refusal.words,
        format!(
            "{FILE}: no account names a control plane that enrolls: `boomdrive enroll` enrolls a \
             partition of `hosted`"
        )
    );
    assert!(refusal.host_to_fix);
}

/// Each implementation lists `partitions` as the one key of its table the
/// runner's local layer may set, and reads its own table, a partition
/// with no `label` refused, opening nothing (design Decision 2; record
/// 0009 as amended 2026-11-07).
#[test]
fn each_implementation_lists_partitions_for_the_local_layer() {
    for entry in &PLANES {
        assert_eq!(entry.local_keys, ["partitions"], "{}", entry.word);
        let others = match entry.word {
            "crew" => "host = \"host-a\"\n",
            "hosted" => "url = \"https://control.example\"\n",
            word => panic!("no table written for {word}"),
        };
        let table: toml::Table = format!("{others}\n[[partitions]]\nrepositories = []\n")
            .parse()
            .expect("a table");
        let refusal = (entry.read)(&table, Path::new(FILE)).unwrap_err();
        assert!(
            refusal.contains("`partitions` entry 1 names no `label`"),
            "{}: {refusal}",
            entry.word
        );
    }
}
