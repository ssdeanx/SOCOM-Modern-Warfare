# AGENTS.md — SOCOM Tactical Shooter

> Wiki freshness: [`docs/wiki/.codewiki-state.json`](docs/wiki/.codewiki-state.json) — generated 2026-06-19

All architecture, crate docs, module deep-dives, class/sequence diagrams, controls, state machine, and messages live in the wiki at **`docs/wiki/`**, starting at [`docs/wiki/README.md`](docs/wiki/README.md).

## Stack (locked versions)

| Dep | Version |
|-----|---------|
| bevy | **0.18.1** (NOT 0.18.0 or 0.19) |
| avian3d | **0.6.1** |
| leafwing-input-manager | 0.20.x |
| kira | 0.12 (standalone, not bevy_kira_audio) |
| bevy_hanabi | 0.18.0 |
| serde + ron | 1.x / 0.12 |

## Build

```bash
cargo check          # prefer this over build — saves ~10GB disk
cargo clippy         # 0 errors, ~36 warnings (known)
cargo fmt
cargo add / cargo rm # deps
```

## Hard Rules

- **`core` must never depend on Bevy.** Other crates use `features = ["bevy"]`.
- **Messages, not Events.** Always `#[derive(Message)]` + `app.add_message::<T>()`. Never `EventReader`/`EventWriter`.
- **Every external crate in Cargo.toml needs its plugin registered in the app builder** — not just in deps.
- **`#[expect(dead_code, reason = "...")]`** for forward-looking infrastructure. Never `#[allow()]`.

## Bevy 0.18.1 Quirks

- Mouse look: `Res<AccumulatedMouseMotion>`, not `EventReader<MouseMotion>`
- Audio: `(AudioPlayer(handle), PlaybackSettings::ONCE)`, not `Res<Audio>.play()`
- UI: `Text + TextFont + TextColor + Node`, NOT `TextBundle`/`TextStyle`/`Style`
- Despawn: `commands.entity(e).despawn()` (always recursive)
- Projection: `&mut bevy::camera::Projection`, match `Perspective(ref mut persp)`
- Fullscreen: `WindowMode::BorderlessFullscreen(MonitorSelection::Current)`
- Camera3d FOV: on `Projection::Perspective(...)`, not `Camera3d`
