# The service-modules spec's three values of services.boomdrive less its
# `enable`, its `accounts` and `settings`, each rendered to its file under
# nix/tests/files, and crew's own, by the names of core.41: the accounts
# `own`, `home` and `work`, the drives `home`, `garden`, `away` and
# `field`, the owner `agentplot`, the state repositories under `example`,
# and crew's hosts `host-a` and `host-b`. crew's are written
# by crew's names at the lock's crew, its teams file's version 6 (crew
# record 0064 as amended on 2026-11-09): its drives keyed by label, each
# with `runs` and `review`'s `after_changes` and `after_hours`, beside
# `hosts`, `host-a` and `host-b`, each with its `ssh` name and `kind`.
let
  # A drive of crew's teams file, services.crew.settings.drives.<label>,
  # by crew's own keys alone: every key crew's module requires at its tag
  # (design Decision 5) but its `machine` and `state`, which the set gives
  # crew from the accounts (the change a-drives-machine-and-state-are-named-once,
  # design Decision 7). `label` is crew's own, read-only, the drive's name.
  driveOf = label: {
    blueprints = [ "example/blueprints" ];
    session = label;
    folder = "/srv/crew/${label}";
    repos = [ ];
    accounts = {
      design = "example";
      planner = "example";
      reviewer = "example";
      ops = "example";
      operator = "example";
    };
    runs = {
      design = "standing";
      planner = "standing";
      reviewer = "on-demand";
      ops = "on-demand";
      operator = "standing";
    };
    review = {
      after_changes = 5;
      after_hours = 4;
    };
  };

  # crew's drive `label` for a system whose accounts give crew nothing,
  # its `machine` and `state` written in crew's own settings.
  writtenDriveOf =
    label: machine: state:
    driveOf label // { inherit machine state; };

  # Each host crew reaches, every drive's machine among them.
  hosts = {
    host-a = {
      ssh = "host-a";
      kind = "darwin";
    };
    host-b = {
      ssh = "host-b";
      kind = "nixos";
    };
  };

  # Under crew with no crew table, the one account `work`, naming the
  # state repository `example/away`, whose one drive `away` is on `host-b`.
  elsewhere = {
    accounts.work = {
      control_plane = "crew";
      state_repository = "example/away";
      drives.away.machine = "host-b";
    };
  };

  # "A host under crew": the account `own` names its crew state
  # repository, which the runner's file does not hold.
  crew = {
    accounts.own = {
      control_plane = "crew";
      state_repository = "example/state";
      drives.home.repositories.agentplot = [
        "flume"
        "sluice"
      ];
    };
    settings = {
      repository_root = "~/Code";
      fetch_interval = "2m";
      code_workspaces = true;
      forge = {
        app_id = 1;
        key_file = "~/.config/boomdrive/app.pem";
      };
      control_plane = {
        crew = {
          host = "host-a";
          partitions = [
            {
              label = "home";
              state_repository = "example/state";
            }
          ];
        };
      };
    };
  };
in
{
  inherit crew;

  # The file `boomdrive setup` writes for one account's drive under crew
  # (design Decision 6 of the change a-persons-drive-is-set-up-by-one-command):
  # the account `own`'s drive `home`, crew's `host` and the drive's one
  # partition, and nothing else set, which setup's own test holds setup's
  # file to past its first line.
  setup = {
    accounts.own = {
      control_plane = "crew";
      drives.home.repositories.agentplot = [
        "flume"
        "sluice"
      ];
    };
    settings.control_plane = {
      crew = {
        host = "host-a";
        partitions = [
          {
            label = "home";
            state_repository = "example/state";
          }
        ];
      };
    };
  };

  # "Only accounts": nothing set.
  empty = { };

  # "A host under a hosted control plane": the account `home`, whose drive
  # `garden` keeps the one repository the host keeps, served it, and the
  # account `work`, with no drive, none.
  hosted = {
    accounts = {
      home = {
        control_plane = "hosted";
        drives.garden.repositories.agentplot = [ "flume" ];
      };
      work.control_plane = "hosted";
    };
    settings.control_plane = {
      hosted = {
        url = "https://control.example";
        partitions = [
          {
            label = "home";
            repositories = [ "agentplot/flume" ];
          }
          {
            label = "work";
            repositories = [ ];
          }
        ];
      };
    };
  };

  # "A host under crew" with no crew table, for the module to write.
  crewUnwritten = crew // {
    settings = removeAttrs crew.settings [ "control_plane" ];
  };

  # crew's teams file for one drive, `home`, on `host-a`.
  crewModule = {
    inherit hosts;
    drives.home = writtenDriveOf "home" "host-a" "example/state";
  };

  # crew's own keys for the drive `home` of `crew`'s account, its machine
  # and state given by the accounts.
  crewKeys = {
    inherit hosts;
    drives.home = driveOf "home";
  };

  # The accounts `own`, naming the state repository `example/state`, with
  # the drives `garden` on `host-a` and `home` naming no machine, and
  # `work`, naming `example/work`, with the drives `away` on `host-b` and
  # `field` naming no machine.
  crewAccounts = {
    own = {
      control_plane = "crew";
      state_repository = "example/state";
      drives = {
        garden.machine = "host-a";
        home = { };
      };
    };
    work = {
      control_plane = "crew";
      state_repository = "example/work";
      drives = {
        away.machine = "host-b";
        field = { };
      };
    };
  };

  # crew's own keys for each drive of crewAccounts.
  crewDrives = {
    inherit hosts;
    drives = {
      garden = driveOf "garden";
      home = driveOf "home";
      away = driveOf "away";
      field = driveOf "field";
    };
  };

  inherit elsewhere;

  # crew's own keys for elsewhere's drive `away`.
  crewElsewhere = {
    inherit hosts;
    drives.away = driveOf "away";
  };

  # elsewhere beside the account `own`, naming the state repository
  # `example/state`, whose drive `home`, naming no machine, keeps
  # agentplot's flume; it goes with crewElsewhere, which sets up `away`
  # alone (the change crew-runs-only-the-drives-set-up-for-it, design
  # Decision 5).
  elsewhereHome = elsewhere // {
    accounts = elsewhere.accounts // {
      own = {
        state_repository = "example/state";
        drives.home.repositories.agentplot = [ "flume" ];
      };
    };
  };

  # Under crew with no crew table, the account `own`, naming no state
  # repository, whose drive `home` keeps agentplot's flume and sluice; it
  # goes with crewModule, which writes home's machine and state.
  stateless = {
    accounts.own = {
      control_plane = "crew";
      drives.home.repositories.agentplot = [
        "flume"
        "sluice"
      ];
    };
  };

  # Under crew with no crew table, the account `own`, naming the state
  # repository `example/state`, whose drive `home` keeps agentplot's flume,
  # and the account `work`, naming no state repository, whose drive `field`
  # keeps agentplot's sluice; it goes with crewKeys, which sets up `home`
  # alone (the change crew-runs-only-the-drives-set-up-for-it, design
  # Decision 5).
  reading = {
    accounts = {
      own = {
        control_plane = "crew";
        state_repository = "example/state";
        drives.home.repositories.agentplot = [ "flume" ];
      };
      work = {
        control_plane = "crew";
        drives.field.repositories.agentplot = [ "sluice" ];
      };
    };
  };

  # crewKeys with a second drive, `stray`, whose machine and state crew's
  # settings write, and which no account names.
  crewStray = {
    inherit hosts;
    drives = {
      home = driveOf "home";
      stray = writtenDriveOf "stray" "host-a" "example/state";
    };
  };

  # crewModule by crew's old way of running, version 5's: `home` names
  # `flume`, each of its five roles false, in place of `runs`.
  crewOldRuns = {
    inherit hosts;
    drives.home = removeAttrs (writtenDriveOf "home" "host-a" "example/state") [ "runs" ] // {
      flume = {
        design = false;
        planner = false;
        reviewer = false;
        ops = false;
        operator = false;
      };
    };
  };

  # crewModule by crew's old batch names, version 5's: `home`'s `review`
  # is `every` and `hours`.
  crewOldReview = {
    inherit hosts;
    drives.home = writtenDriveOf "home" "host-a" "example/state" // {
      review = {
        every = 5;
        hours = 4;
      };
    };
  };
}
