/// Audio relay for weapon-specific sounds (reload, select, melee).
use bevy::prelude::*;

/// Simple audio relay that reads equipment/weapon messages and plays placeholder sounds.
pub struct WeaponAudioPlugin;

impl Plugin for WeaponAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dummy_audio_relay_system);
    }
}

/// Placeholder: will play audio when EquipmentUsedMessage fires.
/// Currently a no-op until placeholder assets are added.
#[allow(clippy::all)]
pub fn dummy_audio_relay_system() {
    // Awaiting audio asset generation. System runs as a future wiring point.
}
