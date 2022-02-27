use bevy::prelude::Component;

use crate::AppState;

#[derive(Component)]
pub struct Screen(pub AppState);
