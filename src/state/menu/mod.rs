use bevy::prelude::*;

use crate::state::AppState;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        // app
        //     .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(menu))
        //     .add_system_set(SystemSet::on_update(AppState::Menu).with_system(options_button))
        //     .add_system_set(SystemSet::on_update(AppState::Menu).with_system(play_button))
        //     .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(menu_cleanup))
        //     .add_system_set(SystemSet::on_enter(AppState::Options).with_system(options))
        //     .add_system_set(SystemSet::on_update(AppState::Options).with_system(back_button))
        //     .add_system_set(SystemSet::on_exit(AppState::Options).with_system(options_cleanup))
        //     .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(level_select))
        //     .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(back_button))
        //     .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(level_select_cleanup));
    }
}
