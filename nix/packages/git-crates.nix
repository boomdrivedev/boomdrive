# The runner's git crates: each git package of Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash
# of fetchgit's tree there, which the runner's package fetches it by
# (design Decision 3). A revision moves with its hash, in one change:
# nix:test fails where an entry's revision is not Cargo.lock's. A hash is
# taken by building `fetchgit { url; rev; sha256 = lib.fakeHash; }` and
# reading the hash nix got.
{
  "weir-bounded-0.0.0" = {
    rev = "9cba69196e9269a36b02024861340355bdbe6d4b";
    hash = "sha256-BS0qHjt+4qQMUnbOvUiQjgs33CdG91twIAOUAsUjQpc=";
  };
  "weir-branch-0.0.0" = {
    rev = "9cba69196e9269a36b02024861340355bdbe6d4b";
    hash = "sha256-BS0qHjt+4qQMUnbOvUiQjgs33CdG91twIAOUAsUjQpc=";
  };
  "weir-capture-0.0.0" = {
    rev = "9cba69196e9269a36b02024861340355bdbe6d4b";
    hash = "sha256-BS0qHjt+4qQMUnbOvUiQjgs33CdG91twIAOUAsUjQpc=";
  };
  "weir-signals-0.0.0" = {
    rev = "9cba69196e9269a36b02024861340355bdbe6d4b";
    hash = "sha256-BS0qHjt+4qQMUnbOvUiQjgs33CdG91twIAOUAsUjQpc=";
  };
}
