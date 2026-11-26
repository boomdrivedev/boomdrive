//! `boomdrive enroll`'s ways, on a scratch home, with a fake enrollment in
//! place of the control plane and the credential store, and a fake agent
//! or a stand-in `launchctl`, so no test reaches a server, a keychain or
//! launchd (the hosted-control-plane spec, as the change
//! a-host-enrolls-from-one-command amends it; the configuration spec's "A
//! partition enrolled from the local layer").

use super::{enroll, Answer, Asked, Host};
use crate::config::{self, Env};
use crate::planes::{Enrolled, Refusal, Token};
use crate::setup;
use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};

const URL: &str = "https://control.example";

/// A scratch home, its runner file at the default path.
struct Home {
    dir: tempfile::TempDir,
}

impl Home {
    fn new() -> Home {
        Home {
            dir: tempfile::tempdir().expect("a scratch home"),
        }
    }

    fn env(&self) -> Env {
        Env {
            home: self.dir.path().to_path_buf(),
            config_home: None,
            data_home: None,
        }
    }

    fn managed(&self) -> PathBuf {
        self.dir.path().join(".config/boomdrive/config.toml")
    }

    fn local(&self) -> PathBuf {
        self.dir.path().join(".config/boomdrive/config.local.toml")
    }

    fn write(&self, path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    /// The runner's file as a link to `text`, as Nix's activation makes
    /// it.
    fn link(&self, text: &str) -> PathBuf {
        let target = self.dir.path().join("store-config.toml");
        std::fs::write(&target, text).unwrap();
        std::fs::create_dir_all(self.managed().parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(&target, self.managed()).unwrap();
        target
    }
}

fn asked(drive: &str, repositories: &[&str]) -> Asked {
    Asked {
        control_plane: URL.to_string(),
        drive: drive.to_string(),
        repositories: repositories.iter().map(ToString::to_string).collect(),
    }
}

/// What a run of enroll did beside its answer.
#[derive(Default)]
struct Seen {
    token_read: Cell<bool>,
    enrolled: RefCell<Option<(toml::Table, String)>>,
    agent: RefCell<Option<bool>>,
}

/// enroll of `asked` on `home` under `system`, its enrollment answering
/// `answer` once it read the token, its agent answering `loaded`.
fn run(
    home: &Home,
    system: &str,
    asked: &Asked,
    answer: Result<Enrolled, Refusal>,
) -> (Result<Answer, Refusal>, Seen) {
    let seen = Seen::default();
    let env = home.env();
    let host_of = Host {
        system,
        env: &env,
        config: None,
    };
    let result = enroll(
        asked,
        &host_of,
        |table: &toml::Table, _file: &Path, label: &str, token: Token<'_>| {
            assert_eq!(token().as_deref(), Ok("a-token"));
            *seen.enrolled.borrow_mut() = Some((table.clone(), label.to_string()));
            answer
        },
        |_home: &Path, managed: &Path, restart: bool| {
            *seen.agent.borrow_mut() = Some(restart);
            Ok(
                json!({"file": managed.with_extension("plist").display().to_string(), "outcome": "loaded"}),
            )
        },
        || {
            seen.token_read.set(true);
            Ok("a-token".to_string())
        },
    );
    (result, seen)
}

fn enrolled() -> Result<Enrolled, Refusal> {
    Ok(Enrolled {
        host: "h-1".to_string(),
    })
}

/// A refusal the host must fix, its words.
fn host_refusal(result: Result<Answer, Refusal>) -> String {
    let refusal = result.expect_err("a refusal");
    assert!(refusal.host_to_fix, "{}", refusal.words);
    refusal.words
}

/// Nothing was asked of the host: no token read, no enrollment, no agent.
fn nothing_asked(seen: &Seen) {
    assert!(!seen.token_read.get(), "the token was read");
    assert!(seen.enrolled.borrow().is_none(), "an enrollment was sent");
    assert!(seen.agent.borrow().is_none(), "the agent was loaded");
}

/// A file setup wrote, holding the account `own`, served by crew, with
/// its drive `home` keeping `example/app`, and crew's table.
fn setups_crew_file() -> String {
    format!(
        "{}\n[accounts.own]\ncontrol_plane = \"crew\"\n\n[accounts.own.drives.home]\n\
         repositories = [\"example/app\"]\n\n[control_plane.crew]\nhost = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\nlabel = \"home\"\nstate_repository = \"example/state\"\n",
        setup::FIRST_LINE
    )
}

/// A file naming the account `atlas` served by the hosted control plane
/// at `URL`, its drive and its partition `atlas`, each with
/// `madswan/atlas`.
fn names_atlas() -> String {
    format!(
        "[accounts.atlas]\ncontrol_plane = \"hosted\"\n\n[accounts.atlas.drives.atlas]\n\
         repositories = [\"madswan/atlas\"]\n\n[control_plane.hosted]\nurl = \"{URL}\"\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"atlas\"\nrepositories = [\"madswan/atlas\"]\n"
    )
}

/// "A drive name out of form".
#[test]
fn a_drive_name_out_of_form() {
    let home = Home::new();
    let (result, seen) = run(&home, "macos", &asked("Home", &["example/app"]), enrolled());
    let words = host_refusal(result);
    assert!(words.contains("Home"), "{words}");
    assert!(words.contains(config::NAME_FORM), "{words}");
    nothing_asked(&seen);
    assert!(!home.managed().exists());
}

/// A repository out of form is refused before the token.
#[test]
fn a_repository_out_of_form() {
    let home = Home::new();
    let (result, seen) = run(&home, "macos", &asked("atlas", &["atlas"]), enrolled());
    assert!(host_refusal(result).contains("`--repository atlas`"));
    nothing_asked(&seen);
}

/// "The names" and "A host with no file": the account, the drive and the
/// partition named after the drive, the file written after the
/// enrollment with setup's first line, and the agent loaded to read it.
#[test]
fn a_host_with_no_file() {
    let home = Home::new();
    let (result, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );

    let answer = result.expect("enrolled");
    assert_eq!(answer.partition, "atlas");
    assert_eq!(answer.host, "h-1");
    assert_eq!(answer.config["outcome"], "written");
    assert_eq!(*seen.agent.borrow(), Some(true));
    let text = std::fs::read_to_string(home.managed()).unwrap();
    assert_eq!(text.lines().next(), Some(setup::FIRST_LINE));
    let read = config::read(&home.managed(), home.dir.path()).expect("the runner reads it");
    let account = &read.accounts[0];
    assert_eq!(account.name, "atlas");
    assert_eq!(account.control_plane.as_deref(), Some("hosted"));
    assert_eq!(account.drives[0].name, "atlas");
    assert_eq!(
        account.drives[0].repositories[0].to_string(),
        "madswan/atlas"
    );
    let table = read
        .control_plane
        .as_ref()
        .unwrap()
        .table("hosted")
        .unwrap();
    assert_eq!(table["url"].as_str(), Some(URL));
    let (sent, label) = seen.enrolled.borrow().clone().unwrap();
    assert_eq!(&sent, table);
    assert_eq!(label, "atlas");
    assert!(text.contains("label = \"atlas\""), "{text}");
}

/// "Beside crew's drives": crew's account, drive and table stand as they
/// were.
#[test]
fn beside_crews_drives() {
    let home = Home::new();
    home.write(&home.managed(), &setups_crew_file());
    let (result, _) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    result.expect("enrolled");

    let read = config::read(&home.managed(), home.dir.path()).unwrap();
    let names: Vec<_> = read
        .accounts
        .iter()
        .map(|account| (account.name.as_str(), account.control_plane.as_deref()))
        .collect();
    // The writer lays accounts out by name, as Nix's module does.
    assert_eq!(names, [("atlas", Some("hosted")), ("own", Some("crew"))]);
    let plane = read.control_plane.unwrap();
    assert_eq!(
        plane.table("crew").unwrap()["host"].as_str(),
        Some("host-a")
    );
    assert!(plane.table("hosted").is_some());
}

/// "A drive another account holds": refused naming the drive and the
/// account, before the token.
#[test]
fn a_drive_another_account_holds() {
    let home = Home::new();
    home.write(&home.managed(), &setups_crew_file());
    let (result, seen) = run(
        &home,
        "macos",
        &asked("home", &["madswan/atlas"]),
        enrolled(),
    );
    let words = host_refusal(result);
    assert!(
        words.contains("`own`") || words.contains("\"own\""),
        "{words}"
    );
    assert!(words.contains("home"), "{words}");
    nothing_asked(&seen);
    assert_eq!(
        std::fs::read_to_string(home.managed()).unwrap(),
        setups_crew_file()
    );
}

/// "Another hosted control plane": refused naming both addresses, before
/// the token, writing nothing.
#[test]
fn another_hosted_control_plane() {
    let home = Home::new();
    let other = format!(
        "{}\n[accounts.cloud]\ncontrol_plane = \"hosted\"\n\n[accounts.cloud.drives.home]\n\
         repositories = [\"example/app\"]\n\n[control_plane.hosted]\nurl = \"https://other.example\"\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n",
        setup::FIRST_LINE
    );
    home.write(&home.managed(), &other);
    let (result, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    let words = host_refusal(result);
    assert!(words.contains("https://other.example"), "{words}");
    assert!(words.contains(URL), "{words}");
    nothing_asked(&seen);
    assert_eq!(std::fs::read_to_string(home.managed()).unwrap(), other);
}

/// "The token refused" and "A lapsed token": no runner file and no agent.
#[test]
fn the_token_refused() {
    let home = Home::new();
    let refused = Err(Refusal {
        words: "the token has lapsed".to_string(),
        host_to_fix: false,
    });
    let (result, seen) = run(&home, "macos", &asked("atlas", &["madswan/atlas"]), refused);
    let refusal = result.expect_err("refused");
    assert_eq!(refusal.words, "the token has lapsed");
    assert!(!refusal.host_to_fix);
    assert!(!home.managed().exists());
    assert!(seen.agent.borrow().is_none());
}

/// "A host Nix set up": the link names the enrollment, so enroll writes
/// nothing, touches no agent, and enrolls.
#[test]
fn a_host_nix_set_up() {
    let home = Home::new();
    let target = home.link(&names_atlas());
    let (result, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["MadSwan/Atlas"]),
        enrolled(),
    );

    let answer = result.expect("enrolled");
    assert_eq!(answer.config["outcome"], "unchanged");
    assert_eq!(answer.service, None);
    assert!(seen.agent.borrow().is_none());
    assert_eq!(std::fs::read_link(home.managed()).unwrap(), target);
    assert_eq!(std::fs::read_to_string(&target).unwrap(), names_atlas());
}

/// "The file is Nix's": a link that names no hosted control plane is
/// changed in nothing, the module's options named, before the token.
#[test]
fn the_file_is_nixs() {
    let home = Home::new();
    let target = home.link("accounts = {}\n");
    let (result, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    let words = host_refusal(result);
    for wanted in [
        home.managed().display().to_string(),
        "services.boomdrive.accounts.atlas = { control_plane = \"hosted\"; drives = { atlas = { repositories = { madswan = [ \"atlas\" ]; }; }; }; };".to_string(),
        "services.boomdrive.settings.control_plane.hosted".to_string(),
        "the token is not spent".to_string(),
    ] {
        assert!(words.contains(&wanted), "{wanted} in {words}");
    }
    nothing_asked(&seen);
    assert_eq!(std::fs::read_link(home.managed()).unwrap(), target);
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "accounts = {}\n");
}

/// "A person's own file": changed in nothing, the TOML keys to add named,
/// before the token.
#[test]
fn a_persons_own_file() {
    let home = Home::new();
    home.write(&home.managed(), "accounts = {}\n");
    let (result, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    let words = host_refusal(result);
    for wanted in [
        "a file a person wrote",
        "[accounts.atlas]",
        "control_plane = \"hosted\"",
        "url = \"https://control.example\"",
        "the token is not spent",
    ] {
        assert!(words.contains(wanted), "{wanted} in {words}");
    }
    nothing_asked(&seen);
    assert_eq!(
        std::fs::read_to_string(home.managed()).unwrap(),
        "accounts = {}\n"
    );
}

/// "On Linux with no file": changed in nothing, naming the system and
/// `services.boomdrive`, before the token.
#[test]
fn on_linux_with_no_file() {
    let home = Home::new();
    let (result, seen) = run(
        &home,
        "linux",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    let words = host_refusal(result);
    assert!(words.contains("linux"), "{words}");
    assert!(words.contains("`services.boomdrive`"), "{words}");
    nothing_asked(&seen);
    assert!(!home.managed().exists());
}

/// The configuration spec's "A partition enrolled from the local layer":
/// the local layer names the drive and the partition, so enroll enrolls
/// with the table both layers make, and writes nothing when the control
/// plane cannot be reached.
#[test]
fn a_partition_enrolled_from_the_local_layer() {
    let home = Home::new();
    home.write(
        &home.managed(),
        &format!(
            "[accounts.cloud]\ncontrol_plane = \"hosted\"\n\n[accounts.cloud.drives.home]\n\
             repositories = [\"example/service\"]\n\n[control_plane.hosted]\nurl = \"{URL}\"\n\n\
             [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = [\"example/service\"]\n"
        ),
    );
    home.write(
        &home.local(),
        "[accounts.cloud.drives.work]\nrepositories = [\"example/app\"]\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"work\"\nrepositories = [\"example/app\"]\n",
    );
    let before = std::fs::read_to_string(home.managed()).unwrap();
    let unreachable = Err(Refusal {
        words: "the control plane at https://control.example is unreachable".to_string(),
        host_to_fix: false,
    });
    let (result, seen) = run(
        &home,
        "macos",
        &asked("work", &["example/app"]),
        unreachable,
    );
    let refusal = result.expect_err("refused");
    assert!(!refusal.host_to_fix);
    assert!(refusal.words.contains("is unreachable"));
    assert_eq!(seen.enrolled.borrow().as_ref().unwrap().1, "work");
    assert_eq!(std::fs::read_to_string(home.managed()).unwrap(), before);
}

/// "The answer": the JSON names the partition, the host, the file and
/// the agent.
#[test]
fn the_answer() {
    let home = Home::new();
    let (result, _) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    let answer: Value = result.expect("enrolled").json();
    assert_eq!(answer["enrolled"]["partition"], "atlas");
    assert_eq!(answer["enrolled"]["host"], "h-1");
    assert_eq!(answer["enrolled"]["config"]["outcome"], "written");
    assert_eq!(
        answer["enrolled"]["config"]["file"],
        home.managed().display().to_string()
    );
    assert_eq!(answer["enrolled"]["service"]["outcome"], "loaded");
}

/// Run again with the same answers on the file it wrote, enroll writes
/// nothing and touches no agent.
#[test]
fn run_again_writes_nothing() {
    let home = Home::new();
    let (first, _) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    first.expect("enrolled");
    let text = std::fs::read_to_string(home.managed()).unwrap();
    let (again, seen) = run(
        &home,
        "macos",
        &asked("atlas", &["madswan/atlas"]),
        enrolled(),
    );
    assert_eq!(again.expect("enrolled").config["outcome"], "unchanged");
    assert!(seen.agent.borrow().is_none());
    assert_eq!(std::fs::read_to_string(home.managed()).unwrap(), text);
}

/// A stand-in `launchctl` under `home`, which appends each call's
/// arguments to `calls` and exits `print_exit` for `print`, 0 otherwise.
fn launchctl(home: &Home, print_exit: i32) -> (PathBuf, PathBuf) {
    let calls = home.dir.path().join("launchctl.calls");
    let program = home.dir.path().join("launchctl");
    std::fs::write(
        &program,
        format!(
            "#!/bin/sh\necho \"$*\" >> '{}'\nif [ \"$1\" = print ]; then exit {print_exit}; fi\nexit 0\n",
            calls.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).unwrap();
    (program, calls)
}

/// "No agent loaded": the plist written and booted.
#[test]
fn no_agent_loaded() {
    let home = Home::new();
    let (program, calls) = launchctl(&home, 1);
    let service = setup::load_agent(home.dir.path(), &home.managed(), &program, true).unwrap();
    assert_eq!(service["outcome"], "loaded");
    let calls = std::fs::read_to_string(calls).unwrap();
    assert!(calls.contains("bootstrap gui/"), "{calls}");
    assert!(!calls.contains("kickstart"), "{calls}");
}

/// "The agent running": a loaded, unchanged agent restarted by
/// `launchctl kickstart -k`, and no bootstrap; with no restart asked, it
/// is left running.
#[test]
fn the_agent_running() {
    let home = Home::new();
    let plist = home
        .dir
        .path()
        .join(format!("Library/LaunchAgents/{}.plist", setup::LABEL));
    home.write(&plist, &setup::agent(home.dir.path(), &home.managed()));
    let (program, calls) = launchctl(&home, 0);

    let service = setup::load_agent(home.dir.path(), &home.managed(), &program, true).unwrap();
    assert_eq!(service["outcome"], "restarted");
    let said = std::fs::read_to_string(&calls).unwrap();
    assert!(
        said.contains("kickstart -k gui/") && said.contains(setup::LABEL),
        "{said}"
    );
    assert!(!said.contains("bootstrap"), "{said}");

    std::fs::remove_file(&calls).unwrap();
    let service = setup::load_agent(home.dir.path(), &home.managed(), &program, false).unwrap();
    assert_eq!(service["outcome"], "running");
    assert!(!std::fs::read_to_string(&calls)
        .unwrap()
        .contains("kickstart"));
}
