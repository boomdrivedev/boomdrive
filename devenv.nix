{
  pkgs,
  lib,
  ...
}:

let
  # proto from its own release build: nixpkgs trails it by minor versions or
  # more. Linux takes the static musl build, so no loader needs patching.
  proto =
    let
      version = "0.63.1";
      system = pkgs.stdenv.hostPlatform.system;
      target =
        {
          aarch64-darwin = "aarch64-apple-darwin";
          x86_64-darwin = "x86_64-apple-darwin";
          aarch64-linux = "aarch64-unknown-linux-musl";
          x86_64-linux = "x86_64-unknown-linux-musl";
        }
        .${system} or (throw "proto ${version}: no release build for ${system}");
      hashes = {
        aarch64-apple-darwin = "8bfde4c862381eeb6e82d0858bb6215732cbd1e384e4eae45da8fa08095e0ffd";
        x86_64-apple-darwin = "aaaff78c7125de1b765f00177f35da75ee16fb1363e07632ace27213ff5d893b";
        aarch64-unknown-linux-musl = "111f5b6a38d3979ebafbc095c8d91ef69c99d1f018addb3582179d4ab91b3441";
        x86_64-unknown-linux-musl = "8596b98ccd833d1203e71931ba25e866d3239f9d3859c922a035a7b830f29d17";
      };
    in
    pkgs.stdenvNoCC.mkDerivation {
      pname = "proto";
      inherit version;
      src = pkgs.fetchurl {
        url = "https://github.com/moonrepo/proto/releases/download/v${version}/proto_cli-${target}.tar.xz";
        sha256 = hashes.${target};
      };
      sourceRoot = "proto_cli-${target}";
      installPhase = ''
        runHook preInstall
        install -Dm755 -t $out/bin proto proto-shim
        runHook postInstall
      '';
    };

  # The application toolchain .prototools pins (moon and Rust), on PATH.
  # proto's home is this repository's, in the git common dir so every
  # worktree shares it, and only this repository's pins are read.
  protoHome = ''
    PROTO_HOME="$(${lib.getExe pkgs.git} rev-parse --path-format=absolute --git-common-dir)/proto"
    export PROTO_HOME
  '';
  activate = ''
    ${protoHome}
    eval "$(${proto}/bin/proto activate bash --export --no-bin --config-mode upwards 2>/dev/null)"
  '';

  # git, which flywheel:test runs (design 11): nixpkgs' build, pinned by
  # devenv.lock and asserted to be this version where it goes on PATH. Bump
  # the version with the lock.
  gitVersion = "2.55.0";

  # OpenSpec, which root:specs and root:changes validate with: flume's
  # release, built as nixpkgs builds its older one. Bump the version, then
  # each hash from the mismatch the build reports.
  openspecVersion = "1.14.3";
  openspec = pkgs.openspec.overrideAttrs (
    finalAttrs: _: {
      version = openspecVersion;
      src = pkgs.fetchFromGitHub {
        owner = "Fission-AI";
        repo = "OpenSpec";
        tag = "v${finalAttrs.version}";
        hash = "sha256-tdTX0nkDe/aNZgLckh6/B6BtoSb40w0Y5qzg+d1LjHU=";
      };
      pnpmDeps = pkgs.fetchPnpmDeps {
        inherit (finalAttrs) pname version src;
        pnpm = pkgs.pnpm_10;
        fetcherVersion = 3;
        hash = "sha256-fKm4sBdTDeQfIiE0k301jtRbiuJrBE7YIEBu8RZxuXI=";
      };
    }
  );

in
{
  # The system tools are this file's, pinned by devenv.lock; moon and Rust
  # are .prototools', installed by the flywheel:proto:install task on
  # entering the shell and activated by enterShell. git and OpenSpec are the
  # releases pinned above, never the host's (core.8).
  packages = [
    (
      assert pkgs.git.version == gitVersion;
      pkgs.git
    )
    proto
    openspec
  ];

  # moon's task archives (.moon/cache/outputs) are a link into the git common
  # dir, so every worktree shares them. The rest of .moon/cache records the
  # checkout's absolute paths and stays its own.
  enterShell = ''
    if _moon_root="$(git rev-parse --show-toplevel 2>/dev/null)" &&
      _moon_common="$(git rev-parse --path-format=absolute --git-common-dir 2>/dev/null)"; then
      _moon_cache="$_moon_root/.moon/cache"
      [ -L "$_moon_cache" ] && rm -f "$_moon_cache"
      mkdir -p "$_moon_common/moon-outputs" "$_moon_cache"
      if [ ! -L "$_moon_cache/outputs" ]; then
        rm -rf "$_moon_cache/outputs"
        ln -s "$_moon_common/moon-outputs" "$_moon_cache/outputs"
      fi
      # moon puts $MOON_HOME/bin on every task's PATH too: this repository's,
      # beside proto's, so no moon of the user's runs in a task.
      MOON_HOME="$_moon_common/moon-home"
      export MOON_HOME
    fi
    unset _moon_root _moon_common _moon_cache
    # A sluice run is handed its share of the host's cores (SLUICE_JOBS):
    # cargo builds and runs tests with that many, so the runs the host's
    # bound lets go at once share its cores rather than each taking them
    # all. A session's own shell, with no SLUICE_JOBS, keeps every core.
    if [ -n "''${SLUICE_JOBS:-}" ]; then
      CARGO_BUILD_JOBS="$SLUICE_JOBS"
      RUST_TEST_THREADS="$SLUICE_JOBS"
      export CARGO_BUILD_JOBS RUST_TEST_THREADS
    fi
    # Only activates: sluice sources this inside its executor, where
    # nothing may be installed.
    ${activate}
  '';

  # The versions .prototools lists, installed on entering the shell, before
  # enterShell activates them and before devenv's own treefmt run formats
  # with the pinned rustfmt. Rust is rustup's first, with the components
  # rust-toolchain.toml names: proto's own install of it leaves them out.
  # A task, not part of enterShell: the script `devenv print-dev-env` prints
  # holds enterShell and none of the tasks, so a check run sourcing it never
  # installs.
  tasks."flywheel:proto:install" = {
    exec = ''
      cd "$(${lib.getExe pkgs.git} rev-parse --show-toplevel)"
      ${pkgs.rustup}/bin/rustup toolchain install
      ${protoHome}
      ${proto}/bin/proto install --config-mode upwards
    '';
    before = [
      "devenv:enterShell"
      "devenv:treefmt:run"
    ];
  };

  treefmt.enable = true;
  treefmt.config = {
    imports = [ ./treefmt.nix ];
    projectRootFile = "devenv.nix";
  };
}
