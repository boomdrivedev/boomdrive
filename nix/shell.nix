# The flake's shell, devShells.<system>.default (record 0023; design
# Decision 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents): the
# set's runner, Flume, sluice, crew, herdr, portless and crew's companions
# on PATH, made with the set's own nixpkgs, and a hook that starts herdr
# on entering. It links no plugin, which the modules do, starts no proxy of
# portless's, and writes no file. portless is in it because crew's board
# reads its routes (design Decision 7 of the change
# portless-comes-with-the-flake), and crew's companions, Pending You,
# Plannotator and plannotator-tui, because crew's sessions run them (design
# Decision 10 of the change crews-companions-come-with-the-flake); Pending
# You's folder is the modules' to write. Ghostty and the launcher are not
# in it, since a person in the shell is in a terminal already. The
# repository's own development shell is devenv's (devenv.nix).
{ pkgs, set }:
pkgs.mkShellNoCC {
  packages = [
    set.boomdrive
    set.flume
    set.sluice
    set.crew
    set.herdr
    set.portless
    set.pendingyou
    set.plannotator
    set.plannotator-tui
  ];
  shellHook = builtins.readFile ./shell-hook.sh;
}
