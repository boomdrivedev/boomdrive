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
  # The entries leave out those .gitignore names at the top of the tree:
  # git tracks none of them, so the flake's own tree never holds one, and
  # a task beside the check may make one while it runs, as cargo makes
  # target/. The filter reads the root apart from this listing, and
  # takes an entry its read did not see, so such an entry made between
  # the two reads failed the case.
  ignored = lib.pipe (builtins.readFile ../../.gitignore) [
    (lib.splitString "\n")
    (lib.filter (line: line != "" && !lib.hasPrefix "#" line))
    (map (line: lib.removeSuffix "/" (lib.removePrefix "/" line)))
    (lib.filter (name: !lib.hasInfix "/" name))
  ];
  entries = removeAttrs (builtins.readDir ../..) ignored;
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

  # The set's herdr against the herdr Flume's tree pins and each plugin's
  # floor (design Decision 1): `pinned` and `herdr` are versions, and
  # `plugins` maps a plugin's name to its manifest's `min_herdr_version`.
  herdrHeld =
    {
      pinned,
      herdr,
      plugins,
    }:
    t.expect (herdr == pinned)
      "the set's herdr is ${herdr}, and Flume's tree pins ${pinned}, the herdr its terminal profile was checked against"
    ++ lib.concatLists (
      lib.mapAttrsToList (
        name: floor:
        t.expect (
          !lib.versionOlder herdr floor
        ) "${name} asks for herdr ${floor}, and the set's herdr is ${herdr}"
      ) plugins
    );

  # The herdr Flume's devenv.nix pins at the tag the lock names for Flume,
  # `herdrVersion = "<v>";`, or null where no line names one.
  flumesDevenv = "${inputs.flume.outPath}/devenv.nix";
  flumesHerdr =
    let
      found = lib.concatMap (
        line:
        let
          matched = builtins.match ''[[:space:]]*herdrVersion = "([^"]+)";[[:space:]]*'' line;
        in
        lib.optional (matched != null) (builtins.head matched)
      ) (lib.splitString "\n" (builtins.readFile flumesDevenv));
    in
    if builtins.length found == 1 then builtins.head found else null;

  # The set's worktrunk against the worktrunk sluice's tree pins (design
  # Decision 1): `pinned` and `worktrunk` are versions.
  worktrunkHeld =
    { pinned, worktrunk }:
    t.expect (worktrunk == pinned)
      "the set's worktrunk is ${worktrunk}, and sluice's tree pins ${pinned}, the worktrunk its worktree driver was checked against";

  # The worktrunk sluice's devenv.nix pins at the tag the lock names for
  # sluice: the `version` of the one block naming worktrunk's repository,
  # read up to the block's first closing line, or null where there is not
  # exactly one such block naming exactly one version.
  sluicesDevenv = "${inputs.sluice.outPath}/devenv.nix";
  sluicesWorktrunk =
    let
      lines = lib.splitString "\n" (builtins.readFile sluicesDevenv);
      matching = pattern: line: builtins.match pattern line != null;
      repos = lib.concatLists (
        lib.imap0 (
          i: line: lib.optional (matching ''[[:space:]]*repo = "max-sixty/worktrunk";[[:space:]]*'' line) i
        ) lines
      );
      rest = lib.drop (builtins.head repos + 1) lines;
      block = lib.take (lib.lists.findFirstIndex (matching "[[:space:]]*};[[:space:]]*")
        (builtins.length rest)
        rest
      ) rest;
      versions = lib.concatMap (
        line:
        let
          matched = builtins.match ''[[:space:]]*version = "([^"]+)";[[:space:]]*'' line;
        in
        lib.optional (matched != null) (builtins.head matched)
      ) block;
    in
    if builtins.length repos == 1 && builtins.length versions == 1 then
      builtins.head versions
    else
      null;

  # A system's Ghostty against the release Flume's ghostty window profile
  # was checked against (Flume's statement, C.8; design Decision 2). Flume's
  # tree pins no Ghostty to read, so the constant moves only with a check
  # of Flume's profile.
  checked = "1.4.0";
  ghosttyHeld =
    system: package:
    t.expect (package.version == checked)
      "packages.${system}.ghostty is Ghostty ${package.version}, not ${checked}, the release Flume's window profile was checked against";

  # The set's portless against its tag (design Decision 1): `version` is
  # the package's, read from its tagged tree, and `locked` the lock's tag
  # without the v.
  portlessHeld =
    system: version: locked:
    t.expect (version == locked) "packages.${system}.portless says ${version}, not its tag's ${locked}";

  # Each system's portless.
  portlessOf = system: outputs.packages.${system}.portless;

  # Each of the set's herdr plugins' floors, from its manifest.
  floorsOf =
    packages:
    lib.genAttrs [ "flume-herdr-plugin" "sluice-herdr-plugin" ] (
      name:
      (builtins.fromTOML (builtins.readFile "${packages.${name}.src}/herdr-plugin.toml"))
      .min_herdr_version
    );
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

  "herdr through its own flake" = each (
    system: packages:
    let
      inherit (packages) herdr;
      own = inputs.herdr.packages.${system}.herdr;
      version = t.lockedVersion "herdr";
    in
    t.expect (
      herdr.drvPath == own.drvPath
    ) "packages.${system}.herdr is not herdr's flake's packages.${system}.herdr"
    ++ t.expect (
      herdr.version == version
    ) "packages.${system}.herdr says ${herdr.version}, not its tag's ${version}"
  );

  # worktrunk's flake gives its build its commit's short revision as its
  # git describe, and the set gives it its tag, as worktrunk's release
  # build gets it (design Decision 2).
  "worktrunk through its own flake" = each (
    system: packages:
    let
      inherit (packages) worktrunk;
      own = inputs.worktrunk.packages.${system}.worktrunk;
      version = t.lockedVersion "worktrunk";
      name = "packages.${system}.worktrunk";
      program = worktrunk.meta.mainProgram or "none";
      given = worktrunk.drvAttrs.VERGEN_GIT_DESCRIBE or "nothing";
    in
    t.expect (
      toString worktrunk.src == toString own.src
    ) "${name}'s source is ${toString worktrunk.src}, not worktrunk's flake's ${toString own.src}"
    ++
      t.expect (worktrunk.cargoArtifacts.drvPath == own.cargoArtifacts.drvPath)
        "${name}'s dependencies are ${worktrunk.cargoArtifacts.drvPath}, not worktrunk's flake's ${own.cargoArtifacts.drvPath}"
    ++ t.expect (program == "wt") "${name}'s program is ${program}, not wt"
    ++ t.expect (
      worktrunk.version == version
    ) "${name} says ${worktrunk.version}, not its tag's ${version}"
    ++ t.expect (
      given == "v${version}"
    ) "${name}'s build is given VERGEN_GIT_DESCRIBE ${given}, not its tag v${version}"
  );

  "Ghostty on each system" = each (
    system: packages:
    let
      nixpkgs = inputs.nixpkgs.legacyPackages.${system};
      own = if lib.hasSuffix "-darwin" system then nixpkgs.ghostty-bin else nixpkgs.ghostty;
      named = if lib.hasSuffix "-darwin" system then "ghostty-bin" else "ghostty";
    in
    t.expect (
      packages.ghostty.drvPath == own.drvPath
    ) "packages.${system}.ghostty is not the flake's nixpkgs' ${named}"
    ++ ghosttyHeld system packages.ghostty
  );

  "Ghostty moved with nixpkgs" =
    let
      found = ghosttyHeld "x86_64-linux" { version = "1.5.0"; };
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "x86_64-linux"
        "1.5.0"
        "1.4.0"
      ]
    ) "a Ghostty 1.5.0 gives ${builtins.toJSON found}";

  "herdr is Flume's" =
    if flumesHerdr == null then
      [
        "Flume's pin of herdr could not be read: ${flumesDevenv} holds no one line herdrVersion = \"<version>\";"
      ]
    else
      each (
        _: packages:
        herdrHeld {
          pinned = flumesHerdr;
          herdr = packages.herdr.version;
          plugins = floorsOf packages;
        }
      );

  "herdr other than Flume's" =
    let
      found = herdrHeld {
        pinned = "0.9.6";
        herdr = "0.9.5";
        plugins = { };
      };
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "0.9.6"
        "0.9.5"
      ]
    ) "a herdr 0.9.5 beside Flume's pin of 0.9.6 gives ${builtins.toJSON found}";

  "worktrunk is sluice's" =
    if sluicesWorktrunk == null then
      [
        "sluice's pin of worktrunk could not be read: ${sluicesDevenv} holds no one worktrunk block naming its version"
      ]
    else
      each (
        _: packages:
        worktrunkHeld {
          pinned = sluicesWorktrunk;
          worktrunk = packages.worktrunk.version;
        }
      );

  "worktrunk other than sluice's" =
    let
      found = worktrunkHeld {
        pinned = "0.81.0";
        worktrunk = "0.80.0";
      };
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "0.81.0"
        "0.80.0"
      ]
    ) "a worktrunk 0.80.0 beside sluice's pin of 0.81.0 gives ${builtins.toJSON found}";

  "A plugin newer than herdr" =
    let
      found = herdrHeld {
        pinned = "0.9.5";
        herdr = "0.9.5";
        plugins.flume-herdr-plugin = "0.9.9";
      };
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "flume-herdr-plugin"
        "0.9.9"
        "0.9.5"
      ]
    ) "a plugin asking for herdr 0.9.9 of a set's 0.9.5 gives ${builtins.toJSON found}";

  "portless from its source" = each (
    system: packages:
    let
      inherit (packages) portless;
      source = inputs.portless.outPath;
    in
    t.expect (
      toString portless.src == source
    ) "packages.${system}.portless is of ${toString portless.src}, not ${source}"
    ++
      t.expect ((portless.meta.mainProgram or null) == "portless")
        "packages.${system}.portless's program is ${
          toString (portless.meta.mainProgram or "none")
        }, not portless"
    ++ portlessHeld system portless.version (t.lockedVersion "portless")
  );

  "portless other than its tag" =
    let
      found = portlessHeld "x86_64-linux" "0.17.1" "0.17.0";
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "0.17.1"
        "0.17.0"
      ]
    ) "a portless 0.17.1 under the lock's tag v0.17.0 gives ${builtins.toJSON found}";

  # Under pnpm 11 the fetch holds every platform's optional packages, so
  # its one hash is every system's (design Decision 3).
  "One fetch for every system" =
    let
      fetches = map (system: (portlessOf system).pnpmDeps) systems;
      hashes = lib.unique (map (deps: deps.outputHash) fetches);
    in
    t.expect (
      builtins.length hashes == 1
    ) "portless's fetches have the hashes ${toString hashes}, not one for every system"
    ++ lib.concatMap (
      system:
      let
        deps = (portlessOf system).pnpmDeps;
        major = lib.versions.major deps.pnpm.version;
      in
      t.expect (deps.pnpmWorkspaces == [ "portless" ])
        "packages.${system}.portless fetches ${builtins.toJSON deps.pnpmWorkspaces}, not the package portless alone"
      ++ t.expect (
        major == "11"
      ) "packages.${system}.portless is fetched with pnpm ${deps.pnpm.version}, not pnpm 11"
    ) systems;

  "The flake's Node.js" = lib.concatMap (
    system:
    t.expect
      ((portlessOf system).node.drvPath == inputs.nixpkgs.legacyPackages.${system}.nodejs_24.drvPath)
      "packages.${system}.portless runs on ${(portlessOf system).node.name}, not the flake's nixpkgs' nodejs_24"
  ) systems;

  # git is the host's own (records 0012 and 0023): portless's PATH is
  # given the flake's openssl, procps, which and hostname alone.
  "portless runs the host's git" = lib.concatMap (
    system:
    let
      given = (portlessOf system).runtimeInputs;
      nixpkgs = inputs.nixpkgs.legacyPackages.${system};
      expected = [
        nixpkgs.openssl
        nixpkgs.procps
        nixpkgs.which
        nixpkgs.hostname
      ];
      names = map lib.getName given;
    in
    t.expect (map (p: p.drvPath) given == map (p: p.drvPath) expected)
      "packages.${system}.portless's PATH is given ${toString names}, not the flake's nixpkgs' openssl, procps, which and hostname"
    ++ t.expect (!builtins.elem "git" names) "packages.${system}.portless's PATH is given git"
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

  "A tool named by no release tag" =
    let
      inherit (t) lock;
      node = lock.nodes.${lock.root}.inputs.crew;
      named = flake.inputs // {
        crew = flake.inputs.crew // {
          url = "github:agentplot/crew/main";
        };
      };
      locked = lock // {
        nodes = lock.nodes // {
          ${node} = lock.nodes.${node} // {
            original = lock.nodes.${node}.original // {
              ref = "main";
            };
          };
        };
      };
      found = tags named locked;
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "crew"
        "main"
      ]
    ) "flake.nix and the lock naming crew by main give ${builtins.toJSON found}";

  "herdr named by no release tag" =
    let
      inherit (t) lock;
      node = lock.nodes.${lock.root}.inputs.herdr;
      named = flake.inputs // {
        herdr = flake.inputs.herdr // {
          url = "github:herdrdev/herdr/main";
        };
      };
      locked = lock // {
        nodes = lock.nodes // {
          ${node} = lock.nodes.${node} // {
            original = lock.nodes.${node}.original // {
              ref = "main";
            };
          };
        };
      };
      found = tags named locked;
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "herdr"
        "main"
      ]
    ) "flake.nix and the lock naming herdr by main give ${builtins.toJSON found}";

  "portless named by no release tag" =
    let
      inherit (t) lock;
      node = lock.nodes.${lock.root}.inputs.portless;
      named = flake.inputs // {
        portless = flake.inputs.portless // {
          url = "github:vercel-labs/portless/main";
        };
      };
      locked = lock // {
        nodes = lock.nodes // {
          ${node} = lock.nodes.${node} // {
            original = lock.nodes.${node}.original // {
              ref = "main";
            };
          };
        };
      };
      found = tags named locked;
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "portless"
        "main"
      ]
    ) "flake.nix and the lock naming portless by main give ${builtins.toJSON found}";

  "worktrunk named by no release tag" =
    let
      inherit (t) lock;
      node = lock.nodes.${lock.root}.inputs.worktrunk;
      named = flake.inputs // {
        worktrunk = flake.inputs.worktrunk // {
          url = "github:max-sixty/worktrunk/main";
        };
      };
      locked = lock // {
        nodes = lock.nodes // {
          ${node} = lock.nodes.${node} // {
            original = lock.nodes.${node}.original // {
              ref = "main";
            };
          };
        };
      };
      found = tags named locked;
    in
    t.expect (
      builtins.length found == 1
      && builtins.all (word: lib.hasInfix word (builtins.head found)) [
        "worktrunk"
        "main"
      ]
    ) "flake.nix and the lock naming worktrunk by main give ${builtins.toJSON found}";
}
