# Game Module: `progression/` — XP, Stats, Achievements

**Path:** `crates/game/src/progression/`  
**Files:** 5 — `mod.rs`, `xp.rs`, `stats.rs`, `achievements.rs`, `specializations.rs`  
**Purpose:** Player progression system — XP levels, stat tracking, achievements, specializations

## Module Map

```
progression/
├── mod.rs             — ProgressionPlugin
├── xp.rs              — PlayerProgression resource, XP event listener
├── stats.rs           — PlayerStats resource, damage/death listeners
├── achievements.rs    — AchievementTracker + achievement_checker
└── specializations.rs — Specialization system (placeholder)
```

## ProgressionPlugin

Registers:
- `PlayerProgression`, `PlayerStats`, `AchievementTracker` resources
- Systems: `xp_event_listener`, `damage_event_listener`, `death_event_listener`, `achievement_checker`

## PlayerProgression Resource

```rust
pub struct PlayerProgression {
    pub xp: u64,
    pub level: u32,
    pub total_xp_earned: u64,
}
```

- XP earned from kills, objectives, and achievements
- Level threshold calculation (placeholder)

## PlayerStats Resource

```rust
pub struct PlayerStats {
    pub kills: u32,
    pub deaths: u32,
    pub shots_fired: u32,
    pub shots_hit: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
}
```

- Updated via `DamageMessage` and `DeathMessage` listeners
- Used for achievements and save data

## AchievementTracker

```rust
pub struct AchievementTracker {
    pub earned: Vec<Achievement>,
}
```

- Achievement enum with unlock conditions
- `achievement_checker` system evaluates conditions against stats
- Emits `AchievementUnlockMessage` when new achievement earned

## Specializations
Placeholder module for future specialization/perk system (Phase 4+).
