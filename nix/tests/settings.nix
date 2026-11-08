# The service-modules spec's three settings, each rendered to its file
# under nix/tests/files, and crew's own, by the names of core.41: the
# accounts `own`, `home` and `work`, the drives `home` and `garden`.
{
  # "Only accounts": nothing set.
  empty = { };

  # "A host under crew".
  crew = {
    repository_root = "~/Code";
    fetch_interval = "2m";
    code_workspaces = true;
    accounts.own.drives.home.repositories = [
      "agentplot/flume"
      "agentplot/sluice"
    ];
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

  # "A host under a hosted control plane": the account `home`, whose drive
  # `garden` keeps the one repository the host keeps, served it, and the
  # account `work`, with no drive, none.
  hosted = {
    accounts = {
      home.drives.garden.repositories = [ "agentplot/flume" ];
      work = { };
    };
    control_plane = {
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

  # crew's teams file, services.crew.settings, for one partition, `home`:
  # every key crew's module asks for at its tag (design Decision 8).
  crewModule = {
    partitions = [
      {
        label = "home";
        partition = "home";
        blueprints = [ "example/blueprints" ];
        state = "example/state";
        machine = "host-a";
        session = "home";
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
      }
    ];
  };
}
