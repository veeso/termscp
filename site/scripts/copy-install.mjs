import { copyFile, access } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const src = join(here, '..', '..', 'install.sh'); // repo-root install.sh (single source of truth)
const dest = join(here, '..', 'public', 'install.sh');

try {
  await access(src);
} catch {
  console.error(`[copy-install] source not found: ${src}`);
  console.error('[copy-install] the repo-root install.sh must be available at build time.');
  process.exit(1);
}

await copyFile(src, dest);
console.log('[copy-install] copied install.sh -> public/install.sh');
