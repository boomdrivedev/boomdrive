# The service-modules spec's file (design Decisions 9 and 12; the change
# each-repository-is-named-once-by-its-owner, design Decisions 1 and 2),
# over options.nix evaluated alone: each of the spec's values renders the
# text of its file under nix/tests/files, byte for byte, through
# accounts.nix's `fileOf`; each drive's owners are turned into addresses; a
# key named for a credential fails evaluation, naming its path; and a flat
# `repositories` of before, the accounts in `settings`, a key in a drive
# the module does not declare, an owner and a name that make no address, a
# drive's name under two accounts, or an address kept twice, fails it too
# (record 0028).
t:
let
  inherit (t) lib;
  render = import ../modules/toml.nix { inherit lib; };
  inherit (import ../modules/accounts.nix { inherit lib; }) fileOf;
  settings = import ./settings.nix;
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;

  # options.nix enabled with services.boomdrive `given`, beside a
  # system's `assertions`.
  evaluated =
    given:
    (lib.evalModules {
      modules = [
        (import ../modules/options.nix { inherit (t.outputs) packages; })
        {
          options.assertions = lib.mkOption {
            type = lib.types.listOf lib.types.unspecified;
            default = [ ];
          };
        }
        {
          _module.args = { inherit pkgs; };
          services.boomdrive.enable = true;
        }
        { services.boomdrive = given; }
      ];
    }).config;

  # The messages of the assertions `given` fails.
  failures = given: map (a: a.message) (lib.filter (a: !a.assertion) (evaluated given).assertions);

  # The text `given` renders.
  rendered = given: render (fileOf (evaluated given).services.boomdrive);

  # The text of `name`'s value, held to its file.
  text = name: rendered settings.${name};
  rendersAs =
    name:
    t.expect (
      text name == builtins.readFile ./files/${name}.toml
    ) "${name}'s value renders ${builtins.toJSON (text name)}, not nix/tests/files/${name}.toml"
    ++
      t.expect (failures settings.${name} == [ ])
        "${name}'s value fails: ${toString (failures settings.${name})}";

  # The hosted value with a token in its table, and with credentials
  # deeper and by their suffixes too.
  token = lib.recursiveUpdate settings.hosted { settings.control_plane.hosted.token = "stand-in"; };
  deeper = lib.recursiveUpdate token {
    settings = {
      monkey = "not a credential";
      control_plane.hosted.partitions = [
        { label = "home"; }
        {
          label = "work";
          private_key = "stand-in";
        }
      ];
      control_plane.crew.state_password = "stand-in";
    };
  };
  says = given: words: lib.hasInfix words (toString (failures given));

  # The flat list of before, and a stream in a drive of the accounts in
  # `settings`.
  flat.settings.repositories = [ "agentplot/flume" ];
  stream.settings.accounts.own.drives.home.stream = "home";

  # A key a drive does not hold, and an owner that makes no address.
  pool.accounts.own.drives.home.pool = "home";
  inner.accounts.own.drives.home.repositories."example/inner" = [ "alpha" ];

  # A drive's name under two accounts, and a repository in two drives and
  # twice in one.
  drivesTwice.accounts = {
    own.drives.home = { };
    work.drives.home = { };
  };
  inTwo.accounts.own.drives = {
    home.repositories.agentplot = [ "flume" ];
    garden.repositories.agentplot = [ "flume" ];
  };
  inOne.accounts.own.drives.home.repositories.agentplot = [
    "flume"
    "flume"
  ];
  # A failure of `given` names every word of `words`.
  namesAll =
    given: words: lib.any (message: lib.all (word: lib.hasInfix word message) words) (failures given);

  # Two owners, the second's names out of their order.
  owners.accounts.own.drives.home.repositories = {
    example = [
      "beta"
      "alpha"
    ];
    agentplot = [ "flume" ];
  };
  ownersLines = lib.splitString "\n" (rendered owners);
  ownersAt = lib.lists.findFirstIndex (line: line == "[accounts.own.drives.home]") null ownersLines;
  ownersDrive = if ownersAt == null then [ ] else lib.drop (ownersAt + 1) ownersLines;
in
{
  "Only accounts" =
    rendersAs "empty"
    ++ t.expect (
      lib.tail (lib.splitString "\n" (text "empty")) == [
        ""
        "[accounts]"
        ""
      ]
    ) "the empty value renders more than an empty `[accounts]` after the header";

  "Owners turned into addresses" =
    t.expect (failures owners == [ ]) "the owners' drive fails: ${toString (failures owners)}"
    ++ t.expect (
      ownersDrive != [ ]
      && lib.head ownersDrive == ''repositories = ["agentplot/flume", "example/beta", "example/alpha"]''
    ) "the owners' drive renders ${builtins.toJSON (rendered owners)}";

  "The flat list in settings" =
    t.expect (failures flat != [ ]) "settings.repositories evaluates"
    ++
      lib.concatMap
        (words: t.expect (says flat words) "the failure names no ${words}: ${toString (failures flat)}")
        [
          "settings.repositories"
          "services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>"
        ];

  "A stream in a drive's settings" =
    t.expect (failures stream != [ ]) "settings.accounts.own.drives.home.stream evaluates"
    ++
      lib.concatMap
        (words: t.expect (says stream words) "the failure names no ${words}: ${toString (failures stream)}")
        [
          "settings.accounts"
          "services.boomdrive.accounts"
        ];

  "A key a drive does not hold" = t.expect (
    !(builtins.tryEval (rendered pool)).success
  ) "accounts.own.drives.home.pool renders";

  "An address of no repository" =
    t.expect (failures inner != [ ]) "the owner example/inner evaluates"
    ++ t.expect (says inner "example/inner/alpha") "the failure names no example/inner/alpha: ${toString (failures inner)}";

  "A drive's name under two accounts" =
    t.expect
      (namesAll drivesTwice [
        "drive home"
        "own"
        "work"
      ])
      "no failure names the drive home and the accounts own and work: ${toString (failures drivesTwice)}";

  "A repository in two drives" =
    t.expect (namesAll inTwo [
      "agentplot/flume"
      "home"
      "garden"
    ]) "no failure names agentplot/flume and the drives home and garden: ${toString (failures inTwo)}"
    ++ t.expect (namesAll inOne [
      "agentplot/flume"
      "drives home and home"
    ]) "no failure names agentplot/flume kept twice by the drive home: ${toString (failures inOne)}";

  "A host under crew" = rendersAs "crew";

  "A host under a hosted control plane" = rendersAs "hosted";

  "A token in the hosted table" =
    t.expect (failures token != [ ]) "settings.control_plane.hosted.token evaluates"
    ++ t.expect (says token "control_plane.hosted.token") "the failure names no control_plane.hosted.token: ${toString (failures token)}"
    ++ t.expect (says token "boomdrive enroll") "the failure does not say `boomdrive enroll` makes a partition's key"
    ++
      lib.concatMap
        (path: t.expect (says deeper path) "the failure of deeper credentials names no ${path}")
        [
          "control_plane.hosted.token"
          "control_plane.hosted.partitions[1].private_key"
          "control_plane.crew.state_password"
        ]
    ++ t.expect (
      !says deeper "monkey"
    ) "the failure names `monkey`, which ends in no credential's word";
}
