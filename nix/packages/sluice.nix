# sluice, sluice's own flake's package (design Decision 3). sluice's
# release build is given its tag's version as SLUICE_VERSION, and its
# tagged tree says 0.0.0; so where the tree says 0.0.0 the package is
# given the tag flake.nix names, without its v, as that build is given
# it. Where the tree carries a version of its own, the package is
# sluice's unchanged.
{
  lib,
  package,
  src,
  tag,
}:
let
  inherit ((builtins.fromTOML (builtins.readFile "${src}/Cargo.toml")).workspace.package) version;
  released = lib.removePrefix "v" tag;
in
if version != "0.0.0" then
  package
else
  package.overrideAttrs (old: {
    # The source is the tag's already: only its version is given.
    __intentionallyOverridingVersion = true;
    version = released;
    env = (old.env or { }) // {
      SLUICE_VERSION = released;
    };
  })
