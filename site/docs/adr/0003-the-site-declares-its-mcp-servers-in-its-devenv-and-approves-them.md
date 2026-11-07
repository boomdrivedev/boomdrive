# The site declares its MCP servers in its devenv and approves them

- Status: accepted for what agentplot/boomdrive's record 0022 decides, which
  the user accepted on 2026-11-07; the shape below is the code stage's own,
  in the chore boomdrive-web-devenv-declares-its-mcp-servers, on 2026-11-07
- Date: 2026-11-07
- Follows: agentplot/boomdrive `docs/adr/0022-each-repository-of-the-family-declares-its-own-mcp-servers-in-its-devenv-and-approves-them-for-its-trusted-sessions.md`

## Context and problem statement

Record 0022 in agentplot/boomdrive settles, in the user's words, "that, when
agent sessions run, the MCP servers start": each repository of the family
declares its servers in its devenv, `claude.code.mcpServers`, devenv's
hosted `mcp.devenv.sh` among them, and approves each by name in the project
settings devenv writes, `enabledMcpjsonServers`, so a session in a trusted
folder starts them unasked. The generated `.mcp.json` and
`.claude/settings.json` are ignored in git, what a repository's Claude Code
settings hold moves into devenv's options, and a repository whose specs
change only through OpenSpec tracks OpenSpec's Claude Code skills and
commands as its pinned OpenSpec writes them.

The site declared no server, tracked no `.mcp.json` and no `.claude/`, and
so had no Claude Code settings to move. It pinned no OpenSpec: `CLAUDE.md`
sends every spec change through `openspec validate --strict` and every tool
through `devenv.nix`, yet the shell held none, so a run took whatever
OpenSpec the host had. What does the site declare, and how do its places
get the files before their agents start?

## Decision drivers

- Record 0022's decision, and its line that each repository's change
  confirms its places have their servers before their agents start.
- Every tool comes from `devenv.nix`, never a global install (`CLAUDE.md`).
- A place's readiness is sluice's equip: it sources the script
  `devenv print-dev-env` prints, the toolchain `.config/sluice.toml` names,
  which holds the shell's `enterShell` and none of devenv's tasks; devenv's
  own task that writes Claude Code's files never runs there.
- Every check is a moon task the gate runs.

## Considered options

For the servers:

- devenv's and Astro's documentation server
- devenv's alone, as boomdrive declares

For how a place gets the files:

- `enterShell` runs devenv's files script itself
- A toolchain that enters the shell and runs its tasks

For the OpenSpec the skills are written by:

- 1.14.3 in the shell, the release flume, boomdrive and sluice pin
- The newest release, 1.16.0, which this host has
- No pin, the host's OpenSpec

## Decision outcome

**Servers.** Chosen option: "devenv's and Astro's documentation server".
`mcp.devenv.sh`, named because a list of the repository's own replaces
devenv's default, and `astro-docs`, `https://mcp.docs.astro.build/mcp`,
which answers for the Astro the site builds on, as record 0022 gives
boomdrive-cloud Cloudflare's documentation. Both are hosted, need no
sign-in and start no process on the host. Declaring devenv's alone would
leave the site's agents reading Astro's documentation from the web, where
the release `package.json` pins is one page among many.

**Approval.** devenv has no option for `enabledMcpjsonServers`: the site
adds it to the settings file devenv writes, naming every server
`claude.code.mcpServers` names, so a server added there is approved with
it.

**A place's files.** Chosen option: "`enterShell` runs devenv's files
script itself", from the repository's root and writing nothing to standard
output, so a place holds both files once its readiness has run and a shell
entered later finds them made; where they cannot be written the shell still
opens and says so on standard error. A toolchain that runs devenv's tasks
would change the environment sluice's gate runs every check under.

**OpenSpec.** Chosen option: "1.14.3 in the shell", built as boomdrive
builds it, so the site's ten skills and ten commands are byte for byte
boomdrive's and sluice's set: `openspec init --tools claude --profile
custom`, with the user's OpenSpec configuration naming propose, explore,
new, continue, apply, ff, sync, archive, bulk-archive and verify. Pinning
1.16.0 would give the site skills no other repository of the family
carries; leaving it to the host is what `CLAUDE.md` forbids. The moon task
`site:openspec-skills`, in the fast layer the gate requires, fails when a
skill names another release than the shell's OpenSpec, so moving the pin
writes them again.

### Consequences

- Good: an agent in a place, and a person's session once they have trusted
  the repository, starts devenv's and Astro's servers and is asked nothing.
- Good: OpenSpec runs as the same release in every place and in the gate,
  and its skills cannot drift from it unseen.
- Bad: the shell builds OpenSpec from source where no cache holds it, a
  minute on a fresh host.
- Bad: a clone nobody trusted still asks before it starts either server.
