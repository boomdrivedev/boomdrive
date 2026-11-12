//! README's examples are what the runner reads (task 1.2, Task notes
//! 9.4, task 6.4): its configuration, the local layer over it, and with
//! it its `[control_plane]` table, which opens crew's implementation, and its hosted table, which
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

/// The ```toml block after the first of `text`, the local layer's.
fn second_toml_block(text: &str) -> &str {
    let (_, after) = text
        .split_once("```toml\n")
        .expect("README holds a ```toml block");
    let (_, after) = after.split_once("```").expect("the block is closed");
    toml_block(after)
}

/// The local layer README shows, read over its configuration, keeps both
/// layers' repositories and is taken: it adds to a drive the managed
/// layer names, and declares one it does not.
#[test]
fn the_readme_local_layer_reads() {
    let (text, readme) = readme();
    let config = config::parse_layers(
        toml_block(&text),
        &readme,
        Some(second_toml_block(&text)),
        &readme.with_file_name("config.local.toml"),
        Path::new("/h"),
    )
    .unwrap();
    assert_eq!(
        config.local,
        config::LocalLayer::Taken {
            passed_over: vec![]
        }
    );
    let managed = config::parse(toml_block(&text), &readme, Path::new("/h")).unwrap();
    assert!(config.repositories().count() > managed.repositories().count());
    for address in managed.repositories() {
        assert_eq!(
            config.drive_of(address).map(|drive| &drive.name),
            managed.drive_of(address).map(|drive| &drive.name),
            "{address}"
        );
    }
    let added_to: Vec<&str> = config
        .drives()
        .filter(|drive| {
            managed.drives().any(|kept| {
                kept.name == drive.name && kept.repositories.len() < drive.repositories.len()
            })
        })
        .map(|drive| drive.name.as_str())
        .collect();
    assert_eq!(added_to, ["home"]);
    let declared: Vec<&str> = config
        .drives()
        .filter(|drive| managed.drives().all(|kept| kept.name != drive.name))
        .map(|drive| drive.name.as_str())
        .collect();
    assert_eq!(declared, ["garden"]);
}

#[test]
fn the_readme_configuration_reads() {
    let (text, readme) = readme();
    let config = config::parse(toml_block(&text), &readme, Path::new("/h")).unwrap();
    let drives: Vec<(&str, &str, Vec<String>)> = config
        .accounts
        .iter()
        .flat_map(|account| {
            account.drives.iter().map(|drive| {
                let repositories = drive.repositories.iter().map(ToString::to_string);
                (
                    account.name.as_str(),
                    drive.name.as_str(),
                    repositories.collect(),
                )
            })
        })
        .collect();
    assert_eq!(
        drives,
        [
            ("own", "home", vec!["agentplot/flume".to_string()]),
            ("own", "work", vec!["example/crew".to_string()]),
        ]
    );
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
    let table = config.control_plane.clone().expect("a control plane named");
    assert_eq!(table.implementation, "hosted");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    let kept: Vec<String> = config
        .repositories()
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
/// up and put away, by sluice's own commands, and when a preview leaves
/// the runner's branch (record 0031 as amended on 2026-11-09).
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
    assert!(
        flat.contains("leaves the runner's branch once its done is answered"),
        "README says not when a preview leaves the runner's branch"
    );
}

/// Design Decision 3 of the-runner-names-each-repository-it-keeps:
/// README's crew section names `repositories.rec` among the files of the
/// runner's branch, naming each repository of the partition's drive.
#[test]
fn the_readme_names_the_repositories_each_drive_keeps() {
    let (text, _) = readme();
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    for named in [
        "repositories.rec",
        "each repository of the partition's drive",
    ] {
        assert!(flat.contains(named), "README names no {named:?}");
    }
}

/// Design Decisions 2, 4 and 8 of
/// the-hosted-protocol-carries-every-end-of-a-place, and Decision 9 of
/// the-hosted-protocol-carries-land-and-preview: README's hosted section
/// says the runner speaks version 2, `boomdrive.v2`, that an end version
/// 2 does not name goes up as its delivery alone, that a control plane's
/// confirm is carried out as crew's is, and that its land and preview are
/// given to the runner as crew's are, what the runner answered of each
/// preview going up as their copies.
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
        "land and preview",
        "as their copies",
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
