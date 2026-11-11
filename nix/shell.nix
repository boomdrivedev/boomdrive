# The flake's shell, devShells.<system>.default (record 0023; design
# Decision 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents): the
# set's runner, Flume, sluice, crew, herdr, portless and worktrunk on PATH,
# made with the set's own nixpkgs, and a hook that starts herdr on
# entering. It links no plugin, which the modules do, starts no proxy of
# portless's, and writes no file. portless is in it because crew's board
# reads its routes (design Decision 7 of the change
# portless-comes-with-the-flake), and worktrunk's `wt` because crew and
# sluice call it (design Decision 4 of the change
# worktrunk-comes-with-the-flake). Ghostty and the launcher are not in it,
# since a person in the shell is in a terminal already. The repository's
# own development shell is devenv's (devenv.nix).
{ pkgs, set }:
pkgs.mkShellNoCC {
  packages = [
    set.boomdrive
    set.flume
    set.sluice
    set.crew
    set.herdr
    set.portless
    set.worktrunk
  ];
  shellHook = builtins.readFile ./shell-hook.sh;
}
