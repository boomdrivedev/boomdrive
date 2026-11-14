# The family-packages spec's cases on timber's plugins in the set (design
# Decisions 4 and 5 of the change timber-comes-with-boomdrives-flake), for
# each system the flake gives: "timber's plugins say the set's timber", "A
# timber plugin other than the set's timber" and "timber's Claude Code
# plugin from its command's source". Each reads a manifest from timber's
# source and builds nothing.
t:
let
  inherit (t) lib outputs;

  # A check's failures over each system's packages.
  each = check: lib.concatMap (system: check system outputs.packages.${system}) t.systems;

  has =
    needle: text:
    lib.hasInfix (builtins.unsafeDiscardStringContext needle) (
      builtins.unsafeDiscardStringContext text
    );

  # timber's plugins against timber (design Decision 5): `timber` is its
  # version, and `plugins` maps each plugin's name to its manifest's
  # version. One failure for each plugin whose version differs.
  timberHeld =
    { timber, plugins }:
    lib.concatLists (
      lib.mapAttrsToList (
        name: version:
        t.expect (version == timber) "${name} says ${version}, and the set's timber is ${timber}"
      ) plugins
    );
in
{
  "timber's plugins say the set's timber" = each (
    _: packages:
    let
      market = packages.boomdrive-claude-plugins;
    in
    timberHeld {
      timber = packages.timber.version;
      plugins = {
        timber-herdr-plugin =
          (builtins.fromTOML (builtins.readFile "${packages.timber-herdr-plugin.src}/herdr-plugin.toml"))
          .version;
        "boomdrive-claude-plugins' timber" =
          (builtins.fromJSON (builtins.readFile "${market.sources.timber}/.claude-plugin/plugin.json"))
          .version;
      };
    }
  );

  "A timber plugin other than the set's timber" =
    let
      found = timberHeld {
        timber = "0.6.0";
        plugins.timber-herdr-plugin = "0.5.3";
      };
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "timber-herdr-plugin"
        "0.5.3"
        "0.6.0"
      ]
    ) "a timber herdr plugin saying 0.5.3 beside the set's timber 0.6.0 gives ${builtins.toJSON found}";

  "timber's Claude Code plugin from its command's source" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      source = "${packages.timber.src}/plugins/claude-code";
    in
    t.expect
      (
        builtins.unsafeDiscardStringContext market.sources.timber
        == builtins.unsafeDiscardStringContext source
      )
      "packages.${system}.boomdrive-claude-plugins' timber plugin is of ${market.sources.timber}, not ${source}"
    ++ t.expect (has "cp -RL ${source} \"$out/plugins/timber\"" market.buildCommand) "packages.${system}.boomdrive-claude-plugins does not copy ${source} whole"
  );
}
