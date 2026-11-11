# The service-modules spec's cases on each kept repository's clone folder
# (the change the-set-states-the-runners-folders, design Decisions 1 to
# 6): the rule of nix/modules/clone-dir.nix held to each row of
# nix/tests/clone-dirs.json, which the runner's own test holds its
# `clone::container` to, and `services.boomdrive.cloneDirs` read from
# each platform's system. A case the spec runs on either platform is a
# case on each, named for its platform.
t:
let
  inherit (t) lib;
  cloneDirOf = import ../modules/clone-dir.nix;
  rows = builtins.fromJSON (builtins.readFile ./clone-dirs.json);

  # The option's value in `evaluated`, or null where reading it fails.
  cloneDirsOf =
    evaluated:
    let
      read = builtins.tryEval (builtins.deepSeq evaluated.config.services.boomdrive.cloneDirs true);
    in
    if read.success then evaluated.config.services.boomdrive.cloneDirs else null;
  gives =
    evaluated: folders: where:
    let
      gave = cloneDirsOf evaluated;
    in
    t.expect (gave == folders)
      "${where}: services.boomdrive.cloneDirs is ${builtins.toJSON gave}, not ${builtins.toJSON folders}";
  givesOne =
    evaluated: address: folder: where:
    let
      gave = cloneDirsOf evaluated;
      one = if gave == null then null else gave.${address} or null;
    in
    t.expect (
      one == folder
    ) "${where}: services.boomdrive.cloneDirs gives ${address} ${builtins.toJSON one}, not ${folder}";
  refuses =
    evaluated: where:
    t.expect (
      !(builtins.tryEval (builtins.deepSeq evaluated.config.services.boomdrive.cloneDirs true)).success
    ) "${where}: services.boomdrive.cloneDirs set by the host evaluates";

  # The drive `home` of the account `own`, keeping the owner agentplot's
  # flume and sluice.
  home = {
    services.boomdrive.accounts.own.drives.home.repositories.agentplot = [
      "flume"
      "sluice"
    ];
  };

  # The drive `garden` of the account `work` on host-b, keeping the owner
  # example's alpha, beside crew's host host-a, as drives.nix's.
  garden = {
    services = {
      boomdrive.accounts.work.drives.garden = {
        machine = "host-b";
        repositories.example = [ "alpha" ];
      };
      crew.host = "host-a";
    };
  };

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      # The job given its own XDG_DATA_HOME, as data-dir.nix's cases give it.
      dataHome = dataHome: {
        launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME = dataHome;
      };
    };
    NixOS = {
      system = t.nixosSystem;
      dataHome = dataHome: { systemd.user.services.boomdrive.environment.XDG_DATA_HOME = dataHome; };
    };
  };

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  eachKept =
    system: home': where:
    gives (system home) {
      "agentplot/flume" = "${home'}/.local/share/boomdrive/repositories/agentplot/flume";
      "agentplot/sluice" = "${home'}/.local/share/boomdrive/repositories/agentplot/sluice";
    } where;

  anotherMachine =
    platform:
    { system, ... }:
    let
      evaluated = system garden;
      read = builtins.tryEval (builtins.seq evaluated.config.services.boomdrive.dataDir true);
      dataDir = if read.success then evaluated.config.services.boomdrive.dataDir else null;
    in
    t.expect (
      dataDir != null
    ) "${platform}, the drive garden on host-b: services.boomdrive.dataDir fails"
    ++ gives evaluated {
      "example/alpha" = "${toString dataDir}/repositories/example/alpha";
    } "${platform}, the drive garden on host-b";

  followsData =
    platform:
    { system, dataHome }:
    givesOne
      (system {
        imports = [
          home
          (dataHome "/srv/data")
        ];
      })
      "agentplot/flume"
      "/srv/data/boomdrive/repositories/agentplot/flume"
      "${platform}, XDG_DATA_HOME /srv/data";

  noneKept =
    platform:
    { system, ... }:
    gives (system { }) { } platform;

  hostSets =
    platform:
    { system, ... }:
    refuses (system {
      imports = [ home ];
      services.boomdrive.cloneDirs."agentplot/flume" = "/srv/flume";
    }) platform;
in
{
  "The runner's own clone rule" = lib.concatMap (
    row:
    let
      gave = cloneDirOf { inherit (row) dataDir address; };
    in
    t.expect (gave == row.folder)
      "dataDir ${builtins.toJSON row.dataDir} and address ${builtins.toJSON row.address} give ${builtins.toJSON gave}, not ${builtins.toJSON row.folder}"
  ) rows;

  "Each kept repository's clone folder on nix-darwin" =
    eachKept t.darwinSystem "/Users/alice"
      "nix-darwin";

  "Each kept repository's clone folder on NixOS" = eachKept t.nixosSystem "/home/alice" "NixOS";

  "The clone folders with the runner not enabled" =
    givesOne
      (t.darwinOf [
        t.outputs.darwinModules.default
        home
      ])
      "agentplot/flume"
      "/Users/alice/.local/share/boomdrive/repositories/agentplot/flume"
      "nix-darwin, the runner not enabled";
}
// onEach "The clone folder of another machine's drive" anotherMachine
// onEach "The clone folders follow the data folder" followsData
// onEach "No repository kept" noneKept
// onEach "A host that sets a clone's folder" hostSets
