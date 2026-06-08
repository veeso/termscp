## 1.1.0

Released on 2026-06-08

### Added

- **site:** catppuccin theme tokens, tailwind 4, self-hosted font
- **site:** i18n string resolver with en fallback
- **site:** build-time man.md fetcher pinned to release ref
- **site:** base layout with og/hreflang, theme bootstrap, umami
- **site:** x-default hreflang, twitter card, og:image:alt; dedupe locales/site in base layout
- **site:** nav, footer, theme toggle, language picker
- **site:** responsive mobile nav, css-driven theme icons, localizePath helper
- **site:** landing page with dual-pane explorer hero
- **site:** install page led by install.sh script, package-manager tabs
- **site:** robots, sitemap reference, vercel redirects + cache headers
- **install:** add Windows PowerShell installer and copy buttons on site
  > Add install.ps1 mirroring install.sh for Windows: arch detection,
  > release zip download, binary extraction, user PATH update.
  > 
  > - copy install.ps1 to site public/ at build time (copy-install.mjs)
  > - serve /install.ps1 with text/plain Content-Type (vercel.json)
  > - add PowerShell one-liner to install page and README
  > - bump install.ps1 default version in bump_version.sh
  > - add CopyButton component next to every install command line
- **config:** move config dir to ~/.config/termscp on macOS and %USERPROFILE%\.termscp on Windows
  > Resolve the config directory through a single per-platform config_dir()
  > function instead of relying on dirs::config_dir everywhere:
  > 
  > - macOS: ~/.config/termscp (was ~/Library/Application Support/termscp)
  > - Windows: %USERPROFILE%\.termscp (was roaming %APPDATA%\termscp)
  > - Linux/other: /termscp (unchanged)
  > 
  > Existing users are migrated automatically on first run: when the new
  > directory is absent and the legacy location exists, the whole config
  > directory is moved to the new path. The cache directory stays at the
  > platform-native location.

### CI

- automated release workflow
  > Single workflow_dispatch (version, dry_run) that bumps versions, regenerates
  > CHANGELOG via git-cliff, rebuilds site CSS, builds all targets, creates the
  > GitHub release, updates the Homebrew tap and publishes Chocolatey.
  > 
  > - dist/release/bump_version.sh: version replacer across all tracked locations (+tests)
  > - .github/workflows/release.yml: prepare -> build matrix -> homebrew/release -> choco
  > - retire build-artifacts.yml (merged into release.yml)
  > - Linux builds via cargo-zigbuild (old glibc) for broad compatibility
- pin all actions to verified SHAs and clear zizmor findings
  > Pin every action to a commit SHA whose tag comment matches (verified via gh api),
  > add least-privilege permissions, set persist-credentials: false, and replace the
  > archived actions-rs/cargo with a plain cargo test. zizmor clean at default persona.
- fix release notes generation in release workflow
  > prepare job failed: git-cliff --latest crashed with 'trim_start_matches on
  > null' because the checkout was shallow (no tags/history) so no release existed.
  > 
  > - checkout prepare with fetch-depth: 0 + fetch-tags so git-cliff sees full
  >   history and tags (also fixes an otherwise-truncated CHANGELOG)
  > - generate release notes with --unreleased --tag v$VERSION instead of --latest:
  >   --latest selected the previous real tag (stale notes); --unreleased --tag
  >   renders the version being released
- remove pages workflow, fix release version-bump for astro site
- **site:** add format/lint/test/build workflow for astro site
  > Add Site GitHub Actions workflow running prettier format check, astro
  > check, tests, and build on changes under site/. Wire prettier into the
  > site package with config, ignore, and scripts, and format existing
  > sources.
- add github pages workflow for docs site
- unify os workflows into one
- **release:** publish to crates.io via OIDC trusted publishing
- rename ci workflow
- **site:** always run site
- **release:** fetch full git history and tags for build.rs
  > build.rs uses vergen-git2 (Git2::all_git), which runs `git describe` and
  > reads commit metadata. The build and publish-crate jobs checked out a
  > shallow clone without tags, so vergen could not resolve the describe
  > string and libgit2 may fail on shallow repos. Add fetch-depth: 0 and
  > fetch-tags: true to both jobs that compile the crate.

### Changed

- **site:** type-safe i18n keys derived from en
- **site:** drop in-site manual, link to external docs.termscp.rs
- **site:** english-only, remove i18n machinery

### Documentation

- point READMEs to termscp.rs + install.sh, docs.termscp.rs
- replace last termscp.veeso.dev refs (install.sh manual/changelog, crate homepage)
- add shared mdbook assets and favicon.ico
- add mdbook language switcher script
- add CNAME for docs.termscp.rs
- scaffold en-US mdbook
- add en-US table of contents
- **en:** write introduction and getting-started pages
- **en:** write usage pages
- **en:** write configuration pages
- **en:** write cli reference and developer pages
- **en:** fix CLI references (no -t / --update flags; -b is a value option)
- add markdownlint config for docs site
- scaffold zh-CN mdbook
- **zh:** translate introduction and getting-started pages
- **zh:** translate section headings in getting-started pages
- **zh:** translate usage pages
- **zh:** translate configuration pages
- **zh:** translate cli reference and developer pages
- disable MD060 table alignment (incompatible with CJK width)
- drop extra language docs/READMEs, point manual links to docs.termscp.rs
- **zh:** align README with root README, drop dead language links
- fix update command (termscp update, not --update) in READMEs
- **zh:** fix keyring deep-link anchor to translated heading slug
- **config:** update config dir paths for macOS/Windows in en & zh
  > Reflect the new config directory locations (~/.config/termscp on macOS,
  > %USERPROFILE%\.termscp on Windows) across the English and Chinese docs,
  > and add a CLAUDE.md note to keep both translations in sync.

### Fixed

- **site:** robust man-fetch invocation guard, add timeout+retry
- **site:** hero selected-row contrast (latte) + mobile row truncation
- **site:** drop @ts-check on astro config to clear false vite type error
- **install:** quote vars, fix set -e cargo check and rustup tmpfile cleanup
  > - silence SC3043 by declaring dash dialect (local is supported)
  > - quote unquoted vars (SC2086/SC1090)
  > - fix set -e aborting arch install before cargo check
  > - fix install_cargo removing unset $archive instead of $rustup
  > - make brew upgrade fallback a real if-then-else (SC2015)
  > - drop leftover starship BASE_URL and debug echo $1
- **progress:** rework transfer progress panel (#424)
  > Migrate the transfer progress UI to tuirealm 4, where the stdlib
  > `ProgressBar` widget was dropped, by rebuilding the dual-bar panel on
  > top of `Gauge`.
  > 
  > - Restore the unified two-bar look: the full bar (top) and partial bar
  >   (bottom) draw joined borders so they read as a single panel; a single
  >   file shows one fully-bordered bar.
  > - Redraw on every file boundary in the send/recv queue loops so the
  >   full bar's (N/total) counter advances even for small files that finish
  >   within one in-loop redraw interval.
  > - Track progress with a single `TransferProgress` (exact file count from
  >   the pre-scan, lazy partial/full computation) and consolidate the theme
  >   progress-bar fields.
- **progress:** equalize dual progress bar heights
  > The bottom (partial) bar carried a block title (the current filename),
  > which forces a 1-row top inset in ratatui's `Block::inner` even though
  > its top border is dropped to join the seam with the full bar. That left
  > the partial bar with one inner row while the full bar kept two, so the
  > two gauges rendered at unequal heights.
  > 
  > - Move the filename from the partial bar's title into its gauge label.
  > - Skip setting an empty title so no phantom top-positioned title triggers
  >   the inset.
  > - Put the panel title on the top (full) bar for multi-file transfers.
  > - Bump the two-bar popup height to fit the joined panel.
  > 
  > Also bump Cargo.lock and adapt the embedded terminal to the new vt100
  > `screen_mut()` API.
- **copy:** prevent emptying file when copy destination is empty (#421)
  > An empty copy destination resolved to the source file's own path, so
  > std::fs::copy truncated the original file to 0 bytes.
  > 
  > - localhost::copy now refuses to copy a file onto itself, returning an
  >   error instead of truncating it (root cause).
  > - action_copy treats an empty/whitespace destination as a cancel.
- **transfer:** enqueue full destination path instead of directory
  > Queued transfers stored only the destination directory as the target
  > path. Downstream upload logic treats the queued destination as the full
  > file path and passes it straight to create_file, so transfers failed
  > with a Failure error when the remote target resolved to a directory.
  > 
  > Append each entry's file name to the destination directory at enqueue
  > time in both enqueue_file and enqueue_all, matching the single-file
  > transfer path which already builds the full target path.

### Build

- add chocolatey package, remove legacy dist build scripts
  > The old manual dist/build/* scripts and dist/{deb,rpm}.sh are superseded by the
  > automated release workflow. Add the chocolatey package consumed by release.yml.
- **site:** copy install.sh from repo root at build time (single source)
## 1.0.0

Released on 2026-04-18

### Added

- rework TransferProgress to track bytes with lazy estimation
- replace dual progress bar components with single TransferProgressBar
- simplify progress bar layout to single component
- update progress bar display for new unified data model
- update transfer loop to use unified TransferProgress
- consolidate theme progress bar fields into single transfer_progress_bar

### CI

- Codeberg mirroring
- run test workflows once
- check fmt with nightly toolchain
- add linux and windows aarch64 build targets

### Changed

- FileTransferActivity pane-agnostic dispatch (#386)
  > Comprehensive design for incremental refactoring of the 13k-line
  > FileTransferActivity god-struct using a unified Pane abstraction.
  > Detailed step-by-step plan covering 6 phases: split monoliths,
  > error handling, Pane struct, action dedup, session split, view reorg.
  > Extract 26 popup components from the monolithic 1,868-line popups.rs
  > into 20 individual files under popups/. Each file contains one or two
  > related components with their own imports. The popups.rs module file
  > now contains only module declarations and re-exports.
  > Replace 8 panic!() calls with error!() logging and early returns/fallthrough.
  > These panics documented invariants (e.g. "this tab can't do X") but would crash
  > the app if somehow triggered. Error logging is safer and more resilient.
  > Replace raw FileExplorer fields in Browser with Pane structs that bundle
  > the explorer and connected state. Move host_bridge_connected and
  > remote_connected from FileTransferActivity into the panes. Add navigation
  > API (fs_pane, opposite_pane, is_find_tab) for future unification tasks.
  > Rename private get_selected_file to get_selected_file_by_id and add three
  > new unified methods (get_selected_entries, get_selected_file, is_selected_one)
  > that dispatch based on self.browser.tab(). Old per-tab methods are kept for
  > now until their callers are migrated in subsequent tasks.
  > Collapse _local_/_remote_ action method pairs (mkdir, delete, symlink,
  > chmod, rename, copy) into unified methods that branch internally on
  > is_local_tab(). This halves the number of action methods and simplifies
  > the update.rs dispatch logic. Also unifies ShowFileInfoPopup and
  > ShowChmodPopup dispatching to use get_selected_entries().
  > Move `host_bridge` and `client` filesystem fields from FileTransferActivity
  > into the Pane struct, enabling tab-agnostic dispatch via `fs_pane()`/
  > `fs_pane_mut()`. This eliminates most `is_local_tab()` branching across
  > 15+ action files.
  > Key changes:
  > - Add `fs: Box<dyn HostBridge>` to Pane, remove from FileTransferActivity
  > - Replace per-side method pairs with unified pane-dispatched methods
  > - Unify navigation (changedir, reload, scan, file_exists, has_file_changed)
  > - Replace 147-line popup if/else chain with data-driven priority table
  > - Replace assert!/panic!/unreachable! with proper error handling
  > - Fix typo "filetransfer_activiy" across ~29 files
  > - Add unit tests for Pane
  > 
  > Net result: -473 lines, single code path for most file operations.
- replace lazy_static with std::sync::LazyLock
- migrate from mod.rs to named module files
- split parser internals into focused modules
- split auth form components by protocol
- split auth update handlers by context
- split auth view helpers by responsibility

### Documentation

- date
- add core module and API documentation
- document core host and ssh modules
- document parser and file transfer params
- complete remaining core module docs
- document ssh key storage API

### Fixed

- replace panics reachable from user input with proper error handling
- replace assert! calls in UI activities with graceful error handling
- correct typos in BadSytax and theme_provider log messages
- replace magic-crypt with aes-gcm for bookmark encryption
  > magic-crypt has known vulnerabilities. Replace it with aes-gcm for new
  > encryption (authenticated, with random nonces) while keeping a legacy
  > AES-128-CBC decryption path to transparently handle existing bookmarks.
- replace recursive byte-counting with entry-based transfer progress (#395)
  > * fix: replace recursive byte-counting with entry-based transfer progress
  > 
  > Replace the expensive recursive `get_total_transfer_size` pre-calculation
  > with a lightweight entry-based counter (`TransferProgress`) for the
  > overall progress bar. This avoids deep `list_dir` traversals before
  > transfers begin, which could cause FTP idle-timeout disconnections on
  > large directory trees.
  > 
  > The per-file byte-level progress bar (`ProgressStates`) remains
  > unchanged. Bytes are still tracked via `TransferStates::add_bytes` for
  > notification threshold logic.
- return after empty terminal prompt
- resolve `.` and `..` in terminal `cd` and prevent panic in path elide
  > `absolutize` now lexically normalizes paths so `cd ..` navigates to the
  > parent directory instead of appending `..` literally. Also guard against
  > `file_name()` returning `None` in `fmt_path_elide_ex`, which caused a
  > panic on paths containing unresolved `..` components.
- pass full command string to exec, not just the first word
  > The `Exec` arm of `Command::from_str` only captured the first
  > whitespace-delimited token, silently dropping all arguments.
  > Now passes the entire input string so e.g. `ls -la /tmp` works.
- sync browsing when entering a directory from filtered/fuzzy view
- stabilize core error handling
  > Remove production panic and unwrap paths from core modules.
  > 
  > Propagate bookmark encryption failures, harden file watcher and temp mapped file handling, and clean up dead code in shared utilities.
- normalize localhost relative path checks
- use time-based redraw interval instead of progress-delta threshold
  > The old 1% progress threshold caused the UI to appear frozen on large
  > files (e.g. 1GB) because many read/write iterations passed between
  > redraws. Switching to a 100ms time-based interval ensures consistent
  > UI responsiveness regardless of file size.
- render progress bar immediately after mounting
  > Call self.view() right after mount_progress_bar() at all 6 call sites
  > so the bar is visible on screen before the transfer loop begins.
- filter self-references and dot entries from remote directory listings
  > Some non-compliant FTP servers (e.g. LiteSpeed) include a self-reference
  > to the listed directory in the LIST response, causing the current folder
  > to appear as a duplicate entry in the explorer.

### Performance

- use sort_by_cached_key to avoid repeated lowercase allocations in file sorting

### Testing

- add parser and bookmark regression coverage
- extend config and explorer regression coverage
- extend system regression coverage

### Build

- removed `hostname`, use `whoami` instead
  > whoami provides `hostname` function, so we don't need the hostname dependency, since whoami is also being used for getting the username
- replaced libssh with russh for remotefs-ssh
- replace `version-compare` crate with `semver`
- remotefs-ssh 0.8.1
- remotefs-ssh 0.8.2
  > this version removes any usage of sh commands from the sftp backend and only uses pure protocol functions
- migrate to tui-realm 4.0
  > Upgrade tuirealm (3.x -> 4.0.0), tui-realm-stdlib (3 -> 4), tui-term
  > (0.2 -> 0.3). Apply all breaking changes from the 4.0 migration guide
  > across the termscp UI.
  > 
  > Key changes:
  > 
  > - Root-level re-exports removed; imports moved to module-qualified
  >   paths (`tuirealm::application`, `::component`, `::event`, `::props`,
  >   `::state`, `::subscription`, `::listener`, `::ratatui`). Same for
  >   stdlib component types (`tui_realm_stdlib::components::*`).
  > - `MockComponent` trait renamed to `Component`; old `Component` trait
  >   renamed to `AppComponent`. `#[derive(MockComponent)]` is now
  >   `#[derive(Component)]`. `Component::on` now takes `&Event<_>`.
  > - `TextSpan` replaced with `SpanStatic`/`LineStatic`/`TextStatic`
  >   (ratatui-based); tuple `(String, Alignment)` titles replaced with
  >   the new `Title` builder; `Alignment` split into
  >   `HorizontalAlignment`/`VerticalAlignment`; stdlib components use
  >   `.alignment_horizontal` instead of `.alignment`.
  > - `State::One`/`PropPayload::One` -> `Single`. `CmdResult::None`
  >   -> `NoChange`. `Props::get_or` removed; `Props::get` now returns a
  >   borrowed `Option<&AttrValue>` (call sites switched to
  >   `.and_then(AttrValue::as_*)`). `Component::query` returns
  >   `Option<QueryResult<'a>>`.
  > - `Attribute::HighlightedColor` -> `HighlightStyle` (a full `Style`).
  >   `.highlighted_*` helpers renamed to `.highlight_*`.
  > - `PollStrategy::UpTo(n)` now requires a `Duration`; tick timeout moved
  >   from `EventListenerCfg::poll_timeout` into `PollStrategy`.
  > - `TerminalBridge` removed; `Context` now holds
  >   `CrosstermTerminalAdapter` directly and enables raw mode + alternate
  >   screen explicitly. The `TerminalAdapter` trait is imported where its
  >   methods are used.
  > - `Update` trait removed; activity `update` methods are plain inherent
  >   functions.
  > - `ProgressBar` replaced by stdlib `Gauge`. Paragraph `.wrap` renamed
  >   to `.wrap_trim`; `.text` now takes an `Into<Text>`. Stdlib `List` row
  >   items are now individual lines (`Vec<Span>` per row) rather than a
  >   `Table` of spans; custom `FileList`/`Log` convert between the two
  >   models.
  > - Radio builders drop `.foreground(color)` so unselected items render
  >   with the terminal default foreground, and set
  >   `highlight_style(Style::default().fg(color).add_modifier(REVERSED))`
  >   so the selected entry is visibly highlighted only with the theme
  >   color.
  > - Custom `FileList` keeps the selected row highlighted with the full
  >   highlight style when focused and falls back to a foreground-only
  >   style when unfocused.
  > - Theme loading is now backwards compatible: `Theme` uses a custom
  >   `Deserialize` through an intermediate `ThemeFile` with optional
  >   fields, so missing keys, unknown values or legacy aliases
  >   (`transfer_progress_bar_full`/`_partial`) fall back to defaults on a
  >   per-field basis instead of failing the whole load.
- upgrade remotefs-ssh to 0.8.3

### Style

- linter
## 0.19.1

Released on 2025-12-20

### CI

- windows artifact name

### Fixed

- install.sh deb name
- install.sh deb name
- Updated dependencies to allow build on NetBSD
## 0.19.0

Released on 2025-11-11

### Added

- Import bookmarks from ssh config with a CLI command (#364)
  > * feat: Import bookmarks from ssh config with a CLI command
  > 
  > Use import-ssh-hosts to import all the possible hosts by the configured ssh config or the default one on your machine
- Changed file overwrite behaviour (#366)
  > Now the user can choose for each file whether to overwrite, skip or overwrite all/skip all.
- Added `<CTRL+S>` keybinding to get the total size of selected paths. (#367)
  > * feat: Added `<CTRL+S>` keybinding to get the total size of selected paths.
- Merge branch '0.19.0'

### CI

- Build artifacts for Windows x86_64 and Ubuntu x86_64 (#368)
- Debian
- debian fix
- deploy site

### Documentation

- User manual and get started links
- Release date

### Fixed

- typo in file open error message (#349)
- SMB support for MacOS with vendored build of libsmbclient.
- Report a message while calculating total size of files to transfer. (#362)
  > * fix: Report a message while calculating total size of files to transfer.
  > 
  > Currently, in case of huge transfers the app may look frozen while calculating the transfer size. We should at least report to the user we are actually doing something.
- Issues with update checks (#363)
  > Removed error popup message if failed to check for updates.
  > Prevent long timeouts when checking for updates if the network is down or the DNS is not working.

### Performance

- Migrated to libssh.org on Linux and MacOS for better ssh agent support.

### Build

- 0.19 deps
- remotefs-ssh 0.7.1
  > This version fixes compatibility with hosts which don't use bash/sh as the default shell.
## 0.18.0

Released on 2025-06-10

### Added

- **Updated dependencies** and updated the Rust edition to `2024`
- Replaced the `Exec` popup with a fully functional terminal emulator (#348)
  > * feat: Replaced the `Exec` popup with a fully functional terminal emulator
- 0.18

### Fixed

- larger file info popup
- lock

### Style

- catppuccin themes
## 0.17.0

Released on 2025-03-23

### Added

- **cli:** added `--wno-keyring` flag to disable keyring
- 132 queuing transfers (#332)
  > the logic of selecting files has been extended!
  > From now on selecting file will put the files into a transfer queue, which is shown on the bottom panel.
  > When a file is selected the file is added to the queue with a destination path, which is the **current other explorer path at the moment of selection.
  > It is possible to navigate to the transfer queue by using `P` and pressing `ENTER` on a file will remove it from the transfer queue.Other commands will work as well on the transfer queue, like `COPY`, `MOVE`, `DELETE`, `RENAME`.

### CI

- **build:** build vendored smb and refactor platform deps (#333)

### Documentation

- **CONTRIBUTING:** docs(CONTRIBUTING): mistakes
- veeso.me instead of veeso.dev
- version

### Fixed

- unused import isolated tests
- isolated-tests for localhost
- clippy error
- **ui:** fixed input mask on host bridge on local dir up
  > if you go up on local dir when localhost is selected it panics
- fixed a crash when the local directory specified in the auth form does not exist
- the return value of `--version` should be `0`
- **aws-s3:** updated remotefs-aws-s3 to 0.4.1
  > should fix #329
- **log:** add suppaftp/pavao/kube to allowed logs
- **bookmarks:** Local directory path is not switching to what's specified in the bookmark

### Testing

- **remotefs_builder:** check result, build doesn't panic anymore

### Build

- **deps:** updated dependencies and edition to 2024
- bump to ssh2-config 0.4 and remotefs 0.6 to have support for Include in config files
- aws-s3 0.4.2
- build docker for x86
- so apparently native-tls vendored tries to build openssl on windows, wtf guys?
## 0.16.1

Released on 2024-11-12

### Fixed

- cfg unix forbidden in rust .82
- gg rust 1.82 for introducing a nice breaking change in config which was not mentioned in changelog
- 0.16.1
## 0.16.0

Released on 2024-10-14

### Added

- version 0.16
- Show `..` directory before all the others in the explorer to navigate to the parent dir (#301)

### Fixed

- Use `uzers` instead of the dead package `users` which has several vulnerabilities (#295)
- vergen (#296)
- users from lock
- tuirealm 2.x (#299)
- issue 292 New version alert was not displayed due to a semver regex issue. (#300)
- 0.16
- tiny ui issue
## 0.15.0

Released on 2024-10-03

### Added

- init 0.15
- Pods and container explorer for Kube protocol (#281)
- it is now possible to cancel find command; show find progress (#284)

### Fixed

- tokio rt builder
- dbus deveL
- bump vers
- keyring test not passing macos
- notify 6
- bump vers
- don't clear screen after terminating termscp
- issue 277 Fix a bug in the configuration page, which caused being stuck if the added SSH key was empty
- popup texts
- `isolated-tests` feature to run tests for releasing on distributions which run in isolated environments (#286)
  > * fix: `isolated-tests` feature to run tests for releasing on distributions which run in isolated environments
  > 
  > * fix: cond
- set date
- github ci is stable and reliable (one worker broken each 2 weeks)
- ci
- readme
- include build.rs
## 0.14.0

Released on 2024-07-17

### Added

- ALT+A to deselect all files (#263)
- ssh-agent (#264)
- issue 256 - filter files (#266)
- kube protocol support (#267)
- termscp 0.14

### Documentation

- update Arch Linux instructions (#243)

### Fixed

- install script version
- correct help text for update subcommand (#259)
- lint
- CLI remote args cannot handle '@' in the username (#261)
- sorted flags in readme
- Jump to next entry after select (#262)
- remotefs-ssh 0.3.1
- german manual
- removed support for RPM
- changelog
## 0.13.0

Released on 2024-03-02

### Added

- WebDAV support (#235)
- termscp 0.13.0

### Fixed

- AWS S3 wasn't working anymore due to rust-s3 outdate
- test
- debian script
- debian script
- lint???
## 0.12.2

Released on 2023-10-01

### Added

- tui-realm 1.9

### Fixed

- fmt
- panic if the terminal screen is too small
## 0.12.1

Released on 2023-07-06

### Added

- smb is now an optional feature (#200)
- build artifacts workflow (#202)

### Fix

- Some URL typos on 'install.sh' and 'README.md'. (#197)

### Fixed

- install workflow
- sh compliant macos.sh build script
- readme site links
- better main runner
- deps
- don't run CI on site/.md change
- rustup target
- don't update path breadcrumb if enter/scan dir failed (#203)
## 0.12.0

Released on 2023-05-16

### Added

- allow unknown fields in ssh2 configuration file (#181)

### Fixed

- #153 show a loading message when loading directory's content (#180)
- specify ssh2 config params
- release date
- build
- pavao 0.2.2
- pavao 0.2.3
- macos script
- release date
## 0.11.3

Released on 2023-04-19

### Added

- site improvements

### Fixed

- relative paths windows (#167)
## 0.11.2

Released on 2023-04-18

### Added

- dependencies up-to-date
- site 0.11.2
## 0.8.1

Released on 2022-03-22

### Fixed

- footer listed "Delete" shortcut as "Make Dir"
## 0.8.0

Released on 2022-01-06

### Arch

- install rust only if not found on local system
## 0.7.0

Released on 2021-10-12

### Option

- prompt user when about to replace an existing file caused by a file transfer
## 0.6.1

Released on 2021-08-30

### Fixed

- When copying files with tricky copy, the upper progress bar shows no text
## 0.5.1

Released on 2021-06-21

### Fix

- target_family unix means also macos and linux; use BSD target_os
## 0.5.0

Released on 2021-05-23

### Coverage

- githubActions
- githubActions

### Grcov

- exclude activities
## 0.4.1

Released on 2021-04-06

### FTP

- transfer type set to binary
- added support for symlinks for Linux servers

### Readme

- one-liner for Homebrew
  > The one-liner command
  > 
  >   brew install veeso/termscp/termscp
  > 
  > is equivalent to the two commands
  > 
  >   brew tap veeso/termscp
  >   brew install termscp

### SCP

- fixed symlink not properly detected
## 0.4.0

Released on 2021-03-27

### Clippy

- don't allow warnings

### Codecov

- ignore activities, context, input but not layout/

### PropsBuilder

- use from trait

### View

- return String instead of id
## 0.3.3

Released on 2021-02-28

### Git

- check for new updates (utils)
## 0.3.2

Released on 2021-01-24

### Testing

- don't run on windows
## 0.3.0

Released on 2021-01-10

### AuthActivity

- enter setup with <CTRL+C>

### ConfigClient

- return key path, not content

### Docs

- private keys with passwords

### Explorers

- append '/' to directories name

### FileTransferActivity

- load ConfigClient; set text editor to configuration's value
- :Explorer refactoring; toggle hidden files with <A>
- sort files with <B>

### FsEntry

- :get_name() returns &str
- :is_hidden() method

### SetupActivity

- <CTRL+E> as <DEL>
## 0.2.0

Released on 2020-12-21

### FsEntry

- :is_file

### Scp

- when username was not provided, it didn't fallback to current username
## 0.1.2

Released on 2020-12-13

### FsEntry

- :*::symlink is now a Option<Box<FsEntry>>; this improved symlinks, which gave errors some times
## 0.1.0

Released on 2020-12-06

### FileTransferActivity

- ftp is now allowed

### Host

- :mkdir and Host::remove
- mkdir support for absolute path

### Scp

- use oneshot channels instead of ptys (more stable; more reliable and overall works)

### Std

- :fmt::Display for HostError
- :fmt::Display for HostError
