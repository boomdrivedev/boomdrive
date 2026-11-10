//! The copies of Flume's and sluice's answers hold only the fields design
//! Decisions 7 to 9 name (the runner-protocol spec's "A place's status is
//! carried as its copy", "The host's capacity is Flume's host part as
//! values, its streams without their queues" and "sluice's entry is
//! carried as its copy"; core.27). Each copy's field paths, read from the
//! schema, equal the list its decision gives, written here: a field added
//! to a copy or taken from it fails, naming it.

mod common;

use common::{address, example, files, from_runner, passes, refused};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The copy of a place's status (design Decision 7). A list's elements
/// are `[]`, and each variant of a value is a field by its name.
const PLACE_STATUS: &[&str] = &[
    "place",
    "kind",
    "version",
    "declared",
    "name",
    "mark",
    "branch",
    "base",
    "head",
    "lands",
    "delivered",
    "stalled",
    "priority",
    "current",
    "recipients",
    "stream",
    "stream.name",
    "stream.from",
    "intent",
    "intent.from",
    "intent.name",
    "intent.base",
    "intent.digest",
    "phase",
    "phase.making",
    "phase.making.by",
    "phase.over",
    "phase.over.over",
    "phase.over.over.landed",
    "phase.over.over.landed.commit",
    "phase.over.kept",
    "phase.over.kept.holds",
    "phase.over.kept.teardown",
    "phase.over.kept.teardown.step",
    "phase.over.kept.agent_live",
    "phase.over.kept.agent_live.identity",
    "phase.over.kept.unreadable",
    "phase.gone",
    "phase.gone.over",
    "phase.gone.over.landed",
    "phase.gone.over.landed.commit",
    "phase.gone.at",
    "phase.tidied",
    "phase.tidied.over",
    "phase.tidied.over.landed",
    "phase.tidied.over.landed.commit",
    "phase.tidied.at",
    "phase.tidied.pending_for",
    "stages",
    "stages[].name",
    "stages[].state",
    "stages[].starts",
    "stages[].end",
    "stages[].end.head",
    "stages[].end.words",
    "stages[].end.needs",
    "stages[].end.commits",
    "stages[].end.made",
    "stages[].end.made[].name",
    "stages[].group",
    "stages[].group.place",
    "stages[].group.of",
    "stages[].group.number",
    "ends",
    "ends[].of",
    "ends[].stage",
    "ends[].start",
    "ends[].state",
    "ends[].end",
    "ends[].end.head",
    "ends[].end.words",
    "ends[].end.needs",
    "ends[].end.commits",
    "ends[].end.made",
    "ends[].end.made[].name",
    "ends[].submit",
    "ends[].verdict",
    "ends[].verdict.landed",
    "ends[].verdict.landed.commit",
    "ends[].verdict.landed.place_removed",
    "ends[].verdict.landed.branch_removed",
    "ends[].verdict.failed",
    "ends[].verdict.conflict",
    "ends[].verdict.refused",
    "ends[].read_at",
    "ends[].deliveries",
    "ends[].deliveries[].recipient",
    "ends[].deliveries[].at",
    "ends[].deliveries[].by",
    "ends[].pending_for",
    "holds",
    "holds[].on",
    "holds[].stage",
    "holds[].since",
    "holds[].words",
    "hold_words",
    "waiting_on",
    "waiting_on.act",
    "waiting_on.stage",
    "waiting_on.why",
    "waiting_on.since",
    "waiting_on.words",
    "submission",
    "submission.held",
    "submission.submits",
    "submission.send_backs",
    "submission.waiting",
    "submission.request",
    "submission.verdict",
    "submission.verdict.landed",
    "submission.verdict.landed.commit",
    "submission.verdict.landed.place_removed",
    "submission.verdict.landed.branch_removed",
    "submission.verdict.failed",
    "submission.verdict.conflict",
    "submission.verdict.refused",
    "submission.read",
    "submission.read.found",
    "submission.read.found.submission",
    "submission.read.found.submission.id",
    "submission.read.found.submission.branch",
    "submission.read.found.submission.target",
    "submission.read.found.submission.request",
    "submission.read.found.submission.state",
    "submission.read.found.submission.state.landed",
    "submission.read.found.submission.state.landed.commit",
    "submission.read.found.submission.state.failed",
    "submission.read.found.submission.state.conflict",
    "submission.read.could_not_read",
    "agent",
    "agent.identity",
    "agent.stage",
    "agent.program",
    "agent.account",
    "agent.account.pool",
    "agent.account.name",
    "agent.account.from",
    "agent.state",
    "agent.started",
    "agent.asking",
    "agent.own_work",
    "agent.past_limit",
    "clues",
    "clues.shown_at",
    "clues.shown_at.agent",
    "tasks",
    "tasks[].ticked",
    "tasks[].group",
];

/// The copy of Flume's host part (design Decision 8).
const HOST_PART: &[&str] = &[
    "running",
    "bound",
    "room",
    "streams",
    "streams[].name",
    "streams[].priority",
    "streams[].running",
    "streams[].bound",
    "streams[].room",
];

/// The copy of sluice's entry (design Decision 9).
const SUBMISSION_ENTRY: &[&str] = &[
    "id",
    "branch",
    "head",
    "state",
    "landed_as",
    "failed",
    "failed[].task",
    "failed[].hash",
    "failed[].flaky",
    "failed[].system",
];

/// The examples whose copies are cut from Flume 0.4.1's places (design
/// Decision 14).
const CUT_FROM_FLUME: &[&str] = &[
    "report-a-place-waiting-on-an-answer.json",
    "report-a-place-on-hold.json",
    "report-a-roles-run-delivered.json",
    "report-a-submission-under-test.json",
    "report-a-landing-kept-by-a-failed-teardown.json",
    "report-a-tidied-place.json",
    "report-a-live-agent-with-its-pane.json",
];

/// Version 2's files by their addresses.
struct Schema(BTreeMap<String, Value>);

impl Schema {
    fn new() -> Schema {
        Schema(
            files()
                .into_iter()
                .map(|(name, value)| (address(name), value))
                .collect(),
        )
    }

    /// What `reference` names, read in the file at `base`, with the
    /// address of the file it is in.
    fn resolve(&self, base: &str, reference: &str) -> (String, &Value) {
        let (file, pointer) = reference.split_once('#').unwrap_or((reference, ""));
        let file = match file {
            "" => base.to_string(),
            name => format!("{}{name}", &base[..=base.rfind('/').unwrap()]),
        };
        let value = self.0[&file]
            .pointer(pointer)
            .unwrap_or_else(|| panic!("{reference} from {base}"));
        (file, value)
    }

    /// The field paths of the schema `node`, in the file at `base`, under
    /// `prefix`.
    fn paths(&self, base: &str, node: &Value, prefix: &str, found: &mut BTreeSet<String>) {
        let under = |name: &str| match prefix {
            "" => name.to_string(),
            _ => format!("{prefix}.{name}"),
        };
        if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
            let (file, target) = self.resolve(base, reference);
            self.paths(&file, target, prefix, found);
        }
        if let Some(properties) = node.get("properties").and_then(Value::as_object) {
            for (name, property) in properties {
                found.insert(under(name));
                self.paths(base, property, &under(name), found);
            }
        }
        if let Some(items) = node.get("items") {
            self.paths(base, items, &format!("{prefix}[]"), found);
        }
        for keyword in ["oneOf", "anyOf", "allOf"] {
            for branch in node
                .get(keyword)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                self.paths(base, branch, prefix, found);
            }
        }
    }

    /// The field paths of the definition `name` of `file`.
    fn fields(&self, file: &str, name: &str) -> BTreeSet<String> {
        let base = address(file);
        let (base, node) = self.resolve(&base, &format!("#/$defs/{name}"));
        let mut found = BTreeSet::new();
        self.paths(&base, node, "", &mut found);
        found
    }
}

/// The fields of `copy`, read from the schema, are `list`, or the test
/// fails naming each field added or taken.
fn holds_the_list(copy: &str, fields: BTreeSet<String>, list: &[&str]) {
    let list: BTreeSet<String> = list.iter().map(|field| field.to_string()).collect();
    let added: Vec<&String> = fields.difference(&list).collect();
    let taken: Vec<&String> = list.difference(&fields).collect();
    assert!(
        added.is_empty() && taken.is_empty(),
        "{copy}'s fields differ from its list: added {added:?}, taken {taken:?}"
    );
}

#[test]
fn the_copys_fields_are_the_list() {
    let fields = Schema::new().fields("place-status.schema.json", "place_status");
    holds_the_list("a place's status", fields, PLACE_STATUS);
}

#[test]
fn the_capacitys_fields_are_the_list() {
    let fields = Schema::new().fields("hello.schema.json", "host_part");
    holds_the_list("the host part", fields, HOST_PART);
}

#[test]
fn the_entrys_fields_are_the_list() {
    let fields = Schema::new().fields("submission-entry.schema.json", "submission_entry");
    holds_the_list("sluice's entry", fields, SUBMISSION_ENTRY);
}

#[test]
fn flumes_answers_as_copies() {
    let validator = from_runner();
    for name in CUT_FROM_FLUME {
        let report = example("from-runner", name);
        let launches = report["body"]["report"]["launches"].as_object().unwrap();
        assert!(
            launches
                .values()
                .all(|launched| launched["status"]["copy"].is_object()),
            "{name} carries a status copy"
        );
        passes(&validator, &report, name);
    }
}

/// A thing an end made is carried whole, as the run wrote it: a line of
/// text, or an object naming the thing with fields of its own (design
/// Decision 7).
#[test]
fn a_thing_an_end_made_is_carried_whole() {
    let validator = from_runner();
    let made = |things: Value| {
        let mut report = example("from-runner", "report-a-roles-run-delivered.json");
        let copy = &mut report["body"]["report"]["launches"]["offer-3"]["status"]["copy"];
        copy["stages"][0]["end"]["made"] = things;
        report
    };
    let things = json!([
        "the week's plan, in the place's report",
        {"name": "preview", "url": "https://preview.example.com/weekly", "revision": "1"}
    ]);
    passes(&validator, &made(things), "an end that made two things");
    let unnamed = json!([{"url": "https://preview.example.com/weekly"}]);
    refused(&validator, &made(unnamed), "a made thing with no name");
}
