// Every path the site links to goes through here, so the site works under
// whatever base it is served from: GitHub Pages serves it at /boomdrive/.
const base = import.meta.env.BASE_URL.replace(/\/$/, "");

export const url = (path: string): string => `${base}${path}`;
