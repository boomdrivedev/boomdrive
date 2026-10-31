# flywheel-runner

The runner on each host, one binary named `flywheel`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface, with
two implementations: crew's, which reads and writes records in crew's
state repository, and a hosted control plane's, an outgoing WebSocket.

For now it prints its version:

    devenv shell -- cargo run -p flywheel -- --version

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`.
