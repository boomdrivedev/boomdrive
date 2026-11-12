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
# `refuse` names, saying `manifest not found`. The signing's cases add
# stubs of security, codesign and openssl (design Decision 7 of the change
# a-new-build-keeps-the-runners-macos-access), described at freshSign, and
# a keychain that is a word they take, never a keychain. It prints each
# failure as one line, nix: <case>: <what>, and nothing where every case
# holds.
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
# configuration in `openssl.cnf`.
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
  chmod +x "$dir/bin/security" "$dir/bin/codesign" "$dir/bin/openssl"
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

# The stubs' calls of `$1`, on one line each.
calledOf() {
  grep "^$1 " "$calls" 2>/dev/null || true
}
# The first line of `$1`.
first() {
  printf '%s\n' "$1" | sed -n 1p
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
