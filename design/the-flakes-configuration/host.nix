# Sketch, proposed: one host's whole configuration through the Boomdrive
# flake, its runner and its Flume, as the talk of 2026-11-06 shapes it
# (agentplot/boomdrive's
# design/research/the-flakes-configuration-holds-streams-and-repositories.md).
# Not evaluated: the shape of the options, not their code.
{ boomdrive, pkgs, ... }:
{
  imports = [ boomdrive.darwinModules.default ]; # the runner, Flume, sluice and crew

  services.boomdrive = {
    enable = true;

    # Each owner once, with the stream its repositories run in and their
    # names. The runner's file gets each address, `agentplot/boomdrive`,
    # and Flume's file one rule for each, `[repos."agentplot/boomdrive"]
    # stream = "family"`. An owner with no stream gives Flume no rule.
    repositories = {
      agentplot = {
        stream = "family";
        names = [
          "blueprints"
          "boom"
          "boomdrive"
          "crew"
          "flume"
          "sluice"
          "timber"
          "weir"
        ];
      };
      example = {
        stream = "work";
        names = [
          "app"
          "blueprints"
        ];
      };
    };

    # The rest of the runner's file.
    settings = {
      repository_root = "~/Code";
      fetch_interval = "2m";
      code_workspaces = true;
      # Its `[control_plane.crew]` is written from crew's partitions below:
      # host host-a, and each partition whose machine is host-a.
      control_plane.type = "crew";
    };
  };

  # Flume's file, the layer nix writes, each key named as Flume's file
  # names it. Its `repos` rules come from the owner block above.
  services.flume.settings = {
    host = {
      pool = "pool-a";
      terminal = "herdr";
      terminal_app = "ghostty";
    };

    streams = {
      family = {
        agents = 4;
        priority = 11;
        pool = "pool-a";
        labels.operation = {
          reserve = 1;
          keep_free = 0;
        };
        labels.planning = {
          reserve = 2;
          keep_free = 0;
        };
      };
      work = {
        agents = 2;
        priority = 10;
        pool = "pool-a";
        labels.operation = {
          reserve = 1;
          keep_free = 1;
        };
        labels.planning = {
          reserve = 1;
          keep_free = 0;
        };
      };
    };

    pools.pool-a = {
      strategy = "most-left";
      threshold = 90;
      # Flume reads each account's usage itself: no reading is written for it.
      accounts =
        map
          (name: {
            inherit name;
            program = "claude";
            env = {
              CLAUDE_CONFIG_DIR = "~/.claude-${name}";
              PATH = "${pkgs.claude-code}/bin:$PATH";
            };
          })
          [
            "pool-a-1"
            "pool-a-2"
          ];
    };

    kinds = {
      fix.label = "operation";
      change.agents = {
        mode = "full";
        instructions = "…";
      };
      research.agents = {
        mode = "full";
        instructions = "…";
      };
    };
  };

  # sluice's host settings, each key named as its file names it. A key's
  # file is named by its path; no key is in the store.
  services.sluice.settings = {
    runs = {
      cores = 8;
      max = 2;
      reserved = 1;
    };
    caches = [
      {
        dir = "~/Library/Caches/sccache";
        env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
      }
      {
        dir = "~/.moon/cache/shared";
        env.MOON_CACHE_SHARED_WORKTREE_CACHE = "true";
      }
    ];
    peers.name = "host-a";
    forge = {
      app_id = 0;
      key = "<path of the app's key>";
    };
    signing = {
      key = "<path of the signing key>";
      signer = "sessions";
    };
  };

  # crew's host file, the fleet's every partition, written once. The
  # runner's `[control_plane.crew]` is written from the ones on host-a.
  services.crew = {
    host = "host-a";
    settings.partitions = [
      {
        label = "home";
        machine = "host-a";
        state = "example/crew-state";
        blueprints = [ "agentplot/blueprints" ];
        session = "home";
      }
      {
        label = "work";
        machine = "host-a";
        state = "example/crew-state";
        blueprints = [
          "example/blueprints"
          "agentplot/blueprints"
        ];
        session = "work";
      }
    ];
  };
}
