# flywheel-runner

The runner on each host, one binary named `flywheel`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface, with
two implementations: crew's, which reads and writes records in crew's
state repository, and a hosted control plane's, an outgoing WebSocket.

For now it prints its version:

    devenv shell -- cargo run -p flywheel -- --version

## The configuration

`flywheel` reads one TOML file: the one `--config <file>` names, else
`$XDG_CONFIG_HOME/flywheel/config.toml`, else
`~/.config/flywheel/config.toml`. It names the repositories the host
keeps by their GitHub address, where their views are kept and how often
they are fetched:

```toml
# Where each repository's view is kept, at <views>/<owner>/<name>; ~/Code where none is named.
views = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_every = "2m"
# Each repository this host keeps, by its GitHub address.
repositories = ["agentplot/flume", "afterthought/crew"]
```

`repositories` is required, so a host that keeps none says
`repositories = []`. Addresses are compared without case, so two that
differ only in case are refused as one repository named twice. `views`
is an absolute path or begins with `~/`. `fetch_every` is `<n>s`, `<n>m` or `<n>h`. A key the runner does
not read is refused, so the file has no place for a credential: the
host's own git credential helper answers every fetch.

## Rules and checks

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`.
