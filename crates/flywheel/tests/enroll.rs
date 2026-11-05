//! `flywheel enroll` (design Decisions 4 and 14; the hosted-control-plane
//! spec's "A partition is enrolled once by flywheel enroll", its
//! scenarios "The answer as JSON" and "The token never shown", and
//! "flywheel enroll refuses what it cannot enrol"). The binary runs under
//! the scratch host, its token piped to it, with a stub `systemd-creds`
//! first on its `PATH` and a loopback port with nothing on it as its
//! control plane, so no test reaches a server, a keychain or a
//! credential: on macOS the scratch host's `HOME` holds no keychain, and
//! an enrolment that is refused writes no key.

mod common;
mod fakes;

use common::{keeping, Scratch};
use fakes::CredsStub;
use serde_json::Value;
use std::ffi::OsString;
use std::io::Write;
use std::process::{Output, Stdio};

/// The token every enrolment here is given, a test's own.
const TOKEN: &str = "secret-enrolment-token";

/// A loopback address with nothing listening on it.
fn loopback_with_nothing_on_it() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}/flywheel")
}

/// A configuration naming the hosted control plane at `url`, with the
/// partitions `home` and `work`.
fn hosted(url: &str) -> String {
    format!(
        "{}[control_plane]\ntype = \"hosted\"\n\n\
         [control_plane.hosted]\nurl = {url:?}\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"work\"\n\
         repositories = [\"example/service\", \"example/app\"]\n",
        keeping(&["example/app", "example/service"])
    )
}

/// The scratch host and its stub `systemd-creds`.
struct Host {
    scratch: Scratch,
    creds: CredsStub,
}

impl Host {
    /// A scratch host whose configuration is `text`.
    fn configured(text: &str) -> Host {
        let host = Host {
            scratch: Scratch::new(),
            creds: CredsStub::new(),
        };
        host.scratch.configure(text);
        host
    }

    /// `flywheel enroll` on `args`, given `stdin` on standard input.
    fn enroll(&self, args: &[&str], stdin: &str) -> Output {
        let rest = std::env::var_os("PATH").unwrap_or_default();
        let path: OsString = std::env::join_paths(
            std::iter::once(self.creds.dir.path().to_path_buf())
                .chain(std::env::split_paths(&rest)),
        )
        .expect("a PATH");
        let mut child = self
            .scratch
            .flywheel()
            .arg("enroll")
            .args(args)
            .env("PATH", path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("flywheel runs");
        child
            .stdin
            .take()
            .expect("its standard input")
            .write_all(stdin.as_bytes())
            .expect("the token given");
        child.wait_with_output().expect("flywheel answers")
    }

    /// Nothing was kept: no key by the stub store, and no data
    /// directory made.
    fn kept_nothing(&self) {
        assert!(!self.creds.kept_or_read(), "{:?}", self.creds.calls());
        assert!(
            !self.scratch.data().exists(),
            "{} was made",
            self.scratch.data().display()
        );
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The hosted-control-plane spec's "The answer as JSON": standard output
/// is one object, a refusal naming the control plane as unreachable, and
/// the command exits 1.
#[test]
fn the_answer_as_json() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(&["--json", "home"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let text = stdout(&output);
    assert_eq!(text.lines().count(), 1, "{text}");
    let answer: Value = serde_json::from_str(&text).expect("one JSON object");
    let reason = answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("{answer} is no refusal"));
    assert!(reason.contains("is unreachable"), "{reason}");
    assert_eq!(
        answer,
        serde_json::json!({"refused": {"refusal": {"reason": reason}}})
    );
    host.kept_nothing();
}

/// The hosted-control-plane spec's "The token never shown": neither
/// standard output nor standard error holds the token, in words or as
/// JSON.
#[test]
fn the_token_never_shown() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    for args in [&["home"][..], &["--json", "home"]] {
        let output = host.enroll(args, &format!("{TOKEN}\n"));

        assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
        assert!(!stdout(&output).contains(TOKEN), "{}", stdout(&output));
        assert!(!stderr(&output).contains(TOKEN), "{}", stderr(&output));
    }
}

/// The hosted-control-plane spec's "crew's control plane": refused naming
/// crew's control plane as one that enrols nothing, exiting 2.
#[test]
fn crews_control_plane() {
    let host = Host::configured(&format!(
        "{}[control_plane]\ntype = \"crew\"\n\n[control_plane.crew]\nhost = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\nlabel = \"home\"\n\
         state_repository = \"example/state\"\n",
        keeping(&["example/app"])
    ));

    let output = host.enroll(&["home"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(
        said.contains("`type` is \"crew\", a control plane that enrols nothing"),
        "{said}"
    );
    assert_eq!(stdout(&output), "");
    host.kept_nothing();
}

/// The hosted-control-plane spec's "An unknown label": refused naming
/// `garden`, exiting 2, before any request.
#[test]
fn an_unknown_label() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(&["garden"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("`garden`"), "{}", stderr(&output));
    host.kept_nothing();
}

/// The hosted-control-plane spec's "Nothing listening": refused naming the
/// control plane as unreachable, exiting 1, with no key written.
#[test]
fn nothing_listening() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(&["home"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("is unreachable"),
        "{}",
        stderr(&output)
    );
    assert_eq!(stdout(&output), "");
    host.kept_nothing();
}

/// Design Decision 14: an empty token is refused, exiting 2, before any
/// request.
#[test]
fn an_empty_token() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    for given in ["", "\n"] {
        let output = host.enroll(&["home"], given);

        assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
        assert!(
            stderr(&output).contains("the enrolment token is empty"),
            "{}",
            stderr(&output)
        );
    }
    host.kept_nothing();
}

/// A configuration naming no control plane has no partition to enrol,
/// and is refused, exiting 2.
#[test]
fn no_control_plane() {
    let host = Host::configured(&keeping(&["example/app"]));

    let output = host.enroll(&["--json", "home"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let answer: Value = serde_json::from_str(&stdout(&output)).expect("one JSON object");
    let reason = answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("{answer} is no refusal"));
    assert!(reason.contains("names no `[control_plane]`"), "{reason}");
    host.kept_nothing();
}

/// A usage error, the label missing, exits 2.
#[test]
fn no_label() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(&[], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    host.kept_nothing();
}
