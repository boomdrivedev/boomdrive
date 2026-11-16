# README's **Installing with nix**: the collector a host names for its
# Claude Code sessions. README shows its own copy, which its writers keep
# to this file (record 0049). claude-code.nix evaluates it on each
# platform.
{
  services.boomdrive.claude-code.telemetry.collector = "http://localhost:4318";
}
