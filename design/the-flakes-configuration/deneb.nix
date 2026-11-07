# Sketch, proposed: deneb's whole host configuration through the Boomdrive
# flake, its runner and its Flume, as the talk of 2026-11-06 shapes it
# (design/research/the-flakes-configuration-holds-streams-and-repositories.md).
# Not evaluated: the shape of the options, not their code.
{ boomdrive, pkgs, ... }:
{
  imports = [ boomdrive.darwinModules.default ]; # the runner's module and Flume's

  services.boomdrive = {
    enable = true;

    # Each owner once, with the stream its repositories run in and their
    # names. The runner's file gets the addresses, `agentplot/boomdrive`,
    # and Flume's file the rule `[repos."agentplot/*"] stream = "agentplot"`.
    repositories = {
      agentplot = {
        stream = "agentplot";
        names = [ "agentplot" "agentplot-kit" "blueprints" "boomdrive" "boomdrive-cloud" "flume" "sluice" "timber" "weir" ];
      };
      afterthought = {
        stream = "afterthought";
        names = [ "blueprints" "crew" "swancloud" ];
      };
      madswan-dev = {
        stream = "madswan";
        names = [ "paymaster" ];
      };
      willdangroup = {
        stream = "wldn";
        names = [ ];
      };
    };

    # The rest of the runner's file, as today.
    settings = {
      repository_root = "~/Code/boomdrive";
      fetch_interval = "2m";
      code_workspaces = true;
      control_plane = {
        type = "crew";
        crew = {
          host = "deneb";
          partitions = map (label: { inherit label; state_repository = "afterthought/crew-state"; }) [ "agentplot" "madswan" "afterthought" ];
        };
      };
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
        labels.operation = { reserve = 1; keep_free = 0; };
        labels.planning = { reserve = 2; keep_free = 0; };
      };
      afterthought = {
        agents = 4;
        priority = 11;
        pool = "madswan";
        labels.operation = { reserve = 1; keep_free = 1; };
        labels.planning = { reserve = 2; keep_free = 0; };
      };
      madswan = {
        agents = 4;
        priority = 10;
        pool = "madswan";
        labels.operation = { reserve = 1; keep_free = 0; };
        labels.planning = { reserve = 2; keep_free = 0; };
      };
      wldn = {
        # open: whether deneb runs wldn's agents, or keeps its repositories
        # only to follow them
      };
    };

    pools.madswan = {
      strategy = "most-left";
      threshold = 90;
      accounts = map (name: {
        inherit name;
        program = "claude";
        usage = "~/.local/state/swancloud/usage/readings/${name}.json";
        env = {
          CLAUDE_CONFIG_DIR = "~/.claude-sub-${name}";
          HONCHO_CONFIG_DIR = "~/.honcho-business";
          CLAUDE_CODE_CERT_STORE = "bundled,system";
          PATH = "${pkgs.claude-code}/bin:$PATH";
        };
      }) [ "cswanberg" "amadden" "admin" ];
    };

    kinds = {
      fix.label = "operation";
      change.agents = { mode = "full"; instructions = "Before anything else, read ~/.config/flume/capture.md …"; };
      research.agents = { mode = "full"; instructions = "…"; };
    };
  };
}
