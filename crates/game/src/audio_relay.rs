/// Audio relay — bridges game messages to the kira audio host.
///
/// Listens to combat messages and routes them to the SFX bus
/// via `KiraAudioState`. Audio files not yet present are logged
/// and silently skipped.
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use socom_audio::kira_host::{play_sfx, KiraAudioState};

use crate::messages::{EquipmentUsedMessage, HitConfirmedMessage, WeaponFiredMessage};

/// Base path for weapon audio assets (relative to project root).
const WEAPON_AUDIO_PATH: &str = "audio/weapons/";

/// Registers all audio relay systems.
pub struct AudioRelayPlugin;

impl Plugin for AudioRelayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                weapon_fire_audio_relay,
                weapon_hit_audio_relay,
                equipment_audio_relay,
            ),
        );
    }
}

/// Routes weapon fire messages to the kira SFX bus.
fn weapon_fire_audio_relay(
    mut state: ResMut<KiraAudioState>,
    mut fired_reader: MessageReader<WeaponFiredMessage>,
) {
    for msg in fired_reader.read() {
        let sound_file = match msg.weapon_name.as_str() {
            "M4A1" => "m4a1_fire.ogg",
            "MP5SD" => "mp5sd_fire.ogg",
            "M1911" => "m1911_fire.ogg",
            "AK-47" => "ak47_fire.ogg",
            "M24" | "L96A1" => "sniper_fire.ogg",
            _ => "generic_fire.ogg",
        };
        play_sfx(&mut state, &format!("{}{}", WEAPON_AUDIO_PATH, sound_file), None);
    }
}

/// Routes hit confirmation messages to the kira SFX bus.
fn weapon_hit_audio_relay(
    mut state: ResMut<KiraAudioState>,
    mut hit_reader: MessageReader<HitConfirmedMessage>,
) {
    for msg in hit_reader.read() {
        if msg.hit {
            play_sfx(&mut state, &format!("{}hit_confirmed.ogg", WEAPON_AUDIO_PATH), None);
        }
    }
}

/// Routes equipment (grenade, melee, healing) messages to the kira SFX bus.
fn equipment_audio_relay(
    mut state: ResMut<KiraAudioState>,
    mut equip_reader: MessageReader<EquipmentUsedMessage>,
) {
    for msg in equip_reader.read() {
        let sound_file = match msg.equip_type.as_str() {
            t if t.contains("Grenade") => "grenade_throw.ogg",
            t if t.contains("Knife") => "knife_swing.ogg",
            t if t.contains("Bandage") => "bandage.ogg",
            t if t.contains("Medkit") || t.contains("Medical") => "medkit.ogg",
            _ => continue,
        };
        play_sfx(&mut state, &format!("{}{}", WEAPON_AUDIO_PATH, sound_file), None);
    }
}
