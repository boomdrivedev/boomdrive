# Flume's git crates at the tag flake.nix names, as git-crates.nix holds
# the runner's: each git package of Flume's Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash.
# Flume's tag moves with these, in one change: nix:test fails where an
# entry's revision is not the tag's Cargo.lock's. It goes with flume.nix,
# once Flume publishes a flake.
{
  "weir-kinds-0.0.0" = {
    rev = "0b6eb5abb806a51fec128b01f97c4dcff6170219";
    hash = "sha256-1PWzNpKz0dUIDpscpC84/kGMkLAmFwhhZTwve4mkVps=";
  };
}
