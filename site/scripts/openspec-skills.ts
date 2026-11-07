// OpenSpec's Claude Code skills, which the tree tracks under .claude/, are the
// ones the OpenSpec devenv.nix pins writes, and are written again when that
// pin moves (record 0003): each names the release that wrote it. Run by the
// `openspec-skills` task.
import { Glob } from "bun";

const run = Bun.spawnSync(["openspec", "--version"]);
if (!run.success) {
  console.error("openspec-skills: `openspec --version` failed; run it in the devenv shell");
  process.exit(1);
}
const pinned = run.stdout.toString().trim();

const skills = [...new Glob(".claude/skills/openspec-*/SKILL.md").scanSync()].sort();
if (skills.length === 0) {
  console.error("openspec-skills: no OpenSpec skill under .claude/skills");
  process.exit(1);
}

const stale: string[] = [];
for (const skill of skills) {
  const text = await Bun.file(skill).text();
  if (!text.split("\n").includes(`  generatedBy: "${pinned}"`)) stale.push(skill);
}
if (stale.length > 0) {
  for (const skill of stale) console.error(`openspec-skills: ${skill} is not openspec ${pinned}'s`);
  console.error("Write them again: openspec init --tools claude --profile custom");
  process.exit(1);
}

console.log(`openspec-skills: ${skills.length} skills are openspec ${pinned}'s`);
