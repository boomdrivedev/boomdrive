# The way-in spec's evaluated cases: the launcher, an app on macOS and a
# desktop entry on Linux, and each system's shell (design Decisions 3 and
# 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents).
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
    ++ t.expect parts.script.executable "the aarch64-darwin launcher's script is not executable"
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

  # Each system's shell: a derivation of the set's five tools and the
  # hook (design Decision 4).
  hookText = builtins.readFile ../shell-hook.sh;
  shellOf =
    system:
    let
      shell = outputs.devShells.${system}.default or null;
      set = outputs.packages.${system};
      held = map (p: p.outPath) (shell.nativeBuildInputs or [ ]);
    in
    if shell == null then
      [ "devShells.${system} gives no default" ]
    else
      t.expect (lib.isDerivation shell) "devShells.${system}.default is not a derivation"
      ++ lib.concatMap (
        name:
        t.expect (builtins.elem set.${name}.outPath
          held
        ) "devShells.${system}.default holds no set's ${name}"
      ) shellTools
      ++
        t.expect (builtins.length held == builtins.length shellTools)
          "devShells.${system}.default holds ${toString (builtins.length held)} packages, not the set's ${toString shellTools}"
      ++ t.expect (
        (shell.shellHook or null) == hookText
      ) "devShells.${system}.default's hook is not nix/shell-hook.sh";
  shellTools = [
    "boomdrive"
    "flume"
    "sluice"
    "crew"
    "herdr"
    "portless"
    "worktrunk"
  ];
in
{
  "The app on macOS" = appOnMacOS;
  "The desktop entry on Linux" = desktopOnLinux;
  "Every system's shell" =
    t.expect (builtins.attrNames (outputs.devShells or { }) == lib.sort lib.lessThan t.systems)
      "devShells gives ${
        toString (builtins.attrNames (outputs.devShells or { }))
      }, not ${toString t.systems}"
    ++ lib.concatMap shellOf t.systems;
}
