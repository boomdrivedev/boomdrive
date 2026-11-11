# What the Boomdrive module writes into Pending You's folder,
# `<home>/.config/pendingyou`, standing as Pending You's module, since
# Pending You gives none (record 0044, point 7; design Decision 7 of the
# change crews-companions-come-with-the-flake): a function, not a module,
# of the runner's user's `home` and the Pending You `package` the modules
# install, giving one store folder of the four files Pending You's `init`
# writes there, which activate.sh's `pendingyou` copies into the folder,
# each whole and only where it differs:
#
# - bin/pendingyou-hook, the launcher Pending You's hooks run, as its
#   `init`'s (dist/shim.js), with what a crew session needs: a Stop check
#   in a session crew started (CREW_AGENT set) runs and its verdict is
#   thrown away, since an agent's last words are for its conductor and not
#   a card;
# - bin/pendingyou-mcp-headers, its MCP server's headers helper;
# - claude-code.json, the manifest, in `installClaude`'s shape: the origin,
#   the helper, the hook lines by the folder's path and permission cards
#   on, no MCP server added, no skill and no copy of the command line;
# - signin/.claude.json, the Claude folder `pendingyou-signin` signs
#   Claude Code's connection in with, naming Pending You's server with the
#   folder's helper.
#
# It holds no copy of the wake mod, which rides in the set's marketplace
# (record 0044, point 5). The sign-ins, the computer's key, the cards and
# the state are the command line's own, and never here. `folder` and
# `texts` are given to the evaluated cases.
{
  lib,
  runCommand,
  writeText,
  home,
  package,
}:
let
  folder = "${home}/.config/pendingyou";
  pendingyou = lib.getExe' package "pendingyou";
  origin = "https://www.pendingyou.com";

  # A shell word as Pending You writes one: bare where it is safe, else
  # quoted.
  word =
    text:
    if builtins.match "[A-Za-z0-9_@%+=:,./-]+" text != null then
      text
    else
      "'${lib.replaceStrings [ "'" ] [ "'\\''" ] text}'";
  # A launcher's path as Pending You's hook lines quote it.
  quoted = text: ''"${lib.replaceStrings [ "\\" "\"" "$" "`" ] [ "\\\\" "\\\"" "\\$" "\\`" ] text}"'';

  launcher = "${folder}/bin/pendingyou-hook";
  helper = "${folder}/bin/pendingyou-mcp-headers";
  lineOf = sub: "${quoted launcher} ${sub} --app claude-code || true";

  head = about: ''
    #!/bin/sh
    # ${about}
    # Written by the Boomdrive module's activation at each switch, for the
    # Pending You it installs.
    PENDINGYOU_CONFIG_DIR=${word folder}
    export PENDINGYOU_CONFIG_DIR
  '';

  texts = {
    "bin/pendingyou-hook" = head "Pending You's hooks run this, by this path." + ''
      # A Claude Code tool call ran or a session ended: nothing to do unless a
      # permission card may be open.
      if [ "$1" = permission-done ]; then
        for open in "$PENDINGYOU_CONFIG_DIR"/permission-cards/*.json; do
          [ -e "$open" ] && break
          exit 0
        done
      fi
      # A session crew started: its last words are for its conductor, not a
      # card, so the Stop check runs and its verdict is thrown away.
      if [ "$1" = stopcheck ] && [ -n "''${CREW_AGENT:-}" ]; then
        ${pendingyou} "$@" >/dev/null
        exit 0
      fi
      exec ${pendingyou} "$@"
    '';
    "bin/pendingyou-mcp-headers" =
      head "Pending You's sign-in for the pendingyou MCP server, which Claude Code runs at each connection."
      + ''
        exec ${pendingyou} mcp-headers "$@"
      '';
    "claude-code.json" = builtins.toJSON {
      version = 1;
      inherit origin;
      mcpAdded = false;
      skill = null;
      hooks = {
        SessionStart = lineOf "pickup";
        UserPromptSubmit = lineOf "handoff";
        Stop = lineOf "stopcheck";
        SessionEnd = lineOf "presence";
      };
      permissions = [ ];
      cli = null;
      modDir = null;
      helper = {
        path = helper;
        command = word helper;
      };
      permissionHooks = {
        PermissionRequest = lineOf "permission";
        PostToolUse = lineOf "permission-done";
        PostToolUseFailure = lineOf "permission-done";
        SessionEnd = lineOf "permission-done";
        Notification = lineOf "notify";
      };
      permissionCards = true;
    };
    "signin/.claude.json" = builtins.toJSON {
      mcpServers.pendingyou = {
        type = "http";
        url = "${origin}/mcp";
        headersHelper = word helper;
      };
    };
  };
  modes = {
    "bin/pendingyou-hook" = "0755";
    "bin/pendingyou-mcp-headers" = "0755";
    "claude-code.json" = "0644";
    "signin/.claude.json" = "0644";
  };
in
runCommand "pendingyou-folder"
  {
    passthru = { inherit folder texts modes; };
  }
  (
    lib.concatStrings (
      lib.mapAttrsToList (path: text: ''
        install -D -m ${modes.${path}} ${writeText "pendingyou-${lib.removePrefix "." (baseNameOf path)}" text} "$out/${path}"
      '') texts
    )
  )
