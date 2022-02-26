use bevy::prelude::*;

use crate::sprite::{SPRITES, SpriteType, SpriteTypeStates};

#[derive(Component)]
pub struct LoadingBar {
    pub lower_bound: f32,
    pub upper_bound: f32,
}

impl LoadingBar {
    pub fn get_image(&self, percent: f32) -> &str {
        if percent < self.lower_bound {
            SPRITES[&SpriteType::Heart][&SpriteTypeStates::Empty]
        } else if percent >= self.upper_bound {
            SPRITES[&SpriteType::Heart][&SpriteTypeStates::Full]
        } else {
            SPRITES[&SpriteType::Heart][&SpriteTypeStates::Half]
        }
    }
}
