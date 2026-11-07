//! The files the flake's modules render (the service-modules spec's "The
//! runner reads each file", design Decision 9): each file under
//! `nix/tests/files`, which `nix:test` holds byte for byte to what the
//! modules render of the spec's settings, is read by the runner's own
//! reader against a scratch home, refusing no key, with the accounts,
//! drives, repositories and control plane its settings named; each
//! control plane's table is one its implementation reads.

mod common;

use boomdrive::config::{self, Config};
use boomdrive::planes;
use common::Scratch;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The folder of the rendered files.
fn files() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../nix/tests/files")
}

/// The rendered file `name`, read as the runner reads its file, with `~`
/// as the scratch home.
fn read(scratch: &Scratch, name: &str) -> Config {
    let file = files().join(name);
    config::read(&file, &scratch.home).unwrap_or_else(|refusal| panic!("{refusal}"))
}

/// The repositories `config` keeps, as written, in the order kept.
fn kept(config: &Config) -> Vec<String> {
    config.repositories().map(ToString::to_string).collect()
}

/// A drive's name with its repositories as written.
type Drive<'a> = (&'a str, Vec<String>);

/// Each account `config` names, with each of its drives and their
/// repositories as written.
fn accounts(config: &Config) -> Vec<(&str, Vec<Drive<'_>>)> {
    config
        .accounts
        .iter()
        .map(|account| {
            let drives = account
                .drives
                .iter()
                .map(|drive| {
                    let repositories = drive.repositories.iter().map(ToString::to_string);
                    (drive.name.as_str(), repositories.collect())
                })
                .collect();
            (account.name.as_str(), drives)
        })
        .collect()
}

#[test]
fn each_rendered_file_is_one_read_here() {
    let mut names: Vec<String> = std::fs::read_dir(files())
        .expect("nix/tests/files")
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, ["crew.toml", "empty.toml", "hosted.toml"]);
}

/// "Only accounts": the runner's own defaults hold for every key the
/// file leaves out.
#[test]
fn the_empty_file_keeps_nothing_with_the_runners_defaults() {
    let scratch = Scratch::new();
    let config = read(&scratch, "empty.toml");
    assert!(config.accounts.is_empty());
    assert!(kept(&config).is_empty());
    assert_eq!(config.repository_root, scratch.home.join("Code"));
    assert_eq!(config.fetch_interval, Duration::from_secs(2 * 60));
    assert!(!config.code_workspaces);
    assert_eq!(config.control_plane, None);
}

/// "A host under crew": its keys as its settings named them, and a table
/// crew's implementation opens.
#[test]
fn the_crew_file_opens_crews_implementation() {
    let scratch = Scratch::new();
    let file = files().join("crew.toml");
    let config = read(&scratch, "crew.toml");
    assert_eq!(
        accounts(&config),
        [(
            "own",
            vec![(
                "home",
                vec![
                    "agentplot/flume".to_string(),
                    "agentplot/sluice".to_string()
                ]
            )]
        )]
    );
    assert_eq!(kept(&config), ["agentplot/flume", "agentplot/sluice"]);
    assert_eq!(config.repository_root, scratch.home.join("Code"));
    assert_eq!(config.fetch_interval, Duration::from_secs(2 * 60));
    assert!(config.code_workspaces);
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "crew");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    assert_eq!(table.implementation_table["host"].as_str(), Some("host-a"));
    let opened = planes::open(
        &table.implementation,
        &table.implementation_table,
        &file,
        &scratch.data(),
        table.poll_interval,
        Box::new(|line| panic!("said {line:?} at its opening")),
        Box::new(|| panic!("woken at its opening")),
    );
    assert!(opened.is_ok(), "{:?}", opened.err());
}

/// "A host under a hosted control plane": a table the hosted
/// implementation reads, whose enrollment of a label it lacks is refused
/// naming its partitions, before any credential store is looked for or
/// any request made.
#[test]
fn the_hosted_file_names_its_partitions() {
    let scratch = Scratch::new();
    let file = files().join("hosted.toml");
    let config = read(&scratch, "hosted.toml");
    assert_eq!(
        accounts(&config),
        [
            (
                "home",
                vec![("garden", vec!["agentplot/flume".to_string()])]
            ),
            ("work", vec![]),
        ]
    );
    assert_eq!(kept(&config), ["agentplot/flume"]);
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "hosted");
    assert_eq!(
        table.implementation_table["url"].as_str(),
        Some("https://control.example")
    );
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
}
