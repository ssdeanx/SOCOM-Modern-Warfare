# SOCOM Tactical Shooter — Sequence Diagrams

## 1. Shooting → Damage → Death Flow

```mermaid
sequenceDiagram
    participant P as Player
    participant SS as shooting_system
    participant AD as apply_damage_system
    participant DC as death_check_system
    participant HPD as handle_player_death
    participant FX as feedback/audio

    P->>SS: Press Fire (auto or semi)
    SS->>SS: Check WeaponState.magazine > 0
    SS->>SS: Check fire cooldown
    SS->>SS: Compute bullet_ray(camera, weapon, ADS)
    SS->>SS: spatial_query.cast_ray(origin, dir, range)
    alt Hit Entity
        SS->>AD: DamageMessage { target, amount, source, hit_point }
        SS->>FX: Spawn impact marker mesh
        AD->>AD: Health.current -= damage
        AD->>FX: PlayerDamagedMessage
        AD->>FX: HitConfirmedMessage
        AD->>DC: Health changed event
        alt Health <= 0 AND NOT downed
            DC->>DC: Health.is_downed = true
            DC->>DC: Bleed-out: 30s timer starts
        else Health <= 0 AND downed
            DC->>DC: Insert Dead component
            DC->>HPD: DeathMessage { entity, source }
            HPD->>HPD: RespawnState.is_dead = true
            HPD->>FX: Spawn "YOU DIED" text overlay
        end
    else Miss
        SS->>FX: Spawn tracer marker at max range
    end
    SS->>SS: Update WeaponState.magazine -= 1
    SS->>SS: Auto-reload if magazine = 0
    SS->>FX: Play weapon fire sound + muzzle flash
```

## 2. Drone Deployment → Flight → Detonation

```mermaid
sequenceDiagram
    participant P as Player
    participant DS as drone_control_system
    participant D as Drone Entity

    P->>DS: Press U (Recon Drone)
    DS->>DS: DroneState.recon_active = !recon_active
    alt Deploy
        DS->>D: Spawn ReconDroneBundle(position + Vec3.Y * 8)
        D->>D: Drone.deployed = true
        D->>D: Drone.battery = 120
        
        loop Every Frame
            DS->>D: update_battery(dt) → drain 3/s
            P->>DS: WASD/QE camera-relative flight input
            DS->>D: Lerp velocity toward input direction
            DS->>D: Update transform.position += velocity * dt
            DS->>D: Face movement direction
            
            alt Battery < 15% (auto-return)
                DS->>D: Fly toward player position
                alt Distance < 2m
                    DS->>D: Despawn drone
                    DS->>DS: DroneState.recon_active = false
                end
            end
        end
    else Recall
        DS->>D: Despawn all Recon drones
        DS->>DS: DroneState.recon_active = false
    end

    Note over D,DS: FPV Strike variation:
    P->>DS: Press J → FpvDroneBundle (30s battery, 25m/s)
    P->>DS: Press Space → Manual detonation
    DS->>DS: apply_drone_explosion(spatial_query, damage_writer)
    DS->>D: Despawn drone
    DS->>DS: DroneState.fpv_active = false
```

## 3. Equipment Usage (Grenade)

```mermaid
sequenceDiagram
    participant P as Player
    participant EIS as select_equipment_system
    participant TES as throw_equipment_system
    participant FTS as fuse_timer_system
    participant EDS as apply_explosion_damage_system
    participant DMG as Damage/Death chain

    P->>EIS: Press X to cycle equipment
    EIS->>EIS: Select next EquipmentType from inventory
    
    P->>TES: Press G to throw/use
    TES->>TES: Read selected EquipmentType
    alt FragGrenade (throwable)
        TES->>TES: Spawn GrenadeProjectile entity
        TES->>TES: Apply initial velocity (throw arc)
        TES->>FTS: GrenadeProjectile.fuse_timer starts (3.5s)
        
        loop Tick fuse_timer
            FTS->>FTS: Timer.tick(dt)
            alt Timer.just_finished()
                FTS->>FTS: Emit GrenadeDetonatedMessage
                FTS->>EDS: GrenadeDetonatedMessage { pos, damage: 200, radius: 6m }
                EDS->>EDS: Query all DestructionState in radius
                loop For each entity in blast radius
                    EDS->>EDS: Apply spherical falloff damage
                    EDS->>DMG: DamageMessage to entity
                end
                EDS->>FTS: Despawn GrenadeProjectile
            end
        end
    else C4 (deployable)
        TES->>TES: Spawn C4Charge entity at placement point
        P->>TES: Press G again → Remote detonation
        TES->>EDS: Emit GrenadeDetonatedMessage (no fuse)
    else Claymore
        TES->>TES: Spawn Deployable entity
        TES->>TES: Deployable.arm_timer starts (2s)
        LOOP Wait for trigger
            TES->>TES: proximity check via SpatialQuery
            alt Enemy in trigger radius
                TES->>EDS: Emit GrenadeDetonatedMessage
                EDS->>DMG: Forward damage cone
            end
        end
    end
```

## 4. Destruction State Machine

```mermaid
sequenceDiagram
    participant G as Grenade/Gun
    participant AED as apply_explosion_damage_system
    participant DSM as destruction_state_machine_system
    participant DBR as debris_spawner
    participant FX as VFX/Audio

    G->>AED: GrenadeDetonatedMessage / DamageMessage
    AED->>AED: Query DestructionState entities in radius
    AED->>AED: Apply spherical falloff damage
    AED->>DSM: DestructionState.health reduced

    DSM->>DSM: determine_state(health_ratio, material)
    
    alt Pristine → Damaged (health < 60%)
        DSM->>DSM: state = Damaged
        DSM->>FX: DestructionTransitionMessage { from: Pristine, to: Damaged }
        FX->>FX: Visual damage (cracks, scorch marks)
        
    else Damaged → Breached (health < 30%)
        DSM->>DSM: state = Breached
        DSM->>DBR: spawn_debris_for_transition(material, Breached, pos)
        DBR->>DBR: Spawn small debris mesh entities
        DBR->>FX: DestructionTransitionMessage
        FX->>FX: Audio: crack/break sound
        
    else Breached → Destroyed (health = 0%)
        DSM->>DSM: state = Destroyed
        DSM->>DBR: spawn_debris_for_transition(material, Destroyed, pos)
        DBR->>DBR: Spawn large debris + dust particles
        DSM->>DSM: debris_spawned = true (one-shot)
        FX->>FX: Audio: collapse/explosion sound
        DSM->>DSM: collapse_animation_system: apply downward velocity
    end
```

## 5. Squad Command System

```mermaid
sequenceDiagram
    participant P as Player
    participant CW as command_wheel_system
    participant SOD as squad_order_dispatch_system
    participant AO as ActiveOrders
    participant T as Teammate AI
    participant FS as formation_system

    P->>CW: Press Tab → Open command wheel
    CW->>CW: Render 4-slot radial menu
    P->>CW: Press 1-4 to select order
    
    alt Order selected
        CW->>CW: Close command wheel
        CW->>SOD: SquadOrderMessage { order, source: player }
        SOD->>SOD: Query all Teammate entities
        SOD->>AO: Insert (teammate → order) mapping
        
        alt RegroupOnPlayer
            AO->>T: Order = RegroupOnPlayer
            T->>FS: Move to formation position
            FS->>FS: Compute world offset from player
            FS->>T: Lerp velocity toward target
        else HoldPosition
            AO->>T: Order = HoldPosition
            T->>T: Stop movement, hold position
        end
    end
    
    P->>CW: Press Tab again → Close without selection
```

## 6. AI Detection → Engagement

```mermaid
sequenceDiagram
    participant E as Enemy
    participant DET as detection_system
    participant ENG as engage_system
    participant PL as Player

    loop Every frame (Patrol state)
        DET->>DET: Calculate angle to player
        DET->>DET: Check distance ≤ 40m
        DET->>DET: Check FOV (120° horizontal)
        
        alt Player in vision cone
            DET->>DET: Raycast LOS check
            alt LOS clear
                DET->>DET: suspicion += 30*dt
                alt suspicion >= 100
                    DET->>ENG: AiState = Engage
                end
            else LOS blocked
                DET->>DET: suspicion -= 10*dt
            end
        else Player outside cone/range
            DET->>DET: suspicion -= 10*dt
        end
    end

    loop Engage state
        ENG->>ENG: Face player
        ENG->>ENG: Decelerate to stop
        ENG->>ENG: LOS check
        alt LOS blocked
            ENG->>ENG: Transition to Alert state
        else LOS clear
            alt fire_interval elapsed (1.5s)
                ENG->>PL: DamageMessage { player, amount: 10 }
                ENG->>ENG: WeaponState.magazine -= 1
                alt magazine = 0 & reserve > 0
                    ENG->>ENG: Start reload (2.5s)
                end
            end
        end
    end
```
