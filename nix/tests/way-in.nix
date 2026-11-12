# The way-in spec's evaluated cases: the launcher, a menu-bar app on macOS
# and a desktop entry on Linux, made for the drives it is given, and each
# system's shell (design Decisions 3 and 4 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents, and
# Decisions 3 to 5 of the change each-drive-opens-from-the-launcher).
t:
let
  inherit (t) lib outputs;

  launcherOf = system: outputs.packages.${system}.boomdrive-launcher;
  textOf = part: builtins.unsafeDiscardStringContext part.text;
  launcherText = builtins.readFile ../packages/launcher.sh;
  toPlist = lib.generators.toPlist { escape = true; };

  # Two drives, the second opening a session of its own name (Task notes
  # 2.3).
  twoDrives = [
    {
      drive = "garden";
      session = "garden";
    }
    {
      drive = "home";
      session = "house";
    }
  ];

  appOnMacOS =
    let
      inherit (launcherOf "aarch64-darwin") parts info;
      plist = textOf parts.plist;
    in
    t.expect (
      parts.plist.destination == "/Applications/Boomdrive.app/Contents/Info.plist"
    ) "the aarch64-darwin launcher's Info.plist is at ${parts.plist.destination}"
    ++ t.expect (
      info.CFBundleName or null == "Boomdrive"
    ) "the aarch64-darwin launcher's Info.plist names the bundle no Boomdrive: ${builtins.toJSON info}"
    ++
      t.expect (info.CFBundleExecutable or null == "boomdrive-menu")
        "the aarch64-darwin launcher's Info.plist names its executable no boomdrive-menu: ${builtins.toJSON info}"
    ++ t.expect (
      info.LSUIElement or null == true
    ) "the aarch64-darwin launcher's Info.plist gives no LSUIElement true: ${builtins.toJSON info}"
    ++ t.expect (
      plist == toPlist info
    ) "the aarch64-darwin launcher's Info.plist is not its info as a plist: ${plist}"
    ++ t.expect (
      baseNameOf (toString parts.menu.src) == "launcher.m"
    ) "the aarch64-darwin launcher's menu is compiled from ${toString parts.menu.src}, not launcher.m"
    ++ t.expect (lib.hasInfix "/Applications/Boomdrive.app/Contents/MacOS/boomdrive-menu" parts.menu.installPhase) "the aarch64-darwin launcher's menu is installed at no Contents/MacOS/boomdrive-menu: ${parts.menu.installPhase}"
    ++ t.expect (
      parts.script.destination == "/Applications/Boomdrive.app/Contents/MacOS/boomdrive-launcher"
    ) "the aarch64-darwin launcher's script is at ${parts.script.destination}, not in Contents/MacOS"
    ++ t.expect parts.script.executable "the aarch64-darwin launcher's script is not executable"
    ++ t.expect (
      textOf parts.script == launcherText
    ) "the aarch64-darwin launcher's script is not launcher.sh";

  # The aarch64-darwin launcher made for two drives: its Info.plist gives
  # them in order, and its menu is the set's (design Decision 5).
  menuOnMacOS =
    let
      set = launcherOf "aarch64-darwin";
      made = set.override { drives = twoDrives; };
    in
    t.expect (made.info.BoomdriveDrives or null == twoDrives)
      "the aarch64-darwin launcher made for two drives gives BoomdriveDrives ${
        builtins.toJSON (made.info.BoomdriveDrives or null)
      }"
    ++
      t.expect (textOf made.parts.plist == toPlist made.info)
        "the aarch64-darwin launcher made for two drives has an Info.plist that is not its info: ${textOf made.parts.plist}"
    ++ t.expect (
      made.parts.menu.drvPath == set.parts.menu.drvPath
    ) "the aarch64-darwin launcher made for two drives compiles a menu of its own, not the set's";

  # The set's own launcher, made for no drive.
  noDrive =
    let
      darwin = launcherOf "aarch64-darwin";
      plist = textOf darwin.parts.plist;
      # The plist's words after the key BoomdriveDrives.
      after = lib.filter (word: word != "") (
        lib.splitString " " (
          builtins.replaceStrings [ "\n" "\t" ] [ " " " " ] (
            lib.last (lib.splitString "<key>BoomdriveDrives</key>" plist)
          )
        )
      );
      entry = textOf (launcherOf "x86_64-linux").parts.desktop;
    in
    t.expect (darwin.info.BoomdriveDrives or null == [ ])
      "the set's aarch64-darwin launcher gives BoomdriveDrives ${
        builtins.toJSON (darwin.info.BoomdriveDrives or null)
      }"
    ++ t.expect (
      lib.hasInfix "<key>BoomdriveDrives</key>" plist
      &&
        lib.take 2 after == [
          "<array>"
          "</array>"
        ]
    ) "the set's aarch64-darwin Info.plist gives no empty BoomdriveDrives: ${plist}"
    ++ t.expect (
      !lib.any (lib.hasPrefix "Actions=") (lib.splitString "\n" entry)
    ) "the set's x86_64-linux desktop entry gives Actions: ${entry}";

  # The menu's source reads its drives, titles its item and runs the
  # bundle's script (design Decision 3).
  menuSource =
    let
      source = builtins.readFile ../packages/launcher.m;
    in
    lib.concatMap (word: t.expect (lib.hasInfix word source) "nix/packages/launcher.m holds no ${word}")
      [
        ''@"BoomdriveDrives"''
        ''@"Boomdrive"''
        ''@"Default stream"''
        ''@"Quit Boomdrive"''
        ''@"boomdrive-launcher"''
      ];

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

  # The x86_64-linux launcher made for `drives`: its desktop entry's
  # lines, the lines of each action's section, and the script it runs
  # (design Decision 4).
  linuxMadeFor =
    drives:
    let
      inherit ((launcherOf "x86_64-linux").override { inherit drives; }) parts;
      entry = textOf parts.desktop;
    in
    {
      inherit entry;
      lines = lib.splitString "\n" entry;
      actionOf =
        drive:
        let
          section = lib.findFirst (lib.hasPrefix "Desktop Action ${drive}]\n") null (
            lib.splitString "\n[" entry
          );
        in
        if section == null then [ ] else lib.drop 1 (lib.splitString "\n" section);
      exec = builtins.unsafeDiscardStringContext "${parts.script}${parts.script.destination}";
    };

  actionsOnLinux =
    let
      made = linuxMadeFor twoDrives;
      action =
        drive: session:
        t.expect (builtins.elem "Name=${drive}" (made.actionOf drive)) "the x86_64-linux desktop entry made for two drives has no action ${drive} named ${drive}: ${made.entry}"
        ++ t.expect (builtins.elem "Exec=${made.exec} \"${session}\"" (made.actionOf drive)) "the x86_64-linux desktop entry's action ${drive} runs no ${made.exec} \"${session}\": ${made.entry}";
    in
    t.expect (builtins.elem "Actions=garden;home" made.lines) "the x86_64-linux desktop entry made for two drives gives no Actions=garden;home: ${made.entry}"
    ++ action "garden" "garden"
    ++ action "home" "house";

  quotedOnLinux =
    let
      made = linuxMadeFor [
        {
          drive = "home";
          session = "my $work";
        }
      ];
      line = "Exec=${made.exec} \"my \\\\$work\"";
    in
    t.expect (builtins.elem line (made.actionOf "home")) "the x86_64-linux desktop entry's action home has no ${line}: ${made.entry}";

  # Each system's shell: a derivation of the set's ten tools and the
  # hook (design Decision 4; design Decision 4 of the change
  # worktrunk-comes-with-the-flake; design Decision 10 of the change
  # crews-companions-come-with-the-flake).
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
    "pendingyou"
    "plannotator"
    "plannotator-tui"
  ];
in
{
  "The app on macOS" = appOnMacOS;
  "Each drive in the menu on macOS" = menuOnMacOS;
  "No drive" = noDrive;
  "The menu's source" = menuSource;
  "The desktop entry on Linux" = desktopOnLinux;
  "Each drive an action on Linux" = actionsOnLinux;
  "A session quoted in its action" = quotedOnLinux;
  "Every system's shell" =
    t.expect (builtins.attrNames (outputs.devShells or { }) == lib.sort lib.lessThan t.systems)
      "devShells gives ${
        toString (builtins.attrNames (outputs.devShells or { }))
      }, not ${toString t.systems}"
    ++ lib.concatMap shellOf t.systems;
}
