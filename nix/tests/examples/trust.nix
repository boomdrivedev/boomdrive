# README's **Installing with nix**: a host's nix settings that trust the
# family's cache. README shows its own copy, which its writers keep to
# this file (record 0049). cache.nix holds them to the flake's nixConfig.
{
  nix.settings.extra-substituters = [ "https://boomdrive.cachix.org" ];
  nix.settings.extra-trusted-public-keys = [
    "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag="
  ];
}
