{ pkgs, ... }:

let
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

  enterShell = ''
    [ -d node_modules ] || bun install --frozen-lockfile 2>/dev/null || bun install
  '';
}
