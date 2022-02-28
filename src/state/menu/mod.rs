use bevy::prelude::*;

use crate::button::Action;
use crate::camera::MainCamera;
use crate::screen::Screen;
use crate::state::AppState;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(title))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(title_animation))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(play_button))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(options_button))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(buttons))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup))
            .add_system_set(SystemSet::on_enter(AppState::Options).with_system(back_button))
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(buttons))
            .add_system_set(SystemSet::on_exit(AppState::Options).with_system(cleanup))
        //     .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(level_select))
        //     .add_system_set(SystemSet::on_update(AppState::LevelSelect).with_system(back_button))
        //     .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(level_select_cleanup))
        ;
    }
}

#[derive(Component)]
struct Title;

fn title(
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
        .insert(Title)
        .insert(Screen(AppState::Menu));
}

fn title_animation(
    time: Res<Time>,
    mut query: Query<(&Title, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
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

fn play_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default())
        .insert(Screen(AppState::Menu));
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Action::ChangeState(AppState::Game))
        .insert(Screen(AppState::Menu))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            })
                .insert(Screen(AppState::Menu));
        });
}

fn options_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default())
        .insert(Screen(AppState::Menu));
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Action::ChangeState(AppState::Options))
        .insert(Screen(AppState::Menu))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Options",
                    TextStyle {
                        font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            })
                .insert(Screen(AppState::Menu));
        });
}

fn back_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default())
        .insert(Screen(AppState::Options));
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Action::ChangeState(AppState::Menu))
        .insert(Screen(AppState::Options))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Back",
                    TextStyle {
                        font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            })
                .insert(Screen(AppState::Options));
        });
}

fn buttons(
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Interaction, &mut UiColor, &Action), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, action) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::DARK_GRAY.into(),
            Interaction::None => Color::rgb(0.15, 0.15, 0.15).into(),
            Interaction::Clicked => {
                match action {
                    Action::ChangeState(screen) => { state.set(*screen).unwrap(); },
                }
                Color::DARK_GRAY.into()
            },
        }
    }
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
