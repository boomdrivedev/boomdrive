# The service-modules spec's three settings, each rendered to its file
# under nix/tests/files, by the names of core.41: the accounts `own`, `home`
# and `work`, the drives `home` and `garden`.
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
}
