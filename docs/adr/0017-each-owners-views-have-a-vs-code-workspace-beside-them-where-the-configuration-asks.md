---
number: 17
title: Each owner's views have a VS Code workspace beside them where the configuration asks
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Each owner's views have a VS Code workspace beside them where the configuration asks

Status: proposed on 2026-11-05 by the construct stage of the change `a-view-writes-its-vs-code-workspace`. That the runner may write VS Code workspaces for its views, as an option, is the user's decision, in their words below. Where the files go, what they list and how they are kept are the stage's choices, made on the written rules. The record is accepted when the user says yes to it.

Deciders: the user, for the option, and on 2026-11-05 for the hosts turning it on now and their views retiring; the construct stage, for its form.
Sources: the user's words of 2026-11-04 and 2026-11-05 in the brief of the change `a-view-writes-its-vs-code-workspace`; afterthought/swancloud's rule herdr.14 and its record 0031, cited by id; record 0009; rules core.2, core.5, core.12, core.14, core.17, core.35 and core.42; the views and configuration specs.

## Context and problem statement

The user browses code in VS Code through one workspace file per context, which lists that context's repositories. The hosts' own configuration keeps those files today, under swancloud's herdr.14: beside each space's views is a workspace listing exactly the space's repositories. Its `folders` are set at every deploy, and everything else in it is kept, since VS Code writes the user's settings into the file. VS Code titles a workspace's window `— <name> (Workspace)`. A window manager that files windows by their title puts each context's window in a row of its own.

The runner keeps each repository's view, at `<repository_root>/<owner>/<name>` (the views spec), from the repositories its configuration names. A workspace kept anywhere else is a second list beside the runner's, which can name a repository the host no longer keeps or miss one it does. Under a hosted control plane the hosts' configuration does not know the list at all: the managed level holds it (record 0009).

The user, 2026-11-04: "I don't know if we should build it as a feature into Boom Drive, where we automatically generate VS Code workspaces that point to your view. Maybe it's an option that goes in." On 2026-11-05 they asked for it to be rushed, and later that day, in madswan-design's pane: "We need to turn it on and then deprecate the views. The code views are going away. We have the Boom Drive one now that replaces it".

## Considered options

For whether the runner writes them:

1. An option of the runner's configuration, off unless asked for (the user's).
2. Always.
3. Never: the hosts' configuration keeps writing them.

For what one workspace gathers:

- A. One per owner, `<repository_root>/<owner>/<owner>.code-workspace`, listing the owner's kept repositories.
- B. Workspaces named in the configuration, each listing repositories by address.
- C. One for the whole repository root.

For what its folders list: the owner's kept repositories as the configuration lists them, or the views that stand at the pass.

For a file already there: set its `folders` and keep the rest, or write it whole.

## Decision outcome

Option 1, with A, the configuration's list, and the rest of the file kept.

- **An option, off unless asked for.** `code_workspaces = true` in the configuration asks for them, so a host with no editor, a server or a box, gets none. The key names VS Code's `.code-workspace` file as VS Code means it. A bare `workspace` is herdr's word, which Flume uses for its own panes (core.12).
- **One per owner, beside the owner's views.** The owner's folder holds the owner's views as a space's folder holds its views under herdr.14. So the workspace sits there under the owner's name, and each of its folders is a view's name. VS Code titles its window `— <owner> (Workspace)`, by which a window manager files it.
- **The configuration's list.** A workspace lists exactly its owner's kept repositories, in the configuration's order, whether or not a view stands this pass. The workspace and the views follow one list, so the two never name different repositories (core.5), and a fetch that fails never takes a folder out of the window.
- **The rest of the file is kept.** At every pass the runner sets `folders`, keeps every other member as it was, in its order, and writes the file only when its folders differ. A file it cannot read as a JSON object, a person's comment in it among the reasons, is left as it is and named at every pass, as is a link or a folder at its path.
- **It goes with its owner.** When the runner removes a view and the owner's folder then holds only a workspace that no kept owner has there, it removes the workspace and the folder, as it removes an empty owner folder. Where the option is off, the runner never touches a workspace.

Option 2 would leave files on hosts with no editor. Option 3 keeps a second list of the host's repositories, which drifts from the views, and which the hosts' configuration cannot keep once a hosted control plane writes the repositories.

B lets the person choose each workspace's name and repositories. But it lists each repository twice, in two lists that can differ. And under a hosted control plane the managed level would write each group, naming a drive under another key (core.5). C gives a host that keeps several owners' repositories one window for all of them. A name of the person's own for an owner's workspace is a later key, should a host need one.

Listing only the views that stand would move the window's folders whenever a clone or a view fails for a pass. Writing the whole file at every pass would drop the settings VS Code writes into it.

### Consequences

- The runner writes, into the user's code folder, a file that VS Code also writes, and owns only its `folders`. The file is in no view, no clone's container and not the data directory (core.14, core.35).
- A window manager's rules match the owner's name. A person who wants another title keeps a workspace of their own elsewhere.
- A changed repository root starts each owner's workspace afresh at the new root. The old one, with its settings, goes when the old owner's folder is removed.
- The hosts ask for the option now, with crew still the control plane, and the hosts' own views and the workspaces their configuration writes beside them retire (the user, 2026-11-05). The runner's workspaces replace them only once every repository a host's view lists is one the runner keeps, so no window a person opens is lost. That is the hosts' change.
- No message of the protocol changes, and no control plane's implementation reads the key (core.2, core.42). Under a hosted control plane, the managed level may set it as it sets the repositories (core.17, record 0009).

Rule: core.43.
