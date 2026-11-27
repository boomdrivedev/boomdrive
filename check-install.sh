#!/bin/sh
# shellcheck disable=SC2016 # the stubs' bodies expand at their run.
# root:install: the install spec's scenarios, each run of install.sh on
# a scratch home whose PATH holds stubs of the programs it runs (uname,
# curl, sudo, nix, Nix's installer and boomdrive), each of which writes
# its arguments, and boomdrive its standard input, to a log. Each
# scenario runs under the devenv shell's sh and, where BOOMDRIVE_MAC_SH
# is set, under macOS's /bin/sh, bash 3.2.57 in POSIX mode (core.8).
# Each line is one scenario, pass or fail.
#
#   sh check-install.sh
set -u

tree=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/check-install.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
status=0
fail() {
  echo "install: fail: $*"
  status=1
}
pass() {
  echo "install: pass: $*"
}

token=a-one-time-token
version=$(sed -n 's/^version=//p' "$tree/install.sh")
flake=github:boomdrivedev/boomdrive/v$version

# The real programs the install and the stubs run, each linked into one
# folder, so no other program of the host, nix among them, is on the
# PATH a case gives.
tools=$scratch/tools
mkdir -p "$tools"
for tool in sh cat cp mkdir rm mktemp awk sed grep readlink chmod printf tr env shasum sha256sum; do
  found=$(command -v "$tool" 2>/dev/null) || continue
  case $found in
  /*) ln -s "$found" "$tools/$tool" ;;
  esac
done

# The stub installer the stub curl gives, and a copy of install.sh that
# pins its digest in place of Nix's, so a case can run past the check.
installer=$scratch/nix-installer
cat >"$installer" <<'EOF'
#!/bin/sh
echo "installer $*" >>"$STUB_LOG"
while [ $# -gt 0 ]; do
  [ "$1" = --nix-extra-conf-file ] && cat "$2" >"$STUB_LOG.nix-conf"
  shift
done
cp "$STUB_NIX" "$STUB_BIN/nix"
EOF
if command -v shasum >/dev/null 2>&1; then
  stub_digest=$(shasum -a 256 "$installer" | awk '{ print $1 }')
else
  stub_digest=$(sha256sum "$installer" | awk '{ print $1 }')
fi
pinned=$scratch/install-pinned.sh
sed "s/^nix_installer_sha256=.*/nix_installer_sha256=$stub_digest/" "$tree/install.sh" >"$pinned"

# The stubs. STUB_* are the case's, which install.sh reads none of.
stubs=$scratch/stubs
mkdir -p "$stubs"
cat >"$stubs/uname" <<'EOF'
#!/bin/sh
case $1 in
-s) echo "$STUB_SYSTEM" ;;
-m) echo "$STUB_MACHINE" ;;
esac
EOF
cat >"$stubs/curl" <<'EOF'
#!/bin/sh
echo "curl $*" >>"$STUB_LOG"
while [ $# -gt 0 ]; do
  [ "$1" = -o ] && cp "$STUB_INSTALLER" "$2"
  shift
done
EOF
cat >"$stubs/sudo" <<'EOF'
#!/bin/sh
echo "sudo $*" >>"$STUB_LOG"
if [ "$1" = tee ]; then
  cat >>"$STUB_LOG.sudo-tee"
  exit "${STUB_TEE_EXIT:-0}"
fi
EOF
cat >"$stubs/nix" <<'EOF'
#!/bin/sh
echo "nix $*" >>"$STUB_LOG"
case "$*" in
*"config show substituters") echo "$STUB_SUBSTITUTERS" ;;
*"config show trusted-public-keys") echo "$STUB_KEYS" ;;
*"profile add"*)
  mkdir -p "$HOME/.nix-profile/bin"
  cp "$STUB_BOOMDRIVE" "$HOME/.nix-profile/bin/boomdrive"
  ;;
esac
EOF
cat >"$stubs/boomdrive" <<'EOF'
#!/bin/sh
if [ "$1" = --version ]; then
  echo "boomdrive $STUB_VERSION"
  exit 0
fi
echo "boomdrive $*" >>"$STUB_LOG"
cat >"$STUB_LOG.stdin"
EOF
cat >"$stubs/determinate-nixd" <<'EOF'
#!/bin/sh
exit 0
EOF
chmod +x "$stubs"/* "$installer"

shells="sh"
[ -n "${BOOMDRIVE_MAC_SH:-}" ] && shells="sh mac"

# Runs the install `$script` under the shell `$shell` for the case `$1`,
# with the stubs the case names in $with and the flags after it, the
# token on its standard input. Sets $code, $said, $log and $home.
run() {
  case_dir=$scratch/$shell/$1
  shift
  home=$case_dir/home
  bin=$case_dir/bin
  log=$case_dir/log
  mkdir -p "$home" "$bin"
  : >"$log"
  for stub in uname curl sudo $with; do
    cp "$stubs/$stub" "$bin/$stub"
  done
  if [ "$shell" = mac ]; then
    runner="$BOOMDRIVE_MAC_SH --posix"
  else
    runner="sh"
  fi
  code=0
  # shellcheck disable=SC2086 # $runner is a shell and its flag.
  said=$(printf '%s\n' "$token" | env -i HOME="$home" PATH="$bin:$tools" TMPDIR="$case_dir" \
    STUB_LOG="$log" STUB_BIN="$bin" STUB_NIX="$stubs/nix" STUB_BOOMDRIVE="$stubs/boomdrive" \
    STUB_INSTALLER="$installer" STUB_SYSTEM="${system:-Darwin}" STUB_MACHINE="${machine:-arm64}" \
    STUB_SUBSTITUTERS="${substituters:-}" STUB_KEYS="${keys:-}" STUB_TEE_EXIT="${tee_exit:-0}" \
    STUB_VERSION="${installed_version:-$version}" \
    $runner "$script" "$@" 2>&1) || code=$?
}

# Whether the log holds a line equal to $1.
logged() {
  grep -qxF -- "$1" "$log"
}

enroll="--control-plane https://plane.example --drive home --repository owner/one --repository owner/two"
cache=https://boomdrive.cachix.org
key=boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=
add="nix --extra-experimental-features nix-command flakes profile add --inputs-from $flake $flake#boomdrive $flake#flume $flake#sluice $flake#herdr $flake#herdr-plugins nixpkgs#git"

# Whether the token reached boomdrive enroll on its standard input alone,
# with the enroll command's flags, and is in no line of the log.
enrolled() {
  logged "boomdrive enroll $enroll" &&
    [ "$(cat "$log.stdin" 2>/dev/null)" = "$token" ] &&
    ! grep -qF "$token" "$log"
}

for shell in $shells; do
  script=$tree/install.sh
  system='' machine='' substituters='' keys='' tee_exit='' installed_version=''

  # Scenario: A flag that is not the install's.
  with="nix"
  # shellcheck disable=SC2086
  run unknown --yes --token "$token"
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    pass "$shell: a flag that is not the install's exits 2 and runs nothing"
  else
    fail "$shell: a flag that is not the install's exits $code, or runs something: $said"
  fi

  # Scenario: Part of the enroll command.
  run part --yes --drive home
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *--control-plane*--repository*) pass "$shell: part of the enroll command exits 2, names the flags it takes and runs nothing" ;;
    *) fail "$shell: part of the enroll command exits 2 without naming the flags: $said" ;;
    esac
  else
    fail "$shell: part of the enroll command exits $code, or runs something: $said"
  fi

  # Scenario: Linux.
  system=Linux machine=aarch64
  # shellcheck disable=SC2086
  run linux --yes $enroll
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *services.boomdrive*"token is not spent"*) pass "$shell: Linux exits 2, naming services.boomdrive and that the token is not spent, and installs nothing" ;;
    *) fail "$shell: Linux exits 2 without naming services.boomdrive and the token: $said" ;;
    esac
  else
    fail "$shell: Linux exits $code, or installs something: $said"
  fi

  # Scenario: A Mac that is not Apple silicon.
  system=Darwin machine=x86_64
  # shellcheck disable=SC2086
  run intel --yes $enroll
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *"Apple silicon"*) pass "$shell: an Intel Mac exits 2, naming Apple silicon, and installs nothing" ;;
    *) fail "$shell: an Intel Mac exits 2 without naming Apple silicon: $said" ;;
    esac
  else
    fail "$shell: an Intel Mac exits $code, or installs something: $said"
  fi
  system='' machine=''

  # Scenario: Nix sets up the runner.
  with="nix"
  mkdir -p "$scratch/$shell/nix-runner/home/.config/boomdrive"
  ln -s /nix/store/0000000000000000000000000000000-boomdrive-config.toml \
    "$scratch/$shell/nix-runner/home/.config/boomdrive/config.toml"
  # shellcheck disable=SC2086
  run nix-runner --yes $enroll
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *"Nix store"*"boomdrive enroll $enroll"*"token is not spent"*) pass "$shell: a runner's file that links into the store exits 2, names the enroll command alone and installs nothing" ;;
    *) fail "$shell: a runner's file that links into the store exits 2 without naming the enroll command alone: $said" ;;
    esac
  else
    fail "$shell: a runner's file that links into the store exits $code, or installs something: $said"
  fi

  # Scenario: Another runner on the PATH.
  with="nix boomdrive"
  installed_version=0.1.0
  # shellcheck disable=SC2086
  run other-runner --yes $enroll
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *"boomdrive 0.1.0"*"$version"*) pass "$shell: another runner on the PATH exits 2, naming both versions, and installs nothing" ;;
    *) fail "$shell: another runner on the PATH exits 2 without naming both versions: $said" ;;
    esac
  else
    fail "$shell: another runner on the PATH exits $code, or installs something: $said"
  fi
  installed_version=

  # Scenario: This release's runner on the PATH.
  # shellcheck disable=SC2086
  run this-runner $enroll
  if [ "$code" -eq 0 ] && enrolled && ! grep -q '^nix ' "$log"; then
    pass "$shell: this release's runner on the PATH adds nothing and enrolls with the token on its standard input"
  else
    fail "$shell: this release's runner on the PATH exits $code, adds something or does not enroll: $said"
  fi

  # Scenario: No Nix, and no answer.
  with=""
  # shellcheck disable=SC2086
  run no-answer $enroll
  if [ "$code" -eq 2 ] && [ ! -s "$log" ]; then
    case $said in
    *"Nix 2."*"$cache"*"--yes"*"token is not spent"*) pass "$shell: no Nix and no --yes off a terminal names Nix, the cache and --yes, exits 2 and fetches nothing" ;;
    *) fail "$shell: no Nix and no --yes exits 2 without naming Nix, the cache and --yes: $said" ;;
    esac
  else
    fail "$shell: no Nix and no --yes exits $code, or fetches something: $said"
  fi

  # Scenario: Nix's installer whose digest is not the pinned one.
  # shellcheck disable=SC2086
  run digest --yes $enroll
  if [ "$code" -eq 1 ] && ! grep -q '^installer ' "$log" && ! grep -q '^boomdrive ' "$log"; then
    case $said in
    *"$stub_digest"*"is not run"*) pass "$shell: an installer whose digest is not the pinned one exits 1, naming its digest, and is not run" ;;
    *) fail "$shell: an installer whose digest is not the pinned one exits 1 without naming its digest: $said" ;;
    esac
  else
    fail "$shell: an installer whose digest is not the pinned one exits $code, or is run: $said"
  fi

  # Scenario: No Nix, and --yes.
  script=$pinned
  # shellcheck disable=SC2086
  run no-nix --yes $enroll
  conf=$(cat "$log.nix-conf" 2>/dev/null)
  want_conf="experimental-features = nix-command flakes
extra-substituters = $cache
extra-trusted-public-keys = $key"
  if [ "$code" -eq 0 ] &&
    grep -q '^curl .*https://releases\.nixos\.org/nix/nix-[0-9.]*/install$' "$log" &&
    grep -q '^installer --daemon --yes --nix-extra-conf-file ' "$log" &&
    [ "$conf" = "$want_conf" ] && logged "$add" && enrolled; then
    pass "$shell: no Nix and --yes installs Nix with the cache in its configuration, adds the set at v$version and enrolls"
  else
    fail "$shell: no Nix and --yes exits $code, or does not install Nix with the cache, add the set and enroll: $said
$(cat "$log")"
  fi
  script=$tree/install.sh

  # Scenario: Nix that uses the cache.
  with="nix"
  substituters="https://cache.nixos.org/ $cache"
  keys="cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= $key"
  # shellcheck disable=SC2086
  run uses-cache $enroll
  if [ "$code" -eq 0 ] && ! grep -Eq '^(sudo|curl) ' "$log" && logged "$add" && enrolled; then
    pass "$shell: Nix that uses the cache asks nothing, adds the set and enrolls"
  else
    fail "$shell: Nix that uses the cache exits $code, or runs sudo or curl, or does not add the set and enroll: $said"
  fi

  # Scenario: Nix that does not use the cache, and no answer.
  substituters="https://cache.nixos.org/"
  keys="cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
  # shellcheck disable=SC2086
  run no-cache $enroll
  if [ "$code" -eq 2 ] && ! grep -Eq '^sudo |profile add' "$log"; then
    case $said in
    *"/etc/nix/nix.conf"*"extra-substituters = $cache"*"extra-trusted-public-keys = $key"*"--yes"*) pass "$shell: Nix that does not use the cache, with no --yes, names the file and its lines, exits 2 and changes nothing" ;;
    *) fail "$shell: Nix that does not use the cache, with no --yes, exits 2 without naming the file and its lines: $said" ;;
    esac
  else
    fail "$shell: Nix that does not use the cache, with no --yes, exits $code, or changes something: $said"
  fi

  # Scenario: Nix that does not use the cache, and --yes.
  # shellcheck disable=SC2086
  run add-cache --yes $enroll
  if [ "$code" -eq 0 ] && logged "sudo tee -a /etc/nix/nix.conf" &&
    [ "$(cat "$log.sudo-tee")" = "extra-substituters = $cache
extra-trusted-public-keys = $key" ] &&
    logged "sudo launchctl kickstart -k system/org.nixos.nix-daemon" &&
    logged "$add" && enrolled; then
    pass "$shell: Nix that does not use the cache, with --yes, adds its lines to /etc/nix/nix.conf, restarts the daemon, adds the set and enrolls"
  else
    fail "$shell: Nix that does not use the cache, with --yes, exits $code, or does not add the lines, restart the daemon, add the set and enroll: $said"
  fi

  # Scenario: Determinate's Nix.
  with="nix determinate-nixd"
  # shellcheck disable=SC2086
  run determinate --yes $enroll
  if [ "$code" -eq 0 ] && logged "sudo tee -a /etc/nix/nix.custom.conf" &&
    logged "sudo launchctl kickstart -k system/systems.determinate.nix-daemon" && enrolled; then
    pass "$shell: Determinate's Nix takes the lines in /etc/nix/nix.custom.conf and restarts its own daemon"
  else
    fail "$shell: Determinate's Nix exits $code, or does not take the lines in nix.custom.conf and restart its daemon: $said"
  fi

  # Scenario: A Nix configuration the install cannot write.
  with="nix"
  tee_exit=1
  # shellcheck disable=SC2086
  run read-only --yes $enroll
  if [ "$code" -eq 2 ] && ! grep -Eq 'launchctl|profile add' "$log" && ! grep -q '^boomdrive ' "$log"; then
    pass "$shell: a Nix configuration the install cannot write exits 2 and adds nothing"
  else
    fail "$shell: a Nix configuration the install cannot write exits $code, or adds something: $said"
  fi
  tee_exit=

  # Scenario: No enroll command.
  substituters="$cache" keys="$key"
  run install-only --yes
  if [ "$code" -eq 0 ] && logged "$add" && ! grep -q '^boomdrive ' "$log"; then
    pass "$shell: no enroll command adds the set and enrolls nothing"
  else
    fail "$shell: no enroll command exits $code, or does not add the set, or enrolls: $said"
  fi
  substituters='' keys=''
done

exit "$status"
