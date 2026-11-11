# The quick-start spec's evaluated cases (record 0043; site.8; design
# Decisions 4 and 8 of the change
# boomdrives-quick-start-takes-a-stranger-to-a-working-drive): each step's
# file read by its path under site/src/start, as the page shows it, and
# evaluated as it stands, never built.
t:
let
  inherit (t) lib;
  start = ../../site/src/start;

  # The address README's Installing with nix names, its first `github:`
  # in backticks, read as cache.nix reads README.
  address = lib.findFirst (lib.hasPrefix "github:") null (
    lib.splitString "`" (toString t.installing)
  );

  # A shell file's words, split at every run of white space.
  wordsOf = text: lib.filter (word: word != "") (lib.flatten (builtins.split "[[:space:]]+" text));

  # The Mac's install: one `nix profile add` of the set's six and
  # nixpkgs' three, in this order (design Decision 4).
  macInstall =
    let
      words = wordsOf (builtins.readFile (start + "/mac/install.sh"));
      installables = lib.drop 3 words;
      set = map (name: if name == null then address else "${address}#${name}") [
        null
        "flume"
        "sluice"
        "herdr"
        "herdr-plugins"
        "crew"
      ];
      nixpkgs = map (name: "nixpkgs#${name}") [
        "git"
        "gh"
        "nodejs"
      ];
      want = set ++ nixpkgs;
      packageOf =
        installable:
        let
          parts = lib.splitString "#" installable;
        in
        if lib.length parts == 1 then "default" else lib.elemAt parts 1;
      isPackage =
        installable:
        if lib.hasPrefix "nixpkgs#" installable then
          t.inputs.nixpkgs.legacyPackages.aarch64-darwin ? ${packageOf installable}
        else
          lib.hasPrefix "${address}" installable
          && t.outputs.packages.aarch64-darwin ? ${packageOf installable};
    in
    t.expect (address != null) "README's Installing with nix names no `github:` address"
    ++ t.expect (
      lib.take 3 words == [
        "nix"
        "profile"
        "add"
      ]
    ) "mac/install.sh is no `nix profile add`: ${toString words}"
    ++ t.expect (
      installables == want
    ) "mac/install.sh adds ${toString installables}, not ${toString want}"
    ++ lib.concatMap (
      installable:
      t.expect (isPackage installable) "mac/install.sh's ${installable} is no package of aarch64-darwin"
    ) installables;
in
{
  "quick start: the Mac's install" = macInstall;
}
