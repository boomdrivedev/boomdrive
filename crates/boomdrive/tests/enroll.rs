//! `boomdrive enroll` through the binary (the hosted-control-plane spec's
//! "A partition is enrolled once by boomdrive enroll", its scenarios "The
//! answer as JSON" and "The token never shown", "boomdrive enroll refuses
//! before the token what it cannot set up" and "The token comes on
//! standard input alone", as the change a-host-enrolls-from-one-command
//! amends them). Its ways are proved by `enroll`'s own tests, with a fake
//! enrollment in place of the store. The binary runs under
//! the scratch host, its token piped to it, with a stub `systemd-creds`
//! first on its `PATH` and a loopback port with nothing on it as its
//! control plane, so no test reaches a server, a keychain or a
//! credential: on macOS the scratch host's `HOME` holds no keychain, and
//! an enrollment that is refused writes no key.

mod common;
mod fakes;

use common::{keeping, Scratch};
use fakes::CredsStub;
use serde_json::Value;
use std::ffi::OsString;
use std::io::Write;
use std::process::{Output, Stdio};

/// The token every enrollment here is given, a test's own.
const TOKEN: &str = "secret-enrollment-token";

/// A loopback address with nothing listening on it.
fn loopback_with_nothing_on_it() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}/boomdrive")
}

/// A configuration naming the hosted control plane at `url`, served the
/// account `own`, its drive `home` keeping `example/app` and
/// `example/service`, with the partitions `home` and `work`: it names the
/// enrollment of `home` at `url` serving `example/app`.
fn hosted(url: &str) -> String {
    format!(
        "{}accounts.own.control_plane = \"hosted\"\n[control_plane]\n\n\
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

    /// `boomdrive enroll` on `args`, given `stdin` on standard input.
    fn enroll(&self, args: &[&str], stdin: &str) -> Output {
        let rest = std::env::var_os("PATH").unwrap_or_default();
        let path: OsString = std::env::join_paths(
            std::iter::once(self.creds.dir.path().to_path_buf())
                .chain(std::env::split_paths(&rest)),
        )
        .expect("a PATH");
        let mut child = self
            .scratch
            .boomdrive()
            .arg("enroll")
            .args(args)
            .env("PATH", path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("boomdrive runs");
        child
            .stdin
            .take()
            .expect("its standard input")
            .write_all(stdin.as_bytes())
            .expect("the token given");
        child.wait_with_output().expect("boomdrive answers")
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

/// The flags that enroll `home` at `url`, serving `example/app`, as the
/// control plane's page gives them, after `before`.
fn flags<'a>(before: &[&'a str], url: &'a str) -> Vec<&'a str> {
    let mut flags = before.to_vec();
    flags.extend([
        "--control-plane",
        url,
        "--drive",
        "home",
        "--repository",
        "example/app",
    ]);
    flags
}

/// The hosted-control-plane spec's "The answer as JSON": standard output
/// is one object, a refusal naming the control plane as unreachable, and
/// the command exits 1.
#[test]
fn the_answer_as_json() {
    let url = loopback_with_nothing_on_it();
    let host = Host::configured(&hosted(&url));

    let output = host.enroll(&flags(&["--json"], &url), &format!("{TOKEN}\n"));

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
    let url = loopback_with_nothing_on_it();
    let host = Host::configured(&hosted(&url));

    for before in [&[][..], &["--json"]] {
        let output = host.enroll(&flags(before, &url), &format!("{TOKEN}\n"));

        assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
        assert!(!stdout(&output).contains(TOKEN), "{}", stdout(&output));
        assert!(!stderr(&output).contains(TOKEN), "{}", stderr(&output));
    }
}

/// The hosted-control-plane spec's "Nothing listening": refused naming the
/// control plane as unreachable, exiting 1, with no key and no file
/// written.
#[test]
fn nothing_listening() {
    let url = loopback_with_nothing_on_it();
    let host = Host::configured(&hosted(&url));
    let before = std::fs::read_to_string(host.scratch.config_file()).unwrap();

    let output = host.enroll(&flags(&[], &url), &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("is unreachable"),
        "{}",
        stderr(&output)
    );
    assert_eq!(stdout(&output), "");
    assert_eq!(
        std::fs::read_to_string(host.scratch.config_file()).unwrap(),
        before
    );
    host.kept_nothing();
}

/// An empty token is refused, exiting 2, before any request.
#[test]
fn an_empty_token() {
    let url = loopback_with_nothing_on_it();
    let host = Host::configured(&hosted(&url));

    for given in ["", "\n"] {
        let output = host.enroll(&flags(&[], &url), given);

        assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
        assert!(
            stderr(&output).contains("the enrollment token is empty"),
            "{}",
            stderr(&output)
        );
    }
    host.kept_nothing();
}

/// The hosted-control-plane spec's "A flag missing": `--drive` missing is
/// a usage error naming it, exiting 2, with no token read.
#[test]
fn a_flag_missing() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(
        &[
            "--control-plane",
            "https://control.example",
            "--repository",
            "example/app",
        ],
        &format!("{TOKEN}\n"),
    );

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("--drive"), "{}", stderr(&output));
    host.kept_nothing();
}

/// The hosted-control-plane spec's "A token flag": `--token` is a usage
/// error naming it, and no output holds the token.
#[test]
fn a_token_flag() {
    let url = loopback_with_nothing_on_it();
    let host = Host::configured(&hosted(&url));

    let output = host.enroll(&flags(&["--token", TOKEN], &url), "");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("--token"), "{}", stderr(&output));
    assert!(!stdout(&output).contains(TOKEN), "{}", stdout(&output));
    assert!(!stderr(&output).contains(TOKEN), "{}", stderr(&output));
    host.kept_nothing();
}

/// No positional label is taken: the form of before is a usage error.
#[test]
fn a_label_of_before() {
    let host = Host::configured(&hosted(&loopback_with_nothing_on_it()));

    let output = host.enroll(&["home"], &format!("{TOKEN}\n"));

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    host.kept_nothing();
}
