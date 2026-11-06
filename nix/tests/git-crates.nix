# A Cargo.lock's git packages against the file of hashes the package that
# builds it holds for them (Task notes 2.1): each git package has an
# entry, each entry is a git package, and each entry's revision is the one
# Cargo.lock names. Each failure names the package and both revisions.
{ lib }:
file: cargoLock: crates:
let
  git = builtins.filter (package: lib.hasPrefix "git+" (package.source or "")) cargoLock.package;
  locked = builtins.listToAttrs (
    map (
      package:
      lib.nameValuePair "${package.name}-${package.version}" (
        lib.last (lib.splitString "#" package.source)
      )
    ) git
  );

  ofLocked =
    crate: rev:
    if !(crates ? ${crate}) then
      [ "${crate}: Cargo.lock names it at ${rev}, and ${file} holds no hash for it" ]
    else
      lib.optional (
        crates.${crate}.rev != rev
      ) "${crate}: Cargo.lock names ${rev}, and ${file}'s hash is of ${crates.${crate}.rev}";
  ofEntry =
    crate:
    lib.optional (
      !(locked ? ${crate})
    ) "${crate}: ${file} holds a hash for it, and Cargo.lock names no such git package";
in
lib.concatLists (lib.mapAttrsToList ofLocked locked)
++ lib.concatMap ofEntry (builtins.attrNames crates)
