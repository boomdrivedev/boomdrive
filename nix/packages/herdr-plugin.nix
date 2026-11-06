# A tool's herdr plugin: plugins/herdr of the source of the tool's own
# package, copied whole with the tool's version, so a plugin and its tool
# are one release (design Decision 3). Nothing in it is rewritten, its
# scripts' interpreters among them: they find the tool on the user's
# PATH. Linking it into herdr is herdr's own command, the host's to run.
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
