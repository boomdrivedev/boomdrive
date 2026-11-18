# The family-packages spec's cases on the Claude Code plugins the set's
# tools ship, in the set's marketplace (record 0044 as amended on
# 2026-11-18; the design of the fix
# the-sets-marketplace-holds-every-tools-claude-code-plugins), for each
# system the flake gives: "Each tool's plugins from its tool's pin", "Each
# tool's plugins say its tool's version", "A plugin other than its tool"
# and "worktrunk's plugin, which names no version". Each reads a manifest
# from a tool's source and builds nothing.
t:
let
  inherit (t) inputs lib outputs;

  # A check's failures over each system's packages.
  each = check: lib.concatMap (system: check system outputs.packages.${system}) t.systems;

  bare = builtins.unsafeDiscardStringContext;
  has = needle: text: lib.hasInfix (bare needle) (bare text);

  # Each plugin a tool of the set ships, beside the companions', and the
  # tool whose version it is at. `from` is the folder the marketplace
  # copies whole: the tool's own flake's build where it builds the plugin,
  # crew's package's `plugin`, and otherwise the input the set builds the
  # tool from. `manifest` is the plugin's manifest in that input, which
  # crew's package copies unchanged, and `named` is false where the
  # manifest names no version, as worktrunk's, so the marketplace's entry
  # gives it.
  shipped = packages: [
    {
      name = "flume";
      tool = "flume";
      from = "${inputs.flume}/plugins/claude";
      manifest = "${inputs.flume}/plugins/claude/.claude-plugin/plugin.json";
    }
    {
      name = "sluice";
      tool = "sluice";
      from = "${inputs.sluice}/plugins/sluice";
      manifest = "${inputs.sluice}/plugins/sluice/.claude-plugin/plugin.json";
    }
    {
      name = "sluice-board";
      tool = "sluice";
      from = "${inputs.sluice}/plugins/sluice-board";
      manifest = "${inputs.sluice}/plugins/sluice-board/.claude-plugin/plugin.json";
    }
    {
      name = "crew";
      tool = "crew";
      from = "${packages.crew}/plugin";
      manifest = "${inputs.crew}/plugin/.claude-plugin/plugin.json";
    }
    {
      name = "worktrunk";
      tool = "worktrunk";
      from = "${inputs.worktrunk}/plugins/worktrunk";
      manifest = "${inputs.worktrunk}/plugins/worktrunk/.claude-plugin/plugin.json";
      named = false;
    }
  ];

  # Each plugin's manifest against its tool's version: `plugins` maps a
  # plugin's name to what its manifest says, null where it names no
  # version, its tool's version and whether it must name one. One failure
  # for each plugin that says another version, or none where it must name
  # one.
  versionsHeld =
    plugins:
    lib.concatLists (
      lib.mapAttrsToList (
        name:
        {
          said,
          version,
          named ? true,
        }:
        t.expect (said == version || (!named && said == null))
          "${name} says ${
            if said == null then "no version" else said
          }, and the set's version of its tool is ${version}"
      ) plugins
    );

  versionsOf =
    packages:
    lib.listToAttrs (
      map (plugin: {
        inherit (plugin) name;
        value = {
          said = (lib.importJSON plugin.manifest).version or null;
          inherit (packages.${plugin.tool}) version;
          named = plugin.named or true;
        };
      }) (shipped packages)
    );
in
{
  "Each tool's plugins from its tool's pin" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      name = "packages.${system}.boomdrive-claude-plugins";
    in
    t.expect (bare (toString packages.flume.src) == bare (toString inputs.flume))
      "packages.${system}.flume is built from ${toString packages.flume.src}, not the flake's flume input"
    ++
      t.expect (bare (toString packages.sluice.src) == bare (toString inputs.sluice))
        "packages.${system}.sluice is built from ${toString packages.sluice.src}, not the flake's sluice input"
    ++ lib.concatMap (
      plugin:
      let
        copied = market.sources.${plugin.name} or "nothing";
      in
      t.expect (
        bare copied == bare plugin.from
      ) "${name}' ${plugin.name} plugin is of ${bare copied}, not ${bare plugin.from}"
      ++ t.expect (has "cp -RL ${plugin.from} \"$out/plugins/${plugin.name}\"" market.buildCommand) "${name} does not copy ${bare plugin.from} whole"
    ) (shipped packages)
  );

  "Each tool's plugins say its tool's version" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      name = "packages.${system}.boomdrive-claude-plugins";
    in
    versionsHeld (versionsOf packages)
    ++ lib.concatMap (
      plugin:
      t.expect (has "said=$(jq -r '.version // \"none\"' \"$out/plugins/${plugin.name}/.claude-plugin/plugin.json\")" market.buildCommand) "${name}' build does not read ${plugin.name}'s manifest's version"
    ) (shipped packages)
  );

  "A plugin other than its tool" =
    let
      found = versionsHeld {
        sluice-board = {
          said = "0.94.7";
          version = "0.94.8";
        };
      };
    in
    t.expect
      (
        builtins.length found == 1
        && builtins.all (word: lib.hasInfix word (builtins.head found)) [
          "sluice-board"
          "0.94.7"
          "0.94.8"
        ]
      )
      "a sluice-board plugin saying 0.94.7 beside the set's sluice 0.94.8 gives ${builtins.toJSON found}";

  "worktrunk's plugin, which names no version" =
    let
      unnamed = versionsHeld {
        worktrunk = {
          said = null;
          version = "0.80.0";
          named = false;
        };
      };
      other = versionsHeld {
        worktrunk = {
          said = "0.79.0";
          version = "0.80.0";
          named = false;
        };
      };
      flume = versionsHeld {
        flume = {
          said = null;
          version = "0.29.0";
        };
      };
    in
    t.expect (unnamed == [ ]) "a worktrunk plugin naming no version gives ${builtins.toJSON unnamed}"
    ++
      t.expect
        (
          builtins.length other == 1
          && builtins.all (word: lib.hasInfix word (builtins.head other)) [
            "worktrunk"
            "0.79.0"
            "0.80.0"
          ]
        )
        "a worktrunk plugin saying 0.79.0 beside the set's worktrunk 0.80.0 gives ${builtins.toJSON other}"
    ++ t.expect (
      builtins.length flume == 1 && lib.hasInfix "no version" (builtins.head flume)
    ) "a Flume plugin naming no version gives ${builtins.toJSON flume}";
}
