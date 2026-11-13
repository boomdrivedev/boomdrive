# The binary-cache spec over the workflow's text (design Decision 5; Task
# notes 7.2): .github/workflows/cache.yml read line by line, as yamlfmt
# lays it out, beside the flake's packages and nixConfig. GitHub runs it;
# the gate holds what it says.
t:
let
  inherit (t) lib;
  inherit (lib)
    concatMap
    elemAt
    head
    length
    ;
  inherit (lib.lists) findFirstIndex;

  text = builtins.readFile ../../.github/workflows/cache.yml;
  lines = lib.splitString "\n" text;

  # The first capture of `re` on each line that matches it.
  captures =
    re: from:
    concatMap (l: lib.optional (builtins.match re l != null) (head (builtins.match re l))) from;
  has = re: from: lib.any (l: builtins.match re l != null) from;

  # The lines of the top-level block `key`, after its own line, less the
  # empty ones.
  blockOf =
    key:
    let
      at = findFirstIndex (l: l == "${key}:") null lines;
      after = lib.drop (at + 1) lines;
      end = findFirstIndex (l: l != "" && builtins.match " .*" l == null) (length after) after;
    in
    if at == null then [ ] else builtins.filter (l: l != "") (lib.take end after);
  # The items of the list under `key`, at `indent`, in `from`.
  itemsOf =
    indent: key: from:
    let
      at = findFirstIndex (l: l == "${indent}${key}:") null from;
      after = lib.drop (at + 1) from;
      end = findFirstIndex (l: builtins.match "${indent}  - .*" l == null) (length after) after;
    in
    if at == null then
      [ ]
    else
      map (l: lib.removeSuffix "'" (lib.removePrefix "'" (lib.removePrefix "${indent}  - " l))) (
        lib.take end after
      );

  on = blockOf "on";
  jobs = blockOf "jobs";

  # The job's steps, each its lines, in order.
  steps =
    let
      at = findFirstIndex (l: l == "    steps:") null jobs;
      split =
        acc: l:
        if lib.hasPrefix "      - " l then
          acc ++ [ [ l ] ]
        else if acc == [ ] then
          acc
        else
          lib.init acc ++ [ (lib.last acc ++ [ l ]) ];
    in
    if at == null then [ ] else builtins.foldl' split [ ] (lib.drop (at + 1) jobs);
  # The index of the first step with a line matching `re`, or null.
  stepAt = re: findFirstIndex (has re) null steps;
  # The lines of the step at `i`, none where there is no such step.
  stepOf = i: if i == null then [ ] else elemAt steps i;
  # The indices of every step with a line matching `re`.
  stepsAt = re: builtins.filter (i: has re (stepOf i)) (lib.range 0 (length steps - 1));
  before =
    a: b: what:
    t.expect (a != null && b != null && a < b) what;

  # The family's cache, by nixConfig's one substituter.
  cache = head (
    builtins.match "https://([a-z0-9-]+)\\.cachix\\.org" (head t.flake.nixConfig.extra-substituters)
  );

  # Each system's hosted runner, as the spec names it, and the matrix's
  # systems, each with the runner on the line after it.
  runners = {
    x86_64-linux = "ubuntu-24.04";
    aarch64-linux = "ubuntu-24.04-arm";
    aarch64-darwin = "macos-15";
  };
  given = builtins.attrNames t.outputs.packages;

  # What the binary-cache spec names each system's job building and
  # pushing.
  pushed = [
    "boomdrive"
    "boomdrive-launcher"
    "boomdrive-claude-plugins"
    "flume"
    "sluice"
    "crew"
    "herdr"
    "ghostty"
    "portless"
    "pendingyou"
    "plannotator"
    "plannotator-tui"
    "flume-herdr-plugin"
    "sluice-herdr-plugin"
    "pendingyou-herdr-plugin"
    "plannotator-herdr-plugin"
  ];
  systemLine = " *- system: ([^ ]+)";
  matrix = builtins.listToAttrs (
    concatMap (
      i:
      let
        system = builtins.match systemLine (elemAt jobs i);
        runner = builtins.match " *runner: ([^ ]+)" (elemAt jobs (i + 1));
      in
      lib.optional (system != null) {
        name = head system;
        value = if runner == null then null else head runner;
      }
    ) (lib.range 0 (length jobs - 2))
  );
  systemsNamed = captures systemLine jobs;

  # The x64 image's trees that no package's build uses (record 0020 as
  # amended on 2026-11-13), each a line of the step's `rm -rf`.
  freed = [
    "/usr/local/lib/android"
    "/usr/share/dotnet"
    "/opt/ghc"
    "/usr/local/.ghcup"
    "/usr/share/swift"
    "\"$HOME/.cargo\""
    "\"$HOME/.rustup\""
    "/opt/hostedtoolcache"
  ];
  free = stepAt " *sudo rm -rf .*";
  # The step's script, its lines after `run: |`.
  freeScript =
    let
      lines' = stepOf free;
      at = findFirstIndex (l: builtins.match " *run: \\|" l != null) null lines';
    in
    if at == null then [ ] else lib.drop (at + 1) lines';
  nixInstall = stepAt " *(- )?uses: cachix/install-nix-action@.*";

  build = stepAt " *nix build .*";
  push = stepAt " *run: cachix push .*";
  serviceAccountCheck = stepAt ".*::error::OP_SERVICE_ACCOUNT_TOKEN .*";
  load = stepAt " *(- )?uses: 1password/load-secrets-action@.*";
  tokenCheck = stepAt ".*::error::CACHIX_AUTH_TOKEN .*";
  cachix = stepAt " *(- )?uses: cachix/cachix-action@.*";

  serviceAccountToken = ".*\\$\\{\\{ secrets\\.OP_SERVICE_ACCOUNT_TOKEN }}.*";
  cacheToken = ".*\\$\\{\\{ steps\\.cachix-token\\.outputs\\.CACHIX_AUTH_TOKEN }}.*";
in
{
  "Each system has its job" =
    t.expect (
      captures "  ([a-z_-]+):.*" jobs == [ "build" ]
    ) "the workflow's jobs are ${toString (captures "  ([a-z_-]+):.*" jobs)}, not build alone"
    ++ t.expect (has "    runs-on: \\$\\{\\{ matrix\\.runner }}" jobs) "the job runs on no matrix.runner"
    ++ t.expect (
      systemsNamed == lib.unique systemsNamed
    ) "the workflow names a system twice: ${toString systemsNamed}"
    ++
      t.expect (builtins.attrNames matrix == given)
        "the workflow's systems are ${toString (builtins.attrNames matrix)}, not the flake's ${toString given}"
    ++ concatMap (
      system:
      t.expect ((matrix.${system} or null) == runners.${system})
        "${system} runs on ${toString (matrix.${system} or "no runner")}, not ${runners.${system}}"
    ) (builtins.filter (s: runners ? ${s}) given);

  # The x86_64-linux job's first step frees its runner's disk before Nix
  # is installed; no other system's job runs it.
  "The x86_64-linux job has room" =
    t.expect (free == 0) "the job's first step removes nothing"
    ++ before free nixInstall "the runner's disk is not freed before Nix is installed"
    ++ t.expect (has " *if: matrix\\.system == 'x86_64-linux'" (stepOf free)) "the step that frees the disk runs on a system other than x86_64-linux alone"
    ++ t.expect (
      stepsAt " *if: .*" == [ free ]
    ) "steps ${toString (stepsAt " *if: .*")} have a condition, not the step that frees the disk alone"
    ++ concatMap (
      tree:
      t.expect (has " *${lib.escapeRegex tree}( \\\\)?" (stepOf free)) "the step does not remove ${tree}"
    ) freed
    ++ t.expect (
      freeScript != [ ]
      && head freeScript == "          df -h /"
      && lib.last freeScript == "          df -h /"
    ) "the step does not say the free space of / before and after";

  "A branch's push" =
    t.expect (
      captures "  ([a-z_]+):.*" on == [
        "push"
        "workflow_dispatch"
      ]
    ) "the workflow runs on ${toString (captures "  ([a-z_]+):.*" on)}, not push and workflow_dispatch"
    ++ t.expect (
      captures "    ([a-z_-]+):.*" on == [
        "branches"
        "tags"
      ]
    ) "the push's filters are ${toString (captures "    ([a-z_-]+):.*" on)}, not branches and tags"
    ++
      t.expect (itemsOf "    " "branches" on == [ "main" ])
        "the push's branches are ${toString (itemsOf "    " "branches" on)}, not main alone"
    ++
      t.expect (itemsOf "    " "tags" on == [ "v*" ])
        "the push's tags are ${toString (itemsOf "    " "tags" on)}, not v* alone";

  # Each system's job builds every package the flake gives for it, the
  # runner's and its tools' among them, crew's companions with the rest
  # (design Decisions 1 and 6 of the change
  # crews-companions-come-with-the-flake).
  "The runner is pushed with its tools" =
    lib.concatMap (
      system:
      lib.concatMap (
        name:
        t.expect (
          t.outputs.packages.${system} ? ${name}
        ) "packages.${system} gives no ${name} for its job to build"
      ) pushed
    ) t.systems
    ++ t.expect (has " *names=\\$\\(nix eval --json \"\\.#packages\\.\\$SYSTEM\" --apply builtins\\.attrNames .*" (stepOf build)) "the build names no package of nix eval's packages.$SYSTEM"
    ++ t.expect (has " *nix build --no-link --print-out-paths .*>\"\\$RUNNER_TEMP/paths\"" (stepOf build)) "the build writes no store paths to $RUNNER_TEMP/paths"
    ++ t.expect (push == length steps - 1) "the push is not the job's last step"
    ++ t.expect (has " *run: cachix push ${cache} <\"\\$RUNNER_TEMP/paths\"" (stepOf push)) "the push is not of the built paths to ${cache}"
    ++ t.expect (
      captures ".*cachix push ([^ ]+).*" lines == [ cache ]
      && captures " *name: ([a-z0-9-]+)" (stepOf cachix) == [ cache ]
      && lib.all (c: c == cache) (captures ".*[^a-z0-9-]([a-z0-9-]+)\\.cachix\\.org.*" lines)
    ) "the workflow names a cache other than nixConfig's ${cache}";

  "A package that does not build" =
    t.expect (has "      fail-fast: false" jobs) "the job's strategy names no fail-fast: false"
    ++ before build push "the push is not after the build"
    ++ t.expect (has " *skipPush: true" (stepOf cachix)) "cachix-action names no skipPush: true"
    ++ t.expect (!has " *continue-on-error:.*" lines) "a step continues on error"
    ++ t.expect (
      blockOf "defaults" == [
        "  run:"
        "    shell: bash"
      ]
    ) "the run steps' shell is not bash, which fails a pipe";

  "No service account token" =
    t.expect (has "    environment: cachix" jobs) "the job names no environment: cachix"
    ++ before serviceAccountCheck load "the service account's token is not checked before it is used"
    ++ before serviceAccountCheck build "the service account's token is not checked before the build"
    ++
      t.expect
        (
          stepsAt serviceAccountToken == [
            serviceAccountCheck
            load
          ]
        )
        "OP_SERVICE_ACCOUNT_TOKEN is given to steps ${toString (stepsAt serviceAccountToken)}, not the check and the load alone"
    ++ t.expect (has " *exit 1" (stepOf serviceAccountCheck)) "the service account's check does not exit 1";

  "No token" =
    t.expect (has " *id: cachix-token" (stepOf load)) "the load is not the step cachix-token"
    ++ t.expect (has " *CACHIX_AUTH_TOKEN: op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential" (stepOf load)) "the load reads no op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential"
    ++ t.expect (length (captures ".*(op://).*" lines) == 1) "the workflow names op:// more than once"
    ++ before load tokenCheck "the cache's token is not checked after it is loaded"
    ++ before tokenCheck cachix "the cache's token is not checked before cachix is set up"
    ++ before tokenCheck build "the cache's token is not checked before the build"
    ++
      t.expect
        (
          stepsAt cacheToken == [
            tokenCheck
            cachix
            push
          ]
        )
        "CACHIX_AUTH_TOKEN is given to steps ${toString (stepsAt cacheToken)}, not the check, cachix and the push alone"
    ++ t.expect (has " *exit 1" (stepOf tokenCheck)) "the cache token's check does not exit 1"
    ++ t.expect (
      !has ".*echo.*\\$\\{?(OP_SERVICE_ACCOUNT_TOKEN|CACHIX_AUTH_TOKEN).*" lines
    ) "a step echoes a token";

  "Each action pinned by its commit" =
    let
      uses = captures " *-? *uses: (.*)" lines;
      pinned = builtins.filter (u: builtins.match "[^@]+@[0-9a-f]{40} # v[0-9.]+" u == null) uses;
    in
    t.expect (uses != [ ]) "the workflow uses no action"
    ++ t.expect (pinned == [ ]) "an action is not pinned by its commit with its tag: ${toString pinned}"
    ++ t.expect (has "  contents: read" (blockOf "permissions")) "the workflow's permissions are not contents: read"
    ++ t.expect (has "  cancel-in-progress: false" (blockOf "concurrency")) "the workflow cancels a run in flight";
}
