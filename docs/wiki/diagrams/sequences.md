# Sequence Diagrams

## 1. Game Frame Lifecycle

Shows the order of systems executing per frame during `InGame` state.

```mermaid
sequenceDiagram
    participant Win as Window / OS
    participant I as InputPlugin
    participant C as CameraPlugin
    participant P as PlayerPlugin (Startup)
    participant Ph as PhysicsPlugin
    participant A as AudioPlugin
    participant Av as Avian3d Physics

    Note over Win,A: Frame N (InGame state)
    Win->>I: Raw input events (keyboard, mouse)
    I->>I: Update ActionState~PlayerAction~
    I-->>Ph: ActionState available (Resource)

    Ph->>Ph: player_movement_system
    Ph->>Av: Set LinearVelocity on Player
    Ph->>Ph: player_stance_system
    Ph->>Ph: Update MovementState

    Av->>Av: Integrate velocities, solve collisions
    Av-->>C: Player transform updated

    Win->>C: AccumulatedMouseMotion
    C->>C: camera_look_system (update pitch/yaw)
    C->>C: camera_follow_system (lerp position)

    C->>Win: Camera Transform -> Render

    A->>A: footstep_system (check timer, spawn AudioPlayer)
    A->>A: ambient_system (already looping)
```

## 2. State Transition: Menu to InGame

```mermaid
sequenceDiagram
    participant Win as Window
    participant S as State Machine
    participant MM as MainMenuPlugin
    participant L as LoadingPlugin
    participant IG as InGamePlugin

    Win->>S: App::new().init_state(MainMenu)
    S->>MM: OnEnter(AppState::MainMenu)
    MM->>MM: Spawn Camera2d + title UI

    Win->>MM: Key: Space
    MM->>S: next_state.set(Loading)
    S->>MM: OnExit(MainMenu) -> despawn UI

    S->>L: OnEnter(AppState::Loading)
    L->>L: Spawn Camera2d + "Loading..."

    L->>L: 0.5s elapsed
    L->>S: next_state.set(InGame)
    S->>L: OnExit(Loading) -> despawn UI

    S->>IG: OnEnter(AppState::InGame)
    IG->>IG: Spawn ambient light, directional light
    IG->>IG: Add sub-plugins (PlayerPlugin, PhysicsPlugin, LevelPlugin)
    IG->>IG: PlayerPlugin: spawn PlayerBundle + Camera
    IG->>IG: LevelPlugin: spawn greybox room
```

## 3. Input → Movement Pipeline

```mermaid
sequenceDiagram
    participant KB as Keyboard
    participant LW as leafwing-input-manager
    participant AS as ActionState~PlayerAction~
    participant Phy as PhysicsPlugin
    participant Av as Avian3d

    KB-->>LW: KeyDown(KeyW) + KeyDown(ShiftLeft)
    LW->>AS: Update action states
    AS->>AS: Move axis_pair = (0.0, 1.0)
    AS->>AS: Sprint just_pressed = true

    Phy->>AS: Read axis_pair(&Move) -> (0.0, 1.0)
    Phy->>AS: Read just_pressed(&Sprint) -> true
    Phy->>Phy: player_stance_system: Standing -> Sprinting
    Phy->>Phy: speed = SPRINT_SPEED (5.0 m/s)

    Phy->>Av: LinearVelocity = (0.0, 0.0, 5.0)
    Av->>Av: Integrate position += velocity * dt
```

## 4. Camera Orbit

```mermaid
sequenceDiagram
    participant Mouse as Mouse
    participant Acc as AccumulatedMouseMotion
    participant Cam as CameraPlugin
    participant TC as ThirdPersonCamera

    Mouse-->>Acc: delta = (120, -80) pixels
    Acc-->>Acc: Accumulate delta

    Cam->>Acc: Read Res~AccumulatedMouseMotion~
    Cam->>Cam: camera_look_system

    Cam->>TC: yaw += 120 * sensitivity * dt
    Cam->>TC: pitch += (-80) * sensitivity * dt
    Cam->>TC: clamp(pitch, min_pitch, max_pitch)

    Cam->>Cam: camera_follow_system
    Cam->>TC: Compute orbit offset from pitch/yaw/distance
    Cam->>TC: desired = target_pos + shoulder_offset + orbit_offset
    Cam->>TC: transform = lerp(transform, desired, 0.9)
```
