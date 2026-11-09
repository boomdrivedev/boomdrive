# Flume's git crates at the tag flake.nix names, as git-crates.nix holds
# the runner's: each git package of Flume's Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash.
# Flume's tag moves with these, in one change: nix:test fails where an
# entry's revision is not the tag's Cargo.lock's. It goes with flume.nix,
# once Flume publishes a flake.
{
  "weir-capture-0.0.0" = {
    rev = "daacda39e59469029b2841f889c6cbd8cd7839d9";
    hash = "sha256-YJKuoW3lWl5wtEiu99oHlQF+dcdlJu1tdb/aeEvxHeM=";
  };
  "weir-kinds-0.0.0" = {
    rev = "daacda39e59469029b2841f889c6cbd8cd7839d9";
    hash = "sha256-YJKuoW3lWl5wtEiu99oHlQF+dcdlJu1tdb/aeEvxHeM=";
  };
}
