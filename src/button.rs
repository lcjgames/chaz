use bevy::prelude::*;

use crate::AppState;
use crate::options::*;

#[derive(Component)]
pub enum Action {
    ChangeState(AppState),
}

pub struct ButtonBuilder<S: Into<String>> {
    pub text: S,
    pub action: Action,
}

impl<S: Into<String>> ButtonBuilder<S> {
    pub fn build(self, commands: &mut Commands, asset_server: &Res<AssetServer>, state: &Res<State<AppState>>) {
        use crate::screen::Screen;
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
            .insert(self.action)
            .insert(Screen(*state.current()))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        self.text,
                        TextStyle {
                            font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                    .insert(Screen(*state.current()));
            });
    }
}

pub fn buttons(
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

#[derive(Default)]
pub struct OptionToggleBuilder<T> {
    pub value: T,
}

impl<T: Clone + Component + ToString> OptionToggleBuilder<T> {
    pub fn build(self, commands: &mut Commands, asset_server: &Res<AssetServer>, state: &Res<State<AppState>>) {
        use crate::screen::Screen;
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
            .insert(self.value.clone())
            .insert(Screen(*state.current()))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        self.value.to_string(),
                        TextStyle {
                            font: asset_server.load("kenney-fonts/Fonts/Kenney Pixel.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                    .insert(Screen(*state.current()));
            });
    }
}

pub fn toggles<T: Clone + Component + Next + Option + ToString>(
    mut options: ResMut<Options>,
    mut query: Query<(&Interaction, &mut UiColor, &mut T, &Children), (Changed<Interaction>, With<Button>)>,
    mut children_query: Query<&mut Text, Without<Button>>,
) {
    for (interaction, mut color, mut t, children) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::DARK_GRAY.into(),
            Interaction::None => Color::rgb(0.15, 0.15, 0.15).into(),
            Interaction::Clicked => {
                *t = t.clone().next();
                for child in children.iter() {
                    let mut text = children_query.get_mut(*child).unwrap();
                    text.sections[0].value = t.to_string();
                }
                options.set(t.clone());
                Color::DARK_GRAY.into()
            },
        }
    }
}
