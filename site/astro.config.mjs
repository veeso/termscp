// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";

export default defineConfig({
  site: "https://termscp.rs",
  i18n: {
    defaultLocale: "en",
    locales: ["en", "zh-CN", "it", "fr", "es"],
    routing: { prefixDefaultLocale: false },
  },
  integrations: [sitemap()],
  vite: { plugins: [tailwindcss()] },
});
