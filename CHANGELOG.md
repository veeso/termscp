# Changelog

- [Changelog](#changelog)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.1.1

Work in progress

- enhancements:
  - password prompt: ask before performing terminal clear
  - file explorer:
    - file names are now sorted ignoring capital letters
    - file names longer than 23, are now cut to 20 and followed by `...`
    - paths which exceed tab size in explorer are elided with the following formato `ANCESTOR[1]/.../PARENT/DIRNAME`
- keybindings:
  - `I`: show info about selected file or directory
  - Removed `CTRL`; just use keys now.
- bugfix:
  - prevent panic in set_progress, for progress values `> 100.0 or < 0.0`
- dependencies:
  - updated `textwrap` to `0.13.0`

## 0.1.0

Released on 06/12/2020

- First release
