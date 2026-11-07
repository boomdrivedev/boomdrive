//! The configuration chooses the implementation (design Decisions 1 and
//! 4; the configuration spec's "The configuration may name the control
//! plane"; the control-plane-interface spec's "The configuration chooses
//! the implementation"): `planes.rs` opens the one its `type` names,
//! handing it the table of that name and the `poll_interval`, enrolls a
//! partition of the one that enrolls, and refuses a word this binary does
//! not carry, naming those it does. The hosted implementation is opened
//! here only with a table it refuses, before it looks for the host's
//! credential store; `control.rs` opens it in the binary, under a
//! scratch host (design Decision 14).

use boomdrive::config;
use boomdrive::planes::{self, PLANES};
use std::path::Path;

const FILE: &str = "/c/config.toml";

/// Design Decision 4's example.
const EXAMPLE: &str = r#"accounts.own.drives.home.repositories = ["afterthought/crew", "agentplot/flume"]

[control_plane]
type = "crew"
poll_interval = "1m"

[control_plane.crew]
host = "deneb"

[[control_plane.crew.partitions]]
label = "madswan"
state_repository = "afterthought/madswan-state"
"#;

/// Opens the control plane `text` names, keeping nothing anywhere: no
/// implementation touches its data directory until its first pull.
fn open(text: &str) -> Result<planes::Plane, String> {
    let config = config::parse(text, Path::new(FILE), Path::new("/h")).expect("a configuration");
    let table = config.control_plane.expect("a control plane named");
    planes::open(
        &table.implementation,
        &table.implementation_table,
        Path::new(FILE),
        Path::new("/nowhere/boomdrive"),
        table.poll_interval,
        Box::new(|line| panic!("said {line:?} at its opening")),
        Box::new(|| panic!("woken at its opening")),
    )
}

/// The configuration spec's "crew named": the example of Decision 4
/// opens crew's implementation with its table.
#[test]
fn crew_named() {
    assert!(open(EXAMPLE).is_ok());
}

/// The table `type` names is handed to the implementation whole, so its
/// keys are the implementation's own to refuse.
#[test]
fn the_table_is_the_implementations() {
    let refusal = open(&EXAMPLE.replace("host = \"deneb\"\n", ""))
        .err()
        .unwrap();
    assert!(
        refusal.starts_with(&format!("{FILE}: [control_plane.crew]: names no `host`")),
        "{refusal}"
    );
    let refusal = open(&EXAMPLE.replace(
        "host = \"deneb\"\n",
        "host = \"deneb\"\nkinds = \"/k/kinds.toml\"\n",
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
        "accounts = {}\n[control_plane]\ntype = \"hosted\"\n\n[control_plane.hosted]\n\
         url = \"https://control.example\"\ntoken = \"n0t-a-r34l-t0k3n\"\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = []\n",
    )
    .err()
    .unwrap();
    assert!(refusal.contains("`token`"), "{refusal}");
    assert!(refusal.contains("boomdrive enroll"), "{refusal}");
    assert!(!refusal.contains("n0t-a-r34l-t0k3n"), "{refusal}");
}

/// The control-plane-interface spec's "A word not carried": `elsewhere`
/// is refused, naming `crew` and `hosted`, the words this binary carries.
#[test]
fn a_word_not_carried() {
    let refusal = open("accounts = {}\n[control_plane]\ntype = \"elsewhere\"\n")
        .err()
        .unwrap();
    assert_eq!(
        refusal,
        format!(
            "{FILE}: `type` is \"elsewhere\", a control plane this binary does not carry: \
             it carries `crew`, `hosted`"
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

/// The hosted-control-plane spec's "crew's control plane": crew's
/// implementation enrolls nothing, and is refused as the host's to fix
/// before the token is asked for, naming `hosted`, which enrolls.
#[test]
fn crews_control_plane_enrolls_nothing() {
    let refusal = planes::enroll("crew", &toml::Table::new(), Path::new(FILE), "home", || {
        panic!("the token asked for")
    })
    .unwrap_err();
    assert_eq!(
        refusal.words,
        format!(
            "{FILE}: `type` is \"crew\", a control plane that enrolls nothing: `boomdrive enroll` \
             enrolls a partition of `hosted`"
        )
    );
    assert!(refusal.host_to_fix);
}

/// A word not carried is refused at enrollment too, naming the words this
/// binary carries, before the token is asked for.
#[test]
fn a_word_not_carried_enrolls_nothing() {
    let refusal = planes::enroll(
        "elsewhere",
        &toml::Table::new(),
        Path::new(FILE),
        "home",
        || panic!("the token asked for"),
    )
    .unwrap_err();
    assert!(
        refusal.words.ends_with("it carries `crew`, `hosted`"),
        "{}",
        refusal.words
    );
    assert!(refusal.host_to_fix);
}
