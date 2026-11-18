//! README's examples of the runner's configuration are what the runner
//! reads (task 1.2, Task notes 9.4, task 6.4): each is a file of its own
//! under `tests/examples`, of which README shows its own copy (record
//! 0049). The configuration, the App it names, the local layer over it,
//! and with it its `[control_plane]` table, which opens crew's
//! implementation, and its hosted table, which the hosted implementation
//! reads, the App left out, since the hosted control plane holds it
//! (record 0007 as amended on 2026-11-11).

use boomdrive::config::ForgeApp;
use boomdrive::{config, planes};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The example `name`, and where it is.
fn example(name: &str) -> (String, PathBuf) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("tests/examples/{name}.toml"));
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    (text, file)
}

/// The local layer, read over the configuration, keeps both layers'
/// repositories and is taken: it adds to a drive the managed layer names,
/// and declares one it does not.
#[test]
fn the_local_layer_reads() {
    let (text, file) = example("configuration");
    let (local, local_file) = example("local-layer");
    let config =
        config::parse_layers(&text, &file, Some(&local), &local_file, Path::new("/h")).unwrap();
    assert_eq!(
        config.local,
        config::LocalLayer::Taken {
            passed_over: vec![]
        }
    );
    let managed = config::parse(&text, &file, Path::new("/h")).unwrap();
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
fn the_configuration_reads() {
    let (text, file) = example("configuration");
    let config = config::parse(&text, &file, Path::new("/h")).unwrap();
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
            ("own", "home", vec!["boomdrivedev/flume".to_string()]),
            ("own", "work", vec!["example/crew".to_string()]),
        ]
    );
    assert!(config.code_workspaces);
    assert_eq!(config.control_plane, None);
    assert_eq!(
        config.forge,
        Some(ForgeApp {
            app_id: 123456,
            key_file: PathBuf::from("/h/.config/boomdrive/app.pem"),
        })
    );
}

/// The configuration with its `[forge]` left out, as a host under the
/// hosted control plane writes it.
fn without_the_app(text: &str) -> String {
    let mut table: toml::Table = text.parse().expect("the configuration is TOML");
    assert!(
        table.remove("forge").is_some(),
        "the configuration names no [forge]"
    );
    toml::to_string(&table).expect("the configuration written again")
}

/// crew's table, written after the configuration's, opens crew's
/// implementation.
#[test]
fn the_control_plane_opens_crews_implementation() {
    let (text, file) = example("configuration");
    let (crew, _) = example("crew");
    let config = config::parse(&format!("{text}\n{crew}"), &file, Path::new("/h")).unwrap();
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "crew");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    let opened = planes::open(
        &table.implementation,
        &table.implementation_table,
        &file,
        Path::new("/nowhere/boomdrive"),
        table.poll_interval,
        &|_| Box::new(|line| panic!("said {line:?} at its opening")),
        &|_| Box::new(|| panic!("woken at its opening")),
        &weir_bounded::Halt::default(),
    );
    assert!(opened.is_ok(), "{:?}", opened.err());
}

/// The hosted table, written after the configuration's, is a table the
/// hosted implementation reads, each of its partitions served repositories
/// the host keeps. Its enrollment of a label it lacks is refused naming
/// its partitions, before any credential store is looked for or any
/// request made.
#[test]
fn the_hosted_table_reads() {
    let (text, file) = example("configuration");
    let (hosted, _) = example("hosted");
    let beside = format!("{text}\n{hosted}");
    let refused = config::parse(&beside, &file, Path::new("/h")).unwrap_err();
    assert!(refused.contains("holds the App"), "{refused}");
    let written = format!("{}\n{hosted}", without_the_app(&text));
    let config = config::parse(&written, &file, Path::new("/h")).unwrap();
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
        &file,
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
