use bevy::prelude::*;
use bevy_egui::*;

use crate::background::*;
use crate::button::*;
use crate::camera::*;
use crate::options::*;
use crate::screen::Screen;
use crate::state::AppState;

use crate::state::game::map::LEVEL_COUNT; //TODO: map should be moved to top level

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LeaderBoardOptions>()
            .init_resource::<Options>()
            .add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(reset_camera_position))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(ui_camera))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(title))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn_background))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(title_animation))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(move_camera))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(update_background))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(show_menu_buttons))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(buttons))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(clear_background))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup))
            .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(ui_camera))
            .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(show_level_select_buttons))
            .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(buttons))
            .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(move_camera))
            .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(update_background))
            .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(clear_background))
            .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(cleanup))
            .add_system_set(SystemSet::on_enter(AppState::Options).with_system(ui_camera))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(show_options_menu))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(buttons))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(toggles::<Difficulty>))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(move_camera))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(update_background))
            .add_system_set(SystemSet::on_exit(AppState::Options).with_system(clear_background))
            .add_system_set(SystemSet::on_exit(AppState::Options).with_system(cleanup))
            .add_system_set(SystemSet::on_enter(AppState::Leaderboard).with_system(ui_camera))
            .add_system_set(SystemSet::on_update(AppState::Leaderboard).with_system(show_leaderboards_ui))
            .add_system_set(SystemSet::on_update(AppState::Leaderboard).with_system(buttons))
            .add_system_set(SystemSet::on_update(AppState::Leaderboard).with_system(move_camera))
            .add_system_set(SystemSet::on_update(AppState::Leaderboard).with_system(update_background))
            .add_system_set(SystemSet::on_exit(AppState::Leaderboard).with_system(clear_background))
            .add_system_set(SystemSet::on_exit(AppState::Leaderboard).with_system(cleanup));
    }
}

#[derive(Component)]
struct Title;

fn ui_camera(
    mut commands: Commands,
    state: Res<State<AppState>>,
) {
    commands.spawn_bundle(UiCameraBundle::default())
        .insert(Screen(*state.current()));
}

fn title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    let text_style = TextStyle {
        font: asset_server.load("kenney-fonts/Fonts/Kenney Blocks.ttf"),
        font_size: 120.0,
        color: Color::FUCHSIA,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let camera_id = camera_query.single();
    commands.entity(camera_id).with_children(|camera| {
       camera.spawn_bundle(Text2dBundle {
           text: Text::with_section("Chaz", text_style, text_alignment),
           transform: Transform::from_translation(Vec3::new(0.0, 150.0, -10.0)),
           ..Default::default()
       })
           .insert(Title)
           .insert(Screen(AppState::Menu));
    });
}

fn title_animation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Title>>,
) {
    for mut transform in query.iter_mut() {
        //that math major finally came in handy
        const HALF_TURN: f64 = std::f64::consts::PI;
        const ANIMATION_FIRST_ROTATION_START: f64 = 1.0;
        const ANIMATION_FIRST_ROTATION_END: f64 = 1.8;
        const ANIMATION_SECOND_ROTATION_START: f64 = 3.0;
        const ANIMATION_SECOND_ROTATION_END: f64 = 3.1;
        const ANIMATION_TOTAL_TIME: f64 = 6.0;
        let time = time.seconds_since_startup() % ANIMATION_TOTAL_TIME;
        let angle = if time < ANIMATION_FIRST_ROTATION_START {
            0.0
        } else if time < ANIMATION_FIRST_ROTATION_END {
            let proportion = (time - ANIMATION_FIRST_ROTATION_START) / (ANIMATION_FIRST_ROTATION_END - ANIMATION_FIRST_ROTATION_START);
            proportion * HALF_TURN
        } else if time < ANIMATION_SECOND_ROTATION_START {
            let proportion = (time - ANIMATION_FIRST_ROTATION_END) / (ANIMATION_SECOND_ROTATION_START - ANIMATION_FIRST_ROTATION_END);
            let wobble = 0.1 * f64::sin(4.0 * HALF_TURN * proportion);
            HALF_TURN + wobble
        } else if time < ANIMATION_SECOND_ROTATION_END {
            let proportion = (time - ANIMATION_SECOND_ROTATION_START) / (ANIMATION_SECOND_ROTATION_END - ANIMATION_SECOND_ROTATION_START);
            (1.0 - proportion) * HALF_TURN
        } else {
            0.0
        };
        transform.rotation = Quat::from_rotation_z(angle as f32);
    }
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in query.iter_mut() {
        let time = time.seconds_since_startup() as f32;
        transform.translation.x = time * 30.0;
        transform.translation.y = 30.0 * f32::sin(0.5 * time);
    }
}

fn show_menu_buttons(
    state: Res<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    ButtonBuilder {
        text: "Play",
        action: Action::ChangeState(AppState::LevelSelect),
    }.build(&mut commands, &asset_server, &state);
    ButtonBuilder {
        text: "Scores",
        action: Action::ChangeState(AppState::Leaderboard),
    }.build(&mut commands, &asset_server, &state);
    ButtonBuilder {
        text: "Options",
        action: Action::ChangeState(AppState::Options),
    }.build(&mut commands, &asset_server, &state);
}

fn show_level_select_buttons(
    state: Res<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    ButtonBuilder {
        text: "Back",
        action: Action::ChangeState(AppState::Menu),
    }.build(&mut commands, &asset_server, &state);
    for i in 1..LEVEL_COUNT { //it looks like level 0 is a secret level lol
        ButtonBuilder {
            text: format!("Level {}", i),
            action: Action::Play{ level: i},
        }.build(&mut commands, &asset_server, &state);
    }
}

#[derive(Component)]
struct DifficultyText;

fn show_options_menu(
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
    mut options: ResMut<Options>,
    mut state: ResMut<State<AppState>>,
) {
    use egui::*;
    use enum_iterator::IntoEnumIterator;

    let game_window = windows.get_primary().unwrap();

    Window::new("Options")
        .collapsible(false)
        .resizable(false)
        .fixed_pos((game_window.width() * 0.2, game_window.height() * 0.1))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut options.name);
            ui.label("Difficulty: ");
            ComboBox::from_id_source("Difficulty select")
                .selected_text(options.difficulty.to_string())
                .show_ui(ui, |ui| {
                    for difficulty in Difficulty::into_enum_iter() {
                        ui.selectable_value(&mut options.difficulty, difficulty, difficulty.to_string());
                    }
                });
            ui.label("Volume: ");
            ui.add(Slider::new(&mut options.music_volume, 0..=100));
            if ui.button("Back").clicked() {
                state.set(AppState::Menu).unwrap();
            }
        });
}

fn show_leaderboards_ui(
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
    mut options: ResMut<LeaderBoardOptions>,
    mut state: ResMut<State<AppState>>,
) {
    use crate::score::*;
    use egui::*;
    use enum_iterator::IntoEnumIterator;

    let game_window = windows.get_primary().unwrap();

    Window::new("Leaderboard")
        .collapsible(false)
        .resizable(false)
        .fixed_pos((game_window.width() * 0.2, game_window.height() * 0.1))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Level: ");
            ComboBox::from_id_source("Level select")
                .selected_text(options.level.to_string())
                .show_ui(ui, |ui| {
                    ui.label("Level: ");
                    for i in 0..LEVEL_COUNT {
                        ui.selectable_value(&mut options.level, i, i.to_string());
                    }
                });
            ui.label("Difficulty: ");
            ComboBox::from_id_source("Difficulty select")
                .selected_text(options.difficulty.to_string())
                .show_ui(ui, |ui| {
                    for difficulty in Difficulty::into_enum_iter() {
                        ui.selectable_value(&mut options.difficulty, difficulty, difficulty.to_string());
                    }
                });
            for score in get_scores(options.level, options.difficulty) {
                ui.label(format!("{}: {}s", score.name, score.time));
            }
            if ui.button("Back").clicked() {
                state.set(AppState::Menu).unwrap();
            }
        });
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
