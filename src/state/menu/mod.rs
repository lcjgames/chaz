use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::state::AppState;

mod screen;
use screen::Screen;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(title))
        //    .add_system_set(SystemSet::on_update(AppState::Menu).with_system(options_button))
        //    .add_system_set(SystemSet::on_update(AppState::Menu).with_system(play_button))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup))
        //     .add_system_set(SystemSet::on_enter(AppState::Options).with_system(options))
        //     .add_system_set(SystemSet::on_update(AppState::Options).with_system(back_button))
        //     .add_system_set(SystemSet::on_exit(AppState::Options).with_system(options_cleanup))
        //     .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(level_select))
        //     .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(back_button))
        //     .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(level_select_cleanup))
        ;
    }
}

fn title(
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&MainCamera, &Transform)>,
) {
    let camera_position = camera_query.single().1.translation;
    let text_style = TextStyle {
        font: asset_server.load("kenney-fonts/Fonts/Kenney Blocks.ttf"),
        font_size: 120.0,
        color: Color::FUCHSIA,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Chaz", text_style, text_alignment),
        transform: Transform::from_translation(Vec3::new(camera_position.x, camera_position.y + 150.0, 10.0)),
        ..Default::default()
    })
        .insert(Screen(AppState::Menu));
    state.set(AppState::Game).unwrap(); //TODO: remove
}

fn cleanup(
    state: Res<State<AppState>>,
    mut commands: Commands,
    query: Query<(Entity, &Screen)>,
) {
    for (id, screen) in query.iter() {
        if &screen.0 == state.current() {
            commands.entity(id).despawn();
        }
    }
}
