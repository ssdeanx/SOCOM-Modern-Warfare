# Draft: SOCOM-Inspired Tactical Shooter — Spec

## Current State
- User wants to build a 3rd person tactical shooter in Rust, inspired by SOCOM US Navy SEALS (PS2)
- Focus on structure and solid foundation first
- Blender for assets later
- Enterprise quality, modular architecture

## Research Completed
1. **Engine Ecosystem** (partial) — Bevy 0.18.1 confirmed as best choice
2. **SOCOM Mechanics** — Comprehensive design reference from SOCOM / R6 / Splinter Cell / Ghost Recon
3. **Rust Crate Stack** — Full recommended stack with warnings (big-brain abandoned, bincode dead)
4. **Project Architecture** — (still running)

## Technology Decisions Made (tentative)
- **Engine**: Bevy 0.18.1 (stable, ecosystem-wide support)
- **Physics**: Avian3d 0.6.1 (native ECS, move-and-slide character controller)
- **Audio**: bevy_kira_audio for music/UI + firewheel for 3D spatial
- **Animation**: bevy_animation (built-in) + bevy_mod_inverse_kinematics
- **AI**: Custom state machines + bevy_northstar (pathfinding)
- **Input**: leafwing-input-manager
- **Config**: ron + serde
- **Save Games**: rkyv
- **Networking**: lightyear (future phase)

## Questions for User
- Bevy 0.18.1 (stable) vs 0.19 (RC) — recommend 0.18.1
- Single-player first vs co-op from day one?
- What's the scope of the first milestone?
