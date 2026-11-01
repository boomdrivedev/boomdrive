//! The configuration chooses the implementation (design Decisions 1 and
//! 4; the configuration spec's "The configuration may name the control
//! plane"; the control-plane-interface spec's "The configuration chooses
//! the implementation"): `planes.rs` opens the one its `type` names,
//! handing it the table of that name, and refuses a word this binary does
//! not carry, naming those it does.

use flywheel::config;
use flywheel::planes::{self, PLANES};
use std::path::Path;

const FILE: &str = "/c/config.toml";

/// Design Decision 4's example.
const EXAMPLE: &str = r#"repositories = ["afterthought/crew", "agentplot/flume"]

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
        Path::new("/nowhere/flywheel"),
        Box::new(|line| panic!("said {line:?} at its opening")),
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

/// The control-plane-interface spec's "The hosted control plane named":
/// `hosted` is refused, naming `crew`, the one this binary carries.
#[test]
fn the_hosted_control_plane_named() {
    let refusal = open(
        "repositories = []\n[control_plane]\ntype = \"hosted\"\n\n[control_plane.hosted]\n\
         url = \"wss://flywheel.invalid/host\"\n",
    )
    .err()
    .unwrap();
    assert_eq!(
        refusal,
        format!(
            "{FILE}: `type` is \"hosted\", a control plane this binary does not carry: \
             it carries `crew`"
        )
    );
}

/// The list holds each implementation's word once, `crew` among them.
#[test]
fn the_list() {
    let words: Vec<&str> = PLANES.iter().map(|(word, _)| *word).collect();
    assert_eq!(words, ["crew"]);
}
