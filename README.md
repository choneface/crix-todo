# crix-todo 🦀✅
### *Version 1.0.0 – Saracen*

A terminal-based task manager written in Rust — redesigned from the ground up for speed, simplicity, and focus.  
No more cluttered commands. Just one clean, unified experience.

---

## ⚙️ Overview

`crix-todo` now runs entirely from a single command:

```
todo
```

This launches the interactive TUI, where you can **add**, **edit**, **delete**, **split**, and **reorganize** your todos in one streamlined interface.  
The philosophy: *minimal UI, maximal control.*

---

## 📦 Installation

```
cargo install crix-todo
```

---

## 🚀 Highlights

- 🧭 **Single Command Workflow** – Launch everything with just `todo`.
- ➕ **Add Inline** – Press `=` inside the TUI to instantly create a new task.
- ✏️ **Inline Editing** – Edit title or notes directly without leaving the view.
- 🔁 **Undo Support** – Quickly recover from mistakes with `u`.
- 🪶 **Simplified Metadata** – No tags, no due dates, no “done” status. Just titles, priorities, and notes.
- 🌳 **Modern Tree-Style Layout** – Todos and notes are rendered in a clear, visual hierarchy.
- 🧩 **Split Tasks** – Break large todos into smaller parts with a single keypress.
- ⚡ **Instant State Persistence** – All changes are saved immediately to your local JSON store.

---

## 🎨 Example Layout

```
Priority 0
╭─ Workout session
│   ╰─ Notes: Leg day today at the gym
╰─ Schedule dentist appointment

Priority 1
╭─ Buy groceries
│   ╰─ Notes: Don’t forget coffee
╰─ Finish Rust project
╰─ Notes: Add tree-style notes rendering
```

---

## ⌨️ TUI Keybindings

**Normal Mode**
```
╭─ [j/k] Move selection up or down  
├─ [Space] Expand or collapse todo  
├─ [i] Edit (title if collapsed, notes if expanded)  
├─ [=] Add a new todo  
├─ [⌫] Delete selected todo  
├─ [p/l] Toggle priority up/down  
├─ [b] Split a todo into part 1 and part 2  
├─ [u] Undo last action  
├─ [h] Show keybindings help menu  
╰─ [Esc] Quit the TUI
```

**Edit Mode**
```
╭─ [Char/Space] Input character  
├─ [←/→] Move cursor  
├─ [⌫] Backspace  
╰─ [Esc] Return to Normal Mode
```

---

## 📂 Data Storage

Todos are stored locally in a plain JSON file named `todo.json`.  
No sync, no cloud — your data stays on your machine.

---

## 🧪 Development

Clone the repository and run the TUI directly:

```
cargo run
```

---

## 🏈 About the Release

**Saracen (v1.0.0)** is a complete rebuild of `crix-todo`.  
Inspired by the clear-headed leadership of *Matt Saracen* from *Friday Night Lights*, this release is about clarity, confidence, and control.

**Clear eyes. Full hearts. No distractions.**
