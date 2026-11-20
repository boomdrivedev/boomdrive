# shellcheck shell=sh
# Pending You's wake mod on each login's CLAUDE_CODE_PLUGIN_DIRS (record
# 0044, point 5; design Decision 8 of the change
# every-claude-code-session-has-boomdrives-plugins): sessions.nix puts
# these lines in `environment.extraInit` where the modules write Pending
# You's folder, so every login shell runs them, the one Flume reads each
# agent's environment from among them. For a user whose folder holds the
# mod, they put it ahead of the paths already there, once, and export the
# variable; for a user with none, they leave the environment as it is.
if [ -d "$HOME/.config/pendingyou/mod" ]; then
  case ":${CLAUDE_CODE_PLUGIN_DIRS-}:" in
  *":$HOME/.config/pendingyou/mod:"*) ;;
  *)
    CLAUDE_CODE_PLUGIN_DIRS="$HOME/.config/pendingyou/mod${CLAUDE_CODE_PLUGIN_DIRS:+:$CLAUDE_CODE_PLUGIN_DIRS}"
    export CLAUDE_CODE_PLUGIN_DIRS
    ;;
  esac
fi
