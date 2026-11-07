# Flume's git crates at the tag flake.nix names, as git-crates.nix holds
# the runner's: each git package of Flume's Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash.
# Flume's tag moves with these, in one change: nix:test fails where an
# entry's revision is not the tag's Cargo.lock's. It goes with flume.nix,
# once Flume publishes a flake.
{
  "weir-capture-0.0.0" = {
    rev = "5562af52cb7e065e7de7109135dfd7eecb1d61c5";
    hash = "sha256-2DFTZKoYQiYKggsaZTqWCWtcJpux7x9prTEaomLXk9k=";
  };
  "weir-kinds-0.0.0" = {
    rev = "5562af52cb7e065e7de7109135dfd7eecb1d61c5";
    hash = "sha256-2DFTZKoYQiYKggsaZTqWCWtcJpux7x9prTEaomLXk9k=";
  };
}
