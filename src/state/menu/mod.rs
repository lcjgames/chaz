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
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(show_play_button))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(play_button))
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
}

fn show_play_button(
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
            });
        });
}

fn play_button(
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::DARK_GRAY.into(),
            Interaction::None => Color::rgb(0.15, 0.15, 0.15).into(),
            Interaction::Clicked => {
                state.set(AppState::Game).unwrap();
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
