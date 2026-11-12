# The service-modules spec's cases on the runner's data folder (the change
# the-set-names-the-runners-data-folder, design Decisions 1 to 5): the
# rule of nix/modules/data-dir.nix held to each row of
# nix/tests/data-dirs.json, which the runner's own test holds its
# `data_dir` to.
t:
let
  inherit (t) lib;
  dataDirOf = import ../modules/data-dir.nix { inherit (t) lib; };
  rows = builtins.fromJSON (builtins.readFile ./data-dirs.json);

  # The option's value in `evaluated`, or null where reading it fails.
  dataDirOf' =
    evaluated:
    let
      read = builtins.tryEval (builtins.seq evaluated.config.services.boomdrive.dataDir true);
    in
    if read.success then evaluated.config.services.boomdrive.dataDir else null;
  gives =
    evaluated: folder: where:
    let
      gave = dataDirOf' evaluated;
    in
    t.expect (
      gave == folder
    ) "${where}: services.boomdrive.dataDir is ${builtins.toJSON gave}, not ${folder}";
  refuses =
    evaluated: where:
    t.expect (
      !(builtins.tryEval (builtins.seq evaluated.config.services.boomdrive.dataDir true)).success
    ) "${where}: services.boomdrive.dataDir set by the host evaluates";

  # nix-darwin: alice, at /Users/alice, the primary user.
  darwinGiven =
    dataHome:
    t.darwinSystem {
      launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME = dataHome;
    };
  darwinAgentOf = evaluated: evaluated.config.launchd.user.agents.boomdrive.serviceConfig;
  darwinHomeData = "/Users/alice/.local/share/boomdrive";
  darwinOwn = darwinGiven "/srv/data";

  # NixOS: alice, at /home/alice by NixOS's default for a normal user.
  nixosGiven =
    dataHome:
    t.nixosSystem {
      systemd.user.services.boomdrive.environment.XDG_DATA_HOME = dataHome;
    };
  nixosHomeData = "/home/alice/.local/share/boomdrive";
in
{
  "The runner's own rule" = lib.concatMap (
    row:
    let
      gave = dataDirOf {
        home = row.HOME;
        dataHome = row.XDG_DATA_HOME;
      };
    in
    t.expect (gave == row.folder)
      "HOME ${builtins.toJSON row.HOME} and XDG_DATA_HOME ${builtins.toJSON row.XDG_DATA_HOME} give ${builtins.toJSON gave}, not ${builtins.toJSON row.folder}"
  ) rows;

  "The data folder on nix-darwin" = gives (t.darwinSystem { }) darwinHomeData "nix-darwin";

  "The data folder with the runner not enabled" = gives (t.darwinOf [
    t.outputs.darwinModules.default
  ]) darwinHomeData "nix-darwin, the runner not enabled";

  "A job given its own XDG_DATA_HOME on nix-darwin" =
    gives darwinOwn "/srv/data/boomdrive" "nix-darwin, XDG_DATA_HOME /srv/data"
    ++
      t.expect ((darwinAgentOf darwinOwn).EnvironmentVariables.HOME == "/Users/alice")
        "nix-darwin, XDG_DATA_HOME /srv/data: the agent's HOME is ${builtins.toJSON (darwinAgentOf darwinOwn).EnvironmentVariables.HOME}"
    ++ t.expect (
      (darwinAgentOf darwinOwn).EnvironmentVariables.PATH == (darwinAgentOf (t.darwinSystem { }))
      .EnvironmentVariables.PATH
    ) "nix-darwin, XDG_DATA_HOME /srv/data: the agent's PATH is not the module's";

  "An XDG_DATA_HOME the runner takes as unset on nix-darwin" =
    gives (darwinGiven "data") darwinHomeData "nix-darwin, XDG_DATA_HOME data"
    ++ gives (darwinGiven "") darwinHomeData "nix-darwin, XDG_DATA_HOME empty";

  "A host that sets the data folder on nix-darwin" = refuses (t.darwinSystem {
    services.boomdrive.dataDir = "/srv/boomdrive";
  }) "nix-darwin";

  "The data folder on NixOS" = gives (t.nixosSystem { }) nixosHomeData "NixOS";

  "The data folder of another user on NixOS" = gives (t.nixosOf [
    t.outputs.nixosModules.default
    {
      services.boomdrive = {
        enable = true;
        user = "bob";
      };
    }
  ]) "/home/bob/.local/share/boomdrive" "NixOS, the user bob";

  "A job given its own XDG_DATA_HOME on NixOS" =
    gives (nixosGiven "/srv/data") "/srv/data/boomdrive"
      "NixOS, XDG_DATA_HOME /srv/data";

  "An XDG_DATA_HOME the runner takes as unset on NixOS" =
    gives (nixosGiven "data") nixosHomeData "NixOS, XDG_DATA_HOME data"
    ++ gives (nixosGiven "") nixosHomeData "NixOS, XDG_DATA_HOME empty";

  "A host that sets the data folder on NixOS" = refuses (t.nixosSystem {
    services.boomdrive.dataDir = "/srv/boomdrive";
  }) "NixOS";

  "The README's data folder" =
    t.expect (lib.hasInfix "services.boomdrive.dataDir" (toString t.installing)) "the README's Installing with nix names no services.boomdrive.dataDir"
    ++ t.expect (lib.hasInfix "read-only" (toString t.installing)) "the README's Installing with nix does not say services.boomdrive.dataDir is read-only";
}
