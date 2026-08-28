# Install website dependencies from the lockfile
[group('site')]
site_install:
  cd site && npm ci

# Check website formatting
[group('site')]
site_fmt_check:
  cd site && npm run format:check

# Run Astro and TypeScript checks
[group('site')]
site_check:
  cd site && npm run check

# Run website tests when the package defines them
[group('site')]
site_test:
  cd site && npm test --if-present

# Build the website
[group('site')]
site_build:
  cd site && npm run build

# Run every website validation step
[group('site')]
site_ci: site_fmt_check site_check site_test site_build
