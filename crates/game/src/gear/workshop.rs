use crate::gear::attachments::{default_attachments, Attachment, AttachmentSlot};
use bevy::prelude::*;

#[derive(Resource)]
pub struct WeaponWorkshop {
    pub fitted: [Option<Attachment>; 5],
    pub library: Vec<Attachment>,
    pub ui_open: bool,
}

impl Default for WeaponWorkshop {
    fn default() -> Self {
        Self {
            fitted: [None, None, None, None, None],
            library: default_attachments(),
            ui_open: false,
        }
    }
}

impl WeaponWorkshop {
    pub fn apply_modifiers(
        &self,
        base_damage: f32,
        base_spread: f32,
        base_magazine: f32,
    ) -> (f32, f32, f32) {
        let mut dmg = base_damage;
        let mut spread = base_spread;
        let mut mag = base_magazine;
        for att in self.fitted.iter().flatten() {
            for (key, val) in &att.stat_modifiers {
                match key.as_str() {
                    "damage" => dmg += val,
                    "spread" => spread += val,
                    "magazine" => mag += val,
                    _ => {}
                }
            }
        }
        (dmg, spread.max(0.1), mag.max(1.0))
    }
    #[expect(dead_code, reason = "awaiting workshop UI")]
    pub fn attach(&mut self, index: usize) -> bool {
        if index >= self.library.len() {
            return false;
        }
        let attachment = self.library[index].clone();
        let idx = match attachment.slot {
            AttachmentSlot::Sight => 0,
            AttachmentSlot::Barrel => 1,
            AttachmentSlot::Grip => 2,
            AttachmentSlot::Magazine => 3,
            AttachmentSlot::Stock => 4,
        };
        self.fitted[idx] = Some(attachment);
        true
    }
}

pub fn weapon_modification_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut workshop: ResMut<WeaponWorkshop>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        workshop.ui_open = !workshop.ui_open;
    }
}
