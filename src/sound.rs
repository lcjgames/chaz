use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::AppState;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .init_resource::<Music>()
            .add_system_set(SystemSet::on_enter(AppState::PreLoad).with_system(play_main_theme))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(play_main_theme))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(play_main_theme))
            .add_system_set(SystemSet::on_enter(AppState::Options).with_system(play_main_theme))
            .add_system_set(SystemSet::on_enter(AppState::LevelSelect).with_system(play_main_theme))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(play_game_theme))
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(play_game_over_theme))
            .add_system_set(SystemSet::on_enter(AppState::Pause).with_system(play_game_theme));
    }
}

#[derive(PartialEq)]
enum Song {
    MainTheme,
    GameTheme,
    GameOverTheme,
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

fn play_main_theme(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music: ResMut<Music>,
) {
    play_song(Song::MainTheme, &asset_server, &audio, &mut music);
}

fn play_game_theme(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music: ResMut<Music>,
) {
    play_song(Song::GameTheme, &asset_server, &audio, &mut music);
}

fn play_game_over_theme(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music: ResMut<Music>,
) {
    play_song(Song::GameOverTheme, &asset_server, &audio, &mut music);
}

fn play_song(
    song: Song,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
    music: &mut ResMut<Music>,
) {
    if let Some(music_id) = &music.0 {
        if music_id.song == song {
            return;
        }
        audio.stop();
    }
    let id = audio.play_looped(asset_server.load(song.to_string().as_str()));
    music.0 = Some(MusicId { song, id });
}

fn stop_music(
    audio: Res<Audio>,
) {
    audio.stop();
}
