# The way-in spec's evaluated cases, and the service-modules spec's on the
# modules' own options for herdr, Ghostty and the launcher (design
# Decisions 3 to 8 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents). Each
# module case runs on nix-darwin and on NixOS, unless it names a platform.
t:
let
  inherit (t) lib outputs;

  launcherOf = system: outputs.packages.${system}.boomdrive-launcher;
  textOf = part: builtins.unsafeDiscardStringContext part.text;
  launcherText = builtins.readFile ../packages/launcher.sh;

  appOnMacOS =
    let
      inherit (launcherOf "aarch64-darwin") parts;
      plist = textOf parts.plist;
      holds = key: value: lib.hasInfix "<key>${key}</key>\n  <string>${value}</string>" plist;
    in
    t.expect (
      parts.plist.destination == "/Applications/Boomdrive.app/Contents/Info.plist"
    ) "the aarch64-darwin launcher's Info.plist is at ${parts.plist.destination}"
    ++ t.expect (holds "CFBundleName" "Boomdrive") "the aarch64-darwin launcher's Info.plist names the bundle no Boomdrive: ${plist}"
    ++ t.expect (holds "CFBundleExecutable" "boomdrive-launcher") "the aarch64-darwin launcher's Info.plist names its executable no boomdrive-launcher: ${plist}"
    ++ t.expect (
      parts.script.destination == "/Applications/Boomdrive.app/Contents/MacOS/boomdrive-launcher"
    ) "the aarch64-darwin launcher's script is at ${parts.script.destination}, not in Contents/MacOS"
    ++ t.expect (parts.script.executable) "the aarch64-darwin launcher's script is not executable"
    ++ t.expect (
      textOf parts.script == launcherText
    ) "the aarch64-darwin launcher's script is not launcher.sh";

  desktopOnLinux =
    let
      inherit (launcherOf "x86_64-linux") parts;
      entry = textOf parts.desktop;
      lines = lib.splitString "\n" entry;
      exec = "${parts.script}${parts.script.destination}";
    in
    t.expect (
      parts.desktop.destination == "/share/applications/boomdrive.desktop"
    ) "the x86_64-linux launcher's desktop entry is at ${parts.desktop.destination}"
    ++ t.expect (builtins.elem "Name=Boomdrive" lines) "the x86_64-linux desktop entry is named no Boomdrive: ${entry}"
    ++ t.expect (builtins.elem "Terminal=false" lines) "the x86_64-linux desktop entry runs in a terminal: ${entry}"
    ++ t.expect (builtins.elem "Exec=${builtins.unsafeDiscardStringContext exec}" lines) "the x86_64-linux desktop entry runs no ${builtins.unsafeDiscardStringContext exec}: ${entry}"
    ++ t.expect (
      textOf parts.script == launcherText
    ) "the x86_64-linux launcher's script is not launcher.sh";
in
{
  "The app on macOS" = appOnMacOS;
  "The desktop entry on Linux" = desktopOnLinux;
}
