//! The rules held over the tree (Task notes 9.2). core.2: crew's crate is
//! the one that names crew, `planes.rs` the one file of the binary's that
//! names it, and the crates depend on each other as design Decision 1 has
//! them (the control-plane-interface spec's "Only crew's crate names
//! crew"); the hosted implementation's crate is the one that holds the
//! runner's protocol, only the binary's crate depends on it, and
//! `planes.rs` is the one file of the binary's sources that names it (its
//! "The protocol is held in the hosted implementation's crate alone").
//! core.7: every crate is a
//! moon project of its own, its language declared, whose check-fast tasks
//! format, lint, type-check and test it, one task per check, each taking
//! the sources of every crate it builds. core.52: no crate's source asks
//! sluice for a branch. core.3's test, `no_listener.rs`, reads every
//! crate's sources beside these.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The tree's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the tree's root")
}

/// The name of each directory under `crates/`, in name order.
fn crates() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(root().join("crates"))
        .expect("crates/")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.is_dir())
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert!(!names.is_empty(), "no crate read");
    names
}

/// Each file under `dir`.
fn files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).expect("a directory") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            found.extend(files(&path));
        } else {
            found.push(path);
        }
    }
    found
}

/// The crate `name`'s `Cargo.toml`.
fn manifest(name: &str) -> toml::Table {
    let file = root().join("crates").join(name).join("Cargo.toml");
    let text = std::fs::read_to_string(&file).expect("a Cargo.toml");
    toml::from_str(&text).expect("a Cargo.toml that reads")
}

/// The crates of this tree the crate `name` depends on by path, its
/// tests' among them, by their directory's name.
fn path_dependencies(name: &str) -> BTreeSet<String> {
    let manifest = manifest(name);
    ["dependencies", "dev-dependencies"]
        .iter()
        .filter_map(|table| manifest.get(*table)?.as_table())
        .flat_map(|table| table.values())
        .filter_map(|dependency| dependency.get("path")?.as_str())
        .map(|path| {
            Path::new(path)
                .file_name()
                .expect("a crate's directory")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

/// The crates the crate `name` builds: itself, and each it depends on by
/// path, through those it depends on.
fn built(name: &str) -> BTreeSet<String> {
    let mut built = BTreeSet::from([name.to_string()]);
    let mut next = vec![name.to_string()];
    while let Some(crate_name) = next.pop() {
        for dependency in path_dependencies(&crate_name) {
            if built.insert(dependency.clone()) {
                next.push(dependency);
            }
        }
    }
    built
}

/// A task of a moon project, as its `moon.yml` writes it.
#[derive(Debug, Default)]
struct Task {
    command: String,
    inputs: Vec<String>,
    tags: Vec<String>,
}

/// A moon project's `language` and tasks, read from its `moon.yml` as
/// this tree writes one: each task's `command` on one line, and its
/// `inputs` and `tags` each a list on one line.
#[derive(Debug, Default)]
struct Project {
    language: Option<String>,
    tasks: BTreeMap<String, Task>,
}

/// A YAML scalar as written, its quotes left off.
fn unquoted(value: &str) -> String {
    value
        .trim()
        .trim_matches('\'')
        .trim_matches('"')
        .to_string()
}

/// A YAML list written on one line, `[a, 'b']`.
fn listed(value: &str) -> Vec<String> {
    let inner = value
        .trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or_else(|| panic!("{value:?} is no list on one line"));
    inner
        .split(',')
        .map(unquoted)
        .filter(|item| !item.is_empty())
        .collect()
}

/// The moon project of the crate `name`.
fn project(name: &str) -> Project {
    let file = root().join("crates").join(name).join("moon.yml");
    let text = std::fs::read_to_string(&file).expect("a moon.yml");
    let mut project = Project::default();
    let mut in_tasks = false;
    let mut task: Option<String> = None;
    for line in text.lines() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        if !line.starts_with(' ') {
            in_tasks = line == "tasks:";
            if let Some(language) = line.strip_prefix("language:") {
                project.language = Some(unquoted(language));
            }
            continue;
        }
        if !in_tasks {
            continue;
        }
        if let Some(named) = line
            .strip_prefix("  ")
            .filter(|rest| !rest.starts_with(' '))
            .and_then(|rest| rest.strip_suffix(':'))
        {
            task = Some(named.to_string());
            project.tasks.insert(named.to_string(), Task::default());
            continue;
        }
        let Some((key, value)) = line
            .strip_prefix("    ")
            .filter(|rest| !rest.starts_with(' '))
            .and_then(|rest| rest.split_once(':'))
        else {
            continue;
        };
        let task = task
            .as_ref()
            .and_then(|task| project.tasks.get_mut(task))
            .unwrap_or_else(|| panic!("{}: {line:?} is in no task", file.display()));
        match key {
            "command" => task.command = unquoted(value),
            "inputs" => task.inputs = listed(value),
            "tags" => task.tags = listed(value),
            _ => {}
        }
    }
    project
}

/// Each project `.moon/workspace.yml` lists, by its name, with its path.
fn workspace_projects() -> BTreeMap<String, String> {
    let text = std::fs::read_to_string(root().join(".moon/workspace.yml")).expect("the workspace");
    let mut projects = BTreeMap::new();
    let mut in_projects = false;
    for line in text.lines() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        if !line.starts_with(' ') {
            in_projects = line == "projects:";
            continue;
        }
        if let Some((name, path)) = line.trim().split_once(':').filter(|_| in_projects) {
            projects.insert(name.to_string(), unquoted(path));
        }
    }
    projects
}

/// core.2, the control-plane-interface spec's "Only crew's crate names
/// crew": no file of `crates/boomdrive/src` but `planes.rs`, and none of
/// `crates/boomdrive-control`, names `boomdrive_crew`.
#[test]
fn only_crews_crate_names_crew() {
    let root = root();
    let planes = root.join("crates/boomdrive/src/planes.rs");
    let read: Vec<PathBuf> = files(&root.join("crates/boomdrive/src"))
        .into_iter()
        .filter(|file| *file != planes)
        .chain(files(&root.join("crates/boomdrive-control")))
        .collect();
    assert!(read.len() > 10, "{read:?}");
    for file in read {
        let text = std::fs::read_to_string(&file).expect("a file");
        for name in ["boomdrive_crew", "boomdrive-crew"] {
            assert!(!text.contains(name), "{} names {name}", file.display());
        }
    }
    let planes = std::fs::read_to_string(planes).expect("planes.rs");
    assert!(
        planes.contains("boomdrive_crew::open"),
        "planes.rs opens crew"
    );
}

/// core.2 and design Decision 1: `boomdrive-control` depends on no
/// implementation; `boomdrive-crew` on `boomdrive-control`, weir-branch and
/// weir-capture; `boomdrive-hosted` on `boomdrive-control` alone of the
/// tree's crates; and the binary's crate on the interface and the
/// implementations it lists. Record 0042 as amended on 2026-11-11 and
/// core.58: weir's crate `weir-bounded`, which runs a git call to a remote
/// to the family's limit, is the binary's crate's and crew's
/// implementation's, and no crate of the tree's own holds it.
#[test]
fn the_crates_depend_as_decision_1_has_them() {
    let by_path: BTreeMap<String, BTreeSet<String>> = crates()
        .iter()
        .map(|name| (name.clone(), path_dependencies(name)))
        .collect();
    let set = |names: &[&str]| names.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        by_path,
        BTreeMap::from([
            (
                "boomdrive".to_string(),
                set(&["boomdrive-control", "boomdrive-crew", "boomdrive-hosted"])
            ),
            ("boomdrive-control".to_string(), set(&[])),
            ("boomdrive-crew".to_string(), set(&["boomdrive-control"])),
            ("boomdrive-hosted".to_string(), set(&["boomdrive-control"])),
        ])
    );
    let crew = manifest("boomdrive-crew");
    let dependencies = crew["dependencies"].as_table().expect("its dependencies");
    for name in [
        "boomdrive-control",
        "weir-branch",
        "weir-capture",
        "weir-bounded",
    ] {
        assert!(
            dependencies.contains_key(name),
            "boomdrive-crew lacks {name}"
        );
    }
    let binary = manifest("boomdrive");
    let dependencies = binary["dependencies"].as_table().expect("its dependencies");
    assert!(
        dependencies.contains_key("weir-bounded"),
        "boomdrive lacks weir-bounded"
    );
}

/// core.2 and core.42, the control-plane-interface spec's "Only the
/// hosted implementation's crate holds the protocol": no file of
/// `crates/boomdrive/src` but `planes.rs`, which opens it, and none of
/// `crates/boomdrive-control` or `crates/boomdrive-crew`, names it; no file
/// under `crates/` outside `crates/boomdrive-hosted` is a JSON Schema;
/// only the binary's crate depends on it, which depends on
/// `boomdrive-control` alone of the tree's crates.
#[test]
fn only_the_hosted_implementations_crate_holds_the_protocol() {
    let root = root();
    let hosted = root.join("crates/boomdrive-hosted");
    let planes = root.join("crates/boomdrive/src/planes.rs");
    let read: Vec<PathBuf> = files(&root.join("crates/boomdrive/src"))
        .into_iter()
        .filter(|file| *file != planes)
        .chain(files(&root.join("crates/boomdrive-control")))
        .chain(files(&root.join("crates/boomdrive-crew")))
        .collect();
    assert!(read.len() > 30, "{read:?}");
    for file in read {
        let text = String::from_utf8_lossy(&std::fs::read(&file).expect("a file")).into_owned();
        for named in ["boomdrive_hosted", "boomdrive-hosted"] {
            assert!(!text.contains(named), "{} names {named}", file.display());
        }
    }
    let listed: Vec<PathBuf> = files(&root.join("crates"))
        .into_iter()
        .filter(|file| !file.starts_with(&hosted))
        .collect();
    for file in listed {
        let name = file.file_name().unwrap().to_string_lossy();
        assert!(
            !name.ends_with(".schema.json"),
            "{} is a JSON Schema outside the hosted implementation's crate",
            file.display()
        );
    }
    let planes = std::fs::read_to_string(planes).expect("planes.rs");
    assert!(
        planes.contains("boomdrive_hosted::open"),
        "planes.rs opens the hosted implementation"
    );
    for name in crates() {
        let dependencies = path_dependencies(&name);
        match name.as_str() {
            "boomdrive-hosted" => assert_eq!(
                dependencies,
                BTreeSet::from(["boomdrive-control".to_string()])
            ),
            "boomdrive" => assert!(
                dependencies.contains("boomdrive-hosted"),
                "boomdrive does not depend on boomdrive-hosted"
            ),
            _ => assert!(
                !dependencies.contains("boomdrive-hosted"),
                "{name} depends on boomdrive-hosted"
            ),
        }
    }
}

/// core.7: every crate is a crate of the cargo workspace, and a moon
/// project of its own in `.moon/workspace.yml`, its language declared as
/// rust.
#[test]
fn every_crate_is_a_moon_project_of_its_own() {
    let projects = workspace_projects();
    let workspace: toml::Table =
        toml::from_str(&std::fs::read_to_string(root().join("Cargo.toml")).expect("Cargo.toml"))
            .expect("Cargo.toml reads");
    let members: Vec<&str> = workspace["workspace"]["members"]
        .as_array()
        .expect("the workspace's members")
        .iter()
        .filter_map(|member| member.as_str())
        .collect();
    for name in crates() {
        let path = format!("crates/{name}");
        assert!(members.contains(&path.as_str()), "{path} is no member");
        assert_eq!(
            projects.get(&name),
            Some(&path),
            "{name} is no project of .moon/workspace.yml: {projects:?}"
        );
        assert_eq!(project(&name).language.as_deref(), Some("rust"), "{name}");
    }
}

/// core.7: each crate's check-fast tasks format, lint, type-check and
/// test it, one task per check, each one command.
#[test]
fn each_check_is_one_task() {
    let checks = [
        ("format", "cargo fmt --check -p"),
        ("lint", "cargo clippy -p"),
        ("typecheck", "cargo check -p"),
        ("test", "cargo test -p"),
    ];
    for name in crates() {
        let project = project(&name);
        let package = manifest(&name)["package"]["name"]
            .as_str()
            .expect("a package's name")
            .to_string();
        for (check, command) in checks {
            let task = project
                .tasks
                .get(check)
                .unwrap_or_else(|| panic!("{name} has no task {check}"));
            let named = format!("{command} {package}");
            assert!(
                task.command == named || task.command.starts_with(&format!("{named} ")),
                "{name}:{check} runs {:?}",
                task.command
            );
            for joined in ["&&", "||", ";", "|"] {
                assert!(
                    !task.command.contains(joined),
                    "{name}:{check} runs more than one command"
                );
            }
            assert!(
                task.tags.iter().any(|tag| tag == "check-fast"),
                "{name}:{check} is not in check-fast"
            );
        }
    }
}

/// core.7: each check-fast task takes as inputs the sources of every
/// crate it builds: its own, and each it depends on by path.
#[test]
fn each_task_takes_the_sources_of_every_crate_it_builds() {
    for name in crates() {
        let built = built(&name);
        for (check, task) in project(&name).tasks {
            if !task.tags.iter().any(|tag| tag == "check-fast") {
                continue;
            }
            assert!(!task.inputs.is_empty(), "{name}:{check} reads no inputs");
            for crate_name in &built {
                let sources = format!("/crates/{crate_name}/**/*");
                assert!(
                    task.inputs.contains(&sources),
                    "{name}:{check} builds {crate_name} and does not take {sources}"
                );
            }
        }
    }
}

/// core.52: no `.rs` file under any crate's `src` holds the argument list
/// of `sluice branch create` or `sluice branch delete`, so no crate asks
/// sluice for a branch (Task notes 2.2).
#[test]
fn no_crate_asks_sluice_for_a_branch() {
    let root = root();
    let read: Vec<PathBuf> = crates()
        .iter()
        .flat_map(|name| files(&root.join("crates").join(name).join("src")))
        .filter(|file| file.extension().is_some_and(|extension| extension == "rs"))
        .collect();
    assert!(read.len() > 20, "{read:?}");
    let asking: Vec<String> = read
        .iter()
        .filter(|file| {
            let text = std::fs::read_to_string(file).expect("a file");
            ["create", "delete"]
                .iter()
                .any(|verb| text.contains(&format!("\"branch\", \"{verb}\"")))
        })
        .map(|file| file.display().to_string())
        .collect();
    assert!(
        asking.is_empty(),
        "these ask sluice for a branch (core.52): {asking:?}"
    );
}

/// The reader of moon's files reads this tree's own shapes.
#[test]
fn moons_files_are_read() {
    let project = project("boomdrive");
    assert_eq!(
        project.tasks.keys().collect::<Vec<_>>(),
        ["format", "lint", "test", "typecheck"]
    );
    assert!(project.tasks["test"]
        .inputs
        .contains(&"/README.md".to_string()));
    assert_eq!(project.tasks["lint"].tags, ["check-fast"]);
    assert_eq!(
        workspace_projects().get("root").map(String::as_str),
        Some(".")
    );
    assert_eq!(listed("['/a/**/*', '/b']"), ["/a/**/*", "/b"]);
    assert_eq!(
        built("boomdrive"),
        BTreeSet::from([
            "boomdrive".to_string(),
            "boomdrive-control".to_string(),
            "boomdrive-crew".to_string(),
            "boomdrive-hosted".to_string()
        ])
    );
}
