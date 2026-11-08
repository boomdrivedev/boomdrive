---
number: 22
title: Each repository of the family declares its own MCP servers in its devenv and approves them for its trusted sessions
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- the design agent
---

# Each repository of the family declares its own MCP servers in its devenv and approves them for its trusted sessions

Status: accepted by the user on 2026-11-07, on their card.

Deciders: the user, in a main level at 23:20:23Z on 2026-11-06: "We need to make sure that we have the MCP servers defined in their debian/v.yaml and that, when agent sessions run, the MCP servers start. The boomdrive-cloud one is supposed to have Cloudflare and durable object MCP servers and I don't see those." ("debian/v.yaml" is dictation for devenv), which settles that each repository's servers are declared in its devenv, that an agent's session starts them, and that boomdrive-cloud has Cloudflare's servers; and, in the same message, that OpenSpec's Claude Code files are declared where OpenSpec changes are written and devenv's own server is in place. The design agent for the shape below, each choice with the path not taken.

Sources: devenv 2.4.0's `src/modules/integrations/claude.nix` and `src/modules/files.nix` (the installed source), https://devenv.sh/integrations/claude-code/, https://devenv.sh/creating-files/ and https://devenv.sh/mcp/, read 2026-11-06; Claude Code's https://code.claude.com/docs/en/mcp and https://code.claude.com/docs/en/settings-reference, read 2026-11-06; https://developers.cloudflare.com/agents/model-context-protocol/mcp-servers-for-cloudflare/, github.com/cloudflare/mcp-server-cloudflare and github.com/cloudflare/mcp, read 2026-11-06; agentplot/blueprints `design/flume/requirements.md` clauses 34 and 183 and section 10; Flume's research `research--mcp-servers-for-flumes-agents` (2026-11-02, its talk stalled); agentplot/boomdrive-cloud rule core.126 and record 0066; each repository's `devenv.nix`, `.mcp.json` and `.claude/` at origin/main on 2026-11-06.

## Context and problem statement

None of the family's repositories, boomdrive, boomdrive-cloud, boomdrive-web, boom, flume, sluice, timber and weir, declares an MCP server in its devenv. boomdrive-cloud alone keeps a `.mcp.json`, committed by hand, holding Cloudflare's documentation server. sluice alone tracks OpenSpec's Claude Code skills and commands.

An agent Flume starts in an interactive session meets Claude Code's approval question for every project server not yet approved, and the question holds the start: Flume answers only the folder-trust question and never a repository's MCP servers' approval (flume 34). How a host's own servers reach Flume's agents is not decided (flume 183, section 10), and its research puts five questions to the user.

devenv's Claude Code integration, `claude.code`, writes `.mcp.json` from `claude.code.mcpServers` and `.claude/settings.json` from its other options when the shell is entered, each a link into the Nix store; a regular file already at either path is left and reported as conflicting. It writes no approval key. Claude Code honours `enabledMcpjsonServers` or `enableAllProjectMcpServers` in a project's own settings once the folder is trusted, and ignores them in an untrusted one.

## Considered options

1. **Each repository declares its servers in `claude.code.mcpServers` and approves those servers in the project settings devenv writes,** so an agent in a trusted place starts them unasked; both files are generated and never committed.
2. Each repository commits `.mcp.json` by hand, as boomdrive-cloud does, with devenv declaring nothing. It leaves the user's ask unmet and the approval question holding every start.
3. Flume passes each agent an approval of its repository's servers. It answers a question flume 34 rules out, and it is one part of Flume's MCP design, which waits on the user's five questions.
4. The host approves every project's servers in each account's user settings. It approves any repository's servers in any session on the host, and it is the host's configuration, which no repository of the family can declare.

## Decision outcome

Option 1.

**Where a repository declares its servers.** In its devenv: `claude.code.enable` and `claude.code.mcpServers`, each server by name, as stdio or HTTP. devenv writes `.mcp.json` and `.claude/settings.json` at each entry to the shell, and each repository ignores both in git. What a repository's Claude Code settings hold today moves into devenv's options, so nothing is lost; sluice's committed `.claude/settings.json` is the one there is.

**How its sessions start them.** The repository approves its own servers by name in the project settings devenv writes, `enabledMcpjsonServers`, so Claude Code starts them unasked in a trusted folder and asks nothing. Claude Code keys a folder's trust on its main checkout's git root, so a place, a worktree of it, is trusted once the repository is, and Flume answers the trust question where an agent shows it (flume 34); so Flume's agents start them with no change to Flume, and a person's own session in the repository does too once they have trusted it. A clone nobody trusted still asks. The servers a repository approves are no wider than the hooks a trusted folder already runs. An agent's place has its servers only once the repository's shell has been entered there, so a place whose readiness enters no shell before its agent starts has none; each repository's change confirms its places do.

**devenv's server.** Every repository names devenv's hosted server, `mcp.devenv.sh`, the default its option gives, which searches packages and devenv's options and costs the host no process. A repository's own list replaces devenv's default, so each names it beside its own. The local `devenv mcp`, which searches the repository's own pinned inputs, was the other path; it would start one process for every agent in every place, and the user's research names load first.

**boomdrive-cloud's Cloudflare servers.** Cloudflare's documentation server (`https://docs.mcp.cloudflare.com/mcp`), its Agents SDK documentation server (`https://agents.cloudflare.com/mcp`), since the control plane builds on the Agents SDK, and its API server (`https://mcp.cloudflare.com/mcp`). The API server carries Cloudflare's skills, Durable Objects among them, and reads the account through the whole API, a Durable Object namespace's objects included. No Cloudflare server operates on Durable Objects: the API lists namespaces and their objects and nothing more. The API server is signed in with read permissions alone, by OAuth in the client choosing them or by a token holding only reads, since boomdrive-cloud's resources are made and changed only by its Alchemy program's apply (core.126). Cloudflare's bindings, builds and observability servers are deprecated in the API server's favour and are not named.

**OpenSpec's files.** Each repository whose specs change only through OpenSpec tracks OpenSpec's Claude Code skills and commands as its pinned OpenSpec writes them, as sluice does, and writes them again when it moves that pin.

**What this decides of Flume: nothing.** Flume passes nothing about MCP and answers no approval (flume 34). Which host servers an agent gets, who signs them in and where they run stay the user's questions (flume 183, section 10).

### Consequences

- One chore in each of boomdrive, boomdrive-cloud, boomdrive-web, boom, flume, sluice, timber and weir, citing this record: its servers declared and approved in its devenv, devenv's named beside them, the two files ignored, its OpenSpec files tracked; boomdrive-cloud's hand-committed `.mcp.json` gives way to devenv's.
- A remote server that needs a sign-in, Cloudflare's API server, is signed in once in each account folder an agent runs under, until Flume's design decides who signs a host's servers in. An agent whose account has not signed in sees the server needing authentication and is held by nothing.
- No unit in Flume.
