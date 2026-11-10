// The Fleet's views, in the order a reader goes through them: the front page,
// then the three measurements it reads top to bottom, then every other
// measurement by name. The Fleet's tabs, its previous and next, and the front
// page's list of every measurement all read this order.
import { getCollection, type CollectionEntry } from "astro:content";
import { url } from "./url";

export type View = { id: string; label: string; href: string; m?: CollectionEntry<"measurements"> };

/** The measurements the front page reads, top to bottom. */
export const front = ["fleet-utilization", "work-in-flight", "landed"];

export async function fleetViews(): Promise<View[]> {
  const rank = (id: string) => (front.includes(id) ? front.indexOf(id) : front.length);
  const measurements = (await getCollection("measurements")).sort(
    (a, b) => rank(a.id) - rank(b.id) || a.data.name.localeCompare(b.data.name),
  );
  return [
    { id: "overview", label: "Overview", href: url("/fleet/") },
    ...measurements.map((m) => ({ id: m.id, label: m.data.name, href: url(`/fleet/${m.id}/`), m })),
  ];
}
