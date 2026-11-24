# The service-modules spec's "Flume's resident runs with the runner" (the
# change the-runner-starts-no-settle): with the runner on, Flume's own
# module runs the set's Flume's `flume host` as the user's service, the
# launchd agent flume-host on nix-darwin and the user unit flume-host on
# NixOS, and no job, unit or step of the system runs a settle (core.13).
t:
let
  inherit (t) lib;

  darwin = t.darwinSystem { };
  nixos = t.nixosSystem { };

  # The set's Flume's program, as the system's Flume module names it.
  flumeOf = system: "${system.config.services.flume.package}/bin/flume";

  plain = value: builtins.unsafeDiscardStringContext (toString value);

  # Every program line and script of the system's services and steps.
  darwinTexts =
    system:
    let
      inherit (system.config) launchd;
      jobs = launchd.daemons // launchd.agents // launchd.user.agents;
    in
    lib.mapAttrsToList (
      _: job: plain (job.serviceConfig.ProgramArguments or [ ]) + " " + plain (job.command or "")
    ) jobs
    ++ [ (plain system.config.system.activationScripts.script.text) ];
  nixosTexts =
    system:
    let
      units = system.config.systemd.services // system.config.systemd.user.services;
    in
    lib.mapAttrsToList (
      _: unit: plain (unit.script or "") + " " + plain (unit.serviceConfig.ExecStart or "")
    ) units
    ++ lib.mapAttrsToList (_: step: plain (step.text or step)) system.config.system.activationScripts
    ++ [ (plain system.config.system.userActivationScripts.script) ];

  noSettle =
    platform: texts:
    t.expect (!lib.any (lib.hasInfix "settle --every") texts) "the ${platform} system runs a settle";
in
{
  "Flume's resident on nix-darwin" =
    let
      agent = darwin.config.launchd.user.agents.flume-host or null;
      line = plain (agent.serviceConfig.ProgramArguments or [ ]);
    in
    t.expect (agent != null) "the nix-darwin system declares no launchd user agent flume-host"
    ++ t.expect (lib.hasInfix "exec ${plain (flumeOf darwin)} host" line) "flume-host runs ${line}"
    ++ noSettle "nix-darwin" (darwinTexts darwin);

  "Flume's resident on NixOS" =
    let
      unit = nixos.config.systemd.user.services.flume-host or null;
      line = plain (unit.serviceConfig.ExecStart or "");
    in
    t.expect (unit != null) "the NixOS system declares no systemd user service flume-host"
    ++ t.expect (line == "${plain (flumeOf nixos)} host") "flume-host runs ${line}"
    ++ noSettle "NixOS" (nixosTexts nixos);
}
