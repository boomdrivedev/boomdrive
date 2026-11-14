# The family-packages spec's cases on crew's companions in the set (record
# 0044, point 6; design Decisions 2 to 4 of the change
# crews-companions-come-with-the-flake), for each system the flake gives:
# Pending You as it publishes it on npm, held to the version its address
# names; Plannotator and plannotator-tui through numtide/llm-agents.nix,
# held to their release tags; and their herdr plugins, each held to the
# version of the set's tool it runs; and the set's marketplace of their
# Claude Code plugins (design Decision 5).
t:
let
  inherit (t) inputs lib outputs;

  # A check's failures over each system's packages.
  each = check: lib.concatMap (system: check system outputs.packages.${system}) t.systems;

  # Each failing case's failures, `found`, as one line naming each of
  # `words`.
  failsNaming =
    what: found: words:
    t.expect (
      builtins.length found == 1 && builtins.all (word: lib.hasInfix word (builtins.head found)) words
    ) "${what} gives ${builtins.toJSON found}";

  # Pending You's version as the input's address names it,
  # `.../pendingyou-<version>.tgz`, or null where it names none.
  addressed =
    let
      found = builtins.match ".*/pendingyou-([0-9]+\\.[0-9]+\\.[0-9]+)\\.tgz" t.flake.inputs.pendingyou.url;
    in
    if found == null then null else builtins.head found;

  # The set's Pending You against its address (design Decision 2):
  # `version` is the package's, read from its package.json.
  pendingyouHeld =
    system: version: address:
    t.expect (version == address)
      "packages.${system}.pendingyou says ${version}, not ${toString address}, the version its address names";

  # The releases the set names of Plannotator and plannotator-tui, which
  # move with llm-agents.nix's commit in the lock, in the change that
  # moves it (design Decision 3).
  releases = {
    plannotator = {
      version = "0.28.61";
      owner = "backnotprop";
      repo = "plannotator";
    };
    plannotator-tui = {
      version = "0.9.9";
      owner = "plannotator";
      repo = "plannotator-tui";
    };
  };

  # A package of llm-agents.nix's against its release: its version and its
  # source's tag.
  releaseHeld =
    system: name: package:
    let
      release = releases.${name};
      tag = package.src.tag or "no tag";
    in
    t.expect (package.version == release.version)
      "packages.${system}.${name} is ${name} ${package.version}, not ${release.version}, the release the set names"
    ++ t.expect (
      tag == "v${package.version}"
    ) "packages.${system}.${name}'s source is at ${tag}, not v${package.version}"
    ++ t.expect (
      (package.src.owner or null) == release.owner && (package.src.repo or null) == release.repo
    ) "packages.${system}.${name}'s source is not ${release.owner}/${release.repo}";

  # Every version of Pending You a plugin's `bin/run.sh` names, as
  # `pendingyou@<version>`.
  namedIn =
    text:
    lib.concatMap (
      line:
      let
        found = builtins.match ".*pendingyou@([0-9]+\\.[0-9]+\\.[0-9]+).*" line;
      in
      lib.optional (found != null) (builtins.head found)
    ) (lib.splitString "\n" text);

  # Pending You for Herdr against the set's Pending You (design Decision
  # 4): `text` is its run.sh and `version` the set's Pending You's.
  runsHeld =
    system: text: version:
    let
      named = lib.unique (namedIn text);
    in
    if named == [ ] then
      [ "packages.${system}.pendingyou-herdr-plugin's bin/run.sh names no pendingyou@<version>" ]
    else
      lib.concatMap (
        given:
        t.expect (given == version)
          "packages.${system}.pendingyou-herdr-plugin's bin/run.sh names pendingyou@${given}, not the set's Pending You ${version}"
      ) named;

  # Annotate against the set's plannotator-tui (design Decision 4):
  # `pinned` is its plannotator-tui.version and `version` the set's.
  annotateHeld =
    system: pinned: version:
    t.expect (pinned == version)
      "packages.${system}.plannotator-herdr-plugin pins plannotator-tui ${pinned}, not the set's ${version}";

  # `needle` in `text`, as text: a regular expression may not hold a
  # store path.
  has =
    needle: text:
    lib.hasInfix (builtins.unsafeDiscardStringContext needle) (
      builtins.unsafeDiscardStringContext text
    );

  # The hooks Pending You's `init` gives Claude Code, `{ event, sub }` in
  # its order, as its dist/claude.js gives them as text (Task notes 2.1):
  # each event of HOOKS, then each of PERMISSION_HOOKS.
  ownHooks =
    text:
    let
      lines = lib.splitString "\n" text;
      after =
        marker:
        lib.drop (1 + lib.lists.findFirstIndex (lib.hasPrefix marker) (builtins.length lines) lines) lines;
      upTo =
        list:
        lib.take (lib.lists.findFirstIndex (
          line: lib.hasPrefix "}" line || lib.hasPrefix "]" line
        ) (builtins.length list) list) list;
      sessions = lib.concatMap (
        line:
        let
          found = builtins.match " *([A-Za-z]+): '([a-z-]+)',?" line;
        in
        lib.optional (found != null) {
          event = builtins.elemAt found 0;
          sub = builtins.elemAt found 1;
        }
      ) (upTo (after "export const HOOKS = {"));
      permissions = lib.concatMap (
        line:
        let
          found = builtins.match ".*event: '([A-Za-z]+)', sub: '([a-z-]+)'.*" line;
        in
        lib.optional (found != null) {
          event = builtins.elemAt found 0;
          sub = builtins.elemAt found 1;
        }
      ) (upTo (after "export const PERMISSION_HOOKS = ["));
    in
    sessions ++ permissions;

  # Each `{ event, sub }` of `own` among `hooks`, the plugin's, each
  # missing one failing, naming its event and subcommand.
  hooksHeld =
    system: own: hooks:
    lib.concatMap (
      hook:
      t.expect
        (builtins.any (
          group: builtins.any (h: has " ${hook.sub} --app claude-code " h.command) group.hooks
        ) (hooks.${hook.event} or [ ]))
        "packages.${system}.boomdrive-claude-plugins' pendingyou plugin holds no ${hook.event} hook running ${hook.sub}, which Pending You's init gives Claude Code"
    ) own;

  # The plugin's hooks, as the spec gives them: each event's groups, each
  # a subcommand and its matcher.
  expectedHooks = {
    SessionStart = [ { sub = "pickup"; } ];
    UserPromptSubmit = [ { sub = "handoff"; } ];
    Stop = [ { sub = "stopcheck"; } ];
    SessionEnd = [
      { sub = "presence"; }
      { sub = "permission-done"; }
    ];
    PermissionRequest = [ { sub = "permission"; } ];
    PostToolUse = [ { sub = "permission-done"; } ];
    PostToolUseFailure = [ { sub = "permission-done"; } ];
    Notification = [
      {
        sub = "notify";
        matcher = "permission_prompt";
      }
    ];
  };
  lineOf = sub: ''"$HOME/.config/pendingyou/bin/pendingyou-hook" ${sub} --app claude-code || true'';
  groupOf =
    event: expected:
    lib.optionalAttrs (expected ? matcher) { inherit (expected) matcher; }
    // {
      hooks = [
        (
          {
            type = "command";
            command = lineOf expected.sub;
          }
          // lib.optionalAttrs (event != "SessionEnd") { timeout = 10; }
        )
      ];
    };

  marketplaceOf = system: outputs.packages.${system}.boomdrive-claude-plugins;

  readTrimmed = path: lib.trim (builtins.readFile path);
  nixpkgsOf = system: inputs.nixpkgs.legacyPackages.${system};
in
{
  "Pending You from its npm package" = each (
    system: packages:
    let
      inherit (packages) pendingyou;
      source = inputs.pendingyou.outPath;
      name = "packages.${system}.pendingyou";
      own = (lib.importJSON "${source}/package.json").version;
    in
    t.expect (
      toString pendingyou.src == source
    ) "${name} is of ${toString pendingyou.src}, not ${source}"
    ++ t.expect (has "cp -R dist mod package.json $out/lib/node_modules/pendingyou/" pendingyou.installPhase) "${name} copies no dist, mod and package.json"
    ++
      t.expect ((pendingyou.meta.mainProgram or null) == "pendingyou")
        "${name}'s program is ${toString (pendingyou.meta.mainProgram or "none")}, not pendingyou"
    ++ t.expect (
      pendingyou.version == own
    ) "${name} says ${pendingyou.version}, not its package.json's ${own}"
    ++ pendingyouHeld system pendingyou.version addressed
  );

  "Pending You other than its package" =
    failsNaming "a Pending You 0.41.1 under the address's 0.41.0"
      (pendingyouHeld "x86_64-linux" "0.41.1" "0.41.0")
      [
        "0.41.1"
        "0.41.0"
      ];

  "Pending You's Node.js" = each (
    system: packages:
    let
      inherit (packages) pendingyou;
      node = (nixpkgsOf system).nodejs_24;
    in
    t.expect (pendingyou.node.drvPath == node.drvPath)
      "packages.${system}.pendingyou runs on ${pendingyou.node.name}, not the flake's nixpkgs' nodejs_24"
    ++ t.expect (has "makeWrapper ${lib.getExe node} $out/bin/pendingyou" pendingyou.installPhase) "packages.${system}.pendingyou's program does not run the flake's nixpkgs' nodejs_24"
    ++ t.expect (has "--add-flags $out/lib/node_modules/pendingyou/dist/cli.js" pendingyou.installPhase) "packages.${system}.pendingyou's program does not run dist/cli.js"
  );

  "Plannotator and plannotator-tui through llm-agents.nix" = each (
    system: packages:
    lib.concatMap (
      name:
      let
        package = packages.${name};
        own = inputs.llm-agents.packages.${system}.${name};
      in
      t.expect (
        package.drvPath == own.drvPath
      ) "packages.${system}.${name} is not llm-agents.nix's packages.${system}.${name}"
      ++ releaseHeld system name package
    ) (builtins.attrNames releases)
  );

  "Plannotator other than its tag" =
    failsNaming "a Plannotator 0.28.62 at v0.28.62"
      (releaseHeld "x86_64-linux" "plannotator" {
        version = "0.28.62";
        src = {
          tag = "v0.28.62";
          owner = "backnotprop";
          repo = "plannotator";
        };
      })
      [
        "plannotator"
        "0.28.62"
        "0.28.61"
      ];

  "Pending You for Herdr names the set's Pending You" = each (
    system: packages:
    let
      plugin = packages.pendingyou-herdr-plugin;
      source = inputs.herdr-pendingyou.outPath;
      manifest = builtins.fromTOML (builtins.readFile "${source}/herdr-plugin.toml");
    in
    t.expect (
      toString plugin.src == source
    ) "packages.${system}.pendingyou-herdr-plugin is of ${toString plugin.src}, not ${source}"
    ++
      t.expect (plugin.version == manifest.version)
        "packages.${system}.pendingyou-herdr-plugin says ${plugin.version}, not its manifest's ${manifest.version}"
    ++ t.expect (plugin.dontFixup or false
    ) "packages.${system}.pendingyou-herdr-plugin rewrites the plugin's files"
    ++ runsHeld system (builtins.readFile "${source}/bin/run.sh") packages.pendingyou.version
  );

  "Pending You for Herdr other than the set's" =
    failsNaming "a run.sh naming pendingyou@0.40.0 beside a set's 0.41.0"
      (runsHeld "x86_64-linux" "exec npx -y pendingyou@0.40.0 \"$@\"" "0.41.0")
      [
        "0.40.0"
        "0.41.0"
      ];

  "Annotate holds its runtime and the set's plannotator-tui" = each (
    system: packages:
    let
      plugin = packages.plannotator-herdr-plugin;
      inherit (plugin) runtime tui;
      source = inputs.herdr-annotate.outPath;
      name = "packages.${system}.plannotator-herdr-plugin";
      manifest = builtins.fromTOML (builtins.readFile "${source}/herdr-plugin.toml");
      installs = plugin.installPhase;
    in
    t.expect (toString plugin.src == source) "${name} is of ${toString plugin.src}, not ${source}"
    ++ t.expect (
      plugin.version == manifest.version
    ) "${name} says ${plugin.version}, not its manifest's ${manifest.version}"
    ++ t.expect (
      toString runtime.src == "${source}/rust"
    ) "${name}'s runtime is built from ${toString runtime.src}, not ${source}/rust"
    ++
      t.expect (toString (runtime.cargoDeps.lockFile or "none") == "${source}/rust/Cargo.lock")
        "${name}'s runtime is locked by ${
          toString (runtime.cargoDeps.lockFile or "none")
        }, not ${source}/rust/Cargo.lock"
    ++ t.expect (
      tui.drvPath == packages.plannotator-tui.drvPath
    ) "${name} holds a plannotator-tui other than the set's"
    ++ t.expect (has "cp ${lib.getExe runtime} \"$out/bin/herdr-annotate.exe\"" installs) "${name}'s bin holds no runtime"
    ++ t.expect (has "cp ${lib.getExe tui} \"$out/bin/plannotator-tui.exe\"" installs) "${name}'s bin holds no set's plannotator-tui"
    ++ t.expect (has "\"$out/bin/herdr-annotate.version\"" installs) "${name}'s bin holds no herdr-annotate.version"
    ++ t.expect (has "\"$out/bin/plannotator-tui.version\"" installs) "${name}'s bin holds no plannotator-tui.version"
    ++
      annotateHeld system (readTrimmed "${source}/plannotator-tui.version")
        packages.plannotator-tui.version
  );

  "The marketplace's three plugins" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      given = map (plugin: {
        inherit (plugin) name version source;
      }) market.plugins;
      expected = [
        {
          name = "pendingyou";
          inherit (packages.pendingyou) version;
          source = "./plugins/pendingyou";
        }
        {
          name = "plannotator";
          inherit (packages.plannotator) version;
          source = "./plugins/plannotator";
        }
        {
          name = "timber";
          inherit (packages.timber) version;
          source = "./plugins/timber";
        }
      ];
    in
    t.expect (market.marketplace.name == "boomdrive")
      "packages.${system}.boomdrive-claude-plugins is the marketplace ${market.marketplace.name}, not boomdrive"
    ++ t.expect (
      market.marketplace.plugins == market.plugins
    ) "packages.${system}.boomdrive-claude-plugins' marketplace.json lists other plugins than it gives"
    ++
      t.expect (given == expected)
        "packages.${system}.boomdrive-claude-plugins lists ${builtins.toJSON given}, not ${builtins.toJSON expected}"
  );

  "Pending You's hooks" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      expected = lib.mapAttrs (event: map (groupOf event)) expectedHooks;
    in
    t.expect (market.hooks == expected)
      "packages.${system}.boomdrive-claude-plugins' pendingyou hooks are ${builtins.toJSON market.hooks}, not ${builtins.toJSON expected}"
    ++ t.expect (
      market.pendingyou.hooksFile.hooks == market.hooks
    ) "packages.${system}.boomdrive-claude-plugins' hooks.json holds other hooks than it gives"
    ++ t.expect (builtins.elem "skills/pendingyou/SKILL.md" market.pendingyou.files) "packages.${system}.boomdrive-claude-plugins' pendingyou plugin holds no skills/pendingyou/SKILL.md"
    ++ t.expect (has "${../packages/pendingyou-skill.md}" market.buildCommand) "packages.${system}.boomdrive-claude-plugins' skill is not nix/packages/pendingyou-skill.md"
  );

  "Pending You's hooks against its own" =
    each (
      system: packages:
      let
        own = ownHooks (builtins.readFile "${packages.pendingyou.src}/dist/claude.js");
      in
      t.expect (builtins.length own == 9)
        "the set's Pending You's dist/claude.js gives ${builtins.toJSON own}, not the nine hooks of HOOKS and PERMISSION_HOOKS"
      ++ hooksHeld system own packages.boomdrive-claude-plugins.hooks
    )
    ++
      failsNaming "a Pending You whose init gives a PreCompact hook running compact"
        (hooksHeld "x86_64-linux" (ownHooks ''
          export const HOOKS = {
              SessionStart: 'pickup',
          };
          export const PERMISSION_HOOKS = [
              { event: 'PreCompact', sub: 'compact' },
          ];
        '') (marketplaceOf "x86_64-linux").hooks)
        [
          "PreCompact"
          "compact"
        ];

  # The wake mod is Pending You's folder's (record 0044, point 5): the
  # plugin holds none, and its hooks name no module.
  "No wake mod in the plugin" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      mods = lib.filter (
        file: lib.hasSuffix "register.js" file || lib.hasSuffix "wake.js" file
      ) market.pendingyou.files;
    in
    t.expect (
      mods == [ ]
    ) "packages.${system}.boomdrive-claude-plugins' pendingyou plugin holds ${toString mods}"
    ++
      t.expect (!(market.pendingyou.hooksFile ? modules))
        "packages.${system}.boomdrive-claude-plugins' hooks.json names ${
          builtins.toJSON (market.pendingyou.hooksFile.modules or [ ])
        } as its modules"
    ++ t.expect (
      !has "${packages.pendingyou.mod}" market.buildCommand
    ) "packages.${system}.boomdrive-claude-plugins copies the set's Pending You's wake mod"
  );

  "No MCP server in the plugin" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
    in
    t.expect (
      !builtins.elem ".mcp.json" market.pendingyou.files
    ) "packages.${system}.boomdrive-claude-plugins' pendingyou plugin holds a .mcp.json"
    ++ t.expect (
      !(market.pendingyou.manifest ? mcpServers)
    ) "packages.${system}.boomdrive-claude-plugins' pendingyou plugin's manifest names an MCP server"
    ++ t.expect (
      !has ".mcp.json" market.buildCommand
    ) "packages.${system}.boomdrive-claude-plugins' build writes a .mcp.json"
  );

  "Plannotator's plugin from its command's source" = each (
    system: packages:
    let
      market = packages.boomdrive-claude-plugins;
      source = "${packages.plannotator.src}/apps/hook";
    in
    t.expect
      (
        builtins.unsafeDiscardStringContext market.sources.plannotator
        == builtins.unsafeDiscardStringContext source
      )
      "packages.${system}.boomdrive-claude-plugins' plannotator plugin is of ${market.sources.plannotator}, not ${source}"
    ++ t.expect (has "cp -RL ${source} \"$out/plugins/plannotator\"" market.buildCommand) "packages.${system}.boomdrive-claude-plugins does not copy ${source} whole"
  );

  "No link into the store" = each (
    system: packages:
    let
      inherit (packages.boomdrive-claude-plugins) buildCommand;
      copies = lib.filter (line: builtins.match " *cp .*" line != null) (
        lib.splitString "\n" buildCommand
      );
    in
    lib.concatMap (
      line:
      t.expect (builtins.match " *cp -R?L .*" line != null)
        "packages.${system}.boomdrive-claude-plugins copies without following links: ${builtins.unsafeDiscardStringContext line}"
    ) copies
    ++ t.expect (
      has "links=$(find \"$out\" -type l)" buildCommand && has "exit 1" buildCommand
    ) "packages.${system}.boomdrive-claude-plugins' build does not fail on a link"
  );

  "Annotate other than the set's plannotator-tui" =
    failsNaming "an Annotate pinning plannotator-tui 0.9.8 beside a set's 0.9.9"
      (annotateHeld "x86_64-linux" "0.9.8" "0.9.9")
      [
        "0.9.8"
        "0.9.9"
      ];
}
