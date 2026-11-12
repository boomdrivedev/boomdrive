# The set's herdr plugins in one package a Nix profile holds, each at
# share/herdr/plugins/<tool>, from which `boomdrive setup` links it with
# herdr's own `herdr plugin link` (record 0009 as amended 2026-11-11;
# design Decision 8 of the change a-persons-drive-is-set-up-by-one-command).
# Each plugin package is its tool's plugins/herdr at its top, and each has
# bin/open.sh there, so one profile cannot hold the two side by side.
{
  linkFarm,
  boomdrive,
  flume-herdr-plugin,
  sluice-herdr-plugin,
}:
(linkFarm "herdr-plugins" {
  "share/herdr/plugins/flume" = flume-herdr-plugin;
  "share/herdr/plugins/sluice" = sluice-herdr-plugin;
}).overrideAttrs
  (old: {
    inherit (boomdrive) version;
    meta = (old.meta or { }) // {
      description = "The Boomdrive set's herdr plugins, Flume's and sluice's, in one package";
      inherit (boomdrive.meta) homepage;
    };
  })
