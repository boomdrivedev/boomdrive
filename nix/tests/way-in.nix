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
  ];

  # The modules' way in, on each platform (design Decision 5).
  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      without = t.darwinOf [ outputs.darwinModules.default ];
      set = outputs.packages.aarch64-darwin;
    };
    NixOS = {
      system = t.nixosSystem;
      without = t.nixosOf [ outputs.nixosModules.default ];
      set = outputs.packages.x86_64-linux;
    };
  };
  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  wayIn = [
    "herdr"
    "ghostty"
    "boomdrive-launcher"
  ];

  withRunner =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with the runner fails: ${toString (t.failures evaluated)}"
    ++ lib.concatMap (
      name:
      t.expect (builtins.elem set.${name}.outPath
        profile
      ) "the ${platform} system profile holds no set's ${name}"
    ) wayIn;

  # A herdr of the host's own, laid out as a package with a bin.
  ownHerdr =
    { pkgs, ... }:
    {
      services.boomdrive.herdr.package = pkgs.runCommand "own-herdr" { } "mkdir -p $out/bin";
    };
  hostsHerdr =
    platform:
    { system, set, ... }:
    let
      evaluated = system ownHerdr;
      profile = profileOf evaluated;
    in
    t.expect (builtins.elem evaluated.config.services.boomdrive.herdr.package.outPath profile) "the ${platform} system profile holds no host's own herdr"
    ++ t.expect (
      !builtins.elem set.herdr.outPath profile
    ) "the ${platform} system profile holds the set's herdr beside the host's own";

  noGhostty =
    platform:
    { system, set, ... }:
    let
      evaluated = system { services.boomdrive.ghostty.package = null; };
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with no Ghostty fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !builtins.elem set.ghostty.outPath profile
    ) "the ${platform} system profile with no Ghostty holds the set's Ghostty"
    ++
      t.expect (hostOf evaluated ? terminal_app && (hostOf evaluated).terminal_app == "ghostty")
        "the ${platform} system with no Ghostty gives Flume's host ${builtins.toJSON (hostOf evaluated)}, its terminal_app no ghostty";

  # Flume's `[host]`, as Flume's module gives it (design Decision 7).
  hostOf = evaluated: evaluated.config.services.flume.settings.host or null;
  hostIs =
    platform: given: expected:
    let
      evaluated = platforms.${platform}.system given;
      host = hostOf evaluated;
      held = lib.mapAttrs (key: _: host.${key} or null) expected;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (held == expected)
        "the ${platform} system gives Flume's host ${builtins.toJSON held}, not ${builtins.toJSON expected}";

  flumesTerminal =
    platform: _:
    hostIs platform { } {
      multiplexer = "herdr";
      terminal_app = "ghostty";
    };
  hostsWindow =
    platform: _:
    hostIs platform { services.flume.settings.host.terminal_app = "terminal"; } {
      multiplexer = "herdr";
      terminal_app = "terminal";
    };
  wordOfBefore =
    platform: _:
    hostIs platform { services.flume.settings.host.terminal = "herdr"; } {
      terminal = "herdr";
      multiplexer = null;
    };

  withoutRunner =
    platform:
    { without, set, ... }:
    let
      profile = profileOf without;
    in
    lib.concatMap (
      name:
      t.expect (
        !builtins.elem set.${name}.outPath profile
      ) "the ${platform} system profile without the runner holds the set's ${name}"
    ) wayIn;
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
// onEach "The way in with the runner" withRunner
// onEach "A host's own herdr" hostsHerdr
// onEach "No Ghostty of the set's" noGhostty
// onEach "Not without the runner" withoutRunner
// onEach "Flume's terminal and window" flumesTerminal
// onEach "A host's own window" hostsWindow
// onEach "The word of before" wordOfBefore
