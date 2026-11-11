# Each input's ref in flake.nix against the one flake.lock locks for it,
# and each tool of the set named by a release tag, Flume, sluice, crew,
# herdr and worktrunk (core.45; Task notes 2.5). Each failure names the
# input and both refs.
{ lib }:
inputs: lock:
let
  tagOf = import ../packages/tag.nix { inherit lib; };
  roots = lock.nodes.${lock.root}.inputs;
  show = ref: if ref == null then "no ref" else ref;

  ofInput =
    name: input:
    let
      named = tagOf (input.url or "");
      locked = lock.nodes.${roots.${name}}.original.ref or null;
    in
    if !(roots ? ${name}) then
      [ "${name}: flake.nix names it, and flake.lock locks no such input" ]
    else
      lib.optional (
        named != locked
      ) "${name}: flake.nix names ${show named}, and flake.lock locks ${show locked}"
      ++ lib.optional (
        builtins.elem name [
          "crew"
          "flume"
          "herdr"
          "portless"
          "sluice"
          "worktrunk"
        ]
        && builtins.match "v[0-9]+\\.[0-9]+\\.[0-9]+" (show named) == null
      ) "${name}: flake.nix names it by ${show named}, which is no release tag";
in
lib.concatLists (lib.mapAttrsToList ofInput inputs)
