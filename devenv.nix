{
  pkgs,
  lib,
  config,
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

  # git, which boomdrive:test runs (design 11): nixpkgs' build, pinned by
  # devenv.lock and asserted to be this version where it goes on PATH. Bump
  # the version with the lock.
  gitVersion = "2.55.0";

  # recutils, whose recfix weir-branch runs on every record file it writes,
  # and so boomdrive-crew:test runs (design Decision 21): nixpkgs' build,
  # pinned by devenv.lock and asserted to be this version as git is.
  recutilsVersion = "1.9";

  # The nix project's tools (record 0020, design Decision 14): the nix its
  # type check and test evaluate with, nixfmt its format runs, and statix,
  # deadnix and actionlint its lint runs. nixpkgs' builds, pinned by
  # devenv.lock and asserted to be these versions as git is. treefmt.nix's
  # nixfmt and deadnix are the same packages.
  nixVersion = "2.34.8";
  nixfmtVersion = "1.5.0";
  statixVersion = "0.5.8-unstable-2026-07-17";
  deadnixVersion = "1.3.2";
  actionlintVersion = "1.7.12";

  # The site's tools (record 0037, design Decision 5): bun, which installs
  # its packages and runs Astro, and Node.js, which runs Astro's bin under
  # bun, nixpkgs' builds pinned by devenv.lock and asserted to be these
  # versions as git is; and Biome, which formats the site's TypeScript,
  # JavaScript and CSS through treefmt.nix, whose Biome is the same
  # package (core.7).
  bunVersion = "1.4.2";
  nodeVersion = "24.20.0";
  biomeVersion = "2.5.13";

  # Vale, which site:prose runs over the site's words (site.4), from its
  # own release, as boomdrive-web's shell took it: nixpkgs trails it
  # (3.17.1 against 3.24.0). Bump the version and the four hashes from the
  # release's checksums file.
  valeVersion = "3.24.0";
  valeAssets = {
    aarch64-darwin = {
      os = "macOS_arm64";
      sha256 = "87b513f26499f6657c15cf7175ba5b058a7c1ff2bb2d4e07265db2dd43451e44";
    };
    x86_64-darwin = {
      os = "macOS_64-bit";
      sha256 = "eeb39e86f1daac27cc2a77fef3af1824d50559bdc22329fb25fb921c80c79472";
    };
    aarch64-linux = {
      os = "Linux_arm64";
      sha256 = "104cbd349279e50c4cb40adb70adf345d93cc4a3a0191f9e68bdccb97086b582";
    };
    x86_64-linux = {
      os = "Linux_64-bit";
      sha256 = "867534ddb678abca7f214bc4706ead0922bd5252ffe8035fc227013f66c8b214";
    };
  };
  vale =
    let
      system = pkgs.stdenv.hostPlatform.system;
      asset = valeAssets.${system} or (throw "vale ${valeVersion}: no release build for ${system}");
    in
    pkgs.stdenvNoCC.mkDerivation {
      pname = "vale";
      version = valeVersion;
      src = pkgs.fetchurl {
        url = "https://github.com/errata-ai/vale/releases/download/v${valeVersion}/vale_${valeVersion}_${asset.os}.tar.gz";
        inherit (asset) sha256;
      };
      sourceRoot = ".";
      installPhase = ''
        runHook preInstall
        install -Dm755 vale $out/bin/vale
        runHook postInstall
      '';
    };

  # The flake's inputs as flake.lock locks them, fetched by this shell
  # outside the gate's sandbox (design Decision 14).
  flakeInputs = import ./nix/inputs.nix { };

  # The release's tools (record 0024, core.48): git-cliff, which makes the
  # next version and its notes, and convco, which root:commits holds each
  # commit to the convention with. The flake's nixpkgs' builds, the ones
  # the workflow release runs as `nix shell --inputs-from .
  # nixpkgs#git-cliff`, so the gate, release-preview and the release run
  # one git-cliff. Pinned by flake.lock and asserted to be these versions
  # as git is.
  gitCliffVersion = "2.14.2";
  convcoVersion = "0.7.2";
  flakePkgs = flakeInputs.inputs.nixpkgs.legacyPackages.${pkgs.stdenv.hostPlatform.system};

  # The set the runner's tests prove (record 0020, core.45, design
  # Decision 6): the flake's own packages for this shell's system, its
  # outputs called with the lock's inputs as nix/tests calls them, so they
  # are the paths the family's cache holds.
  set =
    let
      outputs = (import ./flake.nix).outputs (flakeInputs.inputs // { inherit self; });
      self = outputs // {
        outPath = toString ./.;
        inherit (flakeInputs) inputs;
        inherit outputs;
        _type = "flake";
      };
    in
    outputs.packages.${pkgs.stdenv.hostPlatform.system};

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

  # devenv's own script that makes the files it declares, Claude Code's
  # among them, run from the repository's root.
  writeFiles = ''
    cd ${lib.escapeShellArg config.devenv.root} || exit
    ${config.tasks."devenv:files".exec}
  '';

in
{
  # The system tools are this file's, pinned by devenv.lock, and the
  # release's flake.lock's; moon and Rust are .prototools', installed by
  # the boomdrive:proto:install task on entering the shell and activated by
  # enterShell. git, recutils, OpenSpec, the nix project's tools, the
  # release's and the site's are the releases pinned above, never the
  # host's (core.8).
  packages = [
    (
      assert pkgs.git.version == gitVersion;
      pkgs.git
    )
    (
      assert pkgs.recutils.version == recutilsVersion;
      pkgs.recutils
    )
    (
      assert pkgs.nix.version == nixVersion;
      pkgs.nix
    )
    (
      assert pkgs.nixfmt.version == nixfmtVersion;
      pkgs.nixfmt
    )
    (
      assert pkgs.statix.version == statixVersion;
      pkgs.statix
    )
    (
      assert pkgs.deadnix.version == deadnixVersion;
      pkgs.deadnix
    )
    (
      assert pkgs.actionlint.version == actionlintVersion;
      pkgs.actionlint
    )
    (
      assert pkgs.bun.version == bunVersion;
      pkgs.bun
    )
    (
      assert pkgs.nodejs_24.version == nodeVersion;
      pkgs.nodejs_24
    )
    (
      assert pkgs.biome.version == biomeVersion;
      pkgs.biome
    )
    vale
    (
      assert flakePkgs.git-cliff.version == gitCliffVersion;
      flakePkgs.git-cliff
    )
    (
      assert flakePkgs.convco.version == convcoVersion;
      flakePkgs.convco
    )
    proto
    openspec
  ];

  # The flake's inputs as flake.lock locks them, fetched by this shell
  # outside the gate's sandbox and given to the nix project's checks by
  # their store paths, which they read with no network and no nix daemon
  # (design Decision 14).
  env.BOOMDRIVE_FLAKE_INPUTS = builtins.toJSON flakeInputs.paths;

  # The set's Flume and sluice, given to the runner's tests by their paths
  # (crates/boomdrive/tests/family.rs) and on no PATH, so a shell's `sluice
  # exec` still runs the host's sluice (design Decision 6). The shell
  # substitutes them from the family's cache where the host's nix trusts
  # it, and builds them where it does not. It pushes nothing: the cache's
  # workflow alone pushes (core.47), until the user rules on devenv's own
  # push (design, The user's to rule on).
  env.BOOMDRIVE_SET_FLUME = lib.getExe set.flume;
  env.BOOMDRIVE_SET_SLUICE = lib.getExe set.sluice;
  cachix.pull = [ "boomdrive" ];

  # Claude Code's project files (record 0022): .mcp.json from the servers
  # below and .claude/settings.json, each a link devenv makes into the
  # store and git ignores. The repository's one server is devenv's hosted
  # one, which searches packages and devenv's options and starts no
  # process on the host; it is named here because a list of the
  # repository's own replaces devenv's default. Every server named is
  # approved by name in the settings, so a session in a trusted folder, a
  # place among them, starts them and asks nothing. OpenSpec's skills and
  # commands under .claude/ are tracked, as the openspec pinned above
  # writes them, and written again when that pin moves: `openspec init
  # --tools claude --profile custom`, with the user's OpenSpec
  # configuration naming sluice's ten workflows, propose, explore, new,
  # continue, apply, ff, sync, archive, bulk-archive and verify.
  claude.code = {
    enable = true;
    mcpServers."mcp.devenv.sh" = {
      type = "http";
      url = "https://mcp.devenv.sh";
    };
  };
  files.${config.claude.code.settingsPath}.json.enabledMcpjsonServers =
    lib.attrNames config.claude.code.mcpServers;

  # moon's task archives (.moon/cache/outputs) are a link into the git common
  # dir, so every worktree shares them. The rest of .moon/cache records the
  # checkout's absolute paths and stays its own.
  enterShell = ''
    # devenv's files, Claude Code's above, made here as well as by devenv's
    # own task: a place's readiness is sluice's equip, which sources this
    # script from `devenv print-dev-env` and runs no task, and the agent
    # started after it reads them as it starts (record 0022). Its words go
    # to no output a caller reads the environment from, and a working copy
    # where they cannot be written still gets its shell.
    if ! (${writeFiles}) >/dev/null; then
      echo "devenv.nix: Claude Code's files were not written in ${config.devenv.root}" >&2
    fi
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
  tasks."boomdrive:proto:install" = {
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

  # What the workflow release would release from here: the next version
  # and its notes, by its git-cliff (record 0024). It writes nothing.
  scripts.release-preview = {
    description = "The next release's version and notes: git-cliff --unreleased --bump";
    exec = ''
      set -euo pipefail
      cd "$(git rev-parse --show-toplevel)"
      exec git-cliff --unreleased --bump --strip all "$@"
    '';
  };

  treefmt.enable = true;
  treefmt.config = {
    imports = [ ./treefmt.nix ];
    projectRootFile = "devenv.nix";
  };
}
