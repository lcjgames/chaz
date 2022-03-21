use bevy::prelude::*;
use bevy_kira_audio::*;
use enum_iterator::IntoEnumIterator;

use crate::AppState;
use crate::options::Options;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .init_resource::<Music>()
            .add_system_set(SystemSet::on_update(AppState::Options).with_system(volume));
        for state in AppState::into_enum_iter() {
            app.add_system_set(SystemSet::on_enter(state).with_system(play_song(state.into())));
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Song {
    MainTheme,
    GameTheme,
    GameOverTheme,
}

impl From<AppState> for Song {
    fn from(state: AppState) -> Self {
        match state {
            AppState::PreLoad | AppState::Loading => Song::MainTheme,
            AppState::Menu | AppState::Options | AppState::LevelSelect | AppState::Leaderboard => Song::MainTheme,
            AppState::Game | AppState::Pause => Song::GameTheme,
            AppState::GameOver => Song::GameOverTheme,
        }
    }
}

impl ToString for Song {
    fn to_string(&self) -> String {
        match self {
            Song::MainTheme => "main_theme.ogg".to_string(),
            Song::GameTheme => "game_theme.ogg".to_string(),
            Song::GameOverTheme => "game_over_theme.ogg".to_string(),
        }
    }
}

struct MusicId {
    song: Song,
    id: InstanceHandle,
}

#[derive(Default)]
struct Music(Option<MusicId>);

fn play_song(song: Song) -> impl Fn(Res<AssetServer>, Res<Audio>, ResMut<Music>) {
    move |asset_server: Res<AssetServer>, audio: Res<Audio>, mut music: ResMut<Music>| {
        if let Some(music_id) = &music.0 {
            if music_id.song == song {
                return;
            }
            audio.stop();
        }
        let id = audio.play_looped(asset_server.load(song.to_string().as_str()));
        music.0 = Some(MusicId { song, id });
    }
}

fn volume(
    options: Res<Options>,
    audio: Res<Audio>,
) {
    let volume = options.music_volume as f32 / 100.0;
    audio.set_volume(volume);
}
