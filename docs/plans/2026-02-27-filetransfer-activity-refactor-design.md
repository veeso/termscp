# FileTransferActivity Refactor Design

**Date:** 2026-02-27
**Approach:** Unified Pane Abstraction (incremental)
**Scope:** Comprehensive refactor of `src/ui/activities/filetransfer/` (~13,071 lines across 43 files)

## Problem Statement

`FileTransferActivity` is a god-struct with ~13k lines split across 43 files. The root cause of complexity is the asymmetric dual-pane architecture: the local side uses `Box<dyn HostBridge>` and the remote side uses `Box<dyn RemoteFs>`, forcing every action to have mirrored `_local_*` / `_remote_*` implementations.

### Current Pain Points

1. **Local/remote duplication**: Every action has two implementations that differ only in which filesystem client and which explorer they access
2. **God-struct**: `FileTransferActivity` holds 12+ fields and all behavior is scattered across 7 `impl` blocks in different files
3. **Monolithic files**: `popups.rs` (1,868 lines, 25+ components), `session.rs` (1,359 lines, 4 concerns), `view.rs` (1,211 lines, 30+ mount/umount pairs)
4. **`misc.rs`**: Grab-bag with no cohesive responsibility (600 lines)
5. **92 `assert!(…is_ok())` calls**: Panic-as-error-handling for UI operations
6. **Spin-loop synchronization**: `save.rs` uses `wait_for_pending_msg()` which blocks in a spin loop
7. **9 explicit `panic!()` calls** in `update.rs` for "impossible" tab states

## Key Insight

`RemoteBridged` already bridges `RemoteFs` -> `HostBridge`. Both sides CAN use the same `HostBridge` trait. The remote `RemoteFs` client just needs to be wrapped in `RemoteBridged` at construction time.

## Design

### 1. Pane Abstraction

Replace the asymmetric fields:

```rust
// BEFORE
pub struct FileTransferActivity {
    host_bridge: Box<dyn HostBridge>,    // local side
    client: Box<dyn RemoteFs>,           // remote side - different trait!
    browser: Browser,                     // holds two FileExplorers
    host_bridge_connected: bool,
    remote_connected: bool,
    // ...
}
```

With symmetric panes:

```rust
pub struct Pane {
    fs: Box<dyn HostBridge>,
    explorer: FileExplorer,
    connected: bool,
}

pub struct Browser {
    local: Pane,
    remote: Pane,
    found: Option<Found>,
    tab: FileExplorerTab,
    sync_browsing: bool,
}
```

### 2. Browser Navigation API

Three accessors handle all routing, including the Find tab special case:

```rust
impl Browser {
    /// The pane whose filesystem is targeted by operations.
    /// FindHostBridge -> local pane, FindRemote -> remote pane.
    pub fn fs_pane(&self) -> &Pane;
    pub fn fs_pane_mut(&mut self) -> &mut Pane;

    /// The explorer to read user selections from.
    /// For Find tabs, returns the found explorer.
    /// For normal tabs, returns the pane's explorer.
    pub fn selection_explorer(&self) -> &FileExplorer;

    /// The opposite pane (transfer destination).
    pub fn opposite_pane(&self) -> &Pane;
    pub fn opposite_pane_mut(&mut self) -> &mut Pane;

    /// Is the current tab a Find result tab?
    pub fn is_find_tab(&self) -> bool;
}
```

### 3. Action Deduplication

Every `_local_*` / `_remote_*` pair collapses:

```rust
// BEFORE: two methods
fn action_local_delete(&mut self) { ... self.host_bridge.remove(...) ... }
fn action_remote_delete(&mut self) { ... self.client.remove_dir_all(...) ... }

// AFTER: one method
fn action_delete(&mut self) {
    let selected = self.browser.get_selected_entries();
    let pane = self.browser.fs_pane_mut();
    match selected {
        SelectedFile::One(entry) => pane.fs.remove(&entry),
        SelectedFile::Many(entries) => { for (e, _) in &entries { pane.fs.remove(e); } }
        SelectedFile::None => {}
    }
}
```

The `update.rs` dispatch simplifies correspondingly:

```rust
// BEFORE
TransferMsg::DeleteFile => match self.browser.tab() {
    FileExplorerTab::HostBridge => self.action_local_delete(),
    FileExplorerTab::Remote => self.action_remote_delete(),
    FileExplorerTab::Find* => self.action_find_delete(),
}

// AFTER
TransferMsg::DeleteFile => self.action_delete(),
```

### 4. Session Decomposition

Split `session.rs` (1,359 lines, 4 concerns) into:

- **`connection.rs`**: `connect()` / `disconnect()` / `get_connection_msg()`. With Pane, these become `pane.connect()` operations.
- **`transfer.rs`**: The recursive transfer engine. Direction becomes implicit: always "from source pane to destination pane" using `HostBridge::open_file()` / `create_file()` / `finalize_write()` on both sides.
- **`navigation.rs`**: `changedir()`, `reload_dir()`, path resolution. These become pane-level operations.

### 5. Popup Split

Split `components/popups.rs` (1,868 lines) into individual files under `components/popups/`:

Each of the ~25 popup components (CopyPopup, DeletePopup, RenamePopup, etc.) gets its own file. Purely mechanical.

### 6. View Reorganization

Split `view.rs` (1,211 lines) into:

- `view/layout.rs` — `init()`, `view()` render function
- `view/popups.rs` — all `mount_*/umount_*` popup lifecycle methods
- `view/file_list.rs` — `update_browser_file_list()`, `reload_*_filelist()`
- `view/status_bar.rs` — status bar refresh

Simplify the `view()` render chain with a popup priority table:

```rust
const POPUP_PRIORITY: &[Id] = &[Id::FatalPopup, Id::ErrorPopup, Id::QuitPopup, ...];

fn active_popup(&self) -> Option<&Id> {
    POPUP_PRIORITY.iter().find(|id| self.app.mounted(id))
}
```

### 7. Error Handling Cleanup

Replace 92 `assert!(…is_ok())` calls with:

```rust
fn try_mount(&mut self, result: ApplicationResult<()>) {
    if let Err(err) = result {
        error!("UI operation failed: {err}");
    }
}
```

Replace 9 `panic!()` calls in `update.rs` with log-and-return or proper error variants.

## Incremental Phases

Each phase produces a compilable, working codebase:

| Phase | Description | Risk | Scope |
|-------|-------------|------|-------|
| **1** | Split `popups.rs` into individual files; split `misc.rs` into coherent modules | Minimal | `components/popups/*`, `misc.rs` |
| **2** | Replace `assert!(…is_ok())` with error handling; replace `panic!()` with graceful fallbacks | Minimal | `view.rs`, `misc.rs`, `update.rs` |
| **3** | Create `Pane` struct; refactor `Browser` to hold two `Pane`s | Medium | `lib/browser.rs`, `mod.rs`, all callers |
| **4** | Migrate actions one-by-one to use `fs_pane()`/`selection_explorer()`/`opposite_pane()` | Medium | `actions/*.rs`, `update.rs` |
| **5** | Decompose `session.rs` into `connection.rs`, `transfer.rs`, `navigation.rs` | Medium | `session.rs` -> 3 files |
| **6** | Split `view.rs` into sub-modules; simplify render chain | Low | `view.rs` -> `view/*.rs` |

## Constraints

- **Keep tuirealm v3** as the UI framework
- **Keep the `remotefs` crate** ecosystem
- **Incremental delivery**: each phase is independently mergeable
- **No functional changes**: this is a pure refactor — behavior stays identical
