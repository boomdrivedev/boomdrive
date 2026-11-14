# The booted-system spec over the boot's definition (design Decision 10 of
# the change a-booted-nixos-system-proves-the-set): checks.aarch64-linux.boot
# read as the flake gives it, its `config`, its machine `nodes.box` and its
# `answers`, and its script's text, without building it. GitHub's arm
# runner boots it; the gate holds what it is. typecheck's "The boot" holds
# that its derivation evaluates.
t:
let
  inherit (t) lib;
  checks = t.outputs.checks or { };
  boot = checks.aarch64-linux.boot;
  box = boot.nodes.box;
  inherit (boot.config.node) pkgs;
  set = t.outputs.packages.aarch64-linux;
  inherit (import ./settings.nix) crewModule;

  # The runner's unit runs the set's runner with the module's file holding
  # `text`, byte for byte, as nixos.nix reads it.
  exec = box.systemd.user.services.boomdrive.serviceConfig.ExecStart;
  runs =
    text:
    exec == lib.concatMapStringsSep " " builtins.toJSON [
      "${set.boomdrive}/bin/boomdrive"
      "run"
      "--config"
      (pkgs.writeText "boomdrive-config.toml" text).outPath
    ];
  profile = map (p: p.outPath) box.environment.systemPackages;
  failures = map (a: a.message) (builtins.filter (a: !a.assertion) box.assertions);

  # The script's text, its string's context discarded, and its lines.
  script = builtins.unsafeDiscardStringContext boot.config.testScriptString;
  lines = lib.splitString "\n" script;

  # The arguments of a call whose text, after its opening parenthesis, is
  # `text`: what comes before the parenthesis that closes it.
  argumentsOf =
    text:
    let
      take =
        state: c:
        let
          depth =
            state.depth
            + (
              if c == "(" then
                1
              else if c == ")" then
                -1
              else
                0
            );
        in
        if state.done || depth < 0 then
          state // { done = true; }
        else
          {
            inherit depth;
            done = false;
            taken = state.taken + c;
          };
    in
    (builtins.foldl' take {
      depth = 0;
      done = false;
      taken = "";
    } (lib.stringToCharacters text)).taken;
  # The arguments of each call of `name` in the script.
  callsOf = name: map argumentsOf (lib.drop 1 (lib.splitString "${name}(" script));
  waits = callsOf "wait_for_unit" ++ callsOf "wait_until_succeeds";
  required = builtins.attrNames (lib.filterAttrs (_: on: on) boot.config.requiredFeatures);
in
{
  "The boot's system" =
    t.expect (failures == [ ]) "the boot's machine fails: ${toString failures}"
    ++ t.expect (
      box.nixpkgs.hostPlatform.system == "aarch64-linux"
    ) "the boot's machine is of ${box.nixpkgs.hostPlatform.system}, not aarch64-linux"
    ++ t.expect box.services.boomdrive.enable "the boot's machine does not enable services.boomdrive"
    ++ t.expect (
      box.services.boomdrive.user == "alice"
    ) "services.boomdrive.user is ${box.services.boomdrive.user}, not alice"
    ++ t.expect box.users.users.alice.isNormalUser "alice is not a normal user"
    ++ t.expect (box.users.users.alice.linger == true) "alice does not linger"
    ++ t.expect box.services.crew.enable "the boot's machine does not enable services.crew"
    ++ t.expect (
      box.services.crew.host == "host-a"
    ) "services.crew.host is ${toString box.services.crew.host}, not host-a"
    ++ t.expect (
      lib.mapAttrs (_: d: { inherit (d) machine state; }) box.services.crew.settings.drives
      == lib.mapAttrs (_: d: { inherit (d) machine state; }) crewModule.drives
      && builtins.attrNames box.services.crew.settings.hosts == builtins.attrNames crewModule.hosts
    ) "services.crew.settings is not nix/tests/settings.nix's crewModule"
    ++ t.expect (
      box.services.boomdrive.accounts == { }
    ) "the boot's machine gives the runner accounts: ${builtins.toJSON box.services.boomdrive.accounts}"
    ++ t.expect (runs (builtins.readFile ./files/empty.toml)) "the runner's file is not nix/tests/files/empty.toml's, with no control plane: ${exec}"
    ++ t.expect (builtins.elem pkgs.git.outPath profile) "the system profile holds no git, the host's own";

  "Never on x86_64-linux" =
    t.expect (
      !(checks ? x86_64-linux) && !(checks ? aarch64-darwin)
    ) "checks gives ${toString (builtins.attrNames checks)}, a system other than aarch64-linux"
    ++ t.expect (
      boot.config.hostPkgs.stdenv.hostPlatform.system == "aarch64-linux"
    ) "the boot runs on ${boot.config.hostPkgs.stdenv.hostPlatform.system}, not aarch64-linux"
    ++
      t.expect (pkgs.stdenv.hostPlatform.system == "aarch64-linux")
        "the boot's machine is made with nixpkgs of ${pkgs.stdenv.hostPlatform.system}, not aarch64-linux";

  "Emulated where there is no KVM" =
    t.expect (boot.config.requiredFeatures.kvm == false) "the boot requires kvm"
    ++ t.expect (
      required == [ "nixos-test" ]
    ) "the boot requires ${toString required}, not nixos-test alone"
    ++ t.expect (
      box.virtualisation.cores == 4
    ) "the boot's machine has ${toString box.virtualisation.cores} cores, not 4"
    ++ t.expect (
      box.virtualisation.memorySize == 4096
    ) "the boot's machine has ${toString box.virtualisation.memorySize} MiB, not 4096";

  "Its waits" =
    t.expect (
      boot.config.globalTimeout == 5400
    ) "the boot's global timeout is ${toString boot.config.globalTimeout} seconds, not 5400"
    ++ t.expect (lib.elem "wait = datetime.timedelta(minutes=30)" lines) "the script defines no wait = datetime.timedelta(minutes=30)"
    ++ t.expect (waits != [ ]) "the script waits for nothing"
    ++ lib.concatMap (
      call:
      t.expect (
        builtins.match "(.*[^a-z_])?timeout=wait([^a-z_].*)?" (lib.replaceStrings [ "\n" ] [ " " ] call)
        != null
      ) "a wait of the script names no timeout=wait: (${call})"
    ) waits;
}
