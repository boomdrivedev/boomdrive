## MODIFIED Requirements

### Requirement: A host substitutes the set, or builds the same paths

The modules' default packages SHALL be the flake's own, made with its locked nixpkgs whatever the host's `pkgs`, so they are the store paths the cache holds: the runner's, each tool's module's, and herdr's, Ghostty's and the launcher's. The launcher made for a host's drives SHALL be the flake's own launcher by its override, made with the flake's locked nixpkgs, whose compiled menu is the flake's own launcher's path. A host whose nix trusts the cache SHALL substitute them, and one that does not SHALL build the same paths from the source the lock names. The modules SHALL add no substituter or key to a host's nix settings.

#### Scenario: A host's own nixpkgs

- **WHEN** a module is enabled, with crew turned on, in a system whose `pkgs` carries an overlay of the host's own
- **THEN** the system profile holds the flake's own `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `ghostty` and `boomdrive-launcher`, the store paths the flake gives

#### Scenario: A host's drives in the launcher

- **WHEN** the same system has the drive `home`
- **THEN** the installed launcher is the flake's own `boomdrive-launcher` made for `home` with the session `home`, and on nix-darwin its compiled menu is the flake's own launcher's

#### Scenario: The host's trust

- **WHEN** a module is enabled
- **THEN** the system's nix substituters and trusted public keys name nothing of the cache's
