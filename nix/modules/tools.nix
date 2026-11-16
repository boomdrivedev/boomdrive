# What the Boomdrive module sets of Flume's, sluice's, crew's and
# timber's own modules, which each tool's flake gives and flake.nix
# imports beside the runner's (the change
# the-flake-configures-each-tool-by-its-own-module, design Decision 5;
# record 0023 and its amendment of 2026-11-15, core.18, core.46). Each
# tool's package is the set's for the host's system by default, whether or
# not the runner is enabled, so it is the path the family's cache holds
# (core.47); a host's own, or `null` where the tool's module takes one,
# wins. With the runner enabled, Flume's, sluice's and timber's modules
# are turned on, since the runner runs Flume and sluice, and timber comes
# with the runner whether or not crew is on, a host's `enable = false`
# winning; crew's is the host's to turn on, with its `host`, its drives
# and its hosts. nixos.nix gives each tool the runner's user. Nothing
# else: no key of a tool's `settings`, crew's `hosts`, `runs` and `review`
# and every binding of timber's among them, and no file, link or
# activation of theirs, which each tool's module writes from its own
# `settings` (the change the-set-takes-timbers-own-module, design
# Decisions 2, 3 and 5).
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
      services = {
        flume.package = lib.mkDefault set.flume;
        sluice.package = lib.mkDefault set.sluice;
        crew.package = lib.mkDefault set.crew;
        timber.package = lib.mkDefault set.timber;
      };
    }
    (lib.mkIf config.services.boomdrive.enable {
      services = {
        flume.enable = lib.mkDefault true;
        sluice.enable = lib.mkDefault true;
        timber.enable = lib.mkDefault true;
      };
    })
  ];
}
