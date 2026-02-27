# FileTransferActivity Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Incrementally refactor the 13k-line FileTransferActivity god-struct into a symmetric Pane-based architecture, eliminating local/remote duplication and splitting monolithic files.

**Architecture:** Introduce a `Pane` struct that encapsulates one side of the dual-pane browser (filesystem client + explorer state). Both local and remote sides use `Box<dyn HostBridge>` (the remote side wraps `RemoteFs` via the existing `RemoteBridged` adapter). Actions become pane-agnostic — they operate on "active pane" / "opposite pane" instead of dispatching local vs remote.

**Tech Stack:** Rust (edition 2024), tuirealm v3, remotefs 0.3.x, thiserror

**Design doc:** `docs/plans/2026-02-27-filetransfer-activity-refactor-design.md`

**Build command:** `cargo build --no-default-features` (minimal build without SMB/keyring for fast iteration)

**Test command:** `cargo test --no-default-features --features github-actions --no-fail-fast`

**Lint command:** `cargo clippy -- -Dwarnings`

---

## Phase 1: Split Monolithic Files

### Task 1.1: Split `popups.rs` into Individual Files

**Files:**
- Delete: `src/ui/activities/filetransfer/components/popups.rs` (the 1,868-line monolith)
- Create: 20 new files under `src/ui/activities/filetransfer/components/popups/`
- Modify: `src/ui/activities/filetransfer/components/popups/mod.rs` (new file, replaces the old `popups.rs`)

**Context:** The existing `popups.rs` file starts with `mod chmod; mod goto;` (already extracted). It then defines 26 component structs inline. Two files already exist in `popups/`: `chmod.rs` and `goto.rs`. The directory already exists because of those.

**Step 1: Create `popups/mod.rs` with module declarations and re-exports**

Delete the old `popups.rs` and replace it with a `popups/` directory containing `mod.rs`. The `mod.rs` should declare all sub-modules and re-export all public types.

Note: the old `popups.rs` already has `mod chmod;` and `mod goto;` at the top. It also defines a public constant `ATTR_FILES`.

```rust
// src/ui/activities/filetransfer/components/popups/mod.rs

mod chmod;
mod copy;
mod delete;
mod disconnect;
mod error;
mod file_info;
mod filter;
mod goto;
mod keybindings;
mod mkdir;
mod newfile;
mod open_with;
mod progress_bar;
mod quit;
mod rename;
mod replace;
mod save_as;
mod sorting;
mod status_bar;
mod symlink;
mod wait;
mod watcher;

// Re-exports
pub use self::chmod::ChmodPopup;
pub use self::copy::CopyPopup;
pub use self::delete::DeletePopup;
pub use self::disconnect::DisconnectPopup;
pub use self::error::{ErrorPopup, FatalPopup};
pub use self::file_info::{FileInfoPopup, ATTR_FILES};
pub use self::filter::FilterPopup;
pub use self::goto::GotoPopup;
pub use self::keybindings::KeybindingsPopup;
pub use self::mkdir::{MkdirPopup, SyncBrowsingMkdirPopup};
pub use self::newfile::NewfilePopup;
pub use self::open_with::OpenWithPopup;
pub use self::progress_bar::{ProgressBarFull, ProgressBarPartial};
pub use self::quit::QuitPopup;
pub use self::rename::RenamePopup;
pub use self::replace::ReplacePopup;
pub use self::save_as::SaveAsPopup;
pub use self::sorting::SortingPopup;
pub use self::status_bar::{StatusBarLocal, StatusBarRemote};
pub use self::symlink::SymlinkPopup;
pub use self::wait::{WaitPopup, WalkdirWaitPopup};
pub use self::watcher::{WatchedPathsList, WatcherPopup};
```

**Step 2: Extract each component into its own file**

For each struct currently inline in `popups.rs`, create a new file under `popups/`. Each file needs:
1. The `use` imports that struct needs (look at what it references from the old file's imports block at lines 1-27)
2. The struct definition with its `#[derive(MockComponent)]` and `impl Component` block
3. Any helper `impl` blocks for that struct

The groupings are:
- `copy.rs` — `CopyPopup` (old lines 29-114)
- `filter.rs` — `FilterPopup` (old lines 115-201)
- `delete.rs` — `DeletePopup` (old lines 202-266)
- `disconnect.rs` — `DisconnectPopup` (old lines 267-330)
- `error.rs` — `ErrorPopup` + `FatalPopup` (old lines 331-398)
- `file_info.rs` — `FileInfoPopup` + `ATTR_FILES` constant (old lines 399-507)
- `keybindings.rs` — `KeybindingsPopup` (old lines 508-704)
- `mkdir.rs` — `MkdirPopup` + `SyncBrowsingMkdirPopup` (old lines 705-788 + 1573-1645)
- `newfile.rs` — `NewfilePopup` (old lines 789-872)
- `open_with.rs` — `OpenWithPopup` (old lines 873-958)
- `progress_bar.rs` — `ProgressBarFull` + `ProgressBarPartial` (old lines 959-1034)
- `quit.rs` — `QuitPopup` (old lines 1035-1098)
- `rename.rs` — `RenamePopup` (old lines 1099-1184)
- `replace.rs` — `ReplacePopup` (old lines 1185-1260)
- `save_as.rs` — `SaveAsPopup` (old lines 1261-1346)
- `sorting.rs` — `SortingPopup` (old lines 1347-1403)
- `status_bar.rs` — `StatusBarLocal` + `StatusBarRemote` (old lines 1404-1483)
- `symlink.rs` — `SymlinkPopup` (old lines 1484-1572)
- `wait.rs` — `WaitPopup` + `WalkdirWaitPopup` (old lines 1646-1714)
- `watcher.rs` — `WatchedPathsList` + `WatcherPopup` (old lines 1715-1868)

For import paths: each popup file uses `use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};` (the absolute path) for message types. Look at existing `chmod.rs` and `goto.rs` for the pattern.

**Step 3: Verify compilation**

Run: `cargo build --no-default-features`
Expected: Compiles with no errors. All re-exports must match what `components/mod.rs` (lines 19-31) expects.

**Step 4: Commit**

```bash
git add -A src/ui/activities/filetransfer/components/popups/
git commit -m "refactor: split popups.rs into individual component files"
```

---

### Task 1.2: Split `misc.rs` into Coherent Modules

**Files:**
- Delete: `src/ui/activities/filetransfer/misc.rs`
- Create: `src/ui/activities/filetransfer/misc/mod.rs`
- Create: `src/ui/activities/filetransfer/misc/log.rs`
- Create: `src/ui/activities/filetransfer/misc/notify.rs`
- Create: `src/ui/activities/filetransfer/misc/filelist.rs`
- Create: `src/ui/activities/filetransfer/misc/host.rs`

**Context:** Current `misc.rs` (601 lines) has functions grouped by responsibility:

| Responsibility | Functions | Old lines |
|---|---|---|
| Event loop | `tick` | 23-40 |
| Logging | `log`, `log_and_alert`, `update_logbox` | 43-70, 424-468 |
| Config/host info | `init_config_client`, `setup_text_editor`, `host_bridge_to_abs_path`, `remote_to_abs_path`, `get_remote_hostname`, `get_hostbridge_hostname`, `get_hostname`, `get_connection_msg` | 72-182 |
| Notifications | `notify_transfer_completed`, `notify_transfer_error`, `transfer_completed_msg` | 184-239 |
| File list UI | `update_host_bridge_filelist`, `reload_host_bridge_filelist`, `update_remote_filelist`, `reload_remote_filelist`, `get_tab_hostname`, `terminal_prompt`, `update_logbox`, `update_progress_bar`, `finalize_find`, `update_find_list`, `update_browser_file_list`, `reload_browser_file_list`, `update_browser_file_list_swapped` | 241-599 |

All functions are `impl FileTransferActivity` methods, so moving them is purely a file reorganization — the `impl` blocks just move to new files.

**Step 1: Create `misc/mod.rs`**

```rust
// src/ui/activities/filetransfer/misc/mod.rs
mod filelist;
mod host;
mod log;
mod notify;

// tick() stays here since it's the event-loop driver
// (move the tick function from old misc.rs lines 23-40 here)
```

**Step 2: Create `misc/log.rs`**

Move `log`, `log_and_alert`, `update_logbox` (old lines 43-70 and 424-468).
These are `impl FileTransferActivity` methods that use `self.log_records`, `self.app`, and `self.redraw`.

**Step 3: Create `misc/notify.rs`**

Move `notify_transfer_completed`, `notify_transfer_error`, `transfer_completed_msg` (old lines 184-239).

**Step 4: Create `misc/filelist.rs`**

Move all file-list update/reload functions (old lines 241-599):
`update_host_bridge_filelist`, `reload_host_bridge_filelist`, `update_remote_filelist`, `reload_remote_filelist`, `get_tab_hostname`, `terminal_prompt`, `update_progress_bar`, `finalize_find`, `update_find_list`, `update_browser_file_list`, `reload_browser_file_list`, `update_browser_file_list_swapped`.

**Step 5: Create `misc/host.rs`**

Move config/host-info functions (old lines 72-182):
`init_config_client`, `setup_text_editor`, `host_bridge_to_abs_path`, `remote_to_abs_path`, `get_remote_hostname`, `get_hostbridge_hostname`, `get_hostname`, `get_connection_msg`.

**Step 6: Verify compilation**

Run: `cargo build --no-default-features`
Expected: Compiles with no errors.

**Step 7: Commit**

```bash
git add -A src/ui/activities/filetransfer/misc/ src/ui/activities/filetransfer/misc.rs
git commit -m "refactor: split misc.rs into log, notify, filelist, host modules"
```

---

## Phase 2: Error Handling Cleanup

### Task 2.1: Replace `assert!(…is_ok())` with Error Logging

**Files:**
- Modify: `src/ui/activities/filetransfer/view.rs` (~67 occurrences)
- Modify: `src/ui/activities/filetransfer/misc/filelist.rs` (~14 occurrences, was `misc.rs`)
- Modify: `src/ui/activities/filetransfer/update.rs` (~11 occurrences)

**Context:** The pattern throughout is:
```rust
assert!(self.app.mount(Id::Foo, Box::new(Widget::new()), vec![]).is_ok());
assert!(self.app.active(&Id::Foo).is_ok());
assert!(self.app.remount(Id::Foo, Box::new(Widget::new()), vec![]).is_ok());
assert!(self.app.attr(&Id::Foo, Attribute::Content, value).is_ok());
```

All of these panic on UI failures (which should never happen but currently crash the app if they do).

**Step 1: Add a helper method to `FileTransferActivity`**

In `src/ui/activities/filetransfer/mod.rs`, add inside the existing `impl FileTransferActivity` block:

```rust
/// Log a UI operation error instead of panicking.
fn ui_result<T>(&self, result: Result<T, impl std::fmt::Display>) {
    if let Err(err) = result {
        error!("UI operation failed: {err}");
    }
}
```

**Step 2: Replace all `assert!(self.app.…is_ok())` calls**

In each file, replace every occurrence of:
```rust
assert!(self.app.mount(...).is_ok());
// or
assert!(self.app.remount(...).is_ok());
// or
assert!(self.app.active(...).is_ok());
// or
assert!(self.app.attr(...).is_ok());
```

With:
```rust
self.ui_result(self.app.mount(...));
// or
self.ui_result(self.app.remount(...));
// or
self.ui_result(self.app.active(...));
// or
self.ui_result(self.app.attr(...));
```

Note: some calls return a value that is then checked. For those that use `assert!` purely for the side effect, the replacement is straightforward. For `self.app.active(...)` the return value is `ApplicationResult<()>`, so `self.ui_result(...)` works directly.

Process each file:
1. `view.rs` — ~67 occurrences across all `mount_*` and `init` functions
2. `misc/filelist.rs` — ~14 occurrences in `reload_host_bridge_filelist`, `reload_remote_filelist`, `update_logbox`, `update_progress_bar`, `finalize_find`, `update_find_list`
3. `update.rs` — ~11 occurrences in tab switching and bottom panel navigation

**Step 3: Verify compilation**

Run: `cargo build --no-default-features`

**Step 4: Commit**

```bash
git add src/ui/activities/filetransfer/
git commit -m "refactor: replace assert!(…is_ok()) with error logging in UI operations"
```

---

### Task 2.2: Replace `panic!()` Calls with Graceful Fallbacks

**Files:**
- Modify: `src/ui/activities/filetransfer/update.rs` (5 panic! calls)
- Modify: `src/ui/activities/filetransfer/view.rs` (3 panic! calls)

**Context:** There are 8 `panic!` calls total:

In `update.rs`:
- Line 60: `_ => panic!("Found tab doesn't support COPY")` — inside `TransferMsg::CopyFileTo`
- Line 72: `_ => panic!("Found tab doesn't support SYMLINK")` — inside `TransferMsg::CreateSymlink`
- Line 152: `_ => panic!("Found tab doesn't support EXEC")` — inside `TransferMsg::ExecuteCmd`
- Line 162: `_ => panic!("Found tab doesn't support GOTO")` — inside `TransferMsg::GoTo`
- Line 222: `_ => panic!("Trying to search for files, while already in a find result")` — inside `TransferMsg::InitFuzzySearch`

In `view.rs`:
- Line 557: `_ => panic!("Cannot mount terminal on this tab")` — inside `mount_exec`
- Line 563: `_ => panic!("Cannot mount terminal on this tab")` — inside `mount_exec`
- Line 591: `_ => panic!("Cannot update terminal prompt on this tab")` — inside terminal prompt update

**Step 1: Replace each panic with a log-and-return**

For each `panic!` in `update.rs`, replace with:
```rust
_ => {
    error!("Operation not supported on current tab");
}
```

For each `panic!` in `view.rs`, replace with:
```rust
_ => {
    error!("Cannot mount terminal on this tab");
    return;
}
```

**Step 2: Verify compilation**

Run: `cargo build --no-default-features`

**Step 3: Commit**

```bash
git add src/ui/activities/filetransfer/update.rs src/ui/activities/filetransfer/view.rs
git commit -m "refactor: replace panic!() with graceful error logging"
```

---

## Phase 3: Pane Struct and Browser Refactor

### Task 3.1: Create the `Pane` Struct

**Files:**
- Create: `src/ui/activities/filetransfer/lib/pane.rs`
- Modify: `src/ui/activities/filetransfer/lib/mod.rs`

**Context:** A `Pane` encapsulates one side of the dual-pane browser: the filesystem client + the file explorer state + connection tracking.

**Step 1: Create `pane.rs`**

```rust
// src/ui/activities/filetransfer/lib/pane.rs

use std::path::{Path, PathBuf};

use remotefs::File;

use crate::explorer::FileExplorer;
use crate::host::{HostBridge, HostResult};

/// One side of the dual-pane file browser.
/// Both local and remote sides have the same shape.
pub struct Pane {
    /// Unified filesystem operations (Localhost or RemoteBridged)
    pub(crate) fs: Box<dyn HostBridge>,
    /// File explorer state (directory listing, sorting, filtering, transfer queue)
    pub(crate) explorer: FileExplorer,
    /// Whether this pane has been connected at least once
    pub(crate) connected: bool,
}

impl Pane {
    /// Create a new Pane
    pub fn new(fs: Box<dyn HostBridge>, explorer: FileExplorer, connected: bool) -> Self {
        Self {
            fs,
            explorer,
            connected,
        }
    }

    /// Whether the underlying filesystem is connected
    pub fn is_connected(&mut self) -> bool {
        self.fs.is_connected()
    }

    /// Whether this is a localhost pane
    pub fn is_localhost(&self) -> bool {
        self.fs.is_localhost()
    }

    /// Connect the filesystem
    pub fn connect(&mut self) -> HostResult<()> {
        self.fs.connect()?;
        self.connected = true;
        Ok(())
    }

    /// Disconnect the filesystem
    pub fn disconnect(&mut self) -> HostResult<()> {
        self.fs.disconnect()
    }

    /// Absolutize a relative path against the current working directory
    pub fn to_abs_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            let mut abs = self.explorer.wrkdir.clone();
            abs.push(path);
            abs
        }
    }
}
```

**Step 2: Register module in `lib/mod.rs`**

Add `pub mod pane;` to `src/ui/activities/filetransfer/lib/mod.rs`.

**Step 3: Verify compilation**

Run: `cargo build --no-default-features`

**Step 4: Commit**

```bash
git add src/ui/activities/filetransfer/lib/
git commit -m "refactor: add Pane struct for symmetric browser sides"
```

---

### Task 3.2: Refactor `Browser` to Hold Two `Pane`s

**Files:**
- Modify: `src/ui/activities/filetransfer/lib/browser.rs`
- Modify: `src/ui/activities/filetransfer/mod.rs`

**Context:** Currently `Browser` holds two `FileExplorer`s. It needs to hold two `Pane`s instead. The `FileExplorer` fields move into `Pane.explorer`. The `Box<dyn HostBridge>` and `Box<dyn RemoteFs>` from `FileTransferActivity` move into the panes.

This is the trickiest task — it changes the ownership model. The `FileTransferActivity` fields `host_bridge`, `client`, `host_bridge_connected`, `remote_connected` all move into `Browser`'s panes.

**Step 1: Update `Browser` struct to hold `Pane`s**

Replace the `Browser` struct definition and constructor:

```rust
use super::pane::Pane;

pub struct Browser {
    local: Pane,
    remote: Pane,
    found: Option<Found>,
    tab: FileExplorerTab,
    sync_browsing: bool,
}

impl Browser {
    pub fn new(local: Pane, remote: Pane) -> Self {
        Self {
            local,
            remote,
            found: None,
            tab: FileExplorerTab::HostBridge,
            sync_browsing: false,
        }
    }
}
```

**Step 2: Add the navigation API methods**

Add to `impl Browser`:

```rust
/// The pane whose filesystem is targeted by the current tab.
/// FindHostBridge -> local pane, FindRemote -> remote pane.
pub fn fs_pane(&self) -> &Pane {
    match self.tab {
        FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.local,
        FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.remote,
    }
}

pub fn fs_pane_mut(&mut self) -> &mut Pane {
    match self.tab {
        FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &mut self.local,
        FileExplorerTab::Remote | FileExplorerTab::FindRemote => &mut self.remote,
    }
}

/// The opposite pane (transfer destination).
pub fn opposite_pane(&self) -> &Pane {
    match self.tab {
        FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.remote,
        FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.local,
    }
}

pub fn opposite_pane_mut(&mut self) -> &mut Pane {
    match self.tab {
        FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &mut self.remote,
        FileExplorerTab::Remote | FileExplorerTab::FindRemote => &mut self.local,
    }
}

/// Is the current tab a Find result tab?
pub fn is_find_tab(&self) -> bool {
    matches!(self.tab, FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote)
}
```

**Step 3: Update existing `Browser` methods to use `Pane.explorer`**

Every method that currently accesses `self.host_bridge` (the `FileExplorer`) should access `self.local.explorer` instead. Every method accessing `self.remote` (the `FileExplorer`) should access `self.remote.explorer`.

Key methods to update:
- `explorer()` / `explorer_mut()` — dispatch `self.local.explorer` / `self.remote.explorer` based on tab
- `other_explorer_no_found()` — dispatch to `self.remote.explorer` / `self.local.explorer`
- `host_bridge()` / `host_bridge_mut()` — return `&self.local.explorer` / `&mut self.local.explorer`
- `remote()` / `remote_mut()` — return `&self.remote.explorer` / `&mut self.remote.explorer`
- `toggle_terminal()`, `is_terminal_open_host_bridge()`, `is_terminal_open_remote()` — use `.explorer`
- Builder methods `build_local_explorer`, `build_remote_explorer` become standalone functions (they create `FileExplorer`s that go into `Pane` construction)

**Step 4: Update `FileTransferActivity` to remove moved fields**

In `mod.rs`, remove:
- `host_bridge: Box<dyn HostBridge>` — now in `browser.local.fs`
- `client: Box<dyn RemoteFs>` — now in `browser.remote.fs` (wrapped in `RemoteBridged`)
- `host_bridge_connected: bool` — now in `browser.local.connected`
- `remote_connected: bool` — now in `browser.remote.connected`

Update the constructor `FileTransferActivity::new()`:
- Build the `Pane`s with `Pane::new(fs, explorer, connected)`
- Pass both panes to `Browser::new(local_pane, remote_pane)`
- The `RemoteFs` client needs to be wrapped: `RemoteBridged::new(client)` to get a `Box<dyn HostBridge>`

Also update/remove the convenience accessors on `FileTransferActivity`:
- `host_bridge()` / `host_bridge_mut()` — now delegate to `self.browser.host_bridge()` (returns `&FileExplorer`)
- `remote()` / `remote_mut()` — now delegate to `self.browser.remote()`
- `found()` / `found_mut()` — delegate to `self.browser.found()`
- `context()`, `context_mut()`, `config()`, `theme()` — stay as-is (unaffected)

**Step 5: Fix all compilation errors**

This step will have many compilation errors. The approach:

1. Start with `mod.rs` — fix the constructor and struct definition
2. Fix `session.rs` — replace `self.host_bridge.method()` with `self.browser.local.fs.method()` and `self.client.method()` with `self.browser.remote.fs.method()`. This is a mechanical find-and-replace for now (action deduplication comes in Phase 4).
3. Fix each `actions/*.rs` file — same mechanical replacement
4. Fix `fswatcher.rs` — same
5. Fix `update.rs` — same
6. Fix `view.rs` — same
7. Fix `misc/*.rs` — same

For each `self.host_bridge.some_method()` call (HostBridge method), replace with `self.browser.local.fs.some_method()`.
For each `self.client.some_method()` call (RemoteFs method), replace with `self.browser.remote.fs.some_method()`.

Important: `self.browser.remote.fs` is now a `Box<dyn HostBridge>` (via `RemoteBridged`), NOT `Box<dyn RemoteFs>`. This means remote calls like `self.client.remove_dir_all(path)` must change to `self.browser.remote.fs.remove(&entry)` — using the `HostBridge` API.

Some RemoteFs-specific calls in `session.rs` that have NO HostBridge equivalent:
- `self.client.create(path, metadata)` → `self.browser.remote.fs.create_file(path, metadata)` (HostBridge equivalent)
- `self.client.on_written(writer)` → `self.browser.remote.fs.finalize_write(writer)` (HostBridge equivalent)
- `self.client.open(path)` → `self.browser.remote.fs.open_file(path)` (HostBridge equivalent)
- `self.client.on_read(reader)` → no-op (HostBridge doesn't have `on_read`, and `RemoteBridged::open_file` handles this internally)
- `self.client.create_file(path, metadata, reader)` → use `create_file` + `io::copy` + `finalize_write` (the HostBridge streaming pattern)
- `self.client.open_file(path, writer)` → use `open_file` + `io::copy` (the HostBridge streaming pattern)

These are the hardest conversions and should be done carefully in `session.rs`. `RemoteBridged` handles the streaming abstraction internally — it buffers through temp files when the remote backend doesn't support streaming.

**Step 6: Verify compilation and run tests**

Run: `cargo build --no-default-features`
Run: `cargo clippy -- -Dwarnings`

**Step 7: Commit**

```bash
git add src/ui/activities/filetransfer/
git commit -m "refactor: move filesystem clients into Browser Panes"
```

---

## Phase 4: Action Deduplication

### Task 4.1: Unify Selection Resolution

**Files:**
- Modify: `src/ui/activities/filetransfer/actions/mod.rs`

**Context:** Currently there are three methods: `get_local_selected_entries()`, `get_remote_selected_entries()`, `get_found_selected_entries()`. These can be unified since both sides now use the same `HostBridge` trait for `stat()`.

**Step 1: Add a unified `get_selected_entries()` method**

```rust
/// Get selected entries from the active explorer.
/// Uses the selection_explorer for UI state and fs_pane for stat calls.
pub(crate) fn get_selected_entries(&mut self) -> SelectedFile {
    let id = match self.browser.tab() {
        FileExplorerTab::HostBridge => Id::ExplorerHostBridge,
        FileExplorerTab::Remote => Id::ExplorerRemote,
        FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => Id::ExplorerFind,
    };
    self.get_selected_files(&id)
}
```

Also unify `get_file_from_path` to use `self.browser.fs_pane_mut().fs.stat(path)` instead of dispatching on `Id`:

```rust
fn get_file_from_path(&mut self, _id: &Id, path: &Path) -> Option<File> {
    self.browser.fs_pane_mut().fs.stat(path).ok()
}
```

**Step 2: Keep old methods as deprecated wrappers (temporary)**

Don't delete the old methods yet — just have them call the new unified one. This avoids a big-bang change in all callers.

**Step 3: Verify compilation**

Run: `cargo build --no-default-features`

**Step 4: Commit**

```bash
git add src/ui/activities/filetransfer/actions/mod.rs
git commit -m "refactor: add unified get_selected_entries() method"
```

---

### Task 4.2: Unify Simple Actions (mkdir, delete, symlink, chmod, rename, copy)

**Files:**
- Modify: `src/ui/activities/filetransfer/actions/mkdir.rs`
- Modify: `src/ui/activities/filetransfer/actions/delete.rs`
- Modify: `src/ui/activities/filetransfer/actions/symlink.rs`
- Modify: `src/ui/activities/filetransfer/actions/chmod.rs`
- Modify: `src/ui/activities/filetransfer/actions/rename.rs`
- Modify: `src/ui/activities/filetransfer/actions/copy.rs`
- Modify: `src/ui/activities/filetransfer/update.rs`

**Context:** Each of these has a `_local_*` / `_remote_*` pair. With both sides now being `Box<dyn HostBridge>`, the implementations are identical. Replace each pair with a single method that operates on `self.browser.fs_pane_mut().fs`.

**Step 1: Unify `mkdir`**

Replace `action_local_mkdir` + `action_remote_mkdir` with:
```rust
pub(crate) fn action_mkdir(&mut self, input: String) {
    match self.browser.fs_pane_mut().fs.mkdir(PathBuf::from(input.as_str()).as_path()) {
        Ok(_) => self.log(LogLevel::Info, format!("Created directory \"{input}\"")),
        Err(err) => self.log_and_alert(
            LogLevel::Error,
            format!("Could not create directory \"{input}\": {err}"),
        ),
    }
}
```

**Step 2: Unify `delete`**

Replace `action_local_delete` + `action_remote_delete` + `action_find_delete` with:
```rust
pub(crate) fn action_delete(&mut self) {
    match self.get_selected_entries() {
        SelectedFile::One(entry) => {
            self.remove_file(&entry);
        }
        SelectedFile::Many(entries) => {
            for (entry, _) in entries.iter() {
                self.remove_file(entry);
            }
            self.browser.fs_pane_mut().explorer.clear_queue();
        }
        SelectedFile::None => {}
    }
}

fn remove_file(&mut self, entry: &File) {
    match self.browser.fs_pane_mut().fs.remove(entry) {
        Ok(_) => self.log(LogLevel::Info, format!("Removed file \"{}\"", entry.path().display())),
        Err(err) => self.log_and_alert(
            LogLevel::Error,
            format!("Could not delete file \"{}\": {}", entry.path().display(), err),
        ),
    }
}
```

**Step 3: Unify remaining actions**

Apply the same pattern to `symlink`, `chmod`, `rename`, `copy`. For `rename`, note the `UnsupportedFeature` fallback — since `HostBridge::rename` is now used on both sides, and `RemoteBridged::rename` handles the `UnsupportedFeature` internally (or should), this may simplify. Check `RemoteBridged::rename` implementation to confirm.

**Step 4: Update `update.rs` dispatch**

Replace the `match self.browser.tab()` dispatches for each unified action. Example for mkdir:

```rust
// BEFORE
TransferMsg::Mkdir(dir) => {
    match self.browser.tab() {
        FileExplorerTab::HostBridge => self.action_local_mkdir(dir),
        FileExplorerTab::Remote => self.action_remote_mkdir(dir),
        _ => {}
    }
    ...
}

// AFTER
TransferMsg::Mkdir(dir) => {
    self.action_mkdir(dir);
    ...
}
```

**Step 5: Remove old `_local_*` / `_remote_*` methods**

Delete the now-unused method pairs.

**Step 6: Verify compilation**

Run: `cargo build --no-default-features`
Run: `cargo clippy -- -Dwarnings`

**Step 7: Commit**

```bash
git add src/ui/activities/filetransfer/
git commit -m "refactor: unify local/remote action pairs into single methods"
```

---

### Task 4.3: Unify Navigation Actions (change_dir, go_to_upper_dir, go_to_previous_dir)

**Files:**
- Modify: `src/ui/activities/filetransfer/actions/change_dir.rs`
- Modify: `src/ui/activities/filetransfer/update.rs`

**Context:** Navigation actions have local/remote mirrors that differ in which explorer/filesystem they access. With Pane, they can use `fs_pane()`.

**Step 1: Unify navigation methods**

Replace the 6 navigation functions (3 pairs) with 3 unified methods:

```rust
pub(crate) fn action_enter_dir(&mut self, dir: File) {
    self.changedir(dir.path(), true);
    if self.browser.sync_browsing && !self.browser.is_find_tab() {
        self.synchronize_browsing(SyncBrowsingDestination::Path(dir.name()));
    }
}

pub(crate) fn action_change_dir(&mut self, input: String) {
    let dir_path = self.browser.fs_pane().to_abs_path(PathBuf::from(input.as_str()).as_path());
    self.changedir(dir_path.as_path(), true);
    if self.browser.sync_browsing && !self.browser.is_find_tab() {
        self.synchronize_browsing(SyncBrowsingDestination::Path(input));
    }
}

pub(crate) fn action_go_to_upper_dir(&mut self) {
    let path = self.browser.fs_pane().explorer.wrkdir.clone();
    if let Some(parent) = path.as_path().parent() {
        self.changedir(parent, true);
        if self.browser.sync_browsing && !self.browser.is_find_tab() {
            self.synchronize_browsing(SyncBrowsingDestination::ParentDir);
        }
    }
}

pub(crate) fn action_go_to_previous_dir(&mut self) {
    if let Some(d) = self.browser.fs_pane_mut().explorer.popd() {
        self.changedir(d.as_path(), false);
        if self.browser.sync_browsing && !self.browser.is_find_tab() {
            self.synchronize_browsing(SyncBrowsingDestination::PreviousDir);
        }
    }
}
```

Where `changedir` is a unified method that calls `self.browser.fs_pane_mut().fs.change_wrkdir()` and updates the explorer.

**Step 2: Update `synchronize_browsing` to use `opposite_pane()`**

The sync browsing logic should use `self.browser.opposite_pane_mut()` instead of dispatching on tab:

```rust
fn synchronize_browsing(&mut self, destination: SyncBrowsingDestination) {
    // Resolve path on the opposite side
    let path = self.resolve_sync_destination(&destination);
    // Check existence on opposite pane
    let exists = self.browser.opposite_pane_mut().fs.exists(path.as_path());
    // Change dir on opposite pane
    // ... (uses opposite_pane_mut throughout)
}
```

**Step 3: Update `update.rs` dispatch**

**Step 4: Verify compilation**

Run: `cargo build --no-default-features`

**Step 5: Commit**

```bash
git add src/ui/activities/filetransfer/
git commit -m "refactor: unify navigation actions using Pane abstraction"
```

---

### Task 4.4: Unify Remaining Actions (exec, open, edit, saveas, submit, find)

**Files:**
- Modify: `src/ui/activities/filetransfer/actions/exec.rs`
- Modify: `src/ui/activities/filetransfer/actions/open.rs`
- Modify: `src/ui/activities/filetransfer/actions/edit.rs`
- Modify: `src/ui/activities/filetransfer/actions/save.rs`
- Modify: `src/ui/activities/filetransfer/actions/submit.rs`
- Modify: `src/ui/activities/filetransfer/actions/find.rs`
- Modify: `src/ui/activities/filetransfer/actions/walkdir.rs`
- Modify: `src/ui/activities/filetransfer/update.rs`

**Context:** These are more complex actions with additional logic beyond simple filesystem calls. Apply the same Pane-based pattern but pay attention to:

- `edit.rs` has `is_localhost()` checks — the Pane carries this info via `pane.is_localhost()`
- `open.rs` needs to know if the file is local (for `open::that()` calls)
- `save.rs` / transfer actions involve BOTH panes (source + destination)
- `find.rs` / `walkdir.rs` scan one pane's filesystem

**Step 1:** Unify each file one at a time, compiling between each.

**Step 2:** For transfer actions (`action_local_send`, `action_remote_recv`, `action_find_transfer`), unify into `action_transfer()` that reads from `fs_pane()` and writes to `opposite_pane()`.

**Step 3: Verify compilation and run clippy**

Run: `cargo build --no-default-features`
Run: `cargo clippy -- -Dwarnings`

**Step 4: Commit**

```bash
git add src/ui/activities/filetransfer/
git commit -m "refactor: unify remaining action pairs (exec, open, edit, save, find)"
```

---

## Phase 5: Session Decomposition

### Task 5.1: Split `session.rs` into Three Modules

**Files:**
- Delete: `src/ui/activities/filetransfer/session.rs`
- Create: `src/ui/activities/filetransfer/session/mod.rs`
- Create: `src/ui/activities/filetransfer/session/connection.rs`
- Create: `src/ui/activities/filetransfer/session/transfer.rs`
- Create: `src/ui/activities/filetransfer/session/navigation.rs`

**Context:** `session.rs` currently has ~1,359 lines mixing four concerns:
1. **Connection** (~100 lines): `connect_to_host_bridge`, `connect_to_remote`, `disconnect`, `disconnect_and_quit`
2. **Navigation** (~200 lines): `host_bridge_changedir`, `remote_changedir`, `reload_host_bridge_dir`, `reload_remote_dir`, abs-path resolution, `local_to_abs_path`, `remote_to_abs_path`
3. **Transfer engine** (~900 lines): `filetransfer_send`, `filetransfer_recv`, recursive send/recv, streaming send/recv, progress tracking
4. **Temp file** (~50 lines): `download_file_as_tmp`

**Step 1: Create `session/mod.rs`**

```rust
mod connection;
mod navigation;
mod transfer;

pub(super) use self::transfer::TransferPayload;
```

**Step 2: Create `session/connection.rs`**

Move: `connect_to_host_bridge`, `connect_to_remote`, `disconnect`, `disconnect_and_quit`.

With the Pane model, connection simplifies to calling `self.browser.local.connect()` / `self.browser.remote.connect()`.

**Step 3: Create `session/navigation.rs`**

Move: `host_bridge_changedir`, `remote_changedir`, `reload_host_bridge_dir`, `reload_remote_dir`.

With the Pane model, these should be refactored to `changedir(&mut self)` operating on the active pane, and `reload_dir(&mut self)` operating on a specific pane. The local/remote split goes away.

**Step 4: Create `session/transfer.rs`**

Move: all `filetransfer_send*`, `filetransfer_recv*`, `download_file_as_tmp`, `TransferPayload`, `TransferError`, and the streaming helpers.

The transfer engine operates on `source_pane.fs` (read) and `dest_pane.fs` (write). With both sides being `HostBridge`, the send/recv distinction collapses: it's always "read from one pane, write to the other."

**Step 5: Verify compilation**

Run: `cargo build --no-default-features`

**Step 6: Commit**

```bash
git add src/ui/activities/filetransfer/session/ src/ui/activities/filetransfer/session.rs
git commit -m "refactor: split session.rs into connection, navigation, transfer modules"
```

---

## Phase 6: View Reorganization

### Task 6.1: Split `view.rs` into Sub-modules

**Files:**
- Delete: `src/ui/activities/filetransfer/view.rs`
- Create: `src/ui/activities/filetransfer/view/mod.rs`
- Create: `src/ui/activities/filetransfer/view/layout.rs`
- Create: `src/ui/activities/filetransfer/view/popups.rs`
- Create: `src/ui/activities/filetransfer/view/status.rs`

**Context:** `view.rs` (1,211 lines) contains:
- `init()` (~90 lines) — mounts all base components
- `view()` (~160 lines) — the render function with the popup if/else chain
- `mount_*/umount_*` popup methods (~700 lines) — ~30 pairs
- `refresh_*` methods (~150 lines) — status bars, transfer queues
- `mount_global_listener` (~50 lines)

**Step 1: Create `view/mod.rs`**

```rust
mod layout;
mod popups;
mod status;
```

**Step 2: Create `view/layout.rs`**

Move: `init()`, `view()`, `mount_global_listener`.

Simplify `view()` with a popup priority table:

```rust
const POPUP_RENDER_ORDER: &[Id] = &[
    Id::FatalPopup,
    Id::ErrorPopup,
    Id::WaitPopup,
    Id::ProgressBarFull,
    // ... all popups in priority order
];
```

**Step 3: Create `view/popups.rs`**

Move: all `mount_*` / `umount_*` methods for popups.

**Step 4: Create `view/status.rs`**

Move: `refresh_local_status_bar`, `refresh_remote_status_bar`, `refresh_host_bridge_transfer_queue`, `refresh_remote_transfer_queue`.

**Step 5: Verify compilation**

Run: `cargo build --no-default-features`

**Step 6: Commit**

```bash
git add src/ui/activities/filetransfer/view/ src/ui/activities/filetransfer/view.rs
git commit -m "refactor: split view.rs into layout, popups, status sub-modules"
```

---

## Final Verification

### Task 7.1: Full Build and Lint Check

**Step 1:** Run full build with all features:
```bash
cargo build
```

**Step 2:** Run clippy:
```bash
cargo clippy -- -Dwarnings
```

**Step 3:** Run format check:
```bash
cargo fmt --all -- --check
```

**Step 4:** Fix any issues found.

**Step 5: Commit any fixes**

```bash
git add -A
git commit -m "fix: address clippy and formatting issues from refactor"
```
