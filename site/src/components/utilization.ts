import type { CollectionEntry } from "astro:content";

// Fleet utilization's series read as one day at a time: every series is
// sampled at the same instant, so the chart, the tiles and the table share
// one list of days.

type Series = CollectionEntry<"measurements">["data"]["series"][number];
export type Reading = { typical: number; peak: number };
export type Day = {
  date: string;
  sample: boolean;
  running: Reading;
  busy: Reading;
  cpu: Reading;
  memory: Reading;
  processes: Reading;
};
export const keys = ["running", "busy", "cpu", "memory", "processes"] as const;

export const iso = (d: Date) => d.toISOString().slice(0, 10);

export function days(record: string, series: Series[]): Day[] {
  const byKey = new Map(series.map((s) => [s.key, s]));
  for (const k of keys) {
    if (!byKey.has(k)) throw new Error(`${record}: the utilization chart needs a "${k}" series`);
  }
  const dates = byKey.get("running")!.points.map((p) => iso(p.date));
  for (const k of keys) {
    const own = byKey.get(k)!.points.map((p) => iso(p.date));
    if (own.join() !== dates.join()) {
      throw new Error(`${record}: series "${k}" is not on the same dates as "running"`);
    }
  }
  return dates.map((date, i) => {
    const at = (k: (typeof keys)[number]) => byKey.get(k)!.points[i];
    const day = { date, sample: keys.some((k) => at(k).sample) } as Day;
    for (const k of keys) day[k] = { typical: at(k).typical, peak: at(k).peak };
    return day;
  });
}

// Busy as a share of running, from the day's typical values.
export const share = (d: Day) => (d.running.typical ? Math.round((100 * d.busy.typical) / d.running.typical) : 0);

export const fmt = {
  running: (v: number) => `${v}`,
  busy: (v: number) => `${v}`,
  cpu: (v: number) => `${v}%`,
  memory: (v: number) => `${v.toFixed(1)} GB`,
  processes: (v: number) => v.toLocaleString("en"),
};

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
export const short = (date: string) => `${Number(date.slice(8))} ${months[Number(date.slice(5, 7)) - 1]}`;
