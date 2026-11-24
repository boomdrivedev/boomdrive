# Design

## Decision 1: flume host keeps the host's watch, and the runner keeps none

flume's decision `flume-host-keeps-watch-and-hears-the-merge-queue-by-one-subscription` makes `flume host` the one caller that keeps watch over the host. It settles at each change it hears from sluice host, and one minute after its last settle. Each settle writes the host's watch with a bound of one minute. A settle of the runner's beside it adds nothing but a second flume process each pass, and the sluice processes that process starts. So the runner runs no `flume settle` (`one-resident-per-tool`; `reads-go-through-the-resident`).

No transition code: a host whose Flume is older than v0.35.0 has no resident, and its places settle only when an agent goes quiet or a caller runs a command, until the host takes the set at v0.35.0 or later.

## Decision 2: The ticks stay at the end of each pass

millpond has no resident. Its tick of each inbox stays where it was: at the end of each pass, keeping or control, run once and awaited on that pass's thread, and shared between passes by one lock, so no two ticks of one inbox run at once. A keeping pass that each part's first control pass follows at once shares their ticks, as it shared their settle. The watch that ran the settle and the ticks becomes `Ticker`, which runs the ticks alone.

## Decision 3: The set's Flume at v0.35.0

The flake's `flume` input moves from `v0.31.2` to `v0.35.0`, and the lock moves that node alone (core.45). Flume's own module declares `flume-host` wherever it installs a package, so the set's module changes nothing to run it (core.46). Its subscription asks sluice host's socket, which the set's sluice, v0.105.1, serves. Flume's tree at v0.35.0 locks the same Rex as at v0.31.2.

## Decision 4: The tests count passes without a settle

The crate's tests waited for the stub `flume`'s settles to know that a pass had ended. They now wait for a sign of the pass itself: a keeping pass's fetch, as the stub git records it, or a control pass's `flume --json status`, or a tick of a stub `millpond` where the case names an inbox. Each case that asserted a settle asserts that the stub `flume` was asked for no settle.

## Decision 5: core.13 and the rules that name the settle

core.13 is amended: the runner's only housekeeping is millpond's tick at the end of each pass, and Flume's watch is `flume host`'s, which the host's service manager runs. core.1's, core.49's and core.50's details lose the settle. The decision goes to Boom as `the-runner-leaves-flumes-watch-to-flume-host`, which changes core.13's words; no record goes in `docs/adr` (`new-design-goes-into-boom-not-blueprints`).
