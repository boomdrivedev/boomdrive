{ pkgs, lib, ... }:

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
  ];

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
  };

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
    ${activate}
    [ -d node_modules ] || bun install --frozen-lockfile 2>/dev/null || bun install
  '';

  # The versions .prototools lists, installed on entering the shell, before
  # enterShell activates them. A task, not part of enterShell: the script
  # `devenv print-dev-env` prints holds enterShell and none of the tasks, so
  # a check run sourcing it never installs.
  tasks."flywheel-web:proto:install" = {
    exec = ''
      ${protoHome}
      ${proto}/bin/proto install --config-mode upwards
    '';
    before = [ "devenv:enterShell" ];
  };
}
