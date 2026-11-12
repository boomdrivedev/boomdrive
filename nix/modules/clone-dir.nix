# The runner's clone folder of one address from its data folder, by the
# runner's own rule, `container` in crates/boomdrive/src/clone.rs (the
# change the-set-states-the-runners-folders, design Decision 2):
# `<dataDir>/repositories/<owner>/<name>`, the folder `flume launch
# --repository` names. The address is `<owner>/<name>` as written, its one
# `/` between the owner and the name, so the folder is `repositories`, the
# owner and the name under `dataDir`, as `container` joins them. `dataDir`
# is data-dir.nix's, which ends in `boomdrive` and never in `/`, so one `/`
# before `repositories` is the runner's own join. nix/tests/clone-dirs.json
# is the table that holds this rule and the runner's together: the flake's
# tests and the runner's both read it. clones.nix takes it as `cloneDirOf`.
{ dataDir, address }:
"${toString dataDir}/repositories/${address}"
