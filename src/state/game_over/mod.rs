use bevy::prelude::*;
use crate::camera::MainCamera;
use crate::GameOverEvent;

use crate::state::AppState;

pub struct GameOver;

impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(show_text))
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(show_button))
            .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(button))
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(cleanup));
    }
}

fn show_text(
    mut commands: Commands,
    mut game_over: EventReader<GameOverEvent>,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&MainCamera, &Transform)>,
) {
    let game_over: GameOverEvent = game_over.iter().next().cloned().unwrap_or_default(); //if there's more than one game over in the same frame, the other ones are discarded
    let camera_position = camera_query.single().1.translation;
    let text_style = TextStyle {
        font: asset_server.load("kenney-fonts/Fonts/Kenney Blocks.ttf"),
        font_size: 96.0,
        color: Color::CRIMSON,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let mut main_text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(game_over.main_message, text_style, text_alignment),
            transform: Transform::from_translation(Vec3::new(camera_position.x, camera_position.y + 150.0, 10.0)),
            ..Default::default()
        });
    if let Some(message) = game_over.secondary_message {
        main_text.with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    message,
                    TextStyle {
                        font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    text_alignment,
                ),
                ..Default::default()
            });
        });
    }
}

fn show_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default()); //TODO: spawn this during pre-load
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
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Restart",
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


fn button(
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
    mut commands: Commands,
    query: Query<Entity, Without<OrthographicProjection>>,
) {
    for id in query.iter() {
        commands.entity(id).despawn();
    }
}
