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

async function fetchText(url, { retries = 2 } = {}) {
  const headers = process.env.GITHUB_TOKEN
    ? { Authorization: `Bearer ${process.env.GITHUB_TOKEN}` }
    : {};
  for (let attempt = 0; ; attempt += 1) {
    try {
      // Abort hung requests and surface them as retryable failures.
      const res = await fetch(url, { headers, signal: AbortSignal.timeout(15000) });
      if (!res.ok) {
        throw new Error(`fetch ${url} failed: ${res.status} ${res.statusText}`);
      }
      const text = await res.text();
      if (text.trim().length === 0) {
        throw new Error(`fetch ${url} returned empty body`);
      }
      return text;
    } catch (err) {
      if (attempt >= retries) {
        throw err;
      }
      await new Promise((resolve) => {
        setTimeout(resolve, 500 * (attempt + 1));
      });
    }
  }
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
// Compare via fileURLToPath so paths with spaces (percent-encoded in
// import.meta.url, raw in argv[1]) still match.
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
