# Annotate, plannotator/herdr-annotate, the herdr plugin whose `doc` pane
# `plannotator-tui herdr open` opens (record 0044, points 2 and 6; design
# Decision 4 of the change crews-companions-come-with-the-flake). It tags
# no release of its own, so it is the commit the lock names, its version
# its manifest's, and the test holds the plannotator-tui it pins to the
# set's.
#
# It is copied with its `bin` filled, so herdr's link runs its two build
# steps and each exits 0 having fetched nothing: `herdr-annotate.exe` is
# its runtime, built from its own `rust` folder with that folder's
# Cargo.lock, and `plannotator-tui.exe` the given plannotator-tui's
# program, copied, each beside the version file its step reads. Nothing
# else in it is rewritten. `runtime` and `tui` are given to the evaluated
# cases.
{
  lib,
  stdenvNoCC,
  rustPlatform,
  plannotator-tui,
  src,
}:
let
  runtime = rustPlatform.buildRustPackage {
    pname = "herdr-annotate";
    inherit ((builtins.fromTOML (builtins.readFile "${src}/rust/Cargo.toml")).package) version;
    src = "${src}/rust";
    cargoLock.lockFile = "${src}/rust/Cargo.lock";
    meta = {
      description = "Annotate's native runtime";
      homepage = "https://github.com/plannotator/herdr-annotate";
      license = lib.licenses.mit;
      mainProgram = "herdr-annotate";
    };
  };
  pinned = lib.trim (builtins.readFile "${src}/herdr-annotate.version");
in
stdenvNoCC.mkDerivation {
  pname = "plannotator-herdr-plugin";
  inherit (builtins.fromTOML (builtins.readFile "${src}/herdr-plugin.toml")) version;
  inherit src;
  dontConfigure = true;
  dontBuild = true;
  installPhase = ''
    runHook preInstall
    cp -R . "$out"
    chmod -R u+w "$out"
    cp ${lib.getExe runtime} "$out/bin/herdr-annotate.exe"
    printf '%s\n' ${lib.escapeShellArg pinned} >"$out/bin/herdr-annotate.version"
    cp ${lib.getExe plannotator-tui} "$out/bin/plannotator-tui.exe"
    printf '%s' ${lib.escapeShellArg plannotator-tui.version} >"$out/bin/plannotator-tui.version"
    runHook postInstall
  '';
  dontFixup = true;
  passthru = {
    inherit runtime;
    tui = plannotator-tui;
  };
  meta = {
    description = "Annotate: comments on terminal selections, and plannotator-tui's review pane, in herdr";
    homepage = "https://github.com/plannotator/herdr-annotate";
    license = lib.licenses.mit;
  };
}
