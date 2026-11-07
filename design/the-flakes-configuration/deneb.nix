# Sketch, proposed: deneb's whole host configuration through the Boomdrive
# flake, its runner and its Flume, as the talk of 2026-11-06 shapes it
# (design/research/the-flakes-configuration-holds-streams-and-repositories.md).
# Not evaluated: the shape of the options, not their code.
{ boomdrive, pkgs, ... }:
{
  imports = [ boomdrive.darwinModules.default ]; # the runner, Flume, sluice and crew

  services.boomdrive = {
    enable = true;

    # Each owner once, with the stream its repositories run in and their
    # names. The runner's file gets each address, `agentplot/boomdrive`,
    # and Flume's file one rule for each, `[repos."agentplot/boomdrive"]
    # stream = "agentplot"`. An owner with no stream gives Flume no rule.
    repositories = {
      agentplot = {
        stream = "agentplot";
        names = [
          "agentplot"
          "agentplot-kit"
          "blueprints"
          "boomdrive"
          "boomdrive-cloud"
          "flume"
          "sluice"
          "timber"
          "weir"
        ];
      };
      afterthought = {
        stream = "afterthought";
        names = [
          "blueprints"
          "crew"
          "swancloud"
        ];
      };
      madswan-dev = {
        stream = "madswan";
        names = [ "paymaster" ];
      };
      # Kept to follow; no wldn agents on deneb, so no stream.
      willdangroup.names = [ "willdan-blueprints" ];
    };

    # The rest of the runner's file, as today.
    settings = {
      repository_root = "~/Code/boomdrive";
      fetch_interval = "2m";
      code_workspaces = true;
      # Its `[control_plane.crew]` is written from crew's partitions below:
      # host deneb, and each partition whose machine is deneb.
      control_plane.type = "crew";
    };
  };

  # Flume's file, the layer nix writes, each key named as Flume's file
  # names it. Its `repos` rules come from the owner block above.
  services.flume.settings = {
    host = {
      pool = "madswan";
      terminal = "herdr";
      terminal_app = "ghostty";
    };

    streams = {
      agentplot = {
        agents = 4;
        priority = 11;
        pool = "madswan";
        labels.operation = {
          reserve = 1;
          keep_free = 0;
        };
        labels.planning = {
          reserve = 2;
          keep_free = 0;
        };
      };
      afterthought = {
        agents = 4;
        priority = 11;
        pool = "madswan";
        labels.operation = {
          reserve = 1;
          keep_free = 1;
        };
        labels.planning = {
          reserve = 2;
          keep_free = 0;
        };
      };
      madswan = {
        agents = 4;
        priority = 10;
        pool = "madswan";
        labels.operation = {
          reserve = 1;
          keep_free = 0;
        };
        labels.planning = {
          reserve = 2;
          keep_free = 0;
        };
      };
    };

    pools.madswan = {
      strategy = "most-left";
      threshold = 90;
      # Flume reads each account's usage itself: no reading is written for it.
      accounts =
        map
          (name: {
            inherit name;
            program = "claude";
            env = {
              CLAUDE_CONFIG_DIR = "~/.claude-sub-${name}";
              HONCHO_CONFIG_DIR = "~/.honcho-business";
              CLAUDE_CODE_CERT_STORE = "bundled,system";
              PATH = "${pkgs.claude-code}/bin:$PATH";
            };
          })
          [
            "cswanberg"
            "amadden"
            "admin"
          ];
    };

    kinds = {
      fix.label = "operation";
      change.agents = {
        mode = "full";
        instructions = "Before anything else, read ~/.config/flume/capture.md …";
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
      cores = 12;
      max = 3;
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
    peers.name = "deneb";
    forge = {
      app_id = 0;
      key = "~/.config/sluice/app.pem";
    };
    signing = {
      key = "<path of the signing key>";
      signer = "sessions";
    };
  };

  # crew's host file, the fleet's every partition, written once. The
  # runner's `[control_plane.crew]` is written from the ones on deneb.
  services.crew = {
    host = "deneb";
    settings.partitions = [
      {
        label = "agentplot";
        machine = "deneb";
        state = "afterthought/crew-state";
        blueprints = [ "agentplot/blueprints" ];
        session = "agentplot";
      }
      {
        label = "madswan";
        machine = "deneb";
        state = "afterthought/crew-state";
        blueprints = [
          "afterthought/blueprints"
          "agentplot/blueprints"
        ];
        session = "madswan";
      }
      {
        label = "afterthought";
        machine = "deneb";
        state = "afterthought/crew-state";
        blueprints = [ "afterthought/blueprints" ];
        session = "afterthought";
      }
      {
        label = "wldn";
        machine = "chuck-herdr-alpha";
        state = "WilldanGroup/crew-state";
        blueprints = [ "WilldanGroup/willdan-blueprints" ];
        session = "wldn-2";
      }
    ];
  };
}
