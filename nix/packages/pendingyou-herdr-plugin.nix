# Pending You for Herdr, recordplane/herdr-pendingyou, which shows the
# cards each agent waits on in herdr and starts Pending You's watcher at
# herdr's start (record 0044, points 2 and 6; design Decision 4 of the
# change crews-companions-come-with-the-flake). It tags no release of its
# own, so it is the commit the lock names, copied whole, its version its
# manifest's. Nothing in it is rewritten, as nothing in the set's other
# plugins is: its `bin/run.sh` runs the launcher in Pending You's folder,
# which each activation writes before it links the plugins, and the test
# holds the Pending You it names to the set's. herdr has no build step of
# its to run.
{
  lib,
  stdenvNoCC,
  src,
}:
stdenvNoCC.mkDerivation {
  pname = "pendingyou-herdr-plugin";
  inherit (builtins.fromTOML (builtins.readFile "${src}/herdr-plugin.toml")) version;
  inherit src;
  dontConfigure = true;
  dontBuild = true;
  installPhase = ''
    runHook preInstall
    cp -R . "$out"
    runHook postInstall
  '';
  dontFixup = true;
  meta = {
    description = "Pending You for Herdr: the cards each agent waits on, in herdr";
    homepage = "https://github.com/recordplane/herdr-pendingyou";
    license = lib.licenses.asl20;
  };
}
