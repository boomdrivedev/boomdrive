# A tool's herdr plugin: plugins/herdr of the source of the tool's own
# package, copied whole with the tool's version, so a plugin and its tool
# are one release (design Decision 3). Nothing in it is rewritten, its
# scripts' interpreters among them: they find the tool on the user's
# PATH. The modules link it into the runner's user's herdr at each
# activation with herdr's own command, `herdr plugin link` (record 0023;
# design Decision 6 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents).
{
  stdenvNoCC,
  tool,
}:
stdenvNoCC.mkDerivation {
  pname = "${tool.pname}-herdr-plugin";
  inherit (tool) version;
  src = "${tool.src}/plugins/herdr";
  dontConfigure = true;
  dontBuild = true;
  installPhase = ''
    runHook preInstall
    cp -R . "$out"
    runHook postInstall
  '';
  dontFixup = true;
  meta = {
    description = "${tool.pname}'s herdr plugin, at ${tool.pname} ${tool.version}";
    inherit (tool.meta) homepage;
  };
}
