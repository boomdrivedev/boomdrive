#!/bin/sh
# The cloud leads the runner's protocol (Boom:
# `the-cloud-leads-the-runners-protocol`; core.42): each version `$2`, the
# runner's spoken.json, marks spoken is one boomdrive-cloud's main marks
# spoken in workers/control-plane/runner-protocol/pin.json, and `$2` marks
# at least one.
#
# `$1` is the cloud's repository. Its main is fetched as the forge holds
# it, by the host's own git credentials, at depth 1 with no blobs, into a
# scratch bare repository, and the pin's blob alone is read; nothing is
# kept or replayed. A fetch that stalls for 20 seconds fails. Where the pin
# cannot be read, the check fails naming the pin and what it could not
# read, and passes nothing. The comparison runs on the Node.js devenv.nix
# pins (core.8).
set -eu

repository=$1
spoken=$2
pin=workers/control-plane/runner-protocol/pin.json

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

refuse() {
  echo "cloud-protocol: $pin of $repository's main: $*" >&2
  exit 1
}

git init -q --bare "$scratch/cloud.git"
git -C "$scratch/cloud.git" remote add origin "$repository"
if ! git -C "$scratch/cloud.git" -c http.lowSpeedLimit=1 -c http.lowSpeedTime=20 \
  fetch -q --no-tags --depth 1 --filter=blob:none origin main 2>"$scratch/fetch.log"; then
  refuse "the repository's main could not be fetched: $(cat "$scratch/fetch.log")"
fi
if ! git -C "$scratch/cloud.git" -c http.lowSpeedLimit=1 -c http.lowSpeedTime=20 \
  cat-file blob "FETCH_HEAD:$pin" >"$scratch/pin.json" 2>"$scratch/read.log"; then
  refuse "main holds no such file: $(cat "$scratch/read.log")"
fi

# The script is JavaScript, whose template strings the shell must not
# expand.
# shellcheck disable=SC2016
node -e '
const fs = require("fs");
const [pinFile, pinName, spokenFile] = process.argv.slice(1);

// The versions a file marks spoken, or the words saying why it cannot
// be read.
function spokenIn(file) {
  let value;
  try {
    value = JSON.parse(fs.readFileSync(file, "utf8"));
  } catch (e) {
    return { refused: `it is not JSON: ${e.message}` };
  }
  const versions = value && value.versions;
  if (!Array.isArray(versions)
    || !versions.every((v) => v && Number.isInteger(v.version) && typeof v.spoken === "boolean")) {
    return { refused: "it does not list versions, each a whole `version` and a `spoken` true or false" };
  }
  return { spoken: versions.filter((v) => v.spoken).map((v) => v.version).sort((a, b) => a - b) };
}

const cloud = spokenIn(pinFile);
if (cloud.refused) {
  console.error(`cloud-protocol: ${pinName}: ${cloud.refused}`);
  process.exit(1);
}
const runner = spokenIn(spokenFile);
if (runner.refused) {
  console.error(`cloud-protocol: ${spokenFile}: ${runner.refused}`);
  process.exit(1);
}
const list = (versions) => (versions.length ? versions.join(", ") : "none");
const unspoken = runner.spoken.filter((v) => !cloud.spoken.includes(v));
const both = `the runner speaks ${list(runner.spoken)}, the cloud speaks ${list(cloud.spoken)}`;
if (!runner.spoken.length) {
  console.error(`cloud-protocol: ${both}, and the runner offers no version`);
  process.exit(1);
}
if (unspoken.length) {
  console.error(`cloud-protocol: ${both}, and the cloud does not speak ${list(unspoken)}`);
  process.exit(1);
}
console.log(`cloud-protocol: ${both}, and the cloud speaks each version the runner offers`);
' "$scratch/pin.json" "$pin of $repository's main" "$spoken"
