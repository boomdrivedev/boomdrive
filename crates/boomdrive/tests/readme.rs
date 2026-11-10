//! README's examples are what the runner reads (task 1.2, Task notes
//! 9.4, task 6.4): its configuration, and with it its `[control_plane]`
//! table, which opens crew's implementation, and its hosted table, which
//! the hosted implementation reads. It names no branch command the
//! runner runs (core.52).

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
        &|_| Box::new(|line| panic!("said {line:?} at its opening")),
        &|_| Box::new(|| panic!("woken at its opening")),
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

/// core.52 and design Decision 8: README says a bolt is a launch and a
/// command is refused, and names no `sluice branch create` the runner
/// runs.
#[test]
fn the_readme_names_no_branch_command() {
    let (text, _) = readme();
    assert!(
        !text.contains("sluice branch create"),
        "README names sluice branch create"
    );
    assert!(
        text.contains("A bolt is a launch"),
        "README says a bolt is a launch"
    );
    assert!(
        text.contains("refuses every command, naming its tool"),
        "README says every command is refused"
    );
}

/// core.55 and design Decision 13: README says how a preview is brought
/// up and put away, by sluice's own commands.
#[test]
fn the_readme_names_how_a_preview_is_brought_up_and_put_away() {
    let (text, _) = readme();
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("sluice preview <branch> <revision>"),
        "README names no sluice preview <branch> <revision>"
    );
    assert!(
        flat.contains("sluice release <name>"),
        "README names no sluice release <name>"
    );
    assert!(
        flat.contains("previews.rec"),
        "README names no previews.rec"
    );
}

/// Design Decisions 2, 4 and 8 of
/// the-hosted-protocol-carries-every-end-of-a-place: README's hosted
/// section says the runner speaks version 2, `boomdrive.v2`, that an end
/// version 2 does not name goes up as its delivery alone, and that a
/// control plane's confirm is carried out as crew's is.
#[test]
fn the_readme_names_version_2_of_the_protocol() {
    let (text, _) = readme();
    let hosted = text
        .split_once("## Dialing the hosted control plane\n")
        .expect("README's hosted section")
        .1;
    let hosted = hosted.split("\n## ").next().unwrap_or(hosted);
    let flat = hosted.split_whitespace().collect::<Vec<_>>().join(" ");
    for named in [
        "version 2 of the protocol, `boomdrive.v2`",
        "the version before it, version 1",
        "goes up as its delivery alone",
        "the rest of its status going up",
        "A control plane's confirm of a reader's ends is an act",
        "carried out as crew's is",
    ] {
        assert!(
            flat.contains(named),
            "README's hosted section names no {named:?}"
        );
    }
    assert!(
        !flat.contains("boomdrive.v1"),
        "README's hosted section names boomdrive.v1"
    );
}
