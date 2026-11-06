# The service-modules spec's three settings, each rendered to its file
# under nix/tests/files, by the names of core.41.
{
  # "Only repositories": nothing set.
  empty = { };

  # "A host under crew".
  crew = {
    repository_root = "~/Code";
    fetch_interval = "2m";
    code_workspaces = true;
    repositories = [
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

  # "A host under a hosted control plane": `home` served the one
  # repository the host keeps, `work` none.
  hosted = {
    repositories = [ "agentplot/flume" ];
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
