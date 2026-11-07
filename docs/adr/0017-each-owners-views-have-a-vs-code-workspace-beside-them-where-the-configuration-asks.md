---
number: 17
title: Each drive's repositories have a VS Code workspace where the configuration asks
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Each drive's repositories have a VS Code workspace where the configuration asks

Status: accepted by the user on 2026-11-07, on their card, with record 0028. Proposed on 2026-11-05 by the construct stage of the change `a-view-writes-its-vs-code-workspace`. That the runner may write VS Code workspaces for its views, as an option, is the user's decision, in their words below. Where the files go, what they list and how they are kept are the stage's choices, made on the written rules. Revised on 2026-11-07 on the user's note on its card, quoted below: one workspace per drive, the drives as record 0028 has them.

Deciders: the user, for the option, and on 2026-11-05 for the hosts turning it on now and their views retiring; the user, on 2026-11-07 on this record's card, for what one workspace gathers: "Per recent discussion with designer agent, this becomes one workspace per boomdrive drive"; the construct stage, for its form, and agentplot-design, for where a drive's workspace goes.
Sources: the user's words of 2026-11-04 and 2026-11-05 in the brief of the change `a-view-writes-its-vs-code-workspace`; afterthought/swancloud's rule herdr.14 and its record 0031, cited by id; record 0009; rules core.2, core.5, core.12, core.14, core.17, core.35 and core.42; the views and configuration specs; record 0028.

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
- D. One per drive, `<repository_root>/<drive>.code-workspace`, listing the drive's kept repositories.

For what its folders list: the drive's kept repositories as the configuration lists them, or the views that stand at the pass.

For a file already there: set its `folders` and keep the rest, or write it whole.

## Decision outcome

Option 1, with D, the configuration's list, and the rest of the file kept.

- **An option, off unless asked for.** `code_workspaces = true` in the configuration asks for them, so a host with no editor, a server or a box, gets none. The key names VS Code's `.code-workspace` file as VS Code means it. A bare `workspace` is herdr's word, which Flume uses for its own panes (core.12).
- **One per drive, at the repository root.** A drive is the body of work the user opens together (record 0028), and its repositories may have several owners. So its workspace sits at the root under the drive's name, and each of its folders is a view's path, `<owner>/<name>`. VS Code titles its window `— <drive> (Workspace)`, by which a window manager files it. A drive's name is unique on a host, being its stream's.
- **The configuration's list.** A workspace lists exactly its drive's kept repositories, in the configuration's order, whether or not a view stands this pass. The workspace and the views follow one list, so the two never name different repositories (core.5), and a fetch that fails never takes a folder out of the window.
- **The rest of the file is kept.** At every pass the runner sets `folders`, keeps every other member as it was, in its order, and writes the file only when its folders differ. A file it cannot read as a JSON object, a person's comment in it among the reasons, is left as it is and named at every pass, as is a link or a folder at its path.
- **It goes with its drive.** When a drive leaves the configuration and the runner has removed its last view, it removes the drive's workspace. Where the option is off, the runner never touches a workspace.

Option 2 would leave files on hosts with no editor. Option 3 keeps a second list of the host's repositories, which drifts from the views, and which the hosts' configuration cannot keep once a hosted control plane writes the repositories.

A gathers an owner's repositories, which can belong to different drives, so a window holds work the user does not open together. B lets the person choose each workspace's name and repositories. But it lists each repository twice, in two lists that can differ. And under a hosted control plane the managed level would write each group, naming a drive under another key (core.5). C gives a host that keeps several drives' repositories one window for all of them. A name of the person's own for a drive's workspace is a later key, should a host need one.

Listing only the views that stand would move the window's folders whenever a clone or a view fails for a pass. Writing the whole file at every pass would drop the settings VS Code writes into it.

### Consequences

- The runner writes, into the user's code folder, a file that VS Code also writes, and owns only its `folders`. The file is in no view, no clone's container and not the data directory (core.14, core.35).
- A window manager's rules match the drive's name. A person who wants another title keeps a workspace of their own elsewhere.
- A changed repository root starts each drive's workspace afresh at the new root. The old one, with its settings, is left where it was.
- A workspace the runner kept per owner before this revision is left as it is, with its settings, and the runner writes none there again. The person removes it.
- The hosts ask for the option now, with crew still the control plane, and the hosts' own views and the workspaces their configuration writes beside them retire (the user, 2026-11-05). The runner's workspaces replace them only once every repository a host's view lists is one the runner keeps, so no window a person opens is lost. That is the hosts' change.
- No message of the protocol changes, and no control plane's implementation reads the key (core.2, core.42). Under a hosted control plane, the managed level may set it as it sets the repositories (core.17, record 0009).

Rule: core.43.

## Amendment, 2026-11-07: how a drive's workspace goes with its drive

Status: decided on 2026-11-07 by the construct stage of the change `a-hosts-configuration-names-its-accounts-and-their-drives`, on the written rules. Its design gives the path not taken for each choice.

- **The clone records its drive.** Each kept repository's clone records the drive the configuration names it under, as `boomdrive.drive` in its bare repository's configuration, beside `boomdrive.address` and `boomdrive.view` (core.14, core.35, core.36).
- **The workspace goes when the drive is gone.** At the end of a pass, where `code_workspaces` is true, the runner looks at each drive the clones recorded when the pass began. Where the configuration no longer names that drive and no clone still standing records it, the runner removes its workspace at the root, if it reads the file as a JSON object. Anything else at that path is left and named.
- **Moved repositories count.** A drive whose repositories all moved to other drives has no view left of its own, so its workspace goes too.
- **Not taken:**
  - a list of written workspaces in the data directory, which core.35 does not hold;
  - removing every workspace at the root that no drive names, which would take a person's own (core.43).

core.43's details are amended with this amendment.
