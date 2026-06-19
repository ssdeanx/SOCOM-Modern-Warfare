use std::collections::HashMap;
use crate::gear::items::Rarity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttachmentSlot { Sight, Barrel, Grip, Magazine, Stock }

#[derive(Debug, Clone)]
pub struct Attachment {
    pub name: String,
    pub slot: AttachmentSlot,
    pub rarity: Rarity,
    pub stat_modifiers: HashMap<String, f32>,
    pub level_required: u32,
}

impl Attachment {
    pub fn new(name: &str, slot: AttachmentSlot, rarity: Rarity, modifiers: Vec<(&str, f32)>) -> Self {
        Self { name: name.to_string(), slot, rarity, stat_modifiers: modifiers.into_iter().map(|(k,v)| (k.to_string(), v)).collect(), level_required: 1 }
    }
}

pub fn default_attachments() -> Vec<Attachment> {
    vec![
        Attachment::new("Red Dot Sight", AttachmentSlot::Sight, Rarity::Common, vec![("spread", -0.15)]),
        Attachment::new("Suppressor", AttachmentSlot::Barrel, Rarity::Rare, vec![("damage", -0.1), ("spread", -0.05)]),
        Attachment::new("Extended Mag", AttachmentSlot::Magazine, Rarity::Uncommon, vec![("magazine", 15.0)]),
    ]
}
