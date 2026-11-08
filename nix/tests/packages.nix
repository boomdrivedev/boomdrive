# The family-packages spec's cases on the flake's package set (design
# Decisions 2 and 3), for each system the flake gives.
t:
let
  inherit (t)
    flake
    inputs
    lib
    outputs
    self
    systems
    ;
  gitCrates = import ./git-crates.nix { inherit lib; };
  tags = import ./tags.nix { inherit lib; };
  runnerCrates = import ../packages/git-crates.nix;
  runnerLock = builtins.fromTOML (builtins.readFile ../../Cargo.lock);

  # A check's failures over each system's packages of a set of outputs.
  eachOf = given: check: lib.concatMap (system: check system given.packages.${system}) systems;
  each = eachOf outputs;

  # A tree whose Cargo.toml says 9.9.9, as an input not a flake.
  tree = {
    outPath = toString ./tree-9.9.9;
  };

  # The runner's source: this tree's top-level entries, and every file
  # under crates/, each as the source's filter takes it or leaves it.
  entries = builtins.readDir ../..;
  held = [
    "Cargo.lock"
    "Cargo.toml"
    "crates"
  ];
  crateFiles = map toString (lib.filesystem.listFilesRecursive ../../crates);

  # A tool's plugin, held to the lock's tag of the tool.
  pluginOf =
    tool: system: packages:
    let
      plugin = packages."${tool}-herdr-plugin";
      source = "${inputs.${tool}.outPath}/plugins/herdr";
      version = t.lockedVersion tool;
    in
    t.expect (
      toString plugin.src == source
    ) "packages.${system}.${tool}-herdr-plugin is ${toString plugin.src}, not ${source}"
    ++ t.expect (
      plugin.version == version
    ) "packages.${system}.${tool}-herdr-plugin says ${plugin.version}, not ${tool}'s ${version}"
    ++ t.expect (plugin.dontFixup or false
    ) "packages.${system}.${tool}-herdr-plugin rewrites the plugin's files";
in
{
  "The runner from this tree" = each (
    system: packages:
    let
      inherit (packages.boomdrive) src;
      name = "packages.${system}.boomdrive";
    in
    t.expect (
      toString src.origSrc == self.outPath
    ) "${name}'s source is of ${toString src.origSrc}, not this tree"
    ++ lib.concatLists (
      lib.mapAttrsToList (
        entry: type:
        let
          takes = src.filter "${self.outPath}/${entry}" type;
          holds = builtins.elem entry held;
        in
        t.expect (takes == holds) "${name}'s source ${if takes then "takes" else "leaves out"} ${entry}"
      ) entries
    )
    ++ lib.concatMap (
      file: t.expect (src.filter file "regular") "${name}'s source leaves out ${file}"
    ) crateFiles
  );

  "The runner's git crates" = gitCrates "nix/packages/git-crates.nix" runnerLock runnerCrates;

  "weir moved without its hash" =
    let
      moved = "1111111111111111111111111111111111111111";
      lock = runnerLock // {
        package = map (
          package:
          if lib.hasPrefix "git+https://github.com/agentplot/weir" (package.source or "") then
            package // { source = "git+https://github.com/agentplot/weir?rev=${moved}#${moved}"; }
          else
            package
        ) runnerLock.package;
      };
      found = gitCrates "nix/packages/git-crates.nix" lock runnerCrates;
      named =
        crate:
        builtins.any (
          line:
          lib.hasPrefix "${crate}: " line
          && lib.hasInfix moved line
          && lib.hasInfix runnerCrates.${crate}.rev line
        ) found;
    in
    t.expect (
      builtins.length found == builtins.length (builtins.attrNames runnerCrates)
    ) "a Cargo.lock that moves weir to ${moved} gives ${builtins.toJSON found}"
    ++ lib.concatMap (
      crate:
      t.expect (named crate) "a Cargo.lock that moves weir fails naming neither ${crate} nor both revisions"
    ) (builtins.attrNames runnerCrates);

  # Flume's flake builds Flume's tagged tree, which carries its version
  # (design Decision 3): the set's Flume is that package, unchanged.
  "Flume through its own flake" = each (
    system: packages:
    let
      inherit (packages) flume;
      own = inputs.flume.packages.${system}.flume;
      version = t.lockedVersion "flume";
    in
    t.expect (
      flume.drvPath == own.drvPath
    ) "packages.${system}.flume is not Flume's flake's packages.${system}.flume"
    ++ t.expect (
      flume.version == version
    ) "packages.${system}.flume says ${flume.version}, not its tag's ${version}"
  );

  "sluice through its own flake" = each (
    system: packages:
    let
      inherit (packages) sluice;
      own = inputs.sluice.packages.${system}.default;
      version = t.lockedVersion "sluice";
      given = sluice.drvAttrs.SLUICE_VERSION or "nothing";
    in
    t.expect (
      toString sluice.src == toString own.src
      && sluice.pname == own.pname
      && sluice.cargoBuildFlags == own.cargoBuildFlags
      && sluice.meta.homepage == own.meta.homepage
    ) "packages.${system}.sluice is not sluice's flake's package"
    ++ t.expect (
      sluice.version == version
    ) "packages.${system}.sluice says ${sluice.version}, not its tag's ${version}"
    ++ t.expect (
      given == version
    ) "packages.${system}.sluice's build is given SLUICE_VERSION ${given}, not its tag's ${version}"
  );

  # crew keeps its version in no file, so the set gives crew's flake's
  # package its tag's (design Decision 3).
  "crew through its own flake" = each (
    system: packages:
    let
      inherit (packages) crew;
      own = inputs.crew.packages.${system}.crew;
      version = t.lockedVersion "crew";
    in
    t.expect (
      toString crew.src == toString own.src
      && crew.pname == own.pname
      && crew.meta.homepage == own.meta.homepage
    ) "packages.${system}.crew is not crew's flake's package"
    ++ t.expect (
      crew.version == version
    ) "packages.${system}.crew says ${crew.version}, not its tag's ${version}"
  );

  # Where sluice's tree carries a version of its own, the package is
  # sluice's flake's, unchanged (design Decision 3).
  "sluice's version from its tree" = lib.concatMap (
    system:
    let
      own = inputs.sluice.packages.${system}.default;
      given = import ../packages/sluice.nix {
        inherit lib;
        package = own;
        src = tree;
        tag = "v0.68.0";
      };
    in
    t.expect (
      given.version == own.version
      && (given.drvAttrs.SLUICE_VERSION or null) == (own.drvAttrs.SLUICE_VERSION or null)
    ) "sluice of a tree that says 9.9.9 is given ${given.version}, not left as its flake's"
  ) systems;

  "Flume's plugin moves with Flume" = each (pluginOf "flume");
  "sluice's plugin moves with sluice" = each (pluginOf "sluice");

  "A tag moved without the lock" =
    let
      inherit (t) lock;
      named = flake.inputs // {
        sluice = flake.inputs.sluice // {
          url = "github:agentplot/sluice/v0.68.0";
        };
      };
      locked = lock // {
        nodes = lock.nodes // {
          sluice = lock.nodes.sluice // {
            original = lock.nodes.sluice.original // {
              ref = "v0.67.0";
            };
          };
        };
      };
      found = tags named locked;
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "sluice"
        "v0.68.0"
        "v0.67.0"
      ]
    ) "flake.nix's sluice v0.68.0 under a lock of v0.67.0 gives ${builtins.toJSON found}";
}
