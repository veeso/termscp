import { copyFile, access } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

// repo-root install scripts (single source of truth)
const scripts = ["install.sh", "install.ps1"];

await Promise.all(
  scripts.map(async (name) => {
    const src = join(here, "..", "..", name);
    const dest = join(here, "..", "public", name);

    try {
      await access(src);
    } catch {
      console.error(`[copy-install] source not found: ${src}`);
      console.error(
        `[copy-install] the repo-root ${name} must be available at build time.`,
      );
      process.exit(1);
    }

    await copyFile(src, dest);
    console.log(`[copy-install] copied ${name} -> public/${name}`);
  }),
);
