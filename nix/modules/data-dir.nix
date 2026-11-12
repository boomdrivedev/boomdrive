# The runner's data folder from a home and an XDG_DATA_HOME, by the
# runner's own rule, `data_dir` and `xdg` in crates/boomdrive/src/config.rs
# (the change the-set-names-the-runners-data-folder, design Decision 2):
# `$XDG_DATA_HOME/boomdrive` where `dataHome` is absolute, else
# `~/.local/share/boomdrive` under `home`, an empty or relative `dataHome`
# taken as unset. Each join is Rust's `PathBuf::push`: a `/` added only
# where the base does not already end in one. nix/tests/data-dirs.json is
# the table that holds this rule and the runner's together: the flake's
# tests and the runner's both read it. darwin.nix and nixos.nix take it
# as `dataDirOf`, each with its job's `XDG_DATA_HOME` or null.
{ lib }:
{ home, dataHome }:
let
  join = base: name: if lib.hasSuffix "/" base then "${base}${name}" else "${base}/${name}";
  given = if dataHome == null then "" else toString dataHome;
  base =
    if lib.hasPrefix "/" given then
      given
    else if home == null then
      throw "the runner's data folder is under its user's home, and no home is given"
    else
      join (toString home) ".local/share";
in
join base "boomdrive"
