# The runner's git crates: each git package of Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash
# of fetchgit's tree there, which the runner's package fetches it by
# (design Decision 3). A revision moves with its hash, in one change:
# nix:test fails where an entry's revision is not Cargo.lock's. A hash is
# taken by building `fetchgit { url; rev; sha256 = lib.fakeHash; }` and
# reading the hash nix got.
{
  "weir-branch-0.0.0" = {
    rev = "0b6eb5abb806a51fec128b01f97c4dcff6170219";
    hash = "sha256-1PWzNpKz0dUIDpscpC84/kGMkLAmFwhhZTwve4mkVps=";
  };
  "weir-capture-0.0.0" = {
    rev = "0b6eb5abb806a51fec128b01f97c4dcff6170219";
    hash = "sha256-1PWzNpKz0dUIDpscpC84/kGMkLAmFwhhZTwve4mkVps=";
  };
  "weir-signals-0.0.0" = {
    rev = "0b6eb5abb806a51fec128b01f97c4dcff6170219";
    hash = "sha256-1PWzNpKz0dUIDpscpC84/kGMkLAmFwhhZTwve4mkVps=";
  };
}
