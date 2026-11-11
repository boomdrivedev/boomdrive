# The service-modules spec's cases on crew's kept repositories (the change
# the-set-gives-crew-each-kept-repositorys-folders, design Decisions 1 to
# 7): the rule of nix/modules/view-dir.nix held to each row of
# nix/tests/view-dirs.json, which the runner's own test holds its
# `config::view_path` to, and `services.crew.settings.repositories` read
# from each platform's system. A case the spec runs on either platform is
# a case on each, named for its platform.
t:
let
  inherit (t) lib;
  viewDirOf = import ../modules/view-dir.nix { inherit lib; };
  rows = builtins.fromJSON (builtins.readFile ./view-dirs.json);
  settings = import ./settings.nix;

  # A row as its failure names it.
  named =
    row:
    "home ${builtins.toJSON row.home}, repository_root ${builtins.toJSON row.repository_root} and address ${builtins.toJSON row.address}";
  # The row's rule, read: `success` and `value` as builtins.tryEval gives
  # them.
  readOf =
    row:
    builtins.tryEval (viewDirOf {
      inherit (row) home address;
      root = row.repository_root;
    });

  # crew's map in `evaluated` as `{ repositories }`, its value null among
  # them, or `{ }` where reading it fails.
  keptOf =
    evaluated:
    let
      inherit (evaluated.config.services.crew.settings) repositories;
      read = builtins.tryEval (builtins.deepSeq repositories true);
    in
    if read.success then { inherit repositories; } else { };
  shown =
    kept:
    if kept ? repositories then "is ${builtins.toJSON kept.repositories}" else "fails to evaluate";
  gives =
    evaluated: repositories: where:
    let
      kept = keptOf evaluated;
    in
    t.expect (kept ? repositories && kept.repositories == repositories)
      "${where}: services.crew.settings.repositories ${shown kept}, not ${builtins.toJSON repositories}";

  # crew turned on as tools.nix's `crewOn`: its host `host-a` and one
  # partition.
  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = settings.crewModule;
    };
  };

  # The drive `home` of the account `own`, keeping the owner agentplot's
  # flume and sluice, as clone-dirs.nix's.
  home = {
    services.boomdrive.accounts.own.drives.home.repositories.agentplot = [
      "flume"
      "sluice"
    ];
  };

  # The drive `garden` of the account `work` on host-b, keeping the owner
  # example's alpha, beside crew's host host-a, as clone-dirs.nix's.
  garden = {
    services = {
      boomdrive.accounts.work.drives.garden = {
        machine = "host-b";
        repositories.example = [ "alpha" ];
      };
      crew.host = "host-a";
    };
  };

  # A system of the runner's with crew on and the modules `given`.
  withCrew = system: given: system { imports = [ crewOn ] ++ given; };

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      homeDir = "/Users/alice";
    };
    NixOS = {
      system = t.nixosSystem;
      homeDir = "/home/alice";
    };
  };

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # Each of the drive `home`'s repositories, with its clone under the
  # data folder of `homeDir` and its view under `homeDir`'s `Code`.
  homeKept = homeDir: {
    "agentplot/flume" = {
      dir = "${homeDir}/.local/share/boomdrive/repositories/agentplot/flume";
      checkout = "${homeDir}/Code/agentplot/flume";
    };
    "agentplot/sluice" = {
      dir = "${homeDir}/.local/share/boomdrive/repositories/agentplot/sluice";
      checkout = "${homeDir}/Code/agentplot/sluice";
    };
  };

  eachKept =
    platform:
    let
      inherit (platforms.${platform}) system homeDir;
    in
    gives (withCrew system [ home ]) (homeKept homeDir) platform;

  anotherMachine =
    platform:
    { system, homeDir }:
    let
      evaluated = withCrew system [ garden ];
      read = builtins.tryEval (
        builtins.seq evaluated.config.services.boomdrive.cloneDirs."example/alpha" true
      );
    in
    t.expect read.success "${platform}, the drive garden on host-b: services.boomdrive.cloneDirs gives no example/alpha"
    ++ lib.optionals read.success (
      gives evaluated {
        "example/alpha" = {
          dir = evaluated.config.services.boomdrive.cloneDirs."example/alpha";
          checkout = "${homeDir}/Code/example/alpha";
        };
      } "${platform}, the drive garden on host-b"
    );

  underRoot =
    platform:
    { system, homeDir }:
    let
      checkoutOf =
        root:
        let
          kept = keptOf (
            withCrew system [
              home
              { services.boomdrive.settings.repository_root = root; }
            ]
          );
        in
        kept.repositories."agentplot/flume".checkout or null;
      given = {
        "~/src" = "${homeDir}/src/agentplot/flume";
        "/srv/code" = "/srv/code/agentplot/flume";
      };
    in
    lib.concatLists (
      lib.mapAttrsToList (
        root: checkout:
        let
          gave = checkoutOf root;
        in
        t.expect (gave == checkout)
          "${platform}, repository_root ${root}: services.crew.settings.repositories gives agentplot/flume the checkout ${builtins.toJSON gave}, not ${checkout}"
      ) given
    );
in
{
  "The runner's own view rule" = lib.concatMap (
    row:
    let
      read = readOf row;
      gave = if read.success then read.value else null;
    in
    t.expect (gave == row.folder) "${named row} give ${builtins.toJSON gave}, not ${row.folder}"
  ) (lib.filter (row: row.folder != null) rows);

  "A repository root the runner refuses" = lib.concatMap (
    row:
    let
      read = readOf row;
    in
    t.expect (
      !read.success
    ) "${named row} give ${builtins.toJSON read.value}, where the runner refuses the root"
  ) (lib.filter (row: row.folder == null) rows);

  "crew's kept repositories on nix-darwin" = eachKept "nix-darwin";

  "crew's kept repositories on NixOS" = eachKept "NixOS";
}
// onEach "crew's kept repository of another machine's drive" anotherMachine
// onEach "crew's checkouts under the host's repository root" underRoot
