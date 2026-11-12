//! The files the flake's modules render (the service-modules spec's "The
//! runner reads each file", design Decision 9): each file under
//! `nix/tests/files`, which `nix:test` holds byte for byte to what the
//! modules render of the spec's settings, is read by the runner's own
//! reader against a scratch home, refusing no key, with the accounts,
//! drives, repositories, control plane and App its settings named; each
//! control plane's table is one its implementation reads. It also holds
//! the data folder the modules name, `services.boomdrive.dataDir`, to the
//! runner's own rule, `config::data_dir`, by `nix/tests/data-dirs.json`,
//! the table `nix:test` holds the modules' rule to (the service-modules
//! spec's "The runner's own rule"). And it holds the clone folders the
//! modules name, `services.boomdrive.cloneDirs`, to the runner's own
//! `clone::container`, by `nix/tests/clone-dirs.json`, the table
//! `nix:test` holds the modules' clone rule to (the service-modules spec's
//! "The runner's own clone rule"). And it holds the views the modules give
//! crew, each kept repository's `checkout`, to the runner's own
//! `config::view_path`, by `nix/tests/view-dirs.json`, the table
//! `nix:test` holds the modules' view rule to (the service-modules spec's
//! "The runner's own view rule" and "A repository root the runner
//! refuses").

mod common;

use boomdrive::clone;
use boomdrive::config::{self, Config, ForgeApp};
use boomdrive::planes;
use common::Scratch;
use std::ffi::OsString;
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
    assert_eq!(
        names,
        ["crew.toml", "empty.toml", "hosted.toml", "setup.toml"]
    );
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
    assert_eq!(config.forge, None);
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
        &|_| Box::new(|line| panic!("said {line:?} at its opening")),
        &|_| Box::new(|| panic!("woken at its opening")),
    );
    assert!(opened.is_ok(), "{:?}", opened.err());
}

/// The file `boomdrive setup` writes for one account's drive (design
/// Decision 6 of the change a-persons-drive-is-set-up-by-one-command): the
/// account `own`'s drive `home`, crew named as the control plane with no
/// other key of the runner's set, so the runner's own defaults hold, and a
/// table crew's implementation reads, naming the host and the drive's one
/// partition with its state repository.
#[test]
fn the_setup_file_keeps_one_drive_under_crew() {
    let scratch = Scratch::new();
    let file = files().join("setup.toml");
    let config = read(&scratch, "setup.toml");
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
    assert_eq!(config.repository_root, scratch.home.join("Code"));
    assert_eq!(config.fetch_interval, Duration::from_secs(2 * 60));
    assert!(!config.code_workspaces);
    let table = config.control_plane.expect("a control plane named");
    assert_eq!(table.implementation, "crew");
    assert_eq!(table.poll_interval, Duration::from_secs(60));
    let crew = planes::carried("crew").expect("crew's implementation");
    let read = (crew.read)(&table.implementation_table, &file);
    assert!(read.is_ok(), "{:?}", read.err());
    let held = &table.implementation_table;
    let mut keys: Vec<&String> = held.keys().collect();
    keys.sort();
    assert_eq!(keys, ["host", "partitions"]);
    assert_eq!(held["host"].as_str(), Some("host-a"));
    let partitions = held["partitions"].as_array().expect("partitions");
    assert_eq!(partitions.len(), 1);
    let partition = partitions[0].as_table().expect("a partition");
    assert_eq!(partition.len(), 2);
    assert_eq!(partition["label"].as_str(), Some("home"));
    assert_eq!(
        partition["state_repository"].as_str(),
        Some("example/state")
    );
}

/// The service-modules spec's "The App named": the App `1`, its key at
/// `.config/boomdrive/app.pem` under the home, whether or not a file
/// stands there (record 0007 as amended on 2026-11-11).
#[test]
fn the_crew_file_names_the_app() {
    let scratch = Scratch::new();
    let config = read(&scratch, "crew.toml");
    assert_eq!(
        config.forge,
        Some(ForgeApp {
            app_id: 1,
            key_file: scratch.home.join(".config/boomdrive/app.pem"),
        })
    );
    assert_eq!(read(&scratch, "hosted.toml").forge, None);
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

/// "The runner's own rule": for each row of `nix/tests/data-dirs.json`,
/// the runner's data folder of the row's `HOME`, and of its
/// `XDG_DATA_HOME` where the row's is not `null`, is the row's `folder`,
/// which the modules' `dataDir` gives of the same row.
#[test]
fn the_modules_data_folder_is_the_runners() {
    let table = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../nix/tests/data-dirs.json");
    let text = std::fs::read_to_string(&table).expect("nix/tests/data-dirs.json");
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text).expect("a list of rows");
    assert!(!rows.is_empty(), "nix/tests/data-dirs.json holds no row");
    for row in &rows {
        let given = |name: &str| row[name].as_str().map(OsString::from);
        let env = config::Env::read(|name| match name {
            "HOME" | "XDG_DATA_HOME" => given(name),
            _ => None,
        })
        .unwrap_or_else(|refusal| panic!("{row}: {refusal}"));
        let folder = row["folder"].as_str().expect("the row's folder");
        assert_eq!(
            config::data_dir(&env),
            Path::new(folder),
            "the runner's data folder of {row}"
        );
    }
}

/// "The runner's own clone rule": for each row of
/// `nix/tests/clone-dirs.json`, the runner's clone folder of the row's
/// `address` under its `dataDir` is the row's `folder`, which the modules'
/// `cloneDirs` gives of the same row.
#[test]
fn the_modules_clone_folders_are_the_runners() {
    let table = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../nix/tests/clone-dirs.json");
    let text = std::fs::read_to_string(&table).expect("nix/tests/clone-dirs.json");
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text).expect("a list of rows");
    assert!(!rows.is_empty(), "nix/tests/clone-dirs.json holds no row");
    for row in &rows {
        let field = |name: &str| {
            row[name]
                .as_str()
                .unwrap_or_else(|| panic!("{row}: no {name}"))
        };
        let address = config::Address::parse(field("address"))
            .unwrap_or_else(|refusal| panic!("{row}: {refusal}"));
        assert_eq!(
            clone::container(Path::new(field("dataDir")), &address),
            Path::new(field("folder")),
            "the runner's clone folder of {row}"
        );
    }
}

/// "The runner's own view rule" and "A repository root the runner
/// refuses": for each row of `nix/tests/view-dirs.json`, the runner's file
/// with the row's `repository_root`, read with the row's `home`, is
/// refused where the row's `folder` is `null`, and else its view of the
/// row's `address` is the row's `folder` as text, which the modules give
/// crew as the address's `checkout`.
#[test]
fn the_modules_views_are_the_runners() {
    let table = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../nix/tests/view-dirs.json");
    let text = std::fs::read_to_string(&table).expect("nix/tests/view-dirs.json");
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text).expect("a list of rows");
    assert!(!rows.is_empty(), "nix/tests/view-dirs.json holds no row");
    for row in &rows {
        let field = |name: &str| {
            row[name]
                .as_str()
                .unwrap_or_else(|| panic!("{row}: no {name}"))
        };
        let mut file = String::from("accounts = {}\n");
        if let Some(root) = row["repository_root"].as_str() {
            let root = toml::Value::String(root.to_owned());
            file.push_str(&format!("repository_root = {root}\n"));
        }
        let parsed = config::parse(&file, Path::new("view-dirs.toml"), Path::new(field("home")));
        match row["folder"].as_str() {
            None => assert!(
                parsed.is_err(),
                "the runner takes the repository root of {row}"
            ),
            Some(folder) => {
                let config = parsed.unwrap_or_else(|refusal| panic!("{row}: {refusal}"));
                let address = config::Address::parse(field("address"))
                    .unwrap_or_else(|refusal| panic!("{row}: {refusal}"));
                assert_eq!(
                    config::view_path(&config, &address).to_str(),
                    Some(folder),
                    "the runner's view of {row}"
                );
            }
        }
    }
}
