# Tasks

## 1. The control plane says whether the runner alone reads it

- [x] 1.1 `ControlPlane::read_through_the_runner` is in the interface. `Hosted` gives true, and `Crew` gives false (Decision 1). True when `devenv shell -- cargo test -p boomdrive-hosted --test plane` passes, proving "A hosted control plane is read through the runner", and `devenv shell -- cargo test -p boomdrive-crew --test shared` passes, proving "Crew's control plane is read on the host".

## 2. The runner names itself alone where no one else reads

- [x] 2.1 `flume::recipients` names the runner alone for a launch that names none, of a control plane read through the runner (Decision 2). True when `devenv shell -- cargo test -p boomdrive --lib flume::` passes, and `devenv shell -- cargo test -p boomdrive --test flume` passes, proving "A launch read through the runner alone" at the command line.
- [x] 2.2 A pass over the stub `flume` launches such an offer with `--recipient boomdrive` alone, and the place's question reaches the report that the control plane takes. True when `devenv shell -- cargo test -p boomdrive --test statuses` passes, proving "A launch read through the runner alone", and `devenv shell -- cargo test -p boomdrive --test offers` passes, proving "The runner is one more recipient".

## 3. The rule and the record

- [x] 3.1 Record 0030 has its amendment of 2026-11-26, with Before and Now. core.54 and the details of core.15 say the new case.

## 4. The gate

- [x] 4.1 The projects' tasks pass. True when `sluice exec main` passes.
