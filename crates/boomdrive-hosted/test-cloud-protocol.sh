#!/bin/sh
# check-cloud-protocol.sh against scratch repositories by file://, made in
# a temporary directory, with no network (the runner-protocol spec's "The
# gate refuses a runner that offers a version the cloud's main does not
# speak"): a runner whose versions the cloud speaks passes; a runner that
# offers a version the cloud does not speak, or none, is refused; a pin
# that cannot be read is refused. The scratch repositories' git reads no
# configuration of the host's, and their names are neutral (core.41).
set -eu

check=$(pwd)/crates/boomdrive-hosted/check-cloud-protocol.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

GIT_CONFIG_GLOBAL=/dev/null
GIT_CONFIG_NOSYSTEM=1
GIT_AUTHOR_NAME=Ada
GIT_AUTHOR_EMAIL=ada@example.com
GIT_COMMITTER_NAME=Ada
GIT_COMMITTER_EMAIL=ada@example.com
export GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL \
  GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL

status=0
fail() {
  echo "test-cloud-protocol: $*" >&2
  status=1
}
pass() {
  echo "test-cloud-protocol: $*"
}

# The `versions` of a file, each of `$@` as `<number>:<spoken>`.
versions() {
  printf '{"versions": ['
  sep=""
  for entry in "$@"; do
    printf '%s{"version": %s, "spoken": %s}' "$sep" "${entry%%:*}" "${entry#*:}"
    sep=", "
  done
  printf ']}\n'
}

# A scratch cloud repository `$1` whose main holds the pin `$2`, or no
# pin where `$2` is empty, that serves a fetch with no blobs. It is `$dir`.
cloud() {
  dir="$scratch/$1"
  git init -q -b main "$dir"
  git -C "$dir" config uploadpack.allowFilter true
  git -C "$dir" config uploadpack.allowAnySHA1InWant true
  if [ -n "$2" ]; then
    mkdir -p "$dir/workers/control-plane/runner-protocol"
    printf '%s\n' "$2" >"$dir/workers/control-plane/runner-protocol/pin.json"
  else
    echo other >"$dir/other"
  fi
  git -C "$dir" add -A
  git -C "$dir" commit -q -m "the cloud"
}

# The runner's spoken.json `$scratch/$1.json`, holding `$2`.
runner() {
  printf '%s\n' "$2" >"$scratch/$1.json"
}

# Runs the check of runner `$2` against the scratch cloud `$1` by
# `file://`, and records its exit and its words.
run() {
  url=file://$scratch/$1
  set +e
  sh "$check" "$url" "$scratch/$2.json" >"$scratch/out" 2>&1
  code=$?
  set -e
  words=$(cat "$scratch/out")
}

# Expects the last run to exit `$1` with words holding `$2`, naming the
# case `$3`.
expect() {
  if [ "$code" -eq "$1" ] && case $words in *"$2"*) true ;; *) false ;; esac then
    pass "$3"
  else
    fail "$3: exit $code, expected $1 and words holding '$2': $words"
  fi
}

pin='workers/control-plane/runner-protocol/pin.json'

# A runner whose versions the cloud speaks passes.
cloud leading "$(versions 3:false 4:true 5:true)"
runner five "$(versions 4:false 5:true)"
run leading five
expect 0 "the runner speaks 5, the cloud speaks 4, 5, and the cloud speaks each version the runner offers" \
  "a runner on a version the cloud speaks passes, the cloud keeping an older one"
runner four-and-five "$(versions 4:true 5:true)"
run leading four-and-five
expect 0 "the runner speaks 4, 5, the cloud speaks 4, 5, and the cloud speaks each version the runner offers" \
  "a runner on two versions the cloud speaks passes"

# A runner that offers a version the cloud does not speak is refused.
cloud behind "$(versions 3:true 4:true)"
run behind five
expect 1 "the runner speaks 5, the cloud speaks 3, 4, and the cloud does not speak 5" \
  "a runner ahead of the cloud is refused"
run behind four-and-five
expect 1 "the runner speaks 4, 5, the cloud speaks 3, 4, and the cloud does not speak 5" \
  "a runner ahead of the cloud is refused though they share a version"
cloud dropped "$(versions 4:false 5:true)"
runner four "$(versions 4:true 5:false)"
run dropped four
expect 1 "the runner speaks 4, the cloud speaks 5, and the cloud does not speak 4" \
  "a runner on a version the cloud dropped is refused"
runner silent "$(versions 4:false 5:false)"
run leading silent
expect 1 "the runner speaks none, the cloud speaks 4, 5, and the runner offers no version" \
  "a runner that offers no version is refused"

# A pin that cannot be read is refused, naming it.
run none four
expect 1 "$pin of file://$scratch/none's main: the repository's main could not be fetched" \
  "a repository that cannot be fetched is refused"
cloud bare ""
run bare four
expect 1 "$pin of file://$scratch/bare's main: main holds no such file" \
  "a main with no pin is refused"
cloud garbled "not json"
run garbled four
expect 1 "$pin of file://$scratch/garbled's main: it is not JSON" \
  "a pin that is not JSON is refused"
cloud listless '{"repository": "elsewhere"}'
run listless four
expect 1 "$pin of file://$scratch/listless's main: it does not list versions" \
  "a pin that lists no versions is refused"

exit $status
