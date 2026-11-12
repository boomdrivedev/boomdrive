# The runner's view of one address from a home and a repository root, by
# the runner's own rule, `view_path` and `root_path` in
# crates/boomdrive/src/config.rs (the change
# the-set-gives-crew-each-kept-repositorys-folders, design Decision 3):
# `<repository_root>/<owner>/<name>`, the root `Code` under `home` where
# `root` is null, a root beginning with `~/` joined under `home`, and an
# absolute root as written. Each join is Rust's `PathBuf::push`: a name
# that is absolute replaces the base, and a `/` is added only where the
# base does not already end in one. The address's one `/` sits between its
# owner and its name, which hold none, so one join of it is the runner's
# two. A root the runner refuses, neither absolute nor beginning with
# `~/`, throws naming it. nix/tests/view-dirs.json is the table that holds
# this rule and the runner's together: the flake's tests and the runner's
# both read it. kept.nix takes it as `viewDirOf`.
{ lib }:
{
  home,
  root,
  address,
}:
let
  join =
    base: name:
    if lib.hasPrefix "/" name then
      name
    else if lib.hasSuffix "/" base then
      "${base}${name}"
    else
      "${base}/${name}";
  homeOf =
    if home == null then
      throw "the runner's view is under its user's home, and no home is given"
    else
      toString home;
  given = toString root;
  base =
    if root == null then
      join homeOf "Code"
    else if lib.hasPrefix "~/" given then
      join homeOf (lib.removePrefix "~/" given)
    else if lib.hasPrefix "/" given then
      given
    else
      throw "services.boomdrive.settings.repository_root is \"${given}\": the runner takes a repository root that is an absolute path or begins with ~/, and refuses this one";
in
join base address
