# The Boomdrive launcher, of this tree (record 0023; design Decision 3 of
# the change herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# launcher.sh, which runs `flume open` at a person's asking. On macOS it is
# Applications/Boomdrive.app, which nix-darwin places in
# /Applications/Nix Apps; LSUIElement keeps a script that runs and ends out
# of the Dock. On Linux it is share/applications/boomdrive.desktop, which
# runs the script in no terminal. Made with the set's own nixpkgs, so it is
# a path the family's cache holds (core.47), and of no host's settings.
# Each part is written whole and copied into the package, since an app
# bundle's parts are files, not links; `parts` gives them to the evaluated
# cases.
{
  lib,
  runCommand,
  writeTextFile,
  makeDesktopItem,
  stdenvNoCC,
}:
let
  inherit ((builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package) version;
  inherit (stdenvNoCC.hostPlatform) isDarwin;
  app = "/Applications/Boomdrive.app/Contents";

  script = writeTextFile {
    name = "boomdrive-launcher";
    text = builtins.readFile ./launcher.sh;
    executable = true;
    destination = if isDarwin then "${app}/MacOS/boomdrive-launcher" else "/bin/boomdrive-launcher";
  };

  plist = writeTextFile {
    name = "boomdrive-launcher-plist";
    destination = "${app}/Info.plist";
    text = ''
      <?xml version="1.0" encoding="UTF-8"?>
      <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
      <plist version="1.0">
      <dict>
        <key>CFBundleName</key>
        <string>Boomdrive</string>
        <key>CFBundleDisplayName</key>
        <string>Boomdrive</string>
        <key>CFBundleExecutable</key>
        <string>boomdrive-launcher</string>
        <key>CFBundleIdentifier</key>
        <string>io.github.agentplot.boomdrive</string>
        <key>CFBundlePackageType</key>
        <string>APPL</string>
        <key>LSUIElement</key>
        <true/>
      </dict>
      </plist>
    '';
  };

  desktop = makeDesktopItem {
    name = "boomdrive";
    desktopName = "Boomdrive";
    exec = "${script}${script.destination}";
    terminal = false;
    comment = "Open the host's agents";
  };

  parts = {
    inherit script;
  }
  // (if isDarwin then { inherit plist; } else { inherit desktop; });
in
runCommand "boomdrive-launcher-${version}"
  {
    pname = "boomdrive-launcher";
    inherit version;
    passthru = { inherit parts; };
    meta = {
      description = "The Boomdrive launcher, which opens the host's agents with flume open";
      homepage = "https://github.com/agentplot/boomdrive";
      platforms = lib.platforms.darwin ++ lib.platforms.linux;
    };
  }
  ''
    mkdir -p "$out"
    ${lib.concatMapStrings (part: ''
      cp -R --no-preserve=mode ${part}/. "$out"/
    '') (builtins.attrValues parts)}
    chmod +x "$out"${lib.escapeShellArg script.destination}
  ''
