{ pkgs, lib, config, ... }:

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

  # The application toolchain .prototools pins (moon), on PATH: what the
  # shell activates, and what a git hook, which runs outside any shell a
  # session opened, activates for itself. proto's home is this repository's,
  # in the git common dir so every worktree shares it, and only this
  # repository's pins are read.
  protoHome = ''
    PROTO_HOME="$(${lib.getExe pkgs.git} rev-parse --path-format=absolute --git-common-dir)/proto"
    export PROTO_HOME
  '';
  activate = ''
    ${protoHome}
    eval "$(${proto}/bin/proto activate bash --export --no-bin --config-mode upwards 2>/dev/null)"
  '';

  # Vale from its own release: nixpkgs trails it (3.17.1 against 3.24.0).
  # Bump the version and the four hashes from the release's checksums file.
  valeVersion = "3.24.0";
  valeAssets = {
    aarch64-darwin = { os = "macOS_arm64"; sha256 = "87b513f26499f6657c15cf7175ba5b058a7c1ff2bb2d4e07265db2dd43451e44"; };
    x86_64-darwin = { os = "macOS_64-bit"; sha256 = "eeb39e86f1daac27cc2a77fef3af1824d50559bdc22329fb25fb921c80c79472"; };
    aarch64-linux = { os = "Linux_arm64"; sha256 = "104cbd349279e50c4cb40adb70adf345d93cc4a3a0191f9e68bdccb97086b582"; };
    x86_64-linux = { os = "Linux_64-bit"; sha256 = "867534ddb678abca7f214bc4706ead0922bd5252ffe8035fc227013f66c8b214"; };
  };
  valeAsset = valeAssets.${pkgs.stdenv.hostPlatform.system};
  vale = pkgs.stdenvNoCC.mkDerivation {
    pname = "vale";
    version = valeVersion;
    src = pkgs.fetchurl {
      url = "https://github.com/errata-ai/vale/releases/download/v${valeVersion}/vale_${valeVersion}_${valeAsset.os}.tar.gz";
      inherit (valeAsset) sha256;
    };
    sourceRoot = ".";
    installPhase = "install -Dm755 vale $out/bin/vale";
  };

  # OpenSpec, which writes and validates the site's changes and specs, and
  # writes the Claude Code skills and commands the tree tracks: the release
  # flume, boomdrive and sluice pin, built as nixpkgs builds its older one.
  # Bump the version, then each hash from the mismatch the build reports,
  # then write the skills and commands again with it (record 0003).
  openspec = pkgs.openspec.overrideAttrs (
    finalAttrs: _: {
      version = "1.14.3";
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
  # Every tool the site's scripts run is declared here, never assumed on the
  # machine: the editor, an agent's worktree, the hook and CI take the same ones.
  packages = [
    pkgs.git
    proto
    pkgs.bun
    pkgs.nodejs_24
    vale
    openspec
  ];

  # Claude Code's project files (record 0003, after agentplot/boomdrive's
  # record 0022): .mcp.json from the servers below and .claude/settings.json,
  # each a link devenv makes into the store and git ignores. The site's one
  # server is devenv's hosted one, which searches packages and devenv's
  # options and starts no process on the host; it is named here because a
  # list of the repository's own replaces devenv's default. Every server
  # named is approved by name in the settings, so a session in a trusted
  # folder, a place among them, starts them and asks nothing. OpenSpec's
  # skills and commands under .claude/ are tracked, as the openspec pinned
  # above writes them: `openspec init --tools claude --profile custom`, with
  # the user's OpenSpec configuration naming the family's ten workflows,
  # propose, explore, new, continue, apply, ff, sync, archive, bulk-archive
  # and verify.
  claude.code = {
    enable = true;
    mcpServers."mcp.devenv.sh" = {
      type = "http";
      url = "https://mcp.devenv.sh";
    };
  };
  files.${config.claude.code.settingsPath}.json.enabledMcpjsonServers =
    lib.attrNames config.claude.code.mcpServers;

  scripts = {
    # The glossary is the one source of the vocabulary Vale enforces.
    vocab.exec = ''
      bun run scripts/vocab.ts
    '';
    prose.exec = ''
      vocab && vale src/content
    '';
    site-dev.exec = ''
      bun run dev
    '';
    # The gate: schemas and types, then the prose, then the build.
    site-check.exec = ''
      bun run check && prose && bun run build
    '';
    # The site served in a place, for a preview: run as `sluice exec <branch>
    # -- site-serve`, on the port sluice holds for the worktree. devenv's
    # process manager keeps it up once the command exits; `sluice release`
    # runs site-stop.
    site-serve.exec = ''
      if [ -z "''${SLUICE_PORT_WEB:-}" ]; then
        echo "site-serve: SLUICE_PORT_WEB is unset; run it as \`sluice exec <branch> -- site-serve\`" >&2
        exit 1
      fi
      mkdir -p "$DEVENV_STATE"
      echo "$SLUICE_PORT_WEB" > "$DEVENV_STATE/site-port"
      devenv up -d site || exit 1
      for _ in $(seq 120); do
        if (exec 3<>"/dev/tcp/127.0.0.1/$SLUICE_PORT_WEB") 2>/dev/null; then
          echo "http://127.0.0.1:$SLUICE_PORT_WEB/boomdrive-web/"
          exit 0
        fi
        sleep 0.5
      done
      echo "site-serve: nothing answered on port $SLUICE_PORT_WEB after a minute" >&2
      exit 1
    '';
    # Nothing to do where nothing was served, which is every candidate's
    # release. A port file left by a process manager that is gone (a reboot)
    # is done once nothing answers on its port.
    site-stop.exec = ''
      [ -f "$DEVENV_STATE/site-port" ] || exit 0
      port="$(cat "$DEVENV_STATE/site-port")"
      rm -f "$DEVENV_STATE/site-port"
      devenv processes down 2>/dev/null ||
        ! (exec 3<>"/dev/tcp/127.0.0.1/$port") 2>/dev/null
    '';
  };

  # The dev server site-serve starts, on the port it wrote for the place.
  processes.site.exec = ''
    cd "$DEVENV_ROOT"
    exec bun run dev --host 127.0.0.1 --port "$(cat "$DEVENV_STATE/site-port")"
  '';

  # moon's task archives (.moon/cache/outputs) are a link into the git common
  # dir, so every worktree shares them. The rest of .moon/cache records the
  # checkout's absolute paths and stays its own.
  enterShell = ''
    # devenv's files, Claude Code's above, made here as well as by devenv's
    # own task: a place's readiness is sluice's equip, which sources this
    # script from `devenv print-dev-env` and runs no task, and the agent
    # started after it reads them as it starts (record 0003). Its words go
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
    ${activate}
    [ -d node_modules ] || bun install --frozen-lockfile 2>/dev/null || bun install
  '';

  # The versions .prototools lists, installed on entering the shell, before
  # enterShell activates them. A task, not part of enterShell: the script
  # `devenv print-dev-env` prints holds enterShell and none of the tasks, so
  # a check run sourcing it never installs.
  tasks."boomdrive-web:proto:install" = {
    exec = ''
      ${protoHome}
      ${proto}/bin/proto install --config-mode upwards
    '';
    before = [ "devenv:enterShell" ];
  };
}
