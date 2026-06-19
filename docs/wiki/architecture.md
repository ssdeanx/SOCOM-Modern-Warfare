# Architecture

SOCOM Tactical Shooter is structured as a **layered ECS application** on top of the Bevy game engine. The Cargo workspace splits responsibilities into five crates with a strict dependency direction: `core` → `input`, `rendering`, `audio` → `game`.

## System Architecture

```mermaid
flowchart TD
    subgraph External
        Win[Window / OS]
        Phys["Avian Physics Engine"]
        AudioDevice["Audio Output"]
    end

    subgraph "Bevy Engine Layer"
        BP["DefaultPlugins
        (render, window, asset,
        audio, UI, input)"]
        PP["PhysicsPlugins
        (avian3d)"]
    end

    subgraph "Internal Crates"
        CORE["socom-core
        data types, components"]
        INPUT["socom-input
        PlayerAction, bindings"]
        REND["socom-rendering
        camera follow, mouse look"]
        AUDIO["socom-audio
        footsteps, ambient"]
    end

    subgraph "Game Binary"
        MAIN["socom-game::main"]
        STATES["State Machine
        MainMenu / Loading / InGame"]
        PLAYER["Player Spawn + Bundle"]
        PHYS_SYS["Movement + Stance Systems"]
        LVL["Procedural Level"]
    end

    Win --> BP
    BP --> MAIN
    MAIN --> PP
    PP --> Phys

    MAIN --> INPUT
    MAIN --> REND
    MAIN --> AUDIO
    MAIN --> CORE
    MAIN --> STATES

    STATES --> PLAYER
    STATES --> PHYS_SYS
    STATES --> LVL

    INPUT --> CORE
    REND --> CORE
    AUDIO --> CORE

    Phys --> PHYS_SYS
    PLAYER --> PHYS_SYS
    PLAYER --> REND

    Win -.->|"Mouse + Keyboard"| INPUT
    AudioDevice -.->|"AudioPlayer commands"| AUDIO
```

## Component / Data Flow

```mermaid
flowchart LR
    subgraph Input[Input Layer]
        KB["Keyboard (WASD/Shift/C/Z)"]
        GP["Gamepad (Stick/Buttons)"]
        MOUSE["Mouse Move (AccumulatedMouseMotion)"]
    end

    subgraph CAM["Camera Layer"]
        CF["camera_follow_system
        lerps toward shoulder offset"]
        CL["camera_look_system
        pitch/yaw from mouse delta"]
    end

    subgraph PHYS["Physics Layer"]
        MS["player_movement_system
        ActionState -> LinearVelocity"]
        SS["player_stance_system
        toggle sprint/crouch/prone"]
    end

    subgraph AUDIO_LAYER["Audio Layer"]
        FS["footstep_system
        timed AudioPlayer spawns"]
        AS["ambient_system
        background loop"]
    end

    KB --> MS
    KB --> SS
    GP --> MS
    GP --> SS
    MOUSE --> CL

    MS -->|"LinearVelocity"| Avian3d
    SS -->|"MovementState"| MS

    CF -->|"Transform"| Camera3d
    CL --> CF

    MS -.->|"MovementState"| FS
```

## Crate Dependency Graph

```mermaid
flowchart TD
    GAME["socom-game (binary)"]
    INPUT_C["socom-input"]
    REND_C["socom-rendering"]
    AUDIO_C["socom-audio"]
    CORE_C["socom-core"]
    BEVY["bevy 0.18"]
    AVN["avian3d 0.6"]
    LW["leafwing-input-manager 0.20"]
    SERDE["serde 1.x"]
    GLAM["glam 0.29"]

    GAME --> INPUT_C
    GAME --> REND_C
    GAME --> AUDIO_C
    GAME --> CORE_C
    GAME --> BEVY
    GAME --> AVN

    INPUT_C --> CORE_C
    INPUT_C --> BEVY
    INPUT_C --> LW

    REND_C --> CORE_C
    REND_C --> BEVY

    AUDIO_C --> CORE_C
    AUDIO_C --> BEVY

    CORE_C -.->|"optional bevy feature"| BEVY
    CORE_C --> SERDE
    CORE_C --> GLAM
```

**Key rule:** `socom-core` must never depend on Bevy. All other crates depend on `core` with `features = ["bevy"]` to derive `Component` on its types.

## State Machine

```mermaid
flowchart LR
    START((App Start))
    MENU["MainMenu
    (Camera2d + UI text)"]
    LOAD["Loading
    (0.5s timer)"]
    GAME["InGame
    + Camera + Level + Player"]

    START -->|"init_state"| MENU
    MENU -->|"Space pressed"| LOAD
    LOAD -->|"Timer just_finished"| GAME
    GAME -.->|"Phase 1:
    return to menu"| MENU
```

States are defined as `AppState` enum with `States` derive from `bevy_state`.

## Key Design Decisions

1. **Core stays pure** — `socom-core` has zero Bevy dependency. The optional `"bevy"` feature gates `bevy_ecs::Component` derives via `#[cfg_attr(feature = "bevy", ...)]`. This allows core types to be shared with non-Bevy systems (server, editor, CLI tools).
2. **Action-based input** — Using `leafwing-input-manager`, all player actions are defined as an enum variant (`PlayerAction::Sprint`, `PlayerAction::Crouch`). The input map binds physical keys/buttons to actions, decoupling game logic from hardware.
3. **Third-person camera is a component** — `ThirdPersonCamera` is an ECS component attached to the camera entity. It stores its target entity, pitch/yaw angles, shoulder side, and lerp factor. Two systems (follow + look) process it independently.
4. **Physics via LinearVelocity** — Phase 0 uses direct `LinearVelocity` manipulation on a `RigidBody::Dynamic` entity. Phase 1 will switch to Avian's `KinematicCharacterController` + `MoveAndSlide` for proper stair/ramp handling.
5. **Bevy 0.18 audio patterns** — No `Res<Audio>`; audio is spawned as entities: `commands.spawn((AudioPlayer::new(handle), PlaybackSettings::ONCE, ...))`.
