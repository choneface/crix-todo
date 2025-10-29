# 🧾 Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.0.0] – 2025-10-07
### Saracen (Major Release)

“Clear eyes. Full hearts. No distractions.”

Version **1.0.0 (Saracen)** marks the first major release of `crix-todo`.  
This release is a complete overhaul focused on speed, clarity, and simplicity.

#### Added
- Unified single-command workflow — launch everything with:
  ```
  todo
  ```
- Inline todo creation (`=`) directly within the TUI
- Inline editing for titles and notes (`i`)
- Undo functionality (`u`) to revert the last action
- Tree-style visual layout for todos and notes
- Local JSON storage with instant persistence
- Task splitting (`b`) to break a todo into part 1 and part 2
- **New keybindings help menu (`h`)** for quick reference inside the TUI
- Edit Mode navigation for cursor movement and text editing

#### Changed
- Removed `add` and `list` CLI subcommands — replaced with a unified TUI experience
- Removed “done” state; completed todos are now deleted instead of marked
- Removed due dates, tags, and extra metadata for a leaner workflow
- Redesigned TUI layout with clean borders and indentation
- Simplified file structure and startup behavior

#### Keybindings Overview

**Normal Mode**
```
╭─ [j/k] Move  
├─ [Space] Expand  
├─ [i] Edit  
├─ [⌫] Delete  
├─ [p/l] Toggle Priority  
├─ [b] Split Todo  
├─ [u] Undo  
├─ [h] Show Keybindings  
╰─ [Esc] Quit
```

**Edit Mode**
```
╭─ [Char/Space] Input Char  
├─ [←/→] Move Cursor  
├─ [⌫] Backspace  
╰─ [Esc] Return to Normal Mode
```

#### Philosophy
The Saracen release embodies clarity and decisiveness.  
No menus, no clutter — just a direct connection between your keyboard and your focus.  
It’s the purest form of task management: minimal UI, maximal control.

---

## [Unreleased]
### Planned
- Theming and color customization
- Search and filter functionality
- Persistent window layout state
- Optional remote sync for cross-device use
- Export and import for `.json` and `.csv`

---

[1.0.0]: https://github.com/yourusername/crix-todo/releases/tag/v1.0.0
