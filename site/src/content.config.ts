import { defineCollection, reference } from "astro:content";
import { file, glob } from "astro/loaders";
import { z } from "astro/zod";

// The types of docs/site-design.md that version 1 needs. Every reference is
// checked at build time, so a note that cites a missing principle fails the
// build.

const principles = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/principles" }),
  schema: z.object({
    order: z.number().int().min(0),
    title: z.string(),
    summary: z.string(),
    // The principle in one line, as Home lists it beside the flywheel.
    line: z.string(),
    status: z.enum(["draft", "stable", "retired"]),
    introducedIn: z.string(),
    lastReviewed: z.coerce.date(),
    related: z.array(reference("principles")).default([]),
    // An animated diagram of the idea, from public/; the Method sets it beside
    // the write-up, and a principle without one shows its write-up alone.
    diagram: z.object({ src: z.string(), alt: z.string() }).optional(),
  }),
});

const artifact = z.object({
  kind: z.enum(["repo", "gist", "release", "config", "script"]),
  url: z.url(),
  runs: z.string(),
});

const notes = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/notes" }),
  schema: z.object({
    title: z.string(),
    date: z.coerce.date(),
    kind: z.enum(["finding", "how-to", "essay", "incident"]),
    draft: z.boolean().default(false),
    methodVersion: z.string(),
    principles: z.array(reference("principles")).min(1),
    artifacts: z.array(artifact).default([]),
    openQuestions: z.array(z.string()).default([]),
    measurements: z.array(reference("measurements")).default([]),
    supersededBy: reference("notes").optional(),
  }),
});

const terms = defineCollection({
  loader: file("./src/content/terms/terms.yaml"),
  schema: z.object({
    term: z.string(),
    definition: z.string(),
    principle: reference("principles").optional(),
    // A name kept lowercase even at the start of a sentence.
    lowercase: z.boolean().default(false),
    // Words Vale flags in favour of this term.
    insteadOf: z.array(z.string()).default([]),
  }),
});

const components = defineCollection({
  loader: file("./src/content/components/components.yaml"),
  schema: z.object({
    name: z.string(),
    url: z.url().optional(),
    category: z.enum([
      "harness",
      "multiplexer",
      "multiplexer-plugin",
      "worktree-manager",
      "secrets",
      "password-manager",
      "toolchain",
      "task-graph",
      "model",
      "ours",
    ]),
    role: z.string(),
    status: z.enum(["trialing", "in-use", "retired"]),
    serves: z.array(reference("principles")).default([]),
    partOf: z.string().optional(),
    ours: z.boolean().default(false),
    becomes: z.string().optional(),
  }),
});

// One value at one date. A sample is an illustration, made up to show how the
// page reads; it is never a measurement, every place that shows one says so,
// and Home never reads one.
const point = z.object({
  date: z.coerce.date(),
  value: z.number(),
  sample: z.boolean().default(false),
});

// The roles an agent of the fleet plays, as Crew names them.
export const roles = ["conductor", "team ops", "unit agent", "design", "planner", "dispatcher", "operator"] as const;
// The stages a unit passes through in flight, as Crew's plan names them.
export const stages = ["construct", "awaiting approval", "code", "verify", "merge"] as const;

const series = z.object({
  name: z.string(),
  unit: z.string(),
  // Which statistic of the day's samples the value is; a sum adds up the
  // day, and a count counts the day's events.
  statistic: z.enum(["median", "max", "sum", "count"]).default("median"),
  // A peak series names the series it is the peak of; the page draws the
  // pair as one band and one tile.
  peakOf: z.string().optional(),
  // A series split from a total names what it is split by, and the page
  // stacks the split beside the total.
  role: z.enum(roles).optional(),
  model: z.string().optional(),
  kind: z.enum(["input", "output", "cache written", "cache read"]).optional(),
  flywheel: z.string().optional(),
  stage: z.enum(stages).optional(),
  // A host's own load is read per machine, never summed across machines.
  host: z.string().optional(),
  points: z.array(point).default([]),
});

// The record's file name is its id, so the two cannot disagree.
const measurements = defineCollection({
  loader: glob({ pattern: "**/*.yaml", base: "./src/content/measurements" }),
  schema: z.object({
    name: z.string(),
    summary: z.string(),
    dimension: z.enum(["flow", "attention", "proof", "quality", "cost", "health", "design"]),
    definitionVersion: z.number().int().positive(),
    definition: z.string(),
    principle: reference("principles"),
    source: z.string(),
    collector: z.string(),
    argues: z.string(),
    cannotShow: z.string(),
    // How the page draws the series; each form checks the series it needs.
    chart: z.enum(["utilization", "tokens", "flight", "landed"]),
    series: z
      .array(series)
      .min(1)
      .superRefine((all, ctx) => {
        const names = all.map((s) => s.name);
        names.forEach((n, i) => {
          if (names.indexOf(n) !== i) ctx.addIssue({ code: "custom", message: `two series are named "${n}"` });
        });
        for (const s of all) {
          if (s.peakOf && !names.includes(s.peakOf)) {
            ctx.addIssue({ code: "custom", message: `"${s.name}" is the peak of "${s.peakOf}", which is not a series` });
          }
        }
        // A summed or counted total is the sum of each of its splits on every
        // day; a median's split need not be.
        const splitBy = ["role", "model", "kind", "flywheel", "stage"] as const;
        for (const t of all.filter((s) => (s.statistic === "sum" || s.statistic === "count") && !splitBy.some((b) => s[b]))) {
          for (const by of splitBy) {
            const parts = all.filter((s) => s[by] && s.name === `${t.name}, ${s[by]}`);
            if (!parts.length) continue;
            t.points.forEach((p, i) => {
              const sum = parts.reduce((a, s) => a + (s.points[i]?.value ?? 0), 0);
              if (Math.abs(sum - p.value) > 0.005) {
                ctx.addIssue({ code: "custom", message: `"${t.name}" by ${by} sums to ${sum}, not ${p.value}, on day ${i + 1}` });
              }
            });
          }
        }
      }),
  }),
});

// List prices in dollars per million tokens, each from a date: a point's
// dollars use the rates in force on its date. A cache write is priced by its
// lifetime; a transcript that does not split its writes by lifetime is priced
// at the 5-minute rate. A rate not yet read from the price list is a sample.
const rates = defineCollection({
  loader: file("./src/content/rates/rates.yaml"),
  schema: z.object({
    model: z.string(),
    from: z.coerce.date(),
    input: z.number().nonnegative(),
    cacheWrite5m: z.number().nonnegative(),
    cacheWrite1h: z.number().nonnegative(),
    cacheRead: z.number().nonnegative(),
    output: z.number().nonnegative(),
    source: z.url(),
    sample: z.boolean().default(false),
  }),
});

// The fleet a measurement was taken in: the flywheels we operate, on which
// machines, over how many repositories, from a date until the next context.
// Only our own appear; a flywheel, machine or repository that carries client
// work is neither named nor counted.
const contexts = defineCollection({
  loader: file("./src/content/contexts/contexts.yaml"),
  schema: z.object({
    from: z.coerce.date(),
    flywheels: z.array(z.object({ name: z.string(), machine: z.string() })).min(1),
    repositories: z.number().int().positive(),
    source: z.string(),
  }),
});

// The words of a page that is laid out by hand, such as the Tools hub, kept
// here so Vale reads them as it reads every other page. The page places the
// prose before the first heading as its lede and each section after it.
const pages = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/pages" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
  }),
});

export const collections = { principles, notes, terms, components, measurements, rates, contexts, pages };
