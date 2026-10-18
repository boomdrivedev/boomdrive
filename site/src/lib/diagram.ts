// A principle's diagram, drawn into the page rather than shown as an image,
// so it follows the page's theme exactly as every other figure on the site
// does. The file in public/diagrams stays a standalone SVG with its own
// palette; here its stylesheet is scoped to it, its keyframes and ids are
// renamed after it, and its palette, the family's diagram colours (--bg,
// --panel, --fg, --muted, --rule, --accent, --soft, --ok, --bad, --ink), is
// mapped onto the page's tokens as millpond's diagrams map it.
import { readFileSync } from "node:fs";
import { join } from "node:path";

const palette =
  "--bg:var(--ap-bg-2);--panel:var(--ap-bg-3);--fg:var(--ap-fg);--muted:var(--ap-fg-3);--rule:var(--ap-line);" +
  "--accent:var(--ap-accent);--soft:color-mix(in oklab,var(--ap-line) 45%,var(--ap-bg-2));" +
  "--ok:var(--ap-ok);--bad:var(--ap-bad);--ink:var(--ap-bg)";

// The end of the block that opens at `open`, a "{".
function close(css: string, open: number): number {
  let depth = 0;
  for (let i = open; i < css.length; i++) {
    if (css[i] === "{") depth++;
    else if (css[i] === "}" && --depth === 0) return i;
  }
  throw new Error("diagram: a stylesheet block is not closed");
}

// Every rule under the diagram's own class; a palette on :root or under a
// colour-scheme query is dropped, since the page's tokens give it.
function scope(css: string, at: string, rename: (body: string) => string): string {
  let out = "";
  let i = 0;
  while (i < css.length) {
    const open = css.indexOf("{", i);
    if (open < 0) break;
    const head = css.slice(i, open).trim();
    const end = close(css, open);
    const body = css.slice(open + 1, end);
    i = end + 1;
    if (head.startsWith("@keyframes")) out += `${rename(head)}{${body}}`;
    else if (head.startsWith("@media")) {
      if (!/prefers-color-scheme/.test(head)) out += `${head}{${scope(body, at, rename)}}`;
    } else if (head !== ":root") {
      const selectors = head.split(",").map((s) => `${at} ${s.trim()}`);
      out += `${selectors.join(",")}{${rename(body)}}`;
    }
  }
  return out;
}

/** The diagram at `src` (as a principle names it, under public/), ready to set into the page. */
export function inlineDiagram(src: string, name: string, label: string): string {
  const file = readFileSync(join(process.cwd(), "public", src), "utf8");
  let svg = file.slice(file.indexOf("<svg"));
  const p = `dg-${name}`;

  // Keyframes and ids take the diagram's name, so two diagrams on one page
  // never share one.
  const frames = [...svg.matchAll(/@keyframes\s+([\w-]+)/g)].map((m) => m[1]);
  const ids = [...svg.matchAll(/\sid="([^"]+)"/g)].map((m) => m[1]);
  const word = (w: string) => new RegExp(`(?<![\\w-])${w.replace(/[-]/g, "\\-")}(?![\\w-])`, "g");
  const rename = (text: string) => frames.reduce((t, f) => t.replace(word(f), `${p}-${f}`), text);

  svg = svg.replace(/<style>([\s\S]*?)<\/style>/, (_, css: string) => `<style>.${p}{${palette}}${scope(css, `.${p}`, rename)}</style>`);
  for (const id of ids) {
    svg = svg
      .replaceAll(`id="${id}"`, `id="${p}-${id}"`)
      .replaceAll(`url(#${id})`, `url(#${p}-${id})`)
      .replaceAll(`href="#${id}"`, `href="#${p}-${id}"`);
  }
  // The principle's alt text names it; its own description stays beside.
  const desc = svg.match(/<desc\s+id="([^"]+)"/)?.[1];
  const esc = label.replace(/&/g, "&amp;").replace(/"/g, "&quot;");
  svg = svg.replace(/<svg\b([^>]*)>/, (_, attrs: string) => {
    const kept = attrs
      .replace(/\s(width|height|aria-labelledby)="[^"]*"/g, "")
      .replace(/\srole="[^"]*"/, "");
    return `<svg${kept} class="${p}" role="img" aria-label="${esc}"${desc ? ` aria-describedby="${desc}"` : ""}>`;
  });
  return svg;
}
