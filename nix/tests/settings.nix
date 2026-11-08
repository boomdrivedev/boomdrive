# The service-modules spec's three values of services.boomdrive less its
# `enable`, its `accounts` and `settings`, each rendered to its file under
# nix/tests/files, and crew's own, by the names of core.41: the accounts
# `own`, `home` and `work`, the drives `home` and `garden`, the owner
# `agentplot`, and crew's hosts `host-a` and `host-b`.
let
  # A partition of crew's teams file, services.crew.settings, on the
  # machine `machine`, its state at `state`: every key crew's module asks
  # for at its tag (design Decision 8).
  partitionOf = label: machine: state: {
    inherit label machine state;
    partition = label;
    blueprints = [ "example/blueprints" ];
    session = label;
    review = {
      every = 5;
      hours = 4;
    };
    flume = {
      design = false;
      planner = false;
      reviewer = false;
      ops = false;
      operator = false;
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

  # crew's teams file for one partition, `home`, on `host-a`.
  crewModule = {
    partitions = [ (partitionOf "home" "host-a" "example/state") ];
  };

  # crew's teams file for a fleet of two machines: `home` on `host-a`, and
  # `away` on `host-b`.
  crewFleet = {
    partitions = [
      (partitionOf "home" "host-a" "example/state")
      (partitionOf "away" "host-b" "example/away")
    ];
  };

  # crew's teams file whose partitions on `host-a` are not in label order,
  # with one of `host-b`'s between them.
  crewOrder = {
    partitions = [
      (partitionOf "garden" "host-a" "example/garden")
      (partitionOf "away" "host-b" "example/away")
      (partitionOf "home" "host-a" "example/state")
    ];
  };

  # crew's teams file with no partition on `host-a`.
  crewElsewhere = {
    partitions = [ (partitionOf "away" "host-b" "example/away") ];
  };
}
