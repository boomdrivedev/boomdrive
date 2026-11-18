## MODIFIED Requirements

### Requirement: NixOS's drive file gives the drive to the runner, crew and Flume

The NixOS drive step SHALL give `/etc/nixos/boomdrive.nix`, a module that trusts the family's cache as the flake's `nixConfig` names it and installs `git`, `gh` and `nodejs`. It SHALL set the account `me`, naming the state repository `your-github-name/drive-state`, whose drive `home` keeps `your-github-name/your-repository` under crew, with crew's host `your-host`. It SHALL give crew's own keys for its drive `home`, and neither its machine nor its state, which the set gives from the account.

#### Scenario: The drive on NixOS

- **WHEN** `nix:test` evaluates a system of `x86_64-linux` with the user `you`, the set's NixOS module and `site/src/start/nixos/boomdrive.nix`
- **THEN** the runner's file names the account `me`'s drive `home` keeping `your-github-name/your-repository` under crew with host `your-host`, crew's drive `home` names `your-host` and `your-github-name/drive-state`, which the file names only as the account `me`'s `state_repository` and nowhere in crew's settings, Flume's host multiplexer is `herdr`, Flume is given the stream `home` with the repository's work in it and crew's source, and the cache and its key are the flake's `nixConfig`'s
