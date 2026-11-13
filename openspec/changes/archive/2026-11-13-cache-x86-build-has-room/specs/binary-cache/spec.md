## ADDED Requirements

### Requirement: The x86_64-linux job has room for the set

Before Nix is installed, the workflow's `x86_64-linux` job SHALL remove from its hosted runner the preinstalled trees that no package's build uses and dpkg does not own, the Android SDK, .NET, GHC, Swift, Rust's own and the tool cache among them, so the store and the builds have room for the set, and SHALL say the free space of `/` before and after. No other system's job SHALL remove anything.

#### Scenario: The x86_64-linux job has room

- **WHEN** the workflow is read
- **THEN** its first step, run only where the system is `x86_64-linux`, removes `/usr/local/lib/android`, `/usr/share/dotnet`, `/opt/ghc`, `/usr/local/.ghcup`, `/usr/share/swift`, `~/.cargo`, `~/.rustup` and `/opt/hostedtoolcache` and says the free space of `/` before and after, before Nix is installed, and no other step has a condition
