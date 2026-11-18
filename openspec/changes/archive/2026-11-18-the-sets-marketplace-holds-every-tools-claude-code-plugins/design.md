# Design

## Context

Record 0044's amendment of 2026-11-18 has the set's marketplace list each Claude Code plugin a tool of the set ships, from the same pin as the tool and at the set's version of it, each taken from the tool's own flake where it builds the plugin and otherwise copied whole from the source the set builds the tool from. At the tags the lock names, the tools ship these plugins:

| tool | plugin | folder | manifest's version | built by its flake |
|---|---|---|---|---|
| Flume v0.29.0 | `flume` | `plugins/claude` | 0.29.0 | no |
| sluice v0.94.8 | `sluice` | `plugins/sluice` | 0.94.8 | no |
| sluice v0.94.8 | `sluice-board` | `plugins/sluice-board` | 0.94.8 | no |
| crew v0.64.2 | `crew` | `plugin` | 0.64.2 | yes, the package's `plugin` |
| worktrunk v0.80.0 | `worktrunk` | `plugins/worktrunk` | none | no |

herdr, Ghostty and portless ship none. No folder holds a link. The rules this change relies on are core.44 and core.45 (each tool at its tag, its plugins from the same pin), core.47 (the marketplace is one of the flake's derivations the cache holds) and core.7 (each check a task of the `nix` project).

## Decisions

### Decision 1: crew's plugin is its package's `plugin`

crew's flake builds its package from its tree's `plugin/`, copied to `$out/plugin` with `release.json` naming the commit and version, which crew reads where it runs from a build. That is the tool's own flake building the plugin, so the marketplace copies `${crew}/plugin` whole. The flake's test reads the manifest from crew's input, whose `plugin/` the package copies unchanged but for `release.json`, since the test evaluates against no store and builds nothing; the build reads the copy's.

Not taken: crew's source's `plugin/`, which lacks the commit the build writes.

### Decision 2: worktrunk's plugin is of its tree at its tag

The set's worktrunk is worktrunk's own flake's package, whose source is a fileset of what its build compiles and holds no `plugins/`. So the marketplace copies `plugins/worktrunk` of the flake's `worktrunk` input, the tree at the tag `flake.nix` names, which the package is built from.

Not taken: worktrunk's package's `src`, which holds no plugin.

### Decision 3: a manifest that names no version takes the marketplace's

worktrunk's manifest names no version, and Claude Code reads a plugin's version from its marketplace entry where its manifest names none. Each plugin the marketplace copies is listed at its tool's version, and each manifest that names a version is held to it, by the flake's test and by the build, naming the plugin and both versions. A plugin whose manifest names none is marked so, and passes while it names none; every other plugin fails where its manifest names none.

Not taken: writing the version into worktrunk's copied manifest, which would change a file of a plugin copied whole.

### Decision 4: one list of copied plugins

Plannotator's, timber's and the five new plugins are one list in `claude-plugins.nix`, each with the folder it is copied from and its tool, and one copy and check for each. Plannotator's and timber's copies are as before, so their cases hold unchanged.

## Risks and trade-offs

The marketplace now depends on crew's package, a copy of files with no compile, which the family's cache holds. A tool that renames its plugin's folder at a later tag fails the build of the set's marketplace at the change that takes that tag.
