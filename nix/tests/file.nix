# The service-modules spec's file (design Decisions 9 and 12), over
# options.nix evaluated alone: each of the spec's settings renders the text
# of its file under nix/tests/files, byte for byte; a key named for a
# credential fails evaluation, naming its path; and a flat `repositories`
# of before, or a key in a drive the runner does not read, fails it too
# (record 0028).
t:
let
  inherit (t) lib;
  render = import ../modules/toml.nix { inherit lib; };
  settings = import ./settings.nix;
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;

  # options.nix enabled with `given`, beside a system's `assertions`.
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
          services.boomdrive = {
            enable = true;
            settings = given;
          };
        }
      ];
    }).config;

  # The messages of the assertions `given` fails.
  failures = given: map (a: a.message) (lib.filter (a: !a.assertion) (evaluated given).assertions);

  # The text `given` renders.
  rendered = given: render (evaluated given).services.boomdrive.settings;

  # The text of `name`'s settings, held to its file.
  text = name: rendered settings.${name};
  rendersAs =
    name:
    t.expect (
      text name == builtins.readFile ./files/${name}.toml
    ) "${name}'s settings render ${builtins.toJSON (text name)}, not nix/tests/files/${name}.toml"
    ++
      t.expect (failures settings.${name} == [ ])
        "${name}'s settings fail: ${toString (failures settings.${name})}";

  # The hosted settings with a token in their table, and with credentials
  # deeper and by their suffixes too.
  token = lib.recursiveUpdate settings.hosted { control_plane.hosted.token = "stand-in"; };
  deeper = lib.recursiveUpdate token {
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
  says = given: words: lib.hasInfix words (toString (failures given));

  # The flat list of before, and a stream in a drive.
  flat.repositories = [ "agentplot/flume" ];
  stream.accounts.own.drives.home.stream = "home";
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
    ) "the empty settings render more than an empty `[accounts]` after the header";

  "The flat list in settings" =
    t.expect (failures flat != [ ]) "settings.repositories evaluates"
    ++
      lib.concatMap
        (words: t.expect (says flat words) "the failure names no ${words}: ${toString (failures flat)}")
        [
          "settings.repositories"
          "settings.accounts.<account>.drives.<drive>.repositories"
        ];

  "A stream in a drive's settings" = t.expect (
    !(builtins.tryEval (rendered stream)).success
  ) "settings.accounts.own.drives.home.stream renders";

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
