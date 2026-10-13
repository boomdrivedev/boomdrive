import { defineConfig } from "astro/config";

// The domain is not chosen; `site` is set when it is.
export default defineConfig({
  // devenv's state links into /nix/store; a watcher that follows it stats the
  // whole store.
  vite: { server: { watch: { ignored: ["**/.devenv/**", "**/.direnv/**"] } } },
});
