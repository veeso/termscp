# File explorer format

You can define a custom format for the file explorer through the configuration.
This is possible for both the local and the remote host, so you can use two
different syntaxes. The fields are named **File formatter syntax (local)** and
**File formatter syntax (remote)**, and they define how the file entries are
displayed in the file explorer.

## Syntax

The syntax for the formatter is the following:

```text
{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...
```

Each key in braces is replaced with the related attribute, while everything
outside braces is left unchanged.

- The key name is mandatory and must be one of the keys below.
- `LENGTH` describes the width reserved to display the field. Static attributes
  do not support it (`GROUP`, `PEX`, `SIZE`, `USER`).
- `EXTRA` is supported only by some keys and provides an additional option. See
  the keys below to check whether `EXTRA` is supported.

## Keys

These are the keys supported by the formatter:

| Key       | Description                                                                                      |
| --------- | ------------------------------------------------------------------------------------------------ |
| `ATIME`   | Last access time (default `%b %d %Y %H:%M`); `EXTRA` is the time format (e.g. `{ATIME:8:%H:%M}`) |
| `CTIME`   | Creation time (default `%b %d %Y %H:%M`); `EXTRA` is the time format (e.g. `{CTIME:8:%H:%M}`)    |
| `GROUP`   | Owner group                                                                                      |
| `MTIME`   | Last change time (default `%b %d %Y %H:%M`); `EXTRA` is the time format (e.g. `{MTIME:8:%H:%M}`) |
| `NAME`    | File name (folders between root and first ancestors are elided if longer than `LENGTH`)          |
| `PATH`    | File absolute path (folders between root and first ancestors are elided if longer than `LENGTH`) |
| `PEX`     | File permissions (UNIX format)                                                                   |
| `SIZE`    | File size (omitted for directories)                                                              |
| `SYMLINK` | Symlink target (if any, `-> {FILE_PATH}`)                                                        |
| `USER`    | Owner user                                                                                       |

## Default format

If left empty, the default formatter syntax is used:

```text
{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}
```
