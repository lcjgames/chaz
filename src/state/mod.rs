use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq, Hash)]
pub enum AppState {
    PreLoad,
    Loading,
    Menu,
    Options,
    LevelSelect,
    Game,
    GameOver,
    Pause,
}

#[derive(Clone)]
pub struct GameOverEvent {
    main_message: String,
    secondary_message: Option<String>,
}

impl Default for GameOverEvent {
    fn default() -> Self {
        Self {
            main_message: "Game\nOver".to_string(),
            secondary_message: None,
        }
    }
}

mod game;
pub use game::Game;
mod loading;
pub use loading::Loading;
mod menu;
pub use menu::Menu;
mod game_over;
pub use game_over::GameOver;
mod pause;
pub use pause::Pause;
