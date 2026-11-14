#!/bin/sh
# site:test, the site's tests: each scenario of the site spec that names
# site:test (design Decision 3). It runs in site/, once site:build has
# built dist.
set -eu

status=0
fail() {
  echo "check-site: $*" >&2
  status=1
}

# Every link under the base, and boomdrive-web's name gone (site.6): each
# page's own links and assets are under /boomdrive/, the one repository a
# page links is boomdrive's, and nothing the site writes names
# boomdrive-web. The family's entries @agentplot/site-theme draws, its
# Boomdrive member's site and its hub, stand as the theme gives them
# (record 0040), read here from the theme itself, so they move with its
# release at the opening.
theme=$(bun -e 'import { member, hub } from "@agentplot/site-theme/family"; console.log(member("boomdrive").site); console.log(hub);') ||
  fail "@agentplot/site-theme gave no Boomdrive site or hub"
pages=$(find dist -name '*.html' | sort)
[ -n "$pages" ] || fail "dist holds no page: site:build builds it"
for page in $pages; do
  links=$(grep -oE '(href|src)="[^"]*"' "$page" | sed -E 's/^(href|src)="(.*)"$/\2/' | sort -u)
  for link in $links; do
    case $link in
    /boomdrive/*) ;;
    /*) fail "$page links $link, outside /boomdrive/" ;;
    esac
    case $link in
    https://github.com/boomdrivedev/boomdrive | https://github.com/boomdrivedev/boomdrive/*) ;;
    https://github.com/*/*) fail "$page links the repository $link, not https://github.com/boomdrivedev/boomdrive" ;;
    esac
  done
  named=$(grep -oE "[^\"' <>()]*boomdrive-web[^\"' <>()]*" "$page" | sort -u || true)
  for name in $named; do
    if ! printf '%s\n' "$theme" | grep -qxF "$name"; then
      fail "$page names $name"
    fi
  done
done
[ "$status" -ne 0 ] || echo "check-site: $(printf '%s\n' "$pages" | wc -l | tr -d ' ') pages, every link under /boomdrive/ and none of boomdrive-web's but the theme's"

# The workflow's trigger, The workflow's build: the workflow pages read
# line by line, as check-releases.sh reads the workflow release, and as
# yamlfmt lays it out.
workflow=../.github/workflows/pages.yml
# The lines of the top-level block `$1`, less the empty ones and comments.
block() {
  awk -v key="$1:" '
    $0 == key { inside = 1; next }
    inside && /^[^ #]/ { exit }
    inside && !/^ *(#|$)/ { print }
  ' "$workflow"
}
on=$(block on)
want_on="  push:
    branches:
      - main
    paths:
      - site/**
      - .github/workflows/pages.yml
      - devenv.nix
      - devenv.yaml
      - devenv.lock
      - flake.lock
  workflow_dispatch:"
if [ "$on" = "$want_on" ]; then
  echo "check-site: the workflow pages runs on a push to main that changes the site, its shell or flake.lock, and when dispatched"
else
  fail "the workflow pages runs on more or less than a push to main that changes the site, its shell or flake.lock, and a dispatch:
$on"
fi
concurrency=$(block concurrency)
want_concurrency="  group: pages
  cancel-in-progress: false"
if [ "$concurrency" = "$want_concurrency" ]; then
  echo "check-site: the workflow pages runs one at a time, none cancelled"
else
  fail "the workflow pages is not one run at a time, none cancelled:
$concurrency"
fi
# Each line `$1` matches, as a basic regular expression, once.
once() {
  [ "$(grep -c -e "$1" "$workflow")" -eq 1 ]
}
if once '^        run: devenv shell -- moon run site:build$' &&
  once '^      - uses: actions/upload-pages-artifact@[0-9a-f]\{40\} # v' &&
  once '^          path: site/dist$' &&
  once '^      name: github-pages$' &&
  once '^        uses: actions/deploy-pages@[0-9a-f]\{40\} # v'; then
  echo "check-site: the workflow pages builds the site in the devenv shell, uploads site/dist and deploys it to github-pages"
else
  fail "the workflow pages does not build the site in the devenv shell, upload site/dist and deploy it to github-pages"
fi
if grep -n -E 'run: .*(site-check|site:(check|prose|test|workflow)|moon run :|sluice)' "$workflow"; then
  fail "the workflow pages runs a check"
else
  echo "check-site: the workflow pages runs no check"
fi

# The kind's prompts (site.5): each stage's prompt of [kinds.content] in
# .config/flume.toml, resolved against that file's own folder as flume
# resolves it, is a file under site/docs/prompts/.
flume=../.config/flume.toml
prompts=$(awk '
  /^\[/ { inside = ($0 == "[kinds.content]" || $0 == "[[kinds.content.stages]]") }
  inside && /^prompt *= *"/ { sub(/^prompt *= *"/, ""); sub(/".*$/, ""); print }
' "$flume")
[ -n "$prompts" ] || fail "$flume gives [kinds.content] no prompt"
here=$(cd docs/prompts && pwd -P)
for prompt in $prompts; do
  folder=$(cd "../.config/$(dirname "$prompt")" 2>/dev/null && pwd -P) || folder=""
  if [ "$folder" = "$here" ] && [ -f "$folder/$(basename "$prompt")" ]; then
    echo "check-site: the content kind's prompt $prompt is site/docs/prompts/$(basename "$prompt")"
  else
    fail "the content kind's prompt $prompt names no file under site/docs/prompts/"
  fi
done

# The kind's declaration (the content-kind spec; record 0045): the
# declaration read as TOML with bun's own parser, so its inline tables
# read as flume reads them, holds the kind to its stages, and its
# preview's one agent names no pane or session of a host's terminal.
kind=$(
  cat <<'CHECK'
import { readFileSync } from "node:fs";

let failed = false;
const fail = (words: string) => {
  console.error(`check-site: ${words}`);
  failed = true;
};
const said = (words: string) => console.log(`check-site: ${words}`);
const shown = (value: unknown) => JSON.stringify(value) ?? "nothing";

type Table = Record<string, unknown>;
const declaration = Bun.TOML.parse(readFileSync("../.config/flume.toml", "utf8")) as Table;
const profiles = (declaration.profiles ?? {}) as Table;
const kind = (((declaration.kinds ?? {}) as Table).content ?? {}) as Table;
const stages = (Array.isArray(kind.stages) ? kind.stages : []) as Table[];
const stage = (name: string) => stages.find((one) => one.name === name) ?? {};
const none = (value: unknown) => value === undefined || (Array.isArray(value) && value.length === 0);

// The declaration reads as the kind, and asks for nothing flume refuses.
if (profiles.merge_queue !== "sluice") fail(`the declaration's merge queue is ${shown(profiles.merge_queue)}, not "sluice"`);
if (kind.lands !== true) fail(`the content kind's lands is ${shown(kind.lands)}, not true`);
if (kind.branch !== "content") fail(`the content kind's branch is ${shown(kind.branch)}, not "content"`);
const names = stages.map((one) => one.name);
if (shown(names) !== shown(["change", "preview"])) fail(`the content kind's stages are ${shown(names)}, not ["change","preview"]`);
for (const one of stages) {
  const agents = one.agents;
  if (!Array.isArray(agents) || agents.length !== 1) fail(`the content kind's stage ${shown(one.name)} gives the agents ${shown(agents)}, not one`);
}
const change = stage("change");
if (shown(change.deliverables) !== shown(["commits"])) fail(`the content kind's change delivers ${shown(change.deliverables)}, not ["commits"]`);
if (change.conversation !== undefined && change.conversation !== false) fail(`the content kind's change is a conversation, ${shown(change.conversation)}`);
const preview = stage("preview");
if (preview.conversation !== true) fail(`the content kind's preview gives conversation ${shown(preview.conversation)}, not true`);
if (preview.wait !== true) fail(`the content kind's preview gives wait ${shown(preview.wait)}, not true`);
if (preview.on_fail !== "change") fail(`the content kind's preview sends back to ${shown(preview.on_fail)}, not "change"`);
if (!none(preview.deliverables)) fail(`the content kind's preview delivers ${shown(preview.deliverables)}, not nothing`);
if (!failed) {
  said("the content kind lands through sluice, change then preview, one agent to each stage, the preview a conversation that waits and sends back to change");
}

// The preview names no place in the terminal.
const agent = Array.isArray(preview.agents) ? (preview.agents[0] as Table | undefined) : undefined;
const places = ["beside", "session"].filter((key) => agent !== undefined && agent !== null && key in agent);
if (agent === undefined || agent === null || typeof agent !== "object") {
  fail(`the content kind's preview gives no agent to read, ${shown(preview.agents)}`);
} else if (places.length > 0) {
  fail(`the content kind's preview's agent names ${places.map((key) => `${key} = ${shown(agent[key])}`).join(" and ")}, a place a host's terminal may lack`);
} else {
  said("the content kind's preview names no pane or session, opening where its place's stream puts it");
}

process.exit(failed ? 1 : 0);
CHECK
)
bun -e "$kind" || status=1

# The brief says where the preview opens (the content-kind spec; record
# 0045): docs/content-brief.md's section "A task", from its heading to the
# next, its lines joined so a rewrap moves no words out of it, names the
# launch's setting that puts the preview beside a pane, and flume open.
task=$(awk '/^#/ { inside = ($0 == "## A task"); next } inside' docs/content-brief.md | tr -s '\n' ' ')
if [ -z "$(printf '%s' "$task" | tr -d ' ')" ]; then
  fail "docs/content-brief.md holds no section ## A task"
else
  unsaid=""
  for words in "--agent preview.beside=" "flume open"; do
    printf '%s\n' "$task" | grep -qF -- "$words" || unsaid="$unsaid${unsaid:+ and }$words"
  done
  if [ -n "$unsaid" ]; then
    fail "docs/content-brief.md's section A task does not say $unsaid"
  else
    echo "check-site: the content brief says a launch names the pane its preview opens beside, and flume open shows the session"
  fi
fi

# The quick start (the quick-start spec; site.8; design Decision 7 of the
# change boomdrives-quick-start-takes-a-stranger-to-a-working-drive): the
# built page /start/ and every page's masthead, the step records of
# src/content/start, read with the site's yaml package through bun, the
# files under src/start/, and the two test files the steps name, which the
# site's build does not read.
quick_start=$(
  cat <<'CHECK'
import { parse } from "yaml";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

let failed = false;
const fail = (words: string) => {
  console.error(`check-site: ${words}`);
  failed = true;
};
const said = (words: string) => console.log(`check-site: ${words}`);
// As Astro escapes a text it sets.
const escape = (text: string) =>
  text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;").replaceAll("'", "&#39;");
const files = (dir: string, under = ""): string[] =>
  readdirSync(join(dir, under), { withFileTypes: true }).flatMap((entry) =>
    entry.isDirectory() ? files(dir, join(under, entry.name)) : [join(under, entry.name)],
  );

type Step = {
  id: string;
  text: string;
  data: {
    title: string;
    at: { mac?: number; nixos?: number };
    files: { path: string; writes?: string }[];
    test: { project: "boomdrive" | "nix"; name: string };
  };
};
const steps: Step[] = readdirSync("src/content/start")
  .filter((name) => name.endsWith(".md"))
  .sort()
  .map((name) => {
    const text = readFileSync(join("src/content/start", name), "utf8");
    const front = text.match(/^---\n([\s\S]*?)\n---\n/);
    if (!front) throw new Error(`src/content/start/${name} has no frontmatter`);
    return { id: name.slice(0, -3), text, data: parse(front[1]) };
  });
const ways = [
  { id: "mac", title: "On a Mac" },
  { id: "nixos", title: "On NixOS" },
] as const;
const on = (way: "mac" | "nixos") =>
  steps.filter((step) => step.data.at[way] !== undefined).sort((a, b) => a.data.at[way]! - b.data.at[way]!);

// A platform's places are its order.
for (const way of ways) {
  const places = on(way.id).map((step) => step.data.at[way.id]);
  if (places.every((place, i) => place === i + 1)) said(`the quick start's ${places.length} steps ${way.title} run 1 to ${places.length}`);
  else fail(`the quick start's places ${way.title} are ${places.join(", ")}, not 1 to ${places.length} each once`);
}

// Each step's test is there.
const tests = {
  boomdrive: { file: "../crates/boomdrive/tests/quick_start.rs", holds: (name: string) => `fn ${name}(` },
  nix: { file: "../nix/tests/quick-start.nix", holds: (name: string) => `"${name}" =` },
};
const untested = steps.filter((step) => {
  const test = tests[step.data.test.project];
  return !readFileSync(test.file, "utf8").includes(test.holds(step.data.test.name));
});
if (untested.length === 0) said(`each of the quick start's ${steps.length} steps names a test its project holds`);
for (const step of untested) fail(`the step ${step.id} names the test ${step.data.test.name}, which its project does not hold`);

// No file is left unshown.
const shown = new Set(steps.flatMap((step) => step.data.files.map((file) => file.path)));
const held = files("src/start").filter((path) => path !== ".shellcheckrc");
const unshown = held.filter((path) => !shown.has(path));
if (unshown.length === 0) said(`every file of src/start/ but .shellcheckrc is a step's, ${held.length} of them`);
else fail(`src/start/ holds ${unshown.join(", ")}, which no step shows`);

// The page and its two ways, and each file as it stands.
const page = readFileSync("dist/start/index.html", "utf8");
const before = failed;
failed = false;
if (/<script[\s>]/.test(page)) fail("the quick start holds a script element");
const lede = page.indexOf('<div class="start-lede">');
const heads = ways.map((way) => page.indexOf(`<h2 id="start-${way.id}">${way.title}</h2>`));
if (lede < 0 || heads.some((at) => at < 0) || !(lede < heads[0] && heads[0] < heads[1])) {
  fail("the quick start is not its lede, then On a Mac, then On NixOS");
}
for (const [w, way] of ways.entries()) {
  const section = page.slice(heads[w], w + 1 < ways.length ? heads[w + 1] : undefined);
  const titles = [...section.matchAll(/<h3>(.*?)<\/h3>/g)].map((m) => m[1]);
  const want = on(way.id).map((step) => escape(step.data.title));
  if (JSON.stringify(titles) !== JSON.stringify(want)) {
    fail(`the quick start's steps ${way.title} are ${titles.join(" | ")}, not ${want.join(" | ")}`);
  }
  if (!/^<h2[^>]*>[^<]*<\/h2><ol class="start-steps">/.test(section)) fail(`the steps ${way.title} are not one numbered list`);
  for (const step of on(way.id)) {
    const start = section.indexOf(`<li class="start-step" id="${way.id}-${step.id}">`);
    const item = start < 0 ? "" : section.slice(start, section.indexOf("</li>", start));
    for (const file of step.data.files) {
      const block = `<pre>${escape(readFileSync(join("src/start", file.path), "utf8"))}</pre>`;
      const at = item.indexOf(block);
      if (at < 0 || item.indexOf(block, at + 1) >= 0) {
        fail(`the step ${step.id} ${way.title} does not show ${file.path} as it stands, in one block`);
      } else if (file.writes && !item.slice(0, at).endsWith(`<figcaption>Write it to <code>${escape(file.writes)}</code></figcaption>`)) {
        fail(`the step ${step.id} ${way.title} does not name ${file.writes} above ${file.path}`);
      }
    }
  }
}
if (!failed) said("the quick start is its lede, then On a Mac and On NixOS, each step numbered in order with each file as it stands, and no script");
failed ||= before;

// No colour of its own, and a long line at phone width.
const blocks = [...page.matchAll(/<(figure class="start-file"|pre)([^>]*)>/g)];
if (blocks.some((m) => /\sstyle=/.test(m[2]))) fail("a step's block on the quick start carries a style of its own");
const css = files("dist/_astro")
  .filter((path) => path.endsWith(".css"))
  .map((path) => readFileSync(join("dist/_astro", path), "utf8"))
  .join("\n");
const rule = css.match(/\.start-file pre\{([^}]*)\}/)?.[1] ?? "";
const colours = [...rule.matchAll(/(?:^|;)(?:color|background|border)(?:-[a-z]+)?:([^;]*)/g)];
if (!rule) fail("the built stylesheet gives a step's block no rule");
else if (!/(?:^|;)overflow-x:auto(?:;|$)/.test(rule) || /white-space:pre-(?:wrap|line)/.test(rule)) {
  fail(`a step's block does not scroll along its own lines: ${rule}`);
} else if (colours.some((m) => /#|rgb|hsl|oklab|oklch/.test(m[1].replace(/var\([^)]*\)/g, "")))) {
  fail(`a step's block carries a colour of its own: ${rule}`);
} else said("a step's block takes the page's tokens and scrolls within itself");

// Reached from the masthead.
const pages = files("dist").filter((path) => path.endsWith(".html"));
const unlinked = pages.filter(
  (path) => !/<a href="\/boomdrive\/start\/"[^>]*>\s*Start\s*<\/a>/.test(readFileSync(join("dist", path), "utf8")),
);
if (unlinked.length === 0) said(`every page's masthead links /boomdrive/start/ as Start, ${pages.length} pages`);
else fail(`${unlinked.join(", ")} link no Start at /boomdrive/start/`);

// None of our drives or machines.
const contexts = parse(readFileSync("src/content/contexts/contexts.yaml", "utf8")) as {
  drives?: { name: string; machine: string }[];
}[];
const ours = [...new Set(contexts.flatMap((c) => (c.drives ?? []).flatMap((d) => [d.name, d.machine])))];
const read = [
  { where: "the built quick start", text: page },
  ...steps.map((step) => ({ where: `src/content/start/${step.id}.md`, text: step.text })),
  { where: "src/content/pages/start.md", text: readFileSync("src/content/pages/start.md", "utf8") },
  ...held.map((path) => ({ where: `src/start/${path}`, text: readFileSync(join("src/start", path), "utf8") })),
];
const named = read.flatMap(({ where, text }) =>
  ours.filter((name) => new RegExp(`\\b${name}\\b`).test(text)).map((name) => `${where} names ${name}`),
);
if (named.length === 0) said(`the quick start names none of our ${ours.length} drives and machines`);
for (const one of named) fail(one);

process.exit(failed ? 1 : 0);
CHECK
)
bun -e "$quick_start" || status=1

# A release where nothing was served: site-stop, which sluice runs after
# every candidate's checks, with a state folder that holds no port, exits
# zero.
state=$(mktemp -d "${TMPDIR:-/tmp}/site-stop.XXXXXX")
if said=$(DEVENV_STATE=$state site-stop 2>&1); then
  echo "check-site: site-stop exits zero where nothing was served"
else
  fail "site-stop fails where nothing was served: $said"
fi
rm -rf "$state"

exit "$status"
