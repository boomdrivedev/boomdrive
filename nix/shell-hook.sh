# shellcheck shell=bash
# The flake's shell's hook (record 0023; design Decision 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents),
# sourced whole by the bash `nix develop` starts: in an interactive shell
# outside herdr, it starts herdr, herdr's own start with no argument, and
# when herdr ends, by its exit or a detach, the person is in the shell with
# the set on PATH. A command run through the shell, `nix develop
# --command` or direnv's `use flake`, runs in a shell that is not
# interactive, and a shell entered from a herdr pane has HERDR_ENV set:
# neither starts herdr. Sourced, it uses no exit and sets nothing.
case $- in
*i*) [ -n "${HERDR_ENV:-}" ] || herdr ;;
esac
