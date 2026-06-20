# Game Module: `menu/` — Menu System

**Path:** `crates/game/src/menu/`  
**Files:** 3 — `mod.rs`, `settings.rs`, `keybinds.rs`  
**Purpose:** Main menu, settings screen, keybind configuration

## Module Map

```
menu/
├── mod.rs       — MainMenuPlugin
├── settings.rs  — Settings screen with volume/sensitivity toggles
└── keybinds.rs  — Keybind display and rebinding UI
```

## MainMenuPlugin
- Title screen with "New Game", "Settings", "Quit" options
- Transitions to Loading state on New Game

## Settings Screen
- Master volume slider
- SFX / Music volume sliders
- Mouse sensitivity slider
- Invert Y toggle
- Fullscreen toggle
- Resolution selector

## Keybinds
- Displays current key mappings
- Rebind UI (click to rebind)
