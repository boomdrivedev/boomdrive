# What the Boomdrive module sets of Flume's, sluice's and crew's own
# modules, which each tool's flake gives and flake.nix imports beside the
# runner's (the change the-flake-configures-each-tool-by-its-own-module,
# design Decision 5; record 0023, core.18, core.46). Each tool's package
# is the set's for the host's system by default, whether or not the runner
# is enabled, so it is the path the family's cache holds (core.47); a
# host's own, or `null` where the tool's module takes one, wins. With the
# runner enabled, Flume's and sluice's modules are turned on, since the
# runner runs both, a host's `enable = false` winning; crew's is the
# host's to turn on, with its `host` and partitions. nixos.nix gives each
# tool the runner's user. Nothing else: no key of a tool's `settings`, and
# no file, link or activation of theirs, which each tool's module writes
# from its own `settings`.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  set = import ./set.nix {
    inherit lib packages;
    inherit (pkgs.stdenv.hostPlatform) system;
  };
in
{
  config = lib.mkMerge [
    {
      services.flume.package = lib.mkDefault set.flume;
      services.sluice.package = lib.mkDefault set.sluice;
      services.crew.package = lib.mkDefault set.crew;
    }
    (lib.mkIf config.services.boomdrive.enable {
      services.flume.enable = lib.mkDefault true;
      services.sluice.enable = lib.mkDefault true;
    })
  ];
}
