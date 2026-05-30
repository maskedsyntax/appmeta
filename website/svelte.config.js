import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      strict: true,
    }),
    paths: {
      // Empty for custom domain (appmeta.maskedsyntax.com).
      // Set BASE_PATH=/appmeta only for github.io project-site builds.
      base: process.env.BASE_PATH ?? "",
    },
  },
};

export default config;
