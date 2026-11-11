# The set's marketplace of crew's companions' Claude Code plugins, named
# `boomdrive` (record 0044, points 1, 2, 4 and 5; design Decision 5 of the
# change crews-companions-come-with-the-flake): one folder of regular
# files, never a link into the store (record 0022 as amended on
# 2026-11-11), from which Claude Code's own commands install each plugin.
# Nothing is compiled. It holds two plugins:
#
# - `pendingyou`: Pending You's hooks and its skill, and no wake mod, which
#   is Pending You's folder's, loaded from there (record 0044, point 5),
#   and no MCP server, since Claude Code renames a plugin's server and
#   Pending You's hooks and words call `mcp__pendingyou__*`. Each hook runs
#   the launcher in Pending You's folder by the session's `HOME`, so one
#   marketplace serves every host.
# - `plannotator`: the `apps/hook` of the source the given Plannotator is
#   built from, whole, as Plannotator's own marketplace names it. Its
#   build fails where that folder's plugin.json says another version.
#
# Pending You's skill, `skill`, is nix/packages/pendingyou-skill.md: the
# text https://www.pendingyou.com/skill-stub.md served on 2026-11-11,
# unchanged, since `init` fetches it from that address, which is not
# versioned.
#
# `plugins` gives the marketplace's list and `hooks` Pending You's hooks,
# each event's groups as hooks.json holds them; `marketplace`, `pendingyou`
# and `sources` give the rest of what is written, for the evaluated cases.
{
  lib,
  runCommand,
  writeText,
  jq,
  pendingyou,
  plannotator,
  skill,
}:
let
  # Pending You's hooks: its `init`'s CLAUDE_HOOKS, then its
  # PERMISSION_HOOKS, in their order, as Pending You 0.41.0's
  # dist/claude.js gives them, which the test holds them to. Each has
  # `init`'s 10-second timeout, but SessionEnd's, which have none.
  specs = [
    {
      event = "SessionStart";
      sub = "pickup";
    }
    {
      event = "UserPromptSubmit";
      sub = "handoff";
    }
    {
      event = "Stop";
      sub = "stopcheck";
    }
    {
      event = "SessionEnd";
      sub = "presence";
      timeout = null;
    }
    {
      event = "PermissionRequest";
      sub = "permission";
    }
    {
      event = "PostToolUse";
      sub = "permission-done";
    }
    {
      event = "PostToolUseFailure";
      sub = "permission-done";
    }
    {
      event = "SessionEnd";
      sub = "permission-done";
      timeout = null;
    }
    {
      event = "Notification";
      sub = "notify";
      matcher = "permission_prompt";
    }
  ];

  # A hook's group, as `init` writes it: the launcher by the session's
  # HOME, as its `hookLine` quotes it.
  groupOf =
    spec:
    let
      timeout = spec.timeout or 10;
    in
    lib.optionalAttrs (spec ? matcher) { inherit (spec) matcher; }
    // {
      hooks = [
        (
          {
            type = "command";
            command = ''"$HOME/.config/pendingyou/bin/pendingyou-hook" ${spec.sub} --app claude-code || true'';
          }
          // lib.optionalAttrs (timeout != null) { inherit timeout; }
        )
      ];
    };
  hooks = lib.mapAttrs (_: map groupOf) (lib.groupBy (spec: spec.event) specs);

  plugins = [
    {
      name = "pendingyou";
      source = "./plugins/pendingyou";
      inherit (pendingyou) version;
      description = "Pending You: ask your person on a card, and pick up their answer";
    }
    {
      name = "plannotator";
      source = "./plugins/plannotator";
      inherit (plannotator) version;
      description = "Plannotator: review a plan in the browser, and annotate a document";
    }
  ];

  json = name: value: writeText name (builtins.toJSON value);
  marketplace = {
    name = "boomdrive";
    description = "The Boomdrive set's Claude Code plugins of crew's companions, Pending You and Plannotator";
    owner.name = "agentplot";
    inherit plugins;
  };

  # Pending You's plugin, file by file: its manifest names no MCP server,
  # it holds no .mcp.json, and its hooks name no module.
  manifest = {
    name = "pendingyou";
    inherit (pendingyou) version;
    description = "Pending You's hooks and skill, for the set's Pending You";
    author.name = "recordplane";
    homepage = "https://www.pendingyou.com";
  };
  hooksFile = {
    description = "Pending You's hooks, which run the launcher in Pending You's folder.";
    inherit hooks;
  };
  pendingyouFiles = {
    ".claude-plugin/plugin.json" = json "plugin.json" manifest;
    "hooks/hooks.json" = json "hooks.json" hooksFile;
    "skills/pendingyou/SKILL.md" = skill;
  };
  plannotatorHook = "${plannotator.src}/apps/hook";
in
runCommand "boomdrive-claude-plugins"
  {
    nativeBuildInputs = [ jq ];
    passthru = {
      inherit hooks plugins marketplace;
      pendingyou = {
        inherit manifest hooksFile;
        files = builtins.attrNames pendingyouFiles;
      };
      sources.plannotator = plannotatorHook;
    };
    meta = {
      description = "The Boomdrive set's Claude Code marketplace of crew's companions";
      license = with lib.licenses; [
        asl20
        mit
      ];
    };
  }
  ''
    mkdir -p "$out/.claude-plugin" "$out/plugins/pendingyou"
    cp -L ${json "marketplace.json" marketplace} "$out/.claude-plugin/marketplace.json"
    ${lib.concatStrings (
      lib.mapAttrsToList (path: source: ''
        install -D -m 0644 ${source} "$out/plugins/pendingyou/${path}"
      '') pendingyouFiles
    )}
    cp -RL ${plannotatorHook} "$out/plugins/plannotator"
    chmod -R u+w "$out"

    said=$(jq -r .version "$out/plugins/plannotator/.claude-plugin/plugin.json")
    if [ "$said" != ${lib.escapeShellArg plannotator.version} ]; then
      echo "plannotator's plugin says $said, not the set's plannotator ${plannotator.version}" >&2
      exit 1
    fi
    links=$(find "$out" -type l)
    if [ -n "$links" ]; then
      echo "the marketplace holds links: $links" >&2
      exit 1
    fi
  ''
