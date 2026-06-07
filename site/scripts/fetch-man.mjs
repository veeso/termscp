import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** Pinned ref for reproducible builds. Bump when cutting a release. */
export const MAN_REF = "v1.0.0";

export const LOCALES = ["en", "zh-CN", "it", "fr", "es"];

const REPO = "veeso/termscp";

export function manUrl(locale) {
  const path = locale === "en" ? "docs/man.md" : `docs/${locale}/man.md`;
  return `https://raw.githubusercontent.com/${REPO}/${MAN_REF}/${path}`;
}

async function fetchText(url) {
  const headers = process.env.GITHUB_TOKEN
    ? { Authorization: `Bearer ${process.env.GITHUB_TOKEN}` }
    : {};
  const res = await fetch(url, { headers });
  if (!res.ok) {
    throw new Error(`fetch ${url} failed: ${res.status} ${res.statusText}`);
  }
  return res.text();
}

async function main() {
  const here = dirname(fileURLToPath(import.meta.url));
  const outDir = join(here, "..", "src", "content", "man");
  await mkdir(outDir, { recursive: true });

  for (const locale of LOCALES) {
    let md;
    try {
      md = await fetchText(manUrl(locale));
    } catch (err) {
      if (locale === "en") throw err; // en is required — fail the build
      console.warn(`[fetch-man] ${locale} missing, falling back to en: ${err.message}`);
      md = await fetchText(manUrl("en"));
    }
    await writeFile(join(outDir, `${locale}.md`), md, "utf8");
    console.log(`[fetch-man] wrote ${locale}.md`);
  }
}

// Run only when invoked directly (not when imported by tests).
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
