# portless, which gives each worktree's dev server a `.localhost` address
# and whose routes crew's board reads (record 0023 as amended on 2026-11-08
# and 2026-11-11): this flake's recipe, as portless publishes no flake,
# built from its source at the release tag the lock names by its hash
# (core.44; design Decisions 2 to 4 of the change
# portless-comes-with-the-flake). Its version is its tagged package.json's,
# which the test holds to the tag.
#
# The workspace's package `packages/portless` alone is fetched and built:
# tsup bundles the command line into one file with no dependency, and the
# declarations its own `build` writes with tsc are a library's, which no
# program of the host reads. Under pnpm 11 the fetch holds every
# platform's optional packages, so its one hash is every system's; it
# moves with portless's tag, in the change that moves the lock.
#
# The program runs the bundle on the flake's Node.js 24, with openssl,
# procps, which and hostname after the host's own on its PATH. git is the
# host's own (records 0012 and 0023), as are `security`, `sudo` and a
# distribution's trust command. `runtimeInputs` and `node` are given to
# the evaluated cases.
{
  lib,
  stdenvNoCC,
  fetchPnpmDeps,
  pnpmConfigHook,
  pnpm_11,
  nodejs_24,
  makeWrapper,
  openssl,
  procps,
  which,
  hostname,
  src,
}:
let
  pname = "portless";
  inherit (lib.importJSON "${src}/packages/portless/package.json") version;
  pnpmWorkspaces = [ "portless" ];
  runtimeInputs = [
    openssl
    procps
    which
    hostname
  ];
in
stdenvNoCC.mkDerivation {
  inherit
    pname
    version
    src
    pnpmWorkspaces
    ;

  pnpmDeps = fetchPnpmDeps {
    inherit
      pname
      version
      src
      pnpmWorkspaces
      ;
    pnpm = pnpm_11;
    fetcherVersion = 5;
    hash = "sha256-e9nKmcWZFM1aFxW03omeEvinfyRc3NHOAGkueEcwrg4=";
  };

  nativeBuildInputs = [
    nodejs_24
    pnpm_11
    pnpmConfigHook
    makeWrapper
  ];

  buildPhase = ''
    runHook preBuild
    pnpm --filter=portless exec tsup
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out/lib/node_modules/portless
    cp -r packages/portless/dist packages/portless/package.json $out/lib/node_modules/portless/
    makeWrapper ${lib.getExe nodejs_24} $out/bin/portless \
      --add-flags $out/lib/node_modules/portless/dist/cli.js \
      --suffix PATH : ${lib.makeBinPath runtimeInputs}
    runHook postInstall
  '';

  passthru = {
    inherit runtimeInputs;
    node = nodejs_24;
  };

  meta = {
    description = "Named .localhost addresses for local dev servers";
    homepage = "https://portless.sh";
    license = lib.licenses.asl20;
    mainProgram = "portless";
  };
}
