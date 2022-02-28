use bevy::prelude::*;
use crate::AppState;

#[derive(Component)]
pub enum Action {
    ChangeState(AppState),
}
