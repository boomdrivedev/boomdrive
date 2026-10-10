import type { CollectionEntry } from "astro:content";

// Fleet utilization's series read as one day at a time: every series is
// sampled at the same instant, so the chart, the tiles and the table share
// one list of days. The agent counts are a median and its peak; the host's
// load is a median alone.

type Series = CollectionEntry<"measurements">["data"]["series"][number];
export type Banded = { median: number; peak: number };
export type Day = {
  date: string;
  sample: boolean;
  running: Banded;
  busy: Banded;
  cpu: number;
  memory: number;
  processes: number;
};

const names = {
  running: "agents running",
  busy: "agents busy",
  cpu: "cpu",
  memory: "memory",
  processes: "processes",
} as const;

export const iso = (d: Date) => d.toISOString().slice(0, 10);

export function days(record: string, series: Series[]): Day[] {
  const find = (name: string, peak: boolean) => {
    const s = series.find((x) => (peak ? x.peakOf === name : x.name === name));
    if (!s) throw new Error(`${record}: the utilization chart needs the series "${name}"${peak ? "'s peak" : ""}`);
    return s;
  };
  const used = [
    find(names.running, false),
    find(names.running, true),
    find(names.busy, false),
    find(names.busy, true),
    find(names.cpu, false),
    find(names.memory, false),
    find(names.processes, false),
  ];
  const dates = used[0].points.map((p) => iso(p.date));
  for (const s of used) {
    if (s.points.map((p) => iso(p.date)).join() !== dates.join()) {
      throw new Error(`${record}: series "${s.name}" is not on the same dates as "${names.running}"`);
    }
  }
  const [rm, rp, bm, bp, cpu, mem, proc] = used;
  return dates.map((date, i) => ({
    date,
    sample: used.some((s) => s.points[i].sample),
    running: { median: rm.points[i].value, peak: rp.points[i].value },
    busy: { median: bm.points[i].value, peak: bp.points[i].value },
    cpu: cpu.points[i].value,
    memory: mem.points[i].value,
    processes: proc.points[i].value,
  }));
}

// Busy divided by running, of the two values shown at a point.
export const share = (d: Day, at: keyof Banded) =>
  d.running[at] ? Math.round((100 * d.busy[at]) / d.running[at]) : 0;

export const fmt = {
  agents: (v: number) => `${v}`,
  cpu: (v: number) => `${v}%`,
  memory: (v: number) => `${v.toFixed(1)} GB`,
  processes: (v: number) => v.toLocaleString("en"),
};

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
export const short = (date: string) => `${Number(date.slice(8))} ${months[Number(date.slice(5, 7)) - 1]}`;
