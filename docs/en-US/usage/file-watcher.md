# Keeping files in sync

The file watcher synchronizes local paths to a remote path. When a change is detected on a watched local path, it is propagated to the configured remote path within about 5 seconds.

You can watch as many paths as you like.

## Watching a path

1. Put the cursor on the local explorer, on the file or directory you want to keep synchronized.
2. In the remote panel, navigate to the directory where the changes should be reported.
3. Press `<T>`.
4. Answer `<YES>` to the popup.

## Unwatching a path

You can stop watching a path in two ways:

- Press `<T>` on the watched local path (or any of its subfolders).
- Press `<CTRL+T>`, then press `<ENTER>` on the path you want to unwatch.

## Propagated changes

The following changes are reported to the remote host:

- New files and file changes
- Files moved or renamed
- Files removed or unlinked

The watcher works in one direction only (local to remote). Changes made on the remote host are **not** synchronized back to the local host.
