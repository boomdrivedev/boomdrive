# The boot (record 0020 as amended on 2026-11-14; design Decision 2 of the
# change a-booted-nixos-system-proves-the-set): a NixOS test of one
# machine, `box`, made with the flake's own nixpkgs for aarch64-linux, the
# system the boxes run, with the flake's NixOS module `module` and the set
# `set` of that system. The flake gives it as checks.aarch64-linux.boot,
# which the workflow `cache`'s job `boot (aarch64-linux)` builds, and the
# nix project's test evaluates without building (nix/tests/boot.nix).
{
  pkgs,
  module,
  set,
}:
let
  settings = import ./tests/settings.nix;

  # Each tool of the set the modules install, its command and the set's
  # version of it, which its answer holds (design Decision 6): its
  # program's `--version`, and crew's `_release`, since crew's `--version`
  # prints its usage.
  tools = [
    "boomdrive"
    "flume"
    "sluice"
    "crew"
    "herdr"
    "worktrunk"
    "ghostty"
    "portless"
    "pendingyou"
    "plannotator"
    "plannotator-tui"
  ];
  answerOf =
    name:
    let
      package = set.${name};
    in
    {
      command = if name == "crew" then "crew _release" else "${package.meta.mainProgram} --version";
      inherit (package) version;
    };
  answers = map answerOf tools;
in
pkgs.testers.runNixOSTest {
  name = "boomdrive-boot";

  # No KVM on GitHub's arm runner: QEMU emulates the guest, so the test
  # requires `nixos-test` alone, and waits as long as an emulated boot
  # takes, its whole run 90 minutes (design Decisions 1 and 4).
  requiredFeatures.kvm = false;
  globalTimeout = 90 * 60;

  passthru = { inherit answers; };

  # A box that takes no work (design Decision 3): the module on for alice,
  # crew's module on for host-a with the gate's own crewModule, no control
  # plane and no repository, and git, the host's own.
  nodes.box =
    { pkgs, ... }:
    {
      imports = [ module ];
      services.boomdrive = {
        enable = true;
        user = "alice";
      };
      services.crew = {
        enable = true;
        host = "host-a";
        settings = settings.crewModule;
      };
      users.users.alice.isNormalUser = true;
      environment.systemPackages = [ pkgs.git ];
      virtualisation = {
        cores = 4;
        memorySize = 4096;
      };
    };

  # alice lingers, her manager starts with no login and the runner's unit
  # is active in it, each read as root before anything logs her in; then
  # each tool answers as alice, and the unit is still active, never
  # restarted (design Decisions 5 and 6).
  testScript = ''
    import datetime, json, shlex

    wait = datetime.timedelta(minutes=30)
    answers = json.loads(${builtins.toJSON (builtins.toJSON answers)})

    box.start()
    box.wait_for_unit("multi-user.target", timeout=wait)
    box.wait_until_succeeds(
        "loginctl show-user alice --property=Linger | grep -qx Linger=yes",
        timeout=wait,
    )
    uid = box.succeed("id -u alice").strip()
    box.wait_for_unit(f"user@{uid}.service", timeout=wait)
    box.wait_until_succeeds(
        "systemctl --user --machine=alice@.host is-active boomdrive.service",
        timeout=wait,
    )

    for answer in answers:
        said = box.succeed("su -l alice -c " + shlex.quote(answer["command"]))
        if answer["version"] not in said:
            raise Exception(
                f"{answer['command']} answers {said!r}, which holds no {answer['version']}"
            )

    box.succeed("systemctl --user --machine=alice@.host is-active boomdrive.service")
    restarts = box.succeed(
        "systemctl --user --machine=alice@.host show boomdrive.service --property=NRestarts"
    ).strip()
    if restarts != "NRestarts=0":
        raise Exception(f"boomdrive.service has {restarts}, not NRestarts=0")
  '';
}
