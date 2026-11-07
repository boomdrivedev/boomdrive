{ pkgs, ... }:
let
  # treefmt's rustfmt: rustup's proxy, which runs the toolchain
  # rust-toolchain.toml names, the version .prototools pins, and installs
  # its rustfmt where a host's toolchain lacks it, so treefmt and the
  # crate's format task run one rustfmt (core.7, core.8).
  rustfmt = pkgs.writeShellApplication {
    name = "rustfmt";
    runtimeInputs = [ pkgs.rustup ];
    text = ''exec rustfmt "$@"'';
  };
in
{
  # A tracked file no formatter claims and no exclude names fails the check
  # (core.7): a new file type is decided in the change that adds it.
  settings.on-unmatched = "fatal";

  # What no formatter owns, each with its reason (core.7).
  settings.excludes = [
    # Prose: the docs, the records, the README, and OpenSpec's Claude Code
    # skills and commands as the pinned openspec writes them (record
    # 0022); no formatter here rewraps prose.
    "*.md"
    # Written by cargo and devenv in their own layouts.
    "*.lock"
    # Lists in gitignore syntax.
    ".gitignore"
    # Empty markers that keep OpenSpec's empty directories in the tree.
    "**/.gitkeep"
    # A tool's own answers, as it printed them: the tests hold the runner
    # to carry each byte as written, so no formatter may change one
    # (design Decision 21). jsonfmt owns every other JSON file, the tree's
    # own.
    "crates/*/tests/fixtures/**"
    # The files the flake's modules render, as the renderer wrote them:
    # nix:test holds the renderer to each byte, so no formatter may change
    # one (design Decision 9).
    "nix/tests/files/**"
    # Build outputs and caches.
    "target/**"
    ".devenv/**"
    ".moon/cache/**"
  ];

  programs.rustfmt.enable = true;
  programs.rustfmt.edition = "2021";
  programs.rustfmt.package = rustfmt;

  programs.deadnix.enable = true;
  programs.nixfmt.enable = true;
  settings.formatter.deadnix.pipeline = "nix";
  settings.formatter.deadnix.priority = 1;
  settings.formatter.nixfmt.pipeline = "nix";
  settings.formatter.nixfmt.priority = 2;

  # shellcheck lints the shell scripts, then shfmt lays them out.
  programs.shellcheck.enable = true;
  programs.shfmt.enable = true;
  programs.shfmt.indent_size = 2;
  settings.formatter.shellcheck.pipeline = "shell";
  settings.formatter.shellcheck.priority = 1;
  settings.formatter.shfmt.pipeline = "shell";
  settings.formatter.shfmt.priority = 2;

  # .prototools is TOML under a name with no extension.
  programs.taplo.enable = true;
  settings.formatter.taplo.includes = [ ".prototools" ];

  # The tree's own JSON, the protocol's schema and its examples among it,
  # written by hand and read by the public. The tests compare JSON as
  # values, never as bytes, so its layout changes nothing they hold.
  programs.jsonfmt.enable = true;

  # .versionrc, convco's, is YAML under a name with no extension.
  programs.yamlfmt.enable = true;
  settings.formatter.yamlfmt.includes = [ ".versionrc" ];
  programs.yamlfmt.settings.formatter = {
    type = "basic";
    retain_line_breaks_single = true;
  };
}
