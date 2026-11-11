## ADDED Requirements

### Requirement: A host's launcher is the flake's own by its override

The launcher the modules make for a host's drives SHALL be the flake's own `boomdrive-launcher` by its override, made with the flake's locked nixpkgs whatever the host's `pkgs`, so its compiled menu is the flake's own launcher's path, which the cache holds.

#### Scenario: A host's drives in the launcher

- **WHEN** a module is enabled, with crew turned on, in a system whose `pkgs` carries an overlay of the host's own, and the system has the drive `home`
- **THEN** the installed launcher is the flake's own `boomdrive-launcher` made for `home` with the session `home`, and on nix-darwin its compiled menu is the flake's own launcher's
