# What the Boomdrive module gives Flume's own module of crew's source of
# kinds (the change the-set-declares-crews-source-to-flume, design
# Decisions 1 to 4; record 0034, point 2; crew record 0057, point 1): with
# crew's module on, whether or not the runner is, the source `crew`,
# `services.flume.sources.crew`, naming the roles file crew writes,
# `~/.local/state/crew/flume/roles.toml`, and crew's kinds folder,
# `~/.local/share/flume/kinds/crew`, each under the home of the user Flume
# runs as, and no other key. Each key is given by `lib.mkDefault`, so a
# host's own value in services.flume.sources.crew wins, key by key; with
# crew's module off, no source of crew's is given. Nothing else: the module
# writes no file of Flume's, which Flume's own module renders from its
# `sources` and links at ~/.config/flume/sources/crew.toml (core.18,
# core.46), makes neither the roles file nor the kinds folder, which are
# crew's, and names no source in the runner's file (core.5).
{ config, lib, ... }:
{
  config = lib.mkIf config.services.crew.enable {
    services.flume.sources.crew = {
      kinds_dir = lib.mkDefault "~/.local/share/flume/kinds/crew";
      roles = lib.mkDefault "~/.local/state/crew/flume/roles.toml";
    };
  };
}
