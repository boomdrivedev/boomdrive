# The service-modules spec's three values of services.boomdrive less its
# `enable`, its `accounts` and `settings`, each rendered to its file under
# nix/tests/files, and crew's own, by the names of core.41: the accounts
# `own`, `home` and `work`, the drives `home` and `garden`, the owner
# `agentplot`, and crew's hosts `host-a` and `host-b`. crew's are written
# by crew's names at the lock's crew, its teams file's version 6 (crew
# record 0064 as amended on 2026-11-09): its drives keyed by label, each
# with `runs` and `review`'s `after_changes` and `after_hours`, beside
# `hosts`, `host-a` and `host-b`, each with its `ssh` name and `kind`.
let
  # A drive of crew's teams file, services.crew.settings.drives.<label>,
  # on the machine `machine`, its state at `state`: every key crew's module
  # requires at its tag (design Decision 5). `label` is crew's own,
  # read-only, the drive's name.
  driveOf = label: machine: state: {
    inherit machine state;
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

  # "A host under crew".
  crew = {
    accounts.own.drives.home.repositories.agentplot = [
      "flume"
      "sluice"
    ];
    settings = {
      repository_root = "~/Code";
      fetch_interval = "2m";
      code_workspaces = true;
      forge = {
        app_id = 1;
        key_file = "~/.config/boomdrive/app.pem";
      };
      control_plane = {
        type = "crew";
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

  # "Only accounts": nothing set.
  empty = { };

  # "A host under a hosted control plane": the account `home`, whose drive
  # `garden` keeps the one repository the host keeps, served it, and the
  # account `work`, with no drive, none.
  hosted = {
    accounts = {
      home.drives.garden.repositories.agentplot = [ "flume" ];
      work = { };
    };
    settings.control_plane = {
      type = "hosted";
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
    settings = crew.settings // {
      control_plane = {
        type = "crew";
      };
    };
  };

  # crew's teams file for one drive, `home`, on `host-a`.
  crewModule = {
    inherit hosts;
    drives.home = driveOf "home" "host-a" "example/state";
  };

  # crew's teams file for a fleet of two machines: `home` on `host-a`, and
  # `away` on `host-b`.
  crewFleet = {
    inherit hosts;
    drives = {
      home = driveOf "home" "host-a" "example/state";
      away = driveOf "away" "host-b" "example/away";
    };
  };

  # crew's teams file whose drives on `host-a`, `garden` and `home`, are
  # written out of label order, with one of `host-b`'s between them.
  crewOrder = {
    inherit hosts;
    drives = {
      garden = driveOf "garden" "host-a" "example/garden";
      away = driveOf "away" "host-b" "example/away";
      home = driveOf "home" "host-a" "example/state";
    };
  };

  # crew's teams file with no drive on `host-a`.
  crewElsewhere = {
    inherit hosts;
    drives.away = driveOf "away" "host-b" "example/away";
  };

  # crewModule by crew's old way of running, version 5's: `home` names
  # `flume`, each of its five roles false, in place of `runs`.
  crewOldRuns = {
    inherit hosts;
    drives.home = removeAttrs (driveOf "home" "host-a" "example/state") [ "runs" ] // {
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
    drives.home = driveOf "home" "host-a" "example/state" // {
      review = {
        every = 5;
        hours = 4;
      };
    };
  };
}
