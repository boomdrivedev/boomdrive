#!/bin/sh
# The activation's cases (design Decision 14, Task notes 5.2), which
# nix/check.sh test runs beside the evaluated ones: each runs
# nix/modules/activate.sh by sh in a scratch folder of its own, with a
# scratch HOME, the module's file in a store of its own, and stubs of
# systemctl and ps first on PATH that record their calls, and of id. The
# stubs answer from the case's folder: id the user running it, alice
# unless `user` says another, and its uid, the system's own unless `uid`
# says another; systemctl's show the unit's MainPID, 0 unless `MainPID`
# says another; and ps the command line in `args`. The plugins' cases add
# a stub of herdr, run by its path as the modules run it, that records its
# arguments and the HERDR_* variables it sees, and refuses the plugin
# `refuse` names, saying `manifest not found`. The managed document's
# cases write in a scratch folder standing for Claude Code's
# managed-settings.d, as the case's own user. The signing's cases add
# stubs of security, codesign and openssl (record 0047; design Decision 7
# of the change the-runners-macos-grant-outlasts-a-new-build), described
# at freshSign, and a keychain that is a word they take, never a keychain.
# Each case runs with a TMPDIR of its own. It prints each failure as one
# line, nix: <case>: <what>, and nothing where every case holds.
set -eu

activate=$(cd "$(dirname "$0")/../modules" && pwd)/activate.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0
uid=$(id -u)

# A case's folder: its home, the module's file, the stubs and their calls.
fresh() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  home=$dir/home
  file=$dir/store/boomdrive-config.toml
  target=$home/.config/boomdrive/config.toml
  aside=$target.before-nix
  calls=$dir/calls
  program=$dir/store/boomdrive/bin/boomdrive
  mkdir -p "$home" "$dir/store" "$dir/bin"
  echo 'repositories = []' >"$file"
  echo alice >"$dir/user"
  echo "$uid" >"$dir/uid"
  echo 0 >"$dir/MainPID"
  : >"$dir/args"
  cat >"$dir/bin/systemctl" <<STUB
#!/bin/sh
echo "systemctl \$*" >>"$calls"
case " \$* " in *" show "*) cat "$dir/MainPID" ;; esac
STUB
  cat >"$dir/bin/ps" <<STUB
#!/bin/sh
echo "ps \$*" >>"$calls"
cat "$dir/args"
STUB
  cat >"$dir/bin/id" <<STUB
#!/bin/sh
case \${1:-} in
-u) cat "$dir/uid" ;;
*) cat "$dir/user" ;;
esac
STUB
  chmod +x "$dir/bin/systemctl" "$dir/bin/ps" "$dir/bin/id"
}

# activate.sh with `$@` in the case's folder: its status, output and
# error.
run() {
  status=0
  mkdir -p "$dir/tmp"
  HOME=$home TMPDIR=$dir/tmp PATH=$dir/bin:$PATH sh "$activate" "$@" >"$dir/out" 2>"$dir/err" || status=$?
  out=$(cat "$dir/out")
  err=$(cat "$dir/err")
}

# A failure of the case `$1`, saying `$2`, where the test after them fails.
expect() {
  named=$1
  what=$2
  shift 2
  if ! "$@"; then
    echo "nix: $named: $what"
  fi
}

linked() {
  [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]
}
holds() {
  [ "$(cat "$1" 2>/dev/null)" = "$2" ]
}
missing() {
  [ ! -e "$1" ] && [ ! -L "$1" ]
}
# `$1` names `$2`, then `$3`.
names() {
  case $1 in *"$2"*"$3"*) ;; *) return 1 ;; esac
}

# The stubs' calls, on one line.
called() {
  if [ -e "$calls" ]; then
    paste -s -d ';' "$calls"
  fi
}

# What every link holds to: it calls neither systemctl nor ps.
quiet() {
  expect "$1" "link called $(called)" missing "$calls"
}

# systemctl was asked to `$1` the unit, without waiting for the job.
asked() {
  grep -qx "systemctl --user --no-block $1 boomdrive.service" "$calls" 2>/dev/null
}
unasked() {
  ! asked "$1"
}

named="An empty home"
fresh
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "boomdrive: linked $target to $file" ]
expect "$named" "link moved something aside" missing "$aside"
quiet "$named"

named="A person's own file"
fresh
mkdir -p "$(dirname "$target")"
echo 'repositories = ["example/own"]' >"$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "$aside does not hold the person's file" \
  holds "$aside" 'repositories = ["example/own"]'
expect "$named" "link says \"$out\", not both paths" \
  [ "$out" = "boomdrive: moved $target to $aside
boomdrive: linked $target to $file" ]
quiet "$named"

fresh
mkdir -p "$target"
echo 'kept' >"$target/inside"
run link "$file"
expect "$named" "link of a folder exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file, where a folder was" linked
expect "$named" "$aside does not hold the person's folder" holds "$aside/inside" kept
quiet "$named"

named="Already linked"
fresh
mkdir -p "$(dirname "$target")"
ln -s "$file" "$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "link moved something aside" missing "$aside"
quiet "$named"

named="A link to another file"
fresh
mkdir -p "$(dirname "$target")"
ln -s "$dir/store/another.toml" "$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "boomdrive: linked $target to $file" ]
expect "$named" "link moved the other link aside" missing "$aside"
quiet "$named"

named="A file moved aside before"
fresh
mkdir -p "$(dirname "$target")"
echo 'repositories = ["example/own"]' >"$target"
echo 'repositories = ["example/older"]' >"$aside"
run link "$file"
expect "$named" "link exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "link changed $target" holds "$target" 'repositories = ["example/own"]'
expect "$named" "link changed $aside" holds "$aside" 'repositories = ["example/older"]'
expect "$named" "link's refusal \"$err\" names not both paths" \
  names "$err" "$target" "$aside"
expect "$named" "link says \"$out\"" [ -z "$out" ]
quiet "$named"

named="A local layer beside it"
fresh
local=$home/.config/boomdrive/config.local.toml
mkdir -p "$(dirname "$local")"
echo 'repositories = ["example/local"]' >"$local"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link changed $local" holds "$local" 'repositories = ["example/local"]'
expect "$named" "link made $local a link" [ ! -L "$local" ]
expect "$named" "link moved something aside" missing "$aside"
expect "$named" "link moved $local aside" missing "$local.before-nix"
quiet "$named"

named="Not running on NixOS"
fresh
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not start the unit: $(called)" asked start
expect "$named" "keep restarted the unit" unasked restart
expect "$named" "keep says \"$out\", not that it started the unit" \
  [ "$out" = "boomdrive: started boomdrive.service" ]

named="Nothing changed on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$program run --config $file" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep read no command line of the unit's process 4242: $(called)" \
  grep -q '^ps .*-p 4242$' "$calls"
expect "$named" "keep started the unit" unasked start
expect "$named" "keep restarted the unit" unasked restart
expect "$named" "keep says \"$out$err\"" [ -z "$out$err" ]

named="A new file on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$program run --config $dir/store/another.toml" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not restart the unit: $(called)" asked restart
expect "$named" "keep started the unit" unasked start
expect "$named" "keep says \"$out\", not that it restarted the unit and what it ran" \
  [ "$out" = "boomdrive: restarted boomdrive.service, which ran $program run --config $dir/store/another.toml" ]

named="A new package on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$dir/store/older/bin/boomdrive run --config $file" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not restart the unit: $(called)" asked restart
expect "$named" "keep started the unit" unasked start

named="Another user's activation"
fresh
echo bob >"$dir/user"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep called $(called)" missing "$calls"
expect "$named" "keep says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "keep made something in the home" missing "$home/.config"

named="keep's usage"
fresh
run keep alice
expect "$named" "keep with no program exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "keep with no program called $(called)" missing "$calls"

# A case's herdr stub, and two plugins in its store.
freshPlugins() {
  fresh
  herdr=$dir/store/herdr/bin/herdr
  first=$dir/store/flume-herdr-plugin
  second=$dir/store/sluice-herdr-plugin
  mkdir -p "$(dirname "$herdr")" "$first" "$second"
  : >"$dir/refuse"
  cat >"$herdr" <<STUB
#!/bin/sh
echo "herdr \$*" >>"$calls"
env | grep '^HERDR_' >>"$dir/herdr-env" || true
if [ "\$3" = "\$(cat "$dir/refuse")" ]; then
  echo "manifest not found" >&2
  exit 1
fi
echo "linked \$3"
STUB
  chmod +x "$herdr"
}

named="Each plugin linked"
freshPlugins
HERDR_SOCKET_PATH=$dir/herdr.sock
HERDR_ENV=1
export HERDR_SOCKET_PATH HERDR_ENV
run plugins "$herdr" "$first" "$second"
unset HERDR_SOCKET_PATH HERDR_ENV
expect "$named" "plugins exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "plugins called $(called), not herdr plugin link for each plugin in order" \
  holds "$calls" "herdr plugin link $first
herdr plugin link $second"
expect "$named" "herdr saw $(cat "$dir/herdr-env" 2>/dev/null)" [ ! -s "$dir/herdr-env" ]
expect "$named" "plugins says \"$out$err\"" [ -z "$out$err" ]

named="A link herdr refuses"
freshPlugins
echo "$first" >"$dir/refuse"
run plugins "$herdr" "$first" "$second"
expect "$named" "plugins exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "plugins said \"$err\", not one line naming $first and manifest not found" \
  [ "$err" = "boomdrive: herdr did not link $first: manifest not found" ]
expect "$named" "plugins called $(called), not the second plugin's link after the first" \
  holds "$calls" "herdr plugin link $first
herdr plugin link $second"

named="plugins' usage"
freshPlugins
run plugins "$herdr"
expect "$named" "plugins with no plugin exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "plugins with no plugin called $(called)" missing "$calls"

# A case's Pending You folder's source, laid out as the module's store
# folder is, its files read-only as the store keeps them, and the folder
# the step writes in the case's home (design Decision 8 of the change
# crews-companions-come-with-the-flake).
freshFolder() {
  fresh
  source=$dir/store/pendingyou-folder
  folder=$home/.config/pendingyou
  mkdir -p "$source/bin" "$source/signin" "$source/mod/hooks" "$source/mod/.claude-plugin"
  printf '#!/bin/sh\nexec pendingyou "$@"\n' >"$source/bin/pendingyou-hook"
  printf '#!/bin/sh\nexec pendingyou mcp-headers "$@"\n' >"$source/bin/pendingyou-mcp-headers"
  echo '{"version":1}' >"$source/claude-code.json"
  echo '{"mcpServers":{}}' >"$source/signin/.claude.json"
  chmod 0555 "$source/bin/pendingyou-hook" "$source/bin/pendingyou-mcp-headers"
  echo '{"name":"pendingyou-wake"}' >"$source/mod/.claude-plugin/plugin.json"
  echo 'The wake mod.' >"$source/mod/README.md"
  echo '{"modules":["./register.js"]}' >"$source/mod/hooks/hooks.json"
  echo 'import "./wake.js";' >"$source/mod/hooks/register.js"
  echo 'export const wake = 1;' >"$source/mod/hooks/wake.js"
  chmod 0444 "$source/claude-code.json" "$source/signin/.claude.json" \
    "$source/mod/.claude-plugin/plugin.json" "$source/mod/README.md" \
    "$source/mod/hooks/hooks.json" "$source/mod/hooks/register.js" "$source/mod/hooks/wake.js"
}
modFiles="mod/hooks/wake.js mod/hooks/register.js mod/hooks/hooks.json mod/.claude-plugin/plugin.json mod/README.md"
# Every file of the source, in the order the step writes them: the wake
# mod's modules, then its hooks, then the rest by their paths.
allFiles="mod/hooks/wake.js mod/hooks/register.js mod/hooks/hooks.json bin/pendingyou-hook bin/pendingyou-mcp-headers claude-code.json mod/.claude-plugin/plugin.json mod/README.md signin/.claude.json"

# `$1` has the mode `$2`, as find reads it.
moded() {
  [ -n "$(find "$1" -prune -perm "$2")" ]
}
# `$1` under the folder is a regular file holding its source's bytes.
written() {
  [ -f "$folder/$1" ] && [ ! -L "$folder/$1" ] && cmp -s "$source/$1" "$folder/$1"
}
# The line the step says for each file written, in order.
wroteAll() {
  for path in $allFiles; do
    echo "boomdrive: wrote $folder/$path"
  done
}

named="A fresh home"
freshFolder
run pendingyou "$source" "$folder"
expect "$named" "pendingyou exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$folder is not made with mode 0700" moded "$folder" 0700
for path in $allFiles; do
  expect "$named" "$folder/$path is not a regular file holding its source" written "$path"
done
for path in $modFiles; do
  expect "$named" "the wake mod's $path is not 0644" moded "$folder/$path" 0644
done
expect "$named" "the launchers are not 0755" moded "$folder/bin/pendingyou-hook" 0755
expect "$named" "the helper is not 0755" moded "$folder/bin/pendingyou-mcp-headers" 0755
expect "$named" "the manifest is not 0644" moded "$folder/claude-code.json" 0644
expect "$named" "the sign-in's Claude folder is not 0644" moded "$folder/signin/.claude.json" 0644
expect "$named" "$folder/bin is not 0755" moded "$folder/bin" 0755
expect "$named" "pendingyou says \"$out\", not one line naming each file" [ "$out" = "$(wroteAll)" ]
expect "$named" "pendingyou left a temporary file: $(find "$folder" -name '*.??????')" \
  [ -z "$(find "$folder" -name '*.??????')" ]
quiet "$named"

named="The wake mod's order"
freshFolder
run pendingyou "$source" "$folder"
first=$(printf '%s\n' "$out" | sed -n '1,3p')
expect "$named" "pendingyou wrote \"$first\" first, not the wake mod's wake.js, register.js and hooks.json in that order" \
  [ "$first" = "boomdrive: wrote $folder/mod/hooks/wake.js
boomdrive: wrote $folder/mod/hooks/register.js
boomdrive: wrote $folder/mod/hooks/hooks.json" ]

named="Nothing differs"
freshFolder
run pendingyou "$source" "$folder"
before=$(ls -li "$folder/bin" "$folder/signin" "$folder/mod/hooks" "$folder")
run pendingyou "$source" "$folder"
expect "$named" "pendingyou exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "pendingyou says \"$out$err\" again" [ -z "$out$err" ]
expect "$named" "pendingyou wrote a file again" [ "$(ls -li "$folder/bin" "$folder/signin" "$folder/mod/hooks" "$folder")" = "$before" ]

named="A file that differs"
freshFolder
run pendingyou "$source" "$folder"
echo '{"version":0}' >"$folder/claude-code.json"
chmod 0600 "$folder/bin/pendingyou-mcp-headers"
run pendingyou "$source" "$folder"
expect "$named" "pendingyou exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "the manifest is not written again" written claude-code.json
expect "$named" "the helper's mode is not set again" moded "$folder/bin/pendingyou-mcp-headers" 0755
expect "$named" "pendingyou says \"$out\", not the two files it wrote" \
  [ "$out" = "boomdrive: wrote $folder/bin/pendingyou-mcp-headers
boomdrive: wrote $folder/claude-code.json" ]

named="The command line's own files"
freshFolder
mkdir -p "$folder/permission-cards" "$folder/bin"
chmod 0700 "$folder"
echo 'sign-in' >"$folder/credentials.json"
echo 'key' >"$folder/machine-key"
echo '{}' >"$folder/permission-cards/card.json"
echo 'own' >"$folder/bin/own-tool"
before=$(cd "$folder" && find . -type f -not -path ./bin/pendingyou-hook \
  -not -path ./bin/pendingyou-mcp-headers -not -path ./claude-code.json \
  -not -path ./signin/.claude.json -not -path './mod/*' | sort | xargs cat)
run pendingyou "$source" "$folder"
expect "$named" "pendingyou exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "pendingyou changed the command line's files" \
  [ "$(cd "$folder" && find . -type f -not -path ./bin/pendingyou-hook \
    -not -path ./bin/pendingyou-mcp-headers -not -path ./claude-code.json \
    -not -path ./signin/.claude.json -not -path './mod/*' | sort | xargs cat)" = "$before" ]
expect "$named" "pendingyou removed the permission card" holds "$folder/permission-cards/card.json" '{}'
expect "$named" "pendingyou changed the folder's own mode" moded "$folder" 0700
for path in $allFiles; do
  expect "$named" "$folder/$path is not written beside the command line's" written "$path"
done

named="A link where a launcher goes"
freshFolder
mkdir -p "$folder/bin" "$dir/store/old/bin"
echo 'old launcher' >"$dir/store/old/bin/pendingyou-hook"
ln -s "$dir/store/old/bin/pendingyou-hook" "$folder/bin/pendingyou-hook"
run pendingyou "$source" "$folder"
expect "$named" "pendingyou exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$folder/bin/pendingyou-hook is still a link" [ ! -L "$folder/bin/pendingyou-hook" ]
expect "$named" "$folder/bin/pendingyou-hook does not hold its source's bytes" written bin/pendingyou-hook
expect "$named" "pendingyou wrote through the link" holds "$dir/store/old/bin/pendingyou-hook" 'old launcher'

named="A file it cannot write"
freshFolder
mkdir -p "$folder/bin"
chmod 0500 "$folder/bin"
run pendingyou "$source" "$folder"
chmod 0755 "$folder/bin"
expect "$named" "pendingyou exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "pendingyou said \"$err\", not one line naming $folder/bin/pendingyou-hook" \
  [ "$(printf '%s\n' "$err" | wc -l | tr -d ' ')" = 1 ]
expect "$named" "pendingyou said \"$err\", not one line naming $folder/bin/pendingyou-hook" \
  names "$err" "did not write" "$folder/bin/pendingyou-hook"
expect "$named" "pendingyou wrote \"$out\", not the wake mod's modules and hooks before the file it could not write and none after it" \
  [ "$out" = "boomdrive: wrote $folder/mod/hooks/wake.js
boomdrive: wrote $folder/mod/hooks/register.js
boomdrive: wrote $folder/mod/hooks/hooks.json" ]

named="pendingyou's usage"
freshFolder
run pendingyou "$source"
expect "$named" "pendingyou with no folder exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "pendingyou with no folder made $folder" missing "$folder"

# A case of the install into Claude Code's homes (design Decisions 2 to 4
# and 6, Task notes 3.1, of the change
# every-claude-code-session-has-boomdrives-plugins): a marketplace in the
# case's store listing flume at 0.31.0, timber at 0.7.0, pendingyou and
# plannotator, Pending You's server's file, `serverFile`, which the step is
# given only where a case sets `server` to it, `-` by default, and stubs of flume, run by its
# path, and of claude, first on PATH. flume records its calls with the
# others, and for `--json status` prints `status.json`, an empty pool by
# default, or refuses where the case removed it, exiting 1. claude records
# each call, after the CLAUDE_CONFIG_DIR it sees or `unset`, in
# `claude-calls`; for `plugin list --json` it prints the home's
# `plugins.json`, `$CLAUDE_CONFIG_DIR/plugins.json` or else
# `$HOME/.claude/plugins.json`, or `[]` where there is none; and it
# refuses the call whose line in `claude-calls` is the first line of
# `refuse`, printing `{"outcome":"failed","message":"<words>"}` with the
# words of its second line and exiting 1. jq is the shell's own.
if ! jq=$(command -v jq); then
  echo "nix: Claude Code's homes: found no jq on PATH for the step's cases"
  jq=jq
fi
freshClaude() {
  fresh
  flume=$dir/store/flume/bin/flume
  market=$dir/store/boomdrive-claude-plugins
  serverFile=$dir/store/pendingyou-server.json
  server=-
  claudes=$dir/claude-calls
  mkdir -p "$(dirname "$flume")" "$market/.claude-plugin"
  cat >"$market/.claude-plugin/marketplace.json" <<'JSON'
{"name":"boomdrive","plugins":[
{"name":"flume","version":"0.31.0","source":"./plugins/flume"},
{"name":"timber","version":"0.7.0","source":"./plugins/timber"},
{"name":"pendingyou","version":"0.41.0","source":"./plugins/pendingyou"},
{"name":"plannotator","version":"0.28.61","source":"./plugins/plannotator"}]}
JSON
  serverJSON='{"type":"http","url":"https://www.pendingyou.com/mcp","headersHelper":"/scratch/pendingyou-mcp-headers"}'
  echo "$serverJSON" >"$serverFile"
  statusOf ""
  cat >"$flume" <<STUB
#!/bin/sh
echo "flume \$*" >>"$calls"
if [ "\$*" = "--json status" ] && [ -e "$dir/status.json" ]; then
  cat "$dir/status.json"
  exit 0
fi
echo '{"refused":{"refusal":{"reason":"no host configuration"}}}'
exit 1
STUB
  cat >"$dir/bin/claude" <<STUB
#!/bin/sh
seen=\${CLAUDE_CONFIG_DIR-unset}
echo "\$seen claude \$*" >>"$claudes"
if [ "\$seen claude \$*" = "\$(sed -n 1p "$dir/refuse" 2>/dev/null)" ]; then
  printf '{"outcome":"failed","message":"%s"}\n' "\$(sed -n 2p "$dir/refuse")"
  echo "refused" >&2
  exit 1
fi
if [ "\$*" = "plugin list --json" ]; then
  cat "\${CLAUDE_CONFIG_DIR:-\$HOME/.claude}/plugins.json" 2>/dev/null || echo '[]'
  exit 0
fi
echo '{"outcome":"ok"}'
STUB
  chmod +x "$flume" "$dir/bin/claude"
  # The two calls every home gets first.
  firstTwo="claude plugin marketplace add $market --json;claude plugin list --json"
}

# Flume's status, its one pool holding the accounts `$1`.
statusOf() {
  printf '{"status":{"host":{"pools":[{"name":"work","accounts":[%s]}]}}}\n' "$1" >"$dir/status.json"
}
# An account named `$1` of the program `$2`, whose home is the JSON `$3`.
accountOf() {
  printf '{"name":"%s","program":"%s","running":0,"bound":null,"spent_until":null,"home":%s}' "$1" "$2" "$3"
}
# The home `$1` (the default home's .claude where empty) lists the
# plugins `$2`, each `<id> <version>`, as `claude plugin list --json`.
listing() {
  into=${1:-$home/.claude}
  mkdir -p "$into"
  printf '%s\n' "$2" | "$jq" -R -s '
    split("\n") | map(select(length > 0) | split(" ") | {id: .[0], version: .[1], scope: "user", enabled: true})
  ' >"$into/plugins.json"
}
# The step with the plugins `$@` to install, then `--` and those to remove.
runClaude() {
  run claude "$flume" "$jq" "$market" "$server" "$@"
}
# The homes claude was called in, by the CLAUDE_CONFIG_DIR each saw, in
# the order first called.
homesCalled() {
  if [ -e "$claudes" ]; then
    awk '!seen[$1]++ { print $1 }' "$claudes" | paste -s -d ' ' -
  fi
}
# The calls in the home `$1`, `unset` for the default, on one line.
callsIn() {
  if [ -e "$claudes" ]; then
    awk -v home="$1" '$1 == home { $1 = ""; sub(/^ /, ""); print }' "$claudes" | paste -s -d ';' -
  fi
}
named="Two accounts and the default home"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\""),$(accountOf work-2 claude "\"$dir/b\"")"
runClaude flume
expect "$named" "claude exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "claude ran in $(homesCalled), not the default home, then $dir/a, then $dir/b" \
  [ "$(homesCalled)" = "unset $dir/a $dir/b" ]
expect "$named" "claude says \"$out$err\"" [ -z "$out$err" ]

named="An account with no home"
freshClaude
statusOf "$(accountOf work-1 claude null),$(accountOf work-2 claude "\"$dir/a\"")"
runClaude flume
expect "$named" "claude ran in $(homesCalled), not the default home once, then $dir/a" \
  [ "$(homesCalled)" = "unset $dir/a" ]
expect "$named" "the default home got $(callsIn unset)" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin install flume@boomdrive --json" ]

named="Two accounts sharing one home"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\""),$(accountOf work-2 claude "\"$dir/a\"")"
runClaude flume
expect "$named" "claude ran in $(homesCalled), not the default home, then $dir/a" \
  [ "$(homesCalled)" = "unset $dir/a" ]
expect "$named" "$dir/a got $(callsIn "$dir/a"), not its commands once" \
  [ "$(callsIn "$dir/a")" = "$firstTwo;claude plugin install flume@boomdrive --json" ]

named="An account of another program"
freshClaude
statusOf "$(accountOf other-1 codex "\"$dir/c\""),$(accountOf work-1 claude "\"$dir/a\"")"
runClaude flume
expect "$named" "claude ran in $(homesCalled), not the default home, then $dir/a" \
  [ "$(homesCalled)" = "unset $dir/a" ]

named="Flume's status cannot be read"
freshClaude
rm "$dir/status.json"
runClaude flume
expect "$named" "claude exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "claude ran in $(homesCalled), not the default home alone" \
  [ "$(homesCalled)" = unset ]
expect "$named" "claude said \"$err\", not one line that Flume's status could not be read" \
  [ "$(printf '%s\n' "$err" | wc -l | tr -d ' ')" -eq 1 ]
expect "$named" "claude said \"$err\"" names "$err" "could not read Flume's status" "no host configuration"

named="A fresh home"
freshClaude
runClaude flume timber -- pendingyou
expect "$named" "the default home got $(callsIn unset), not the marketplace, the list and the install of flume and timber" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin install flume@boomdrive --json;claude plugin install timber@boomdrive --json" ]

named="Nothing differs"
freshClaude
listing "" "flume@boomdrive 0.31.0
timber@boomdrive 0.7.0"
runClaude flume timber -- pendingyou
expect "$named" "the default home got $(callsIn unset), not the marketplace and the list alone" \
  [ "$(callsIn unset)" = "$firstTwo" ]
expect "$named" "claude says \"$out$err\"" [ -z "$out$err" ]

named="A new build of a plugin"
freshClaude
listing "" "flume@boomdrive 0.31.0
timber@boomdrive 0.6.0"
runClaude flume timber
expect "$named" "the default home got $(callsIn unset), not timber's update alone" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin update timber@boomdrive --scope user --json" ]

named="A plugin to remove"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\"")"
listing "" "flume@boomdrive 0.31.0
pendingyou@boomdrive 0.41.0"
listing "$dir/a" "flume@boomdrive 0.31.0"
runClaude flume -- pendingyou plannotator
expect "$named" "the default home got $(callsIn unset), not pendingyou's uninstall alone" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin uninstall pendingyou@boomdrive --json" ]
expect "$named" "$dir/a got $(callsIn "$dir/a"), not the marketplace and the list alone" \
  [ "$(callsIn "$dir/a")" = "$firstTwo" ]

named="The home's own plugins"
freshClaude
listing "" "flume@boomdrive 0.31.0
plannotator@plannotator 0.28.61
foo@other 1.0.0"
runClaude flume plannotator -- pendingyou
expect "$named" "the default home got $(callsIn unset), not plannotator@boomdrive's install alone" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin install plannotator@boomdrive --json" ]
expect "$named" "claude named the home's own plugins: $(callsIn unset)" \
  [ -z "$(grep -e 'plannotator@plannotator' -e 'foo@other' "$claudes")" ]

named="A home with no server"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\"")"
mkdir -p "$dir/a"
echo '{"mcpServers":{"other":{"type":"http","url":"https://example.com/mcp"}}}' >"$dir/a/.claude.json"
server=$serverFile
runClaude flume
expect "$named" "the default home got $(callsIn unset), not the server's add after the plugins" \
  [ "$(callsIn unset)" = "$firstTwo;claude plugin install flume@boomdrive --json;claude mcp add-json --scope user pendingyou $serverJSON" ]
expect "$named" "$dir/a got $(callsIn "$dir/a"), not the server's add after the plugins" \
  [ "$(callsIn "$dir/a")" = "$firstTwo;claude plugin install flume@boomdrive --json;claude mcp add-json --scope user pendingyou $serverJSON" ]

named="A home that names one"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\"")"
mkdir -p "$dir/a"
echo '{"mcpServers":{"pendingyou":{"type":"http","url":"https://example.com/own"}}}' >"$dir/a/.claude.json"
echo '{"mcpServers":{"pendingyou":{"type":"http","url":"https://example.com/own"}}}' >"$home/.claude.json"
server=$serverFile
runClaude flume
expect "$named" "claude ran $(grep ' mcp ' "$claudes")" [ -z "$(grep ' mcp ' "$claudes")" ]

named="No server given"
freshClaude
echo '{"mcpServers":{"pendingyou":{"type":"http","url":"https://example.com/own"}}}' >"$home/.claude.json"
runClaude flume
expect "$named" "claude ran $(grep ' mcp ' "$claudes")" [ -z "$(grep ' mcp ' "$claudes")" ]
expect "$named" "the server changed" \
  holds "$home/.claude.json" '{"mcpServers":{"pendingyou":{"type":"http","url":"https://example.com/own"}}}'

named="No Claude Code"
freshClaude
rm "$dir/bin/claude"
bare=$(
  printf '%s\n' "$PATH" | tr ':' '\n' | while IFS= read -r each; do
    if [ -n "$each" ] && [ ! -x "$each/claude" ]; then
      printf '%s:' "$each"
    fi
  done
)
status=0
mkdir -p "$dir/tmp"
HOME=$home TMPDIR=$dir/tmp PATH=$dir/bin:${bare%:} sh "$activate" claude "$flume" "$jq" "$market" "$server" flume \
  >"$dir/out" 2>"$dir/err" || status=$?
out=$(cat "$dir/out")
err=$(cat "$dir/err")
expect "$named" "claude exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "claude said \"$err\", not one line that no claude was found" \
  [ "$err" = "boomdrive: found no claude on PATH, so no Claude Code home has the set's plugins" ]
expect "$named" "claude ran $(called)" missing "$calls"

named="A command refused"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\"")"
printf '%s\n%s\n' "$dir/a claude plugin install timber@boomdrive --json" "no such plugin" >"$dir/refuse"
server=$serverFile
runClaude timber flume
expect "$named" "claude exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "claude said \"$err\", not one line naming $dir/a, the command and Claude Code's words" \
  [ "$err" = "boomdrive: Claude Code refused \`claude plugin install timber@boomdrive --json\` in $dir/a: no such plugin" ]
expect "$named" "$dir/a got $(callsIn "$dir/a"), not the commands after the one refused" \
  [ "$(callsIn "$dir/a")" = "$firstTwo;claude plugin install timber@boomdrive --json;claude plugin install flume@boomdrive --json;claude mcp add-json --scope user pendingyou $serverJSON" ]

named="Every command held"
freshClaude
statusOf "$(accountOf work-1 claude "\"$dir/a\"")"
listing "$dir/a" "timber@boomdrive 0.6.0
pendingyou@boomdrive 0.41.0"
server=$serverFile
runClaude flume timber -- pendingyou
expect "$named" "claude exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "claude says \"$out$err\"" [ -z "$out$err" ]

named="claude's usage"
freshClaude
run claude "$flume" "$jq" "$market" "$server"
expect "$named" "claude with no plugin exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "claude with no plugin ran claude" missing "$claudes"

# A case's managed telemetry document: its source in the case's store,
# read-only as the store keeps it, and its path in a scratch folder that
# stands for Claude Code's managed-settings.d, missing until a case makes
# it (design Decision 9 and Task notes 1.3 of the change
# every-claude-code-session-exports-to-the-hosts-collector).
freshManaged() {
  fresh
  document=$dir/store/10-boomdrive-telemetry.json
  settings=$dir/ClaudeCode/managed-settings.d
  managed=$settings/10-boomdrive-telemetry.json
  echo '{"env":{"CLAUDE_CODE_ENABLE_TELEMETRY":"1"}}' >"$document"
  chmod 0444 "$document"
}
# The path is a regular file holding the source's bytes, mode 0644.
documented() {
  [ -f "$managed" ] && [ ! -L "$managed" ] && cmp -s "$document" "$managed" && moded "$managed" 0644
}
# What the folder holds, by name, inode and mode.
listing() {
  ls -lai "$settings"
}

named="A fresh host"
freshManaged
run managed "$managed" "$document"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$settings is not made with mode 0755" moded "$settings" 0755
expect "$named" "$managed is not a regular file of mode 0644 holding the document" documented
expect "$named" "managed says \"$out\", not one line naming $managed" \
  [ "$out" = "boomdrive: wrote $managed" ]
expect "$named" "managed says \"$err\" on standard error" [ -z "$err" ]
expect "$named" "managed left a temporary file: $(find "$settings" -name '*.??????')" \
  [ -z "$(find "$settings" -name '*.??????')" ]
quiet "$named"

named="Nothing differs"
freshManaged
run managed "$managed" "$document"
before=$(listing)
run managed "$managed" "$document"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "managed says \"$out$err\" again" [ -z "$out$err" ]
expect "$named" "managed wrote the document again" [ "$(listing)" = "$before" ]

named="A document that differs"
freshManaged
mkdir -p "$settings"
echo '{"env":{}}' >"$managed"
chmod 0644 "$managed"
run managed "$managed" "$document"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$managed is not written again where its bytes differ" documented
expect "$named" "managed says \"$out\", not one line naming $managed" \
  [ "$out" = "boomdrive: wrote $managed" ]
run managed "$managed" "$document"
chmod 0600 "$managed"
run managed "$managed" "$document"
expect "$named" "$managed is not written again where its mode differs" documented
expect "$named" "managed says \"$out\", not one line naming $managed, where its mode differs" \
  [ "$out" = "boomdrive: wrote $managed" ]

named="A link where the document goes"
freshManaged
mkdir -p "$settings" "$dir/store/old"
echo '{"env":{"OTEL_LOG_USER_PROMPTS":"1"}}' >"$dir/store/old/10-boomdrive-telemetry.json"
ln -s "$dir/store/old/10-boomdrive-telemetry.json" "$managed"
run managed "$managed" "$document"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$managed is still a link" [ ! -L "$managed" ]
expect "$named" "$managed is not a regular file of mode 0644 holding the document" documented
expect "$named" "managed wrote through the link" \
  holds "$dir/store/old/10-boomdrive-telemetry.json" '{"env":{"OTEL_LOG_USER_PROMPTS":"1"}}'

named="A folder it cannot write"
freshManaged
mkdir -p "$settings"
chmod 0500 "$settings"
run managed "$managed" "$document"
chmod 0755 "$settings"
expect "$named" "managed exits $status, not 0" [ "$status" -eq 0 ]
expect "$named" "managed said \"$err\", not one line naming $managed" \
  [ "$(printf '%s\n' "$err" | wc -l | tr -d ' ')" = 1 ]
expect "$named" "managed said \"$err\", not one line naming $managed" \
  names "$err" "did not write" "$managed"
expect "$named" "managed says \"$out\"" [ -z "$out" ]
expect "$named" "managed wrote $managed" missing "$managed"

named="The document removed"
freshManaged
mkdir -p "$settings"
cp "$document" "$managed"
echo '{"env":{"OTEL_RESOURCE_ATTRIBUTES":"host=own"}}' >"$settings/20-own.json"
echo '{"permissions":{}}' >"$dir/ClaudeCode/managed-settings.json"
run managed "$managed"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "managed left $managed" missing "$managed"
expect "$named" "managed changed the host's own document" \
  holds "$settings/20-own.json" '{"env":{"OTEL_RESOURCE_ATTRIBUTES":"host=own"}}'
expect "$named" "managed changed the host's own managed-settings.json" \
  holds "$dir/ClaudeCode/managed-settings.json" '{"permissions":{}}'
expect "$named" "managed says \"$out\", not one line naming $managed" \
  [ "$out" = "boomdrive: removed $managed" ]
expect "$named" "managed says \"$err\" on standard error" [ -z "$err" ]
quiet "$named"

freshManaged
mkdir -p "$settings"
ln -s "$document" "$managed"
run managed "$managed"
expect "$named" "managed of a link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "managed left the link $managed" missing "$managed"
expect "$named" "managed removed the link's target" [ -f "$document" ]

named="Nothing to remove"
freshManaged
mkdir -p "$settings"
echo '{"env":{}}' >"$settings/20-own.json"
before=$(listing)
run managed "$managed"
expect "$named" "managed exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "managed says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "managed changed the folder" [ "$(listing)" = "$before" ]

freshManaged
run managed "$managed"
expect "$named" "managed with no folder exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "managed with no folder says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "managed with no folder made $settings" missing "$settings"

named="managed's usage"
freshManaged
run managed
expect "$named" "managed with no path exits $status, not 2" [ "$status" -eq 2 ]
run managed "$managed" "$document" more
expect "$named" "managed with a word after the source exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "managed's usage made $settings" missing "$settings"

# The identities the signing's cases name: the one security's import makes,
# one the keychain holds already, and another.
made=5E4D3C2B1A0F9E8D7C6B5A4F3E2D1C0B9A8F7E6D
held=0F1E2D3C4B5A69788796A5B4C3D2E1F00F1E2D3C
other=9A8B7C6D5E4F30211203F4E5D6C7B8A99A8B7C6D

# A case of the signing: the keychain, a word the stubs take; the folder,
# with a space in its path as on macOS, and not made yet; the program, a
# build of the runner in the case's store; and stubs of security, codesign
# and openssl, which record their calls. security's find-identity lists the
# lines of `identities` in its own form. Its import refuses with the words
# of `refuse-import` where that is there, and adds the identity $made to
# `identities` once it has imported both the key and the certificate
# openssl wrote. codesign's --verify passes where the file holds `signed
# <SHA-1>` of its requirement's leaf, and its --force --sign appends
# `signed <SHA-1>` to the file, or refuses with the words of `refuse-sign`
# where that is there. openssl writes `key` to its -keyout file and `cert`
# to its -out file, and records their folder in `openssl-folder` and its
# configuration in `openssl.cnf`. mv refuses where its last argument is the
# path on the first line of `refuse-move`, saying the lines after it, and
# otherwise runs the system's own mv, found before the stubs are first on
# PATH.
freshSign() {
  fresh
  keychain=$dir/keychain
  folder="$dir/Application Support/Boomdrive"
  copy=$folder/boomdrive
  source=$folder/boomdrive.source
  mkdir -p "$(dirname "$folder")" "$(dirname "$program")"
  echo "the runner" >"$program"
  : >"$dir/identities"
  echo "$made" >"$dir/made"
  command -v mv >"$dir/system-mv"
  cat >"$dir/bin/security" <<'STUB'
#!/bin/sh
here=${0%/bin/*}
echo "security $*" >>"$here/calls"
case $1 in
find-identity)
  echo 'Policy: Code Signing'
  echo '  Matching identities'
  cat "$here/identities"
  echo "     $(wc -l <"$here/identities" | tr -d ' ') identities found"
  echo
  echo '  Valid identities only'
  echo '     0 valid identities found'
  ;;
import)
  if [ -e "$here/refuse-import" ]; then
    cat "$here/refuse-import" >&2
    exit 1
  fi
  case $2 in
  */key.pem) [ "$(cat "$2")" != key ] || : >"$here/imported-key" ;;
  */cert.pem) [ "$(cat "$2")" != cert ] || : >"$here/imported-cert" ;;
  esac
  if [ -e "$here/imported-key" ] && [ -e "$here/imported-cert" ]; then
    echo "  1) $(cat "$here/made") \"Boomdrive runner\" (CSSMERR_TP_NOT_TRUSTED)" >>"$here/identities"
  fi
  ;;
*) exit 64 ;;
esac
STUB
  cat >"$dir/bin/codesign" <<'STUB'
#!/bin/sh
here=${0%/bin/*}
echo "codesign $*" >>"$here/calls"
for file; do :; done
case $1 in
--verify)
  sha=${3#*H\"}
  sha=${sha%\"*}
  grep -qx "signed $sha" "$file" 2>/dev/null
  ;;
--force)
  if [ -e "$here/refuse-sign" ]; then
    cat "$here/refuse-sign" >&2
    exit 1
  fi
  echo "signed $3" >>"$file"
  ;;
*) exit 64 ;;
esac
STUB
  cat >"$dir/bin/openssl" <<'STUB'
#!/bin/sh
here=${0%/bin/*}
echo "openssl $*" >>"$here/calls"
while [ $# -gt 0 ]; do
  case $1 in
  -config) config=$2 ;;
  -keyout) key=$2 ;;
  -out) cert=$2 ;;
  esac
  shift
done
echo key >"$key"
echo cert >"$cert"
dirname "$key" >"$here/openssl-folder"
cp "$config" "$here/openssl.cnf"
STUB
  cat >"$dir/bin/mv" <<'STUB'
#!/bin/sh
here=${0%/bin/*}
echo "mv $*" >>"$here/calls"
for last; do :; done
if [ -e "$here/refuse-move" ] && [ "$last" = "$(sed -n 1p "$here/refuse-move")" ]; then
  sed 1d "$here/refuse-move" >&2
  exit 1
fi
exec "$(cat "$here/system-mv")" "$@"
STUB
  chmod +x "$dir/bin/security" "$dir/bin/codesign" "$dir/bin/openssl" "$dir/bin/mv"
}

# security's line of the identity `$2` named `$3`, numbered `$1`.
identity() {
  echo "  $1) $2 \"$3\" (CSSMERR_TP_NOT_TRUSTED)" >>"$dir/identities"
}

# The folder `$1` holds nothing.
empty() {
  [ -z "$(ls -A "$1")" ]
}

named="A folder another user can write"
for write in g+w o+w; do
  freshSign
  mkdir -m 0755 "$folder"
  chmod "$write" "$folder"
  run sign "$keychain" "$folder" "$program"
  expect "$named" "sign into a folder $write exits $status, not 1" [ "$status" -eq 1 ]
  expect "$named" "sign into a folder $write says \"$err\", not one line naming $folder" \
    [ "$err" = "boomdrive: $folder is writable by a user other than alice: signed nothing" ]
  expect "$named" "sign into a folder $write says \"$out\"" [ -z "$out" ]
  expect "$named" "sign into a folder $write called $(called)" missing "$calls"
  expect "$named" "sign into a folder $write wrote $(ls -A "$folder")" empty "$folder"
done

freshSign
mkdir -m 0755 "$folder"
echo $((uid + 1)) >"$dir/uid"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign into another user's folder exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "sign into another user's folder says \"$err\", not one line naming $folder" \
  [ "$err" = "boomdrive: $folder is writable by a user other than alice: signed nothing" ]
expect "$named" "sign into another user's folder called $(called)" missing "$calls"
expect "$named" "sign into another user's folder wrote $(ls -A "$folder")" empty "$folder"

named="sign's usage"
freshSign
run sign "$keychain" "$folder"
expect "$named" "sign with two words exits $status, not 2" [ "$status" -eq 2 ]
run sign "$keychain" "$folder" "$program" more
expect "$named" "sign with four words exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "sign's usage called $(called)" missing "$calls"
expect "$named" "sign's usage made $folder" missing "$folder"

# The stubs' calls of `$1`, on one line each.
calledOf() {
  grep "^$1 " "$calls" 2>/dev/null || true
}
# The first line of `$1`.
first() {
  printf '%s\n' "$1" | sed -n 1p
}
# The copy holds the program's bytes, then a signature of the identity
# `$1`, and runs.
signedBy() {
  holds "$copy" "the runner
signed $1" && moded "$copy" 0755
}
# No file under the case's folder reads the key or the certificate openssl
# wrote.
keyless() {
  [ -z "$(find "$dir" -type f -exec grep -lx -e key -e cert {} +)" ]
}

named="No identity yet"
freshSign
identity 1 "$other" "Another signer"
run sign "$keychain" "$folder" "$program"
madeIn=$(cat "$dir/openssl-folder" 2>/dev/null || true)
expect "$named" "sign exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sign read the identities by $(calledOf security), not find-identity -p codesigning $keychain" \
  grep -qxF "security find-identity -p codesigning $keychain" "$calls"
expect "$named" "sign ran $(calledOf openssl), not openssl req for a self-signed key and certificate in a folder of its own" \
  [ "$(calledOf openssl)" = "openssl req -x509 -newkey rsa:2048 -nodes -days 7300 -config $madeIn/openssl.cnf -keyout $madeIn/key.pem -out $madeIn/cert.pem" ]
expect "$named" "sign imported by $(calledOf 'security import'), not the key with -x -T /usr/bin/codesign, then the certificate" \
  [ "$(calledOf 'security import')" = "security import $madeIn/key.pem -k $keychain -x -T /usr/bin/codesign
security import $madeIn/cert.pem -k $keychain" ]
for line in 'prompt = no' 'CN = Boomdrive runner' 'basicConstraints = critical,CA:false' \
  'keyUsage = critical,digitalSignature' 'extendedKeyUsage = critical,codeSigning'; do
  expect "$named" "openssl's configuration holds no line $line" grep -qxF "$line" "$dir/openssl.cnf"
done
expect "$named" "sign left the folder openssl wrote in, $madeIn" missing "$madeIn"
expect "$named" "sign says \"$out\", not first that it made the identity $made" \
  [ "$(first "$out")" = "boomdrive: made the code-signing identity Boomdrive runner, $made, in $keychain" ]
expect "$named" "the copy is not signed with the identity made, $made" signedBy "$made"
expect "$named" "sign says \"$out\", not then that it signed the copy with $made" \
  [ "$(printf '%s\n' "$out" | sed -n 2p)" = "boomdrive: signed $copy, made from $program, with Boomdrive runner, $made" ]

named="No key in a file"
expect "$named" "the step's folder for openssl was $madeIn, not one in the case's TMPDIR" \
  names "$madeIn" "$dir/tmp/" ""
expect "$named" "a file holds the key or the certificate: $(find "$dir" -type f -exec grep -lx -e key -e cert {} +)" keyless

named="An identity that cannot be made"
freshSign
printf 'SecKeychainItemImport: Write permissions error.\nsecurity: import failed\n' >"$dir/refuse-import"
run sign "$keychain" "$folder" "$program"
madeIn=$(cat "$dir/openssl-folder" 2>/dev/null || true)
expect "$named" "sign exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "sign says \"$err\", not one line with the keychain's words" \
  [ "$err" = "boomdrive: could not make the code-signing identity in $keychain: SecKeychainItemImport: Write permissions error. security: import failed" ]
expect "$named" "sign says \"$out\"" [ -z "$out" ]
expect "$named" "sign left the folder openssl wrote in, $madeIn" missing "$madeIn"
expect "$named" "sign signed by $(calledOf codesign)" [ -z "$(calledOf codesign)" ]
expect "$named" "sign wrote $(ls -A "$folder")" empty "$folder"
expect "$named" "a file holds the key or the certificate: $(find "$dir" -type f -exec grep -lx -e key -e cert {} +)" keyless

named="Two identities of the name"
freshSign
identity 1 "$held" "Boomdrive runner"
identity 2 "$other" "Boomdrive runner"
identity 3 "$held" "Boomdrive runner"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "sign says \"$err\", not one line naming each identity's SHA-1" \
  [ "$err" = "boomdrive: $keychain holds 2 identities named Boomdrive runner, $held, $other: signed nothing; remove all but one" ]
expect "$named" "sign says \"$out\"" [ -z "$out" ]
expect "$named" "sign ran $(calledOf openssl)$(calledOf 'security import')$(calledOf codesign)" \
  [ -z "$(calledOf openssl)$(calledOf 'security import')$(calledOf codesign)" ]
expect "$named" "sign wrote $(ls -A "$folder")" empty "$folder"

# A copy and its source the step made before, from an older build, signed
# with the identity `$1`, and the keychain holding $held.
older=
signedBefore() {
  older=$dir/store/older/bin/boomdrive
  identity 1 "$held" "Boomdrive runner"
  mkdir -m 0755 "$folder"
  printf 'the older runner\nsigned %s\n' "$1" >"$copy"
  chmod 0755 "$copy"
  echo "$older" >"$source"
}
# The folder holds only the copy and its source, or only the names `$@`.
only() {
  [ "$(inFolder)" = "$*" ]
}
# The folder's names, on one line.
inFolder() {
  find "$folder" -mindepth 1 -maxdepth 1 -exec basename {} \; | sort | paste -s -d ' ' -
}

named="A new build"
freshSign
signedBefore "$held"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sign signed by $(calledOf 'codesign --force'), not the identity's SHA-1 with the identifier boomdrive, no timestamp and the keychain, on boomdrive.new" \
  [ "$(calledOf 'codesign --force')" = "codesign --force --sign $held --keychain $keychain --identifier boomdrive --timestamp=none $copy.new" ]
expect "$named" "sign moved by $(calledOf mv), not the copy onto boomdrive, then its source onto boomdrive.source, each by mv -f" \
  [ "$(calledOf mv)" = "mv -f $copy.new $copy
mv -f $source.new $source" ]
expect "$named" "the copy is not the program signed with $held" signedBy "$held"
expect "$named" "boomdrive.source holds $(cat "$source"), not $program" holds "$source" "$program"
expect "$named" "the folder holds $(inFolder), not boomdrive and boomdrive.source alone" \
  only boomdrive boomdrive.source
expect "$named" "sign says \"$out\", not one line naming the program" \
  [ "$out" = "boomdrive: signed $copy, made from $program, with Boomdrive runner, $held" ]
expect "$named" "sign says \"$err\" on standard error" [ -z "$err" ]
expect "$named" "sign ran $(calledOf openssl)$(calledOf 'security import')" \
  [ -z "$(calledOf openssl)$(calledOf 'security import')" ]

named="The same build"
freshSign
identity 1 "$held" "Boomdrive runner"
run sign "$keychain" "$folder" "$program"
before=$(
  ls -lai "$folder"
  cat "$copy" "$source"
)
rm -f "$calls"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sign signed again by $(calledOf 'codesign --force')" [ -z "$(calledOf 'codesign --force')" ]
expect "$named" "sign did not verify the copy's signature: $(called)" \
  [ "$(calledOf 'codesign --verify')" = "codesign --verify -R =certificate leaf = H\"$held\" $copy" ]
expect "$named" "sign changed a file of the folder" [ "$(
  ls -lai "$folder"
  cat "$copy" "$source"
)" = "$before" ]
expect "$named" "sign says \"$out$err\" again" [ -z "$out$err" ]

named="A copy signed with another identity"
freshSign
signedBefore "$other"
echo "$program" >"$source"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sign signed by $(calledOf 'codesign --force'), not the identity $held" \
  [ "$(calledOf 'codesign --force')" = "codesign --force --sign $held --keychain $keychain --identifier boomdrive --timestamp=none $copy.new" ]
expect "$named" "the copy is not the program signed with $held" signedBy "$held"
expect "$named" "boomdrive.source holds $(cat "$source"), not $program" holds "$source" "$program"
expect "$named" "sign says \"$out\", not one line naming the program" \
  [ "$out" = "boomdrive: signed $copy, made from $program, with Boomdrive runner, $held" ]

named="A signing refused"
freshSign
signedBefore "$held"
before=$(
  ls -lai "$folder"
  cat "$copy" "$source"
)
printf '%s: errSecInternalComponent\n' "$copy.new" >"$dir/refuse-sign"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "sign says \"$err\", not one line with codesign's words" \
  [ "$err" = "boomdrive: could not sign $program into $folder: $copy.new: errSecInternalComponent" ]
expect "$named" "sign says \"$out\"" [ -z "$out" ]
expect "$named" "the folder holds $(inFolder), not boomdrive and boomdrive.source alone" \
  only boomdrive boomdrive.source
expect "$named" "sign changed boomdrive or boomdrive.source" [ "$(
  ls -lai "$folder"
  cat "$copy" "$source"
)" = "$before" ]

named="A copy not put in place"
freshSign
signedBefore "$held"
printf '%s\n%s\n' "$copy" "mv: rename $copy.new to $copy: Operation not permitted" >"$dir/refuse-move"
run sign "$keychain" "$folder" "$program"
expect "$named" "sign exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "sign says \"$err\", not one line with the rename's words" \
  [ "$err" = "boomdrive: could not sign $program into $folder: mv: rename $copy.new to $copy: Operation not permitted" ]
expect "$named" "sign says \"$out\"" [ -z "$out" ]
expect "$named" "sign changed boomdrive" holds "$copy" "the older runner
signed $held"
expect "$named" "sign left boomdrive.source, holding $(cat "$source" 2>/dev/null)" missing "$source"
expect "$named" "the folder holds $(inFolder), not boomdrive alone" only boomdrive
