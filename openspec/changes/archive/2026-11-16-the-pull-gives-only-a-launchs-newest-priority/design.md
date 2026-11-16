# Design

## Context

boomdrive-design ruled on 2026-11-16 that crew's pull gives a priority only while it is its launch's newest priority, or while the runner already holds it, as it gives a preview request (record 0031 as amended on 2026-11-09). Record 0002's amendment of 2026-11-16 holds the ruling. crew's implementation decides which requests are current in `Pulling::current`, and `Pulling::preview_current` with `Main::newest_preview` is the model.

## Decisions

### Decision 1: Newest by the launch its `Of` names, whatever `Host` it names

A priority is moot where a request of a higher number that reads as a priority names the same `Of`. Its `Host` and `Work` do not count. The priority set last is the launch's, wherever crew addressed it. A priority naming another host is refused by that host's runner, which holds no place for it, as before. Not taken: keying on `Host` and `Of`, as previews key on `Host`, `Repo` and `Revision`. A launch is one place on one host, so a second key would keep a moot priority given wherever crew addressed two hosts.

### Decision 2: Only a priority that reads counts as newer

A later request counts only where it reads as a priority by `want`'s rules: a whole `Of` and a whole `Priority`. One that cannot be read sets no priority, so it hides no earlier one. It is given, or left out, by the rules for a request that cannot be read. `newest_preview` likewise counts only a later request that reads as a preview. Not taken: any later `Act: priority` naming the `Of`. That would leave a launch with no priority given at all while crew's newest is unreadable.

### Decision 3: The rule only takes requests away, ahead of the others

A moot priority the runner does not hold is not current. Every other priority is judged as before: given while its work is in the plan or the runner holds it, and, naming no host, only to the runner that holds its launch. A priority the runner holds stays given, so the runner answers it from its report (core.10, core.50), and the report's let-go treats it as before. The pull gives no new want, and the runner's order of wants is unchanged.

## Risks

- A priority request is compared with every later request, as a preview request is. A partition of some 1,100 requests, as `boomdrive` had on 2026-11-16, takes well under a pull's git reads. `a_launch_reordered_many_times_is_given_its_newest_priority_alone` reads 389 priorities of one launch.
