# README's **Installing with nix**: a Mac's flake with the runner under
# crew. README shows its own copy, in a shorter layout, which its writers
# keep to this file (record 0049). tools.nix calls its outputs and reads
# the configuration's module.
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    boomdrive.url = "github:boomdrivedev/boomdrive";
  };

  outputs =
    { nix-darwin, boomdrive, ... }:
    {
      darwinConfigurations.host-a = nix-darwin.lib.darwinSystem {
        modules = [
          boomdrive.darwinModules.default
          {
            # The rest of the host's configuration, system.primaryUser among it.
            services.boomdrive = {
              enable = true;
              accounts.own = {
                state_repository = "example/state";
                drives.home.repositories.boomdrivedev = [
                  "flume"
                  "sluice"
                ];
              };
              settings = {
                repository_root = "~/Code";
                code_workspaces = true;
                control_plane.type = "crew";
              };
            };
            services.crew = {
              enable = true;
              host = "host-a";
              settings = {
                drives.home = {
                  blueprints = [ "example/blueprints" ];
                  session = "home";
                  folder = "/Users/alice/Code/boomdrivedev";
                  repos = [
                    "boomdrivedev/flume"
                    "boomdrivedev/sluice"
                  ];
                  accounts = {
                    design = "home";
                    planner = "home";
                    reviewer = "home";
                    ops = "home";
                    operator = "home";
                  };
                  # Each main-level role "standing", its agent kept running in its pane,
                  # or "on-demand", run by Flume in the drive's stream while work waits for it.
                  runs = {
                    design = "standing";
                    planner = "standing";
                    reviewer = "on-demand";
                    ops = "on-demand";
                    operator = "standing";
                  };
                  # A batch of changes is due for your read once 5 wait, or 4 hours after the last.
                  review = {
                    after_changes = 5;
                    after_hours = 4;
                  };
                };
                hosts.host-a = {
                  ssh = "host-a";
                  kind = "darwin";
                };
              };
            };
          }
        ];
      };
    };
}
