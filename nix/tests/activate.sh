#!/bin/sh
# The activation's cases (design Decision 14, Task notes 5.2), which
# nix/check.sh test runs beside the evaluated ones: each runs
# nix/modules/activate.sh by sh in a scratch folder of its own, with a
# scratch HOME, the module's file in a store of its own, and stubs of
# systemctl and ps first on PATH that record their calls, and of id. The
# stubs answer from the case's folder: id the user running it, alice
# unless `user` says another; systemctl's show the unit's MainPID, 0
# unless `MainPID` says another; and ps the command line in `args`. The
# plugins' cases add a stub of herdr, run by its path as the modules run
# it, that records its arguments and the HERDR_* variables it sees, and
# refuses the plugin `refuse` names, saying `manifest not found`. It
# prints each failure as one line, nix: <case>: <what>, and nothing where
# every case holds.
set -eu

activate=$(cd "$(dirname "$0")/../modules" && pwd)/activate.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0

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
cat "$dir/user"
STUB
  chmod +x "$dir/bin/systemctl" "$dir/bin/ps" "$dir/bin/id"
}

# activate.sh with `$@` in the case's folder: its status, output and
# error.
run() {
  status=0
  HOME=$home PATH=$dir/bin:$PATH sh "$activate" "$@" >"$dir/out" 2>"$dir/err" || status=$?
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
