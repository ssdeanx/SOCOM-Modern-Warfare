# SOCOM Tactical Shooter — Class Diagrams

## Core Type Hierarchy

```mermaid
classDiagram
    class Player {
        <<marker>>
    }

    class Health {
        +f32 current
        +f32 max
        +f32 armor
        +f32 max_armor
        +bool is_downed
        +f32 bleed_out_remaining
        +f32 revive_progress
        +is_alive() bool
        +is_down() bool
        +ratio() f32
        +armor_ratio() f32
    }

    class MovementState {
        <<enumeration>>
        Standing
        Sprinting
        Crouching
        Prone
        InCover
    }

    class Team {
        <<enumeration>>
        Player
        Teammate
        Enemy
    }

    class Shoulder {
        <<enumeration>>
        Right
        Left
    }

    class Weapon {
        +String name
        +f32 fire_rate
        +f32 damage
        +u32 magazine_size
        +u32 reserve_ammo
        +f32 reload_time
        +bool is_automatic
        +f32 spread_degrees
        +f32 max_range
        +m4a1() Weapon
        +mp5sd() Weapon
        +m1911() Weapon
        +ak47() Weapon
    }

    class WeaponSlot {
        +Option~Weapon~ primary
        +Option~Weapon~ sidearm
        +u8 active_slot
        +active_weapon() Option~&Weapon~
        +active_weapon_mut() Option~&mut Weapon~
        +swap_slot()
    }

    class GameSettings {
        +f32 master_volume
        +f32 sfx_volume
        +f32 music_volume
        +f32 sensitivity
        +bool invert_y
        +bool fullscreen
        +bool vsync
    }

    class Paused {
        +bool 0
    }

    class SensitivityMultiplier {
        +f32 0
    }

    Health --> Weapon : entity may have
    Player --> Health : has
    Player --> MovementState : has
    Player --> WeaponSlot : has
    Player --> Team : has
```

## Combat Type Hierarchy

```mermaid
classDiagram
    class WeaponState {
        +u32 magazine
        +u32 reserve
        +f32 last_fire_time
        +bool is_reloading
        +f32 reload_timer
        +u8 slot_index
        +from_weapon(Weapon, u8) WeaponState
    }

    class OffhandWeaponState {
        +WeaponState 0
    }

    class DamageMessage {
        +Entity target
        +f32 amount
        +Entity source
        +Vec3 hit_point
        +Vec3 hit_normal
    }

    class DeathMessage {
        +Entity entity
        +Option~Entity~ source
    }

    class RespawnState {
        +Timer timer
        +bool is_dead
    }

    class Dead {
        <<marker>>
    }

    DeathMessage --> RespawnState : consumed by
    DamageMessage --> Health : reduces
    Health --> Dead : transitions when 0
    WeaponState --> OffhandWeaponState : preserved in
```

## Weapons & Gear Type Hierarchy

```mermaid
classDiagram
    class CompleteWeapon {
        +WeaponChassis chassis
        +Caliber caliber
        +BarrelType barrel
        +SightType sight
        +UnderbarrelType underbarrel
        +MagazineType magazine
        +StockType stock
        +f32 final_damage
        +f32 final_fire_rate
        +u32 final_magazine_size
        +f32 final_reload_time
        +f32 final_spread_hip
        +f32 final_spread_ads
        +f32 final_weight
        +f32 final_ads_speed
        +f32 final_sway
        +f32 final_max_range
        +assemble(...) CompleteWeapon
        +default_m4a1() CompleteWeapon
    }

    class WeaponChassis {
        +String name
        +WeaponClass class
        +Caliber caliber
        +f32 base_damage
        +f32 base_fire_rate
        +u32 base_magazine_size
        +f32 base_weight
        +f32 base_sway
        +m4a1() WeaponChassis
        +ak47() WeaponChassis
    }

    class Caliber {
        <<enumeration>>
        NineMm
        FortyFiveACP
        FiveFiveSixNato
        SevenSixTwoX39
        SevenSixTwoNato
        TwelveGauge
        FiftyBMG
        +damage_mult() f32
        +penetration_mult() f32
    }

    class EquippedWeapon {
        +CompleteWeapon weapon
    }

    class GearItem {
        +String id
        +String name
        +GearSlot slot
        +HashMap~String,f32~ stats
    }

    class PlayerInventory {
        +[Option~GearItem~; 5] equipped
        +Vec~GearItem~ stash
        +u64 credits
        +equip(GearItem) Option~GearItem~
        +weapon_damage_bonus() f32
    }

    class EquipmentType {
        <<enumeration>>
        FragGrenade
        Flashbang
        SmokeGrenade
        C4
        Claymore
        MedicalKit
        Knife
        +name() str
        +base_damage() f32
        +blast_radius() f32
        +fuse_time() f32
    }

    class WeaponWorkshop {
        +bool ui_open
        +Vec~Attachment~ fitted_attachments
        +apply_modifiers(f32, f32, f32) (f32, f32, f32)
    }

    CompleteWeapon --> WeaponChassis : contains
    CompleteWeapon --> Caliber : contains
    CompleteWeapon --> EquippedWeapon : wraps
    WeaponChassis --> Caliber : has base
    PlayerInventory --> GearItem : contains
    WeaponWorkshop --> CompleteWeapon : modifies
```

## Drone & Destruction Type Hierarchy

```mermaid
classDiagram
    class Drone {
        +DroneType drone_type
        +f32 battery
        +f32 max_battery
        +bool deployed
        +Vec3 velocity
        +bool detonated
        +f32 explosion_radius
        +f32 explosion_damage
        +u32 grenade_hardpoints
        +u32 mine_count
        +update_battery(f32)
        +has_power() bool
    }

    class DroneType {
        <<enumeration>>
        Recon
        FpvStrike
        GrenadeDrone
        MineDrone
        +max_battery() f32
        +max_speed() f32
        +altitude_default() f32
    }

    class DroneState {
        +bool recon_active
        +bool fpv_active
        +bool grenade_active
        +bool mine_active
    }

    class DestructionState {
        +DestructionLevel state
        +f32 health
        +f32 max_health
        +MaterialType material
        +Vec~Vec3~ bullet_holes
        +bool debris_spawned
        +for_material(MaterialType) DestructionState
        +ratio() f32
    }

    class DestructionLevel {
        <<enumeration>>
        Pristine
        Damaged
        Breached
        Destroyed
    }

    class MaterialType {
        <<enumeration>>
        Drywall
        Wood
        Brick
        Concrete
        ReinforcedConcrete
        Glass
        Flesh
    }

    class DestructionTransitionMessage {
        +Entity entity
        +DestructionLevel from_state
        +DestructionLevel to_state
        +Vec3 position
    }

    Drone --> DroneType : has type
    DroneState --> Drone : tracks active
    DestructionState --> DestructionLevel : state machine
    DestructionState --> MaterialType : classified by
    DestructionTransitionMessage --> DestructionLevel : reports transition
```

## AI & Squad Type Hierarchy

```mermaid
classDiagram
    class AiState {
        <<enumeration>>
        Patrol
        Alert
        Engage
    }

    class PatrolRoute {
        +Vec~Vec3~ waypoints
        +usize current_index
        +Timer wait_timer
        +bool is_waiting
        +between(Vec3, Vec3) PatrolRoute
    }

    class VisionCone {
        +f32 fov_h
        +f32 fov_v
        +f32 range
        +f32 suspicion
    }

    class Suppression {
        +f32 level
        +Timer decay_timer
    }

    class InCover {
        +Entity cover_entity
        +CoverType cover_type
    }

    class SquadOrder {
        <<enumeration>>
        MoveToTarget
        EngageTarget
        SuppressPosition
        RegroupOnPlayer
        HoldPosition
    }

    class ActiveOrders {
        +HashMap~Entity, SquadOrder~ orders
    }

    class CommandWheelState {
        +bool open
        +usize selected_index
    }

    AiState --> PatrolRoute : patrol uses
    AiState --> VisionCone : detection uses
    Suppression --> InCover : tactical state
    SquadOrder --> ActiveOrders : dispatched to
    CommandWheelState --> SquadOrder : issues orders
```

## Stamina & Weapon Handling

```mermaid
classDiagram
    class Stamina {
        +f32 current
        +f32 max
        +Timer regen_timer
        +bool exhausted
        +ratio() f32
        +is_exhausted() bool
    }

    class WeaponHandling {
        +f32 current_ads_time
        +f32 current_weight_mult
        +Timer deploy_timer
        +bool is_deploying
    }

    class WeaponWeight {
        <<enumeration>>
        Light
        Medium
        Heavy
        Sniper
        +speed_mult() f32
        +ads_time() f32
    }

    class Breathing {
        +bool holding
        +f32 hold_timer
        +Timer cooldown_timer
        +f32 steadiness
    }

    MovementState --> Stamina : affects drain
    WeaponWeight --> WeaponHandling : determines
    Stamina --> Breathing : drained by
    WeaponSlot --> WeaponWeight : derived from
```
