# crew, crew's own flake's package (the change
# the-flake-configures-each-tool-by-its-own-module, design Decision 3).
# crew keeps its version in no file, and its flake versions its package
# `0-unstable-<date>`; so the package is given the tag flake.nix names,
# without its v, as sluice.nix gives sluice its tag (core.44). Nothing
# else of it changes: its source, name and home page are crew's own.
{
  lib,
  package,
  tag,
}:
package.overrideAttrs {
  # The source is the tag's already: only its version is given.
  __intentionallyOverridingVersion = true;
  version = lib.removePrefix "v" tag;
}
