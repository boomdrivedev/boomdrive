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
    status: z.enum(["draft", "stable", "retired"]),
    introducedIn: z.string(),
    lastReviewed: z.coerce.date(),
    related: z.array(reference("principles")).default([]),
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

// A day's roll-up of one series: its peak and its typical value. A sample is
// an illustration, made up to show how the page reads; it is never a
// measurement, and every place that shows one says so.
const point = z.object({
  date: z.coerce.date(),
  typical: z.number(),
  peak: z.number(),
  sample: z.boolean().default(false),
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
    chart: z.enum(["utilization"]),
    series: z
      .array(
        z.object({
          key: z.string(),
          name: z.string(),
          unit: z.string(),
          // The key of the series this one is a share of, where a share means something.
          shareOf: z.string().optional(),
          points: z.array(point).default([]),
        }),
      )
      .min(1),
  }),
});

export const collections = { principles, notes, terms, components, measurements };
