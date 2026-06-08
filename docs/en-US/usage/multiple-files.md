# Working with multiple files

termscp lets you act on several files at once. Use these controls to build a selection.

- `<M>`: mark the highlighted file for selection.
- `<CTRL+A>`: select all files in the current directory.
- `<ALT+A>`: deselect all files.

Once a file is marked for selection, it is displayed with a highlighted background.

When a selection exists, only the selected files are processed for actions; the currently highlighted item is ignored. You can also work on multiple files while in the find results panel.

## Actions on a selection

All actions are available when working with multiple files, but some behave slightly differently. With a selection, the name you enter refers to the destination directory rather than a single destination file.

- **Copy**: you are prompted for a destination. With multiple files selected, this name is the destination directory where all the files are copied.
- **Rename**: same as copy, but the files are moved to the destination directory.
- **Save as**: same as copy, but the files are written to the destination directory.

## The transfer queue

If you select a file in a directory (for example `/home`) and then change directory, the file stays selected and is shown in the **transfer queue** in the bottom panel.

When a file is selected, the remote directory active at that moment is associated with its entry. If the file is later transferred, it is transferred to the remote directory associated with it.

### Example

Suppose you select the local file `/home/a.txt` while the remote panel is at `/tmp`, then you move to `/var`, select `/var/b.txt` while the remote panel is at `/home`, and finally perform a transfer. The result is:

- `/home/a.txt` is transferred to `/tmp/a.txt`
- `/var/b.txt` is transferred to `/home/b.txt`
