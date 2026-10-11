import type { CollectionEntry } from "astro:content";

// Reading a measurement's series by what they are split by: a total, and the
// series beside it that name a role, a model or a kind.

type Series = CollectionEntry<"measurements">["data"]["series"][number];
export type Group = { name: string; values: number[] };
export type Split = "role" | "model" | "kind";

export const iso = (d: Date) => d.toISOString().slice(0, 10);

export function total(series: Series[], name: string): Series {
  const s = series.find((x) => x.name === name && !x.role && !x.model && !x.kind && !x.peakOf);
  if (!s) throw new Error(`the series "${name}" is missing`);
  return s;
}

// The split of a total, in the order the record lists it; every series of the
// split must sit on the total's dates.
export function split(series: Series[], name: string, by: Split): Group[] {
  const dates = total(series, name).points.map((p) => iso(p.date)).join();
  return series
    .filter((s) => s[by] && s.name === `${name}, ${s[by]}`)
    .map((s) => {
      if (s.points.map((p) => iso(p.date)).join() !== dates) throw new Error(`"${s.name}" is not on the dates of "${name}"`);
      return { name: s[by]!, values: s.points.map((p) => p.value) };
    });
}

export const fmt = {
  agents: (v: number) => `${v}`,
  usd: (v: number) => `$${Math.round(v).toLocaleString("en")}`,
  tokens: (v: number) =>
    v >= 1e9 ? `${(v / 1e9).toFixed(2)}B` : v >= 1e6 ? `${(v / 1e6).toFixed(1)}M` : v >= 1e3 ? `${Math.round(v / 1e3)}K` : `${v}`,
};

// A shorter form for an axis.
export const axis = {
  agents: (v: number) => `${v}`,
  usd: (v: number) => (v >= 1000 ? `$${(v / 1000).toFixed(v % 1000 ? 1 : 0)}K` : `$${v}`),
  tokens: (v: number) => (v >= 1e9 ? `${+(v / 1e9).toFixed(2)}B` : v >= 1e6 ? `${Math.round(v / 1e6)}M` : fmt.tokens(v)),
};

// A model is named without its vendor's prefix where space is short.
export const model = (name: string) => name.replace(/^claude-/, "");

export const lastWeek = (values: number[]) => values.slice(-7).reduce((a, b) => a + b, 0);
