# The service-modules spec's file (design Decision 9), over options.nix
# evaluated alone: each of the spec's settings renders the text of its file
# under nix/tests/files, byte for byte.
t:
let
  inherit (t) lib;
  render = import ../modules/toml.nix { inherit lib; };
  settings = import ./settings.nix;
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;

  # The settings options.nix gives of `given`.
  evaluated =
    given:
    (lib.evalModules {
      modules = [
        ../modules/options.nix
        {
          _module.args = { inherit pkgs; };
          services.boomdrive.settings = given;
        }
      ];
    }).config.services.boomdrive.settings;

  # The text of `name`'s settings, held to its file.
  text = name: render (evaluated settings.${name});
  rendersAs =
    name:
    t.expect (
      text name == builtins.readFile ./files/${name}.toml
    ) "${name}'s settings render ${builtins.toJSON (text name)}, not nix/tests/files/${name}.toml";
in
{
  "Only repositories" =
    rendersAs "empty"
    ++ t.expect (
      lib.tail (lib.splitString "\n" (text "empty")) == [
        "repositories = []"
        ""
      ]
    ) "the empty settings render more than `repositories = []` after the header";

  "A host under crew" = rendersAs "crew";

  "A host under a hosted control plane" = rendersAs "hosted";
}
