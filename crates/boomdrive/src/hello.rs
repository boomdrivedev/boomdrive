//! The hello (design Decision 6): what the runner tells its control plane
//! of the host at every pull. It holds the runner's version and its
//! tools', the repositories it keeps with their clones made, apart from
//! them those it drains (record 0021), Flume's host
//! part as Flume printed it, Flume's status of each repository it keeps
//! as Flume printed it, and the leases it holds. It is the Host
//! object's copy of the host (cloud record 0005), each part as its tool
//! gave it (core.11), and registers no repository to anything (cloud
//! record 0007).

use crate::config::Address;
use crate::flume::{Reply, Unrun};
use crate::{facts, report};
use boomdrive_control::{
    Capacity, Hello, HostPart, KeptRepository, Report, RepositoryStatus, ToolVersion,
    INTERFACE_VERSION,
};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// The tools whose versions every hello names: the runner runs no
/// multiplexer's `--version`, and names none (core.29; record 0004 as
/// amended on 2026-11-15).
pub const TOOLS: [&str; 3] = ["flume", "sluice", "git"];

/// The tools' versions as the runner reads them: `Cli` on a host, a fake
/// in a test of the pass.
pub trait Tools {
    /// The first line of `<tool> --version`, or why there is none.
    fn version(&self, tool: &str) -> ToolVersion;
}

/// Each tool's `--version`, run by its name on the host's `PATH`, or by
/// the program given for it.
#[derive(Clone, Debug, Default)]
pub struct Cli {
    programs: BTreeMap<String, PathBuf>,
}

impl Cli {
    /// Each tool by its name on the host's `PATH`.
    pub fn on_path() -> Cli {
        Cli::default()
    }

    /// `tool` run as `program`.
    pub fn with(mut self, tool: &str, program: impl Into<PathBuf>) -> Cli {
        self.programs.insert(tool.to_string(), program.into());
        self
    }
}

impl Tools for Cli {
    fn version(&self, tool: &str) -> ToolVersion {
        let program = self
            .programs
            .get(tool)
            .cloned()
            .unwrap_or_else(|| PathBuf::from(tool));
        let output = match Command::new(program)
            .arg("--version")
            .stdin(Stdio::null())
            .output()
        {
            Ok(output) => output,
            Err(error) => return ToolVersion::Missing(format!("{tool} could not run: {error}")),
        };
        let first = |bytes: &[u8]| {
            String::from_utf8_lossy(bytes)
                .lines()
                .next()
                .unwrap_or_default()
                .to_string()
        };
        if !output.status.success() {
            return ToolVersion::Missing(format!(
                "{tool} --version {}: {}",
                output.status,
                first(&output.stderr)
            ));
        }
        match first(&output.stdout) {
            line if line.is_empty() => {
                ToolVersion::Missing(format!("{tool} --version printed nothing"))
            }
            line => ToolVersion::Printed(line),
        }
    }
}

/// `boomdrive --version`'s text.
pub fn runner_version() -> String {
    format!("boomdrive {}", env!("CARGO_PKG_VERSION"))
}

/// The host's capacity, from the `flume --json status` a pass reads before its pull: its
/// host part's `running`, `bound`, `room` and `streams` as Flume printed
/// each, and none of its `pools`, `addresses`, `bases` or `terminal_kept`,
/// which are Flume's bookkeeping; else why it could not be read.
pub fn capacity(status: &Result<Reply, Unrun>) -> Capacity {
    match status {
        Ok(Reply::Given(text)) => match facts::host_part(text) {
            Some(part) => Capacity::Printed(HostPart {
                running: part.running,
                bound: part.bound,
                room: part.room,
                streams: part.streams,
            }),
            None => Capacity::Missing(
                "Flume's status holds no host part with its running, bound, room and streams"
                    .to_string(),
            ),
        },
        Ok(Reply::Refused(words)) => Capacity::Missing(words.clone()),
        Err(Unrun(words)) => Capacity::Missing(words.clone()),
    }
}

/// The folder of each clone of `drives`, once each, in order: the
/// repositories whose status the pass asks Flume for.
pub fn folders(drives: &BTreeMap<String, Vec<KeptRepository>>) -> Vec<String> {
    let mut folders: Vec<String> = drives
        .values()
        .flatten()
        .map(|kept| kept.container_dir.clone())
        .collect();
    folders.sort();
    folders.dedup();
    folders
}

/// Flume's status of each repository of `drives`, by its address, from
/// the status a pass reads before its pull: the item of
/// `status.repositories` that names its clone's folder, as Flume printed
/// it; else why there is none, Flume's own words where its status could
/// not be read.
pub fn repository_statuses(
    status: &Result<Reply, Unrun>,
    drives: &BTreeMap<String, Vec<KeptRepository>>,
) -> BTreeMap<String, RepositoryStatus> {
    let items = match status {
        Ok(Reply::Given(text)) => facts::repositories(text).ok_or_else(|| {
            "Flume's status is no JSON holding a status, so it gives no repository".to_string()
        }),
        Ok(Reply::Refused(words)) => Err(words.clone()),
        Err(Unrun(words)) => Err(words.clone()),
    };
    let mut statuses = BTreeMap::new();
    for kept in drives.values().flatten() {
        let status = match &items {
            Ok(items) => match items.get(&kept.container_dir) {
                Some(item) => RepositoryStatus::Printed(item.clone()),
                None => RepositoryStatus::Missing(
                    "Flume's status names no status of the repository's clone".to_string(),
                ),
            },
            Err(words) => RepositoryStatus::Missing(words.clone()),
        };
        statuses.entry(kept.address.clone()).or_insert(status);
    }
    statuses
}

/// The hello for a pull: the tools' versions as `tools` reads them, the
/// addresses kept with their clones made, the host's capacity, and each
/// lease `report` holds. It names no repository drained, no drive and no
/// repository's status: the pass sets them apart, from [`draining`], the
/// clones' drives and [`repository_statuses`].
pub fn hello(
    tools: &dyn Tools,
    repositories: Vec<String>,
    capacity: Capacity,
    report: &Report,
) -> Hello {
    Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: runner_version(),
        tool_versions: TOOLS
            .iter()
            .map(|tool| (tool.to_string(), tools.version(tool)))
            .collect(),
        repositories,
        draining: Vec::new(),
        drives: BTreeMap::new(),
        repository_statuses: BTreeMap::new(),
        capacity,
        leases: report.leases(),
    }
}

/// The repositories the runner drains, for the hello to name apart from
/// those it keeps: each address of `dropped`, whose clone stands while the
/// configuration does not name it, that a launch of `report`, the report
/// the pass read before its pull, holds at a place that has not ended
/// (record 0021).
pub fn draining(dropped: Vec<String>, report: &Report) -> Vec<String> {
    dropped
        .into_iter()
        .filter(|address| {
            Address::parse(address)
                .is_ok_and(|address| !report::holding(report, &address).is_empty())
        })
        .collect()
}
