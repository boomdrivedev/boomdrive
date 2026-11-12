# Pending You's command line, which posts the cards a crew session waits on
# a person for and wakes the session when one is answered (record 0044,
# points 2 and 6): the package Pending You publishes on npm, which the lock
# names by its hash, since its source tags no release and locks no
# dependency (design Decisions 1 and 2 of the change
# crews-companions-come-with-the-flake). Nothing is built: it has no
# runtime dependency, so its `dist`, its wake mod `mod` and its
# package.json are copied, and the program runs `dist/cli.js` on the
# flake's Node.js 24, the one portless runs on. Its version is its
# package.json's, which the test holds to the input's address. `mod` and
# `node` are given to the marketplace and the evaluated cases.
{
  lib,
  stdenvNoCC,
  nodejs_24,
  makeWrapper,
  src,
}:
stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "pendingyou";
  inherit (lib.importJSON "${src}/package.json") version;
  inherit src;

  nativeBuildInputs = [ makeWrapper ];

  dontConfigure = true;
  dontBuild = true;

  installPhase = ''
    runHook preInstall
    mkdir -p $out/lib/node_modules/pendingyou
    cp -R dist mod package.json $out/lib/node_modules/pendingyou/
    makeWrapper ${lib.getExe nodejs_24} $out/bin/pendingyou \
      --add-flags $out/lib/node_modules/pendingyou/dist/cli.js
    runHook postInstall
  '';

  passthru = {
    mod = "${finalAttrs.finalPackage}/lib/node_modules/pendingyou/mod";
    node = nodejs_24;
  };

  meta = {
    description = "Pending You for coding agents: cards a session waits on a person for";
    homepage = "https://www.pendingyou.com";
    license = lib.licenses.asl20;
    mainProgram = "pendingyou";
  };
})
