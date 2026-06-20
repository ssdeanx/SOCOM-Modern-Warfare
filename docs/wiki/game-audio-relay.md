# Game Module: `audio_relay/` — Audio Relay Bridge

**Path:** `crates/game/src/audio_relay.rs`  
**Files:** 1  
**Purpose:** Bridges game messages to the kira audio host

## AudioRelayPlugin

Registers 3 Update systems:
1. `weapon_fire_audio_relay` — Route `WeaponFiredMessage` → kira SFX bus
2. `weapon_hit_audio_relay` — Route `HitConfirmedMessage` → kira SFX bus
3. `equipment_audio_relay` — Route `EquipmentUsedMessage` → kira SFX bus

### Weapon Audio Mapping

| Weapon Name | Sound File |
|-------------|-----------|
| M4A1 | `m4a1_fire.ogg` |
| MP5SD | `mp5sd_fire.ogg` |
| M1911 | `m1911_fire.ogg` |
| AK-47 | `ak47_fire.ogg` |
| M24 / L96A1 | `sniper_fire.ogg` |
| Other | `generic_fire.ogg` |

### Equipment Audio Mapping

| Equipment Type | Sound File |
|---------------|-----------|
| Grenade* | `grenade_throw.ogg` |
| Knife* | `knife_swing.ogg` |
| Bandage* | `bandage.ogg` |
| Medkit* / Medical* | `medkit.ogg` |

All audio files loaded from `audio/weapons/` base path. Missing files are silently logged.
