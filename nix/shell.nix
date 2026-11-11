# The flake's shell, devShells.<system>.default (record 0023; design
# Decision 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents): the
# set's runner, Flume, sluice, crew and herdr on PATH, made with the set's
# own nixpkgs, and a hook that starts herdr on entering; and worktrunk's
# `wt`, which crew and sluice call (design Decision 4 of the change
# worktrunk-comes-with-the-flake). It links no plugin, which the modules
# do, and writes no file. Ghostty and the launcher are
# not in it, since a person in the shell is in a terminal already. The
# repository's own development shell is devenv's (devenv.nix).
{ pkgs, set }:
pkgs.mkShellNoCC {
  packages = [
    set.boomdrive
    set.flume
    set.sluice
    set.crew
    set.herdr
    set.worktrunk
  ];
  shellHook = builtins.readFile ./shell-hook.sh;
}
