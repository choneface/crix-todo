# 🧾 Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.0.0] – 2025-10-07
### Saracen (Major Release)

“Clear eyes. Full hearts. No distractions.”

Version **1.0.0 (Saracen)** marks the first major release of `crix-todo`.  
This release is a full redesign focused on simplicity, speed, and focus.

#### Added
- Unified single-command workflow: launch everything with `todo`
- Inline todo creation (`=`) directly within the TUI
- Inline editing for titles and notes (`i`)
- Undo functionality (`u`) to revert the last action
- Tree-style visual layout for todos and notes
- Local JSON storage with instant persistence
- Task splitting (`b`) for dividing long tasks

#### Changed
- Removed `add` and `list` CLI subcommands — replaced with a unified TUI
- Removed “done” state; completed todos are now deleted
- Removed due dates, tags, and extra metadata for a leaner UX
- Simplified file structure and startup behavior
- Redesigned TUI layout with modern borders and indentation

#### Keybindings
| Key | Action |
|-----|---------|
| **j / k** | Move selection |
| **Space** | Expand or collapse todo |
| **i** | Edit (title or notes) |
| **=** | Add a new todo |
| **⌫ (Backspace)** | Delete selected todo |
| **p / l** | Toggle priority |
| **b** | Split todo into part 1 and part 2 |
| **u** | Undo last action |
| **Esc** | Quit TUI |

#### 💡 Philosophy
The Saracen release embodies clarity and decisiveness.  
No menus, no clutter — just a direct connection between your keyboard and your focus.  
It’s the purest form of task management: minimal UI, maximal control.

---

## [Unreleased]
### 🚧 Planned
- Cross-platform sync via optional remote storage
- Custom themes and color profiles
- Smart grouping (e.g., by context or priority)
- Export/import support for `.json` and `.csv`

---

[1.0.0]: https://github.com/yourusername/crix-todo/releases/tag/v1.0.0
