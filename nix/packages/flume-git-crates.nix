# Flume's git crates at the tag flake.nix names, as git-crates.nix holds
# the runner's: each git package of Flume's Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash.
# Flume's tag moves with these, in one change: nix:test fails where an
# entry's revision is not the tag's Cargo.lock's. It goes with flume.nix,
# once Flume publishes a flake.
{
  "weir-capture-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-delivery-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-kinds-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-pack-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
}
