# millpond's git crates at the set's tag of millpond: each git package of
# its Cargo.lock, by `<name>-<version>`, to the revision its hash was
# taken at and the hash of fetchgit's tree there, as git-crates.nix holds
# the runner's (record 0033). millpond's own flake fetches them at
# evaluation, which the gate's evaluation against no store cannot, so the
# set fetches them by these hashes (millpond.nix). A revision moves with
# its hash and millpond's tag, in one change: nix:test fails where an
# entry's revision is not millpond's Cargo.lock's.
let
  weir = {
    rev = "63c995912c82491b8dc8438c34ff834d7d39f2db";
    hash = "sha256-FO7nooBjjPztBvMzv35pqfownjTsEeN4oo8+xhr6Y+U=";
  };
in
{
  "weir-bounded-0.0.0" = weir;
  "weir-branch-0.0.0" = weir;
  "weir-cadence-0.0.0" = weir;
  "weir-capture-0.0.0" = weir;
  "weir-delivery-0.0.0" = weir;
  "weir-engine-0.0.0" = weir;
  "weir-kinds-0.0.0" = weir;
  "weir-pack-0.0.0" = weir;
  "weir-policy-0.0.0" = weir;
  "weir-signals-0.0.0" = weir;
}
