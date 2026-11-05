//! README's examples are what the runner reads (task 1.2, Task notes
//! 9.4, task 6.4): its configuration, and with it its `[control_plane]`
//! table, which opens crew's implementation, and its hosted table, which
//! the hosted implementation reads.

use boomdrive::{config, planes};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// README, and where it is.
fn readme() -> (String, PathBuf) {
    let readme = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let text = std::fs::read_to_string(&readme).unwrap();
    (text, readme)
}

/// The first ```toml block of `text`.
fn toml_block(text: &str) -> &str {
    let (_, block) = text
        .split_once("```toml\n")
        .expect("README holds a ```toml block");
    let (block, _) = block.split_once("```").expect("the block is closed");
    block
}

#[test]
fn the_readme_configuration_reads() {
    let (text, readme) = readme();
    let config = config::parse(toml_block(&text), &readme, Path::new("/h")).unwrap();
    assert!(!config.repositories.is_empty());
    assert!(config.code_workspaces);
    assert_eq!(config.control_plane, None);
}

/// The block under "Carrying the control plane's work", written after the
/// configuration's, opens crew's implementation.
#[test]
fn the_readme_control_plane_opens_crews_implementation() {
    let (text, readme) = readme();
    let (_, carrying) = text
        .split_once("\n## Carrying the control plane's work\n")
        .expect("README says what the runner carries");
    let file = format!("{}\n{}", toml_block(&text), toml_block(carrying));
    let config = config::parse(&file, &readme, Path::new("/h")).unwrap();
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "crew");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    let opened = planes::open(
        &table.implementation,
        &table.implementation_table,
        &readme,
        Path::new("/nowhere/boomdrive"),
        table.poll_interval,
        Box::new(|line| panic!("said {line:?} at its opening")),
        Box::new(|| panic!("woken at its opening")),
    );
    assert!(opened.is_ok(), "{:?}", opened.err());
}

/// The block under "Dialing the hosted control plane", written after the
/// configuration's, is a table the hosted implementation reads, each of
/// its partitions served repositories the host keeps. Its enrollment of a
/// label it lacks is refused naming its partitions, before any credential
/// store is looked for or any request made.
#[test]
fn the_readme_hosted_table_reads() {
    let (text, readme) = readme();
    let (_, dialing) = text
        .split_once("\n## Dialing the hosted control plane\n")
        .expect("README says how the runner dials the hosted control plane");
    let file = format!("{}\n{}", toml_block(&text), toml_block(dialing));
    let config = config::parse(&file, &readme, Path::new("/h")).unwrap();
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "hosted");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    let kept: Vec<String> = config
        .repositories
        .iter()
        .map(|address| format!("{}/{}", address.owner(), address.name()))
        .collect();
    let partitions = table.implementation_table["partitions"]
        .as_array()
        .expect("partitions");
    for partition in partitions {
        for repository in partition["repositories"].as_array().expect("repositories") {
            let repository = repository.as_str().expect("an address");
            assert!(
                kept.iter().any(|address| address == repository),
                "{repository}"
            );
        }
    }
    let refusal = planes::enroll(
        &table.implementation,
        &table.implementation_table,
        &readme,
        "garden",
        || Ok(String::new()),
    )
    .unwrap_err();
    assert_eq!(
        refusal.words,
        "[control_plane.hosted] names no partition `garden`: it names `home`, `work`"
    );
    assert!(refusal.host_to_fix);
}
