# The Boomdrive launcher, of this tree (record 0023 as amended on
# 2026-11-11; design Decisions 3 to 5 of the change
# each-drive-opens-from-the-launcher): launcher.sh, which runs
# `flume open` with the session it is given, or with none, at a person's
# asking, made for `drives`, each `{ drive, session }`. On macOS it is
# Applications/Boomdrive.app, which nix-darwin places in
# /Applications/Nix Apps: a menu-bar app whose executable, boomdrive-menu,
# is compiled from launcher.m, and whose Info.plist gives the drives as
# BoomdriveDrives. On Linux it is share/applications/boomdrive.desktop,
# which runs the script in no terminal, with an action for each drive.
# Made with the set's own nixpkgs, so it is a path the family's cache
# holds (core.47), and of no host's settings: the set's package takes no
# drive, and the compiled menu takes none whatever the launcher is made
# for, so a host's launcher holds the set's one menu path. Each part is
# written whole and copied into the package, since an app bundle's parts
# are files, not links; `parts` gives them to the evaluated cases.
{
  lib,
  runCommand,
  writeTextFile,
  makeDesktopItem,
  stdenv,
  drives ? [ ],
}:
let
  inherit ((builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package) version;
  inherit (stdenv.hostPlatform) isDarwin;
  app = "/Applications/Boomdrive.app/Contents";

  script = writeTextFile {
    name = "boomdrive-launcher";
    text = builtins.readFile ./launcher.sh;
    executable = true;
    destination = if isDarwin then "${app}/MacOS/boomdrive-launcher" else "/bin/boomdrive-launcher";
  };

  # The menu, compiled by the darwin stdenv's compiler against the default
  # Apple SDK, whose linker signs it ad hoc; it takes no drive (design
  # Decision 3).
  menu = stdenv.mkDerivation {
    pname = "boomdrive-menu";
    inherit version;
    src = ./launcher.m;
    dontUnpack = true;
    buildPhase = ''
      runHook preBuild
      $CC -fobjc-arc -Wall -Wextra -Werror -framework AppKit -o boomdrive-menu "$src"
      runHook postBuild
    '';
    installPhase = ''
      runHook preInstall
      install -Dm755 boomdrive-menu "$out"${lib.escapeShellArg "${app}/MacOS/boomdrive-menu"}
      runHook postInstall
    '';
    meta.platforms = lib.platforms.darwin;
  };

  # The bundle's Info.plist (design Decision 3).
  info = {
    CFBundleName = "Boomdrive";
    CFBundleDisplayName = "Boomdrive";
    CFBundleExecutable = "boomdrive-menu";
    CFBundleIdentifier = "io.github.agentplot.boomdrive";
    CFBundlePackageType = "APPL";
    LSUIElement = true;
    BoomdriveDrives = drives;
  };

  plist = writeTextFile {
    name = "boomdrive-launcher-plist";
    destination = "${app}/Info.plist";
    text = lib.generators.toPlist { escape = true; } info;
  };

  # A session as one argument of a desktop entry's Exec, which
  # makeDesktopItem writes as given (design Decision 4): each \, ", ` and
  # $ escaped and the whole in double quotes, each % doubled so no field
  # code is read, then each \ doubled, the string type's escape, which a
  # reader undoes first.
  quoted =
    session:
    builtins.replaceStrings [ "\\" ] [ "\\\\" ] (
      builtins.replaceStrings [ "%" ] [ "%%" ] (
        "\"" + builtins.replaceStrings [ "\\" "\"" "`" "$" ] [ "\\\\" "\\\"" "\\`" "\\$" ] session + "\""
      )
    );

  # The entry's own click opens the default stream, and each drive is an
  # action of its own, its id and name the drive's (design Decision 4).
  exec = "${script}${script.destination}";
  desktop = makeDesktopItem {
    name = "boomdrive";
    desktopName = "Boomdrive";
    inherit exec;
    terminal = false;
    comment = "Open the host's agents";
    actions = lib.listToAttrs (
      map (entry: {
        name = entry.drive;
        value = {
          name = entry.drive;
          exec = "${exec} ${quoted entry.session}";
        };
      }) drives
    );
  };

  parts = {
    inherit script;
  }
  // (if isDarwin then { inherit plist menu; } else { inherit desktop; });
in
runCommand "boomdrive-launcher-${version}"
  {
    pname = "boomdrive-launcher";
    inherit version;
    passthru = {
      inherit parts drives;
    }
    // lib.optionalAttrs isDarwin { inherit info; };
    meta = {
      description = "The Boomdrive launcher, which opens the host's agents with flume open";
      homepage = "https://github.com/boomdrivedev/boomdrive";
      platforms = lib.platforms.darwin ++ lib.platforms.linux;
    };
  }
  ''
    mkdir -p "$out"
    ${lib.concatMapStrings (part: ''
      cp -R --no-preserve=mode ${part}/. "$out"/
    '') (builtins.attrValues parts)}
    chmod +x "$out"${lib.escapeShellArg script.destination}
    ${lib.optionalString isDarwin ''chmod +x "$out"${lib.escapeShellArg "${app}/MacOS/boomdrive-menu"}''}
  ''
