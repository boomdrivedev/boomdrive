# The ref a flake URL `github:<owner>/<repo>/<ref>` names, or null where
# it names none: for a tool of the set, its release tag. The worktrunk
# package reads its tag from flake.nix's URL here, so one string names it,
# and nix:typecheck holds each to the lock's (core.45).
{ lib }:
url:
let
  parts = lib.splitString "/" (lib.removePrefix "github:" url);
in
if lib.hasPrefix "github:" url && builtins.length parts == 3 then builtins.elemAt parts 2 else null
