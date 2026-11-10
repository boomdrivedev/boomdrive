import { defineConfig } from "astro/config";

// Served by GitHub Pages as a project site, under the repository's name,
// /boomdrive/, until a domain is chosen; every internal link goes through
// src/lib/url.ts.
export default defineConfig({
  site: "https://agentplot.github.io",
  base: "/boomdrive",
  // devenv's state links into /nix/store; a watcher that follows it stats the
  // whole store.
  vite: { server: { watch: { ignored: ["**/.devenv/**", "**/.direnv/**"] } } },
});
