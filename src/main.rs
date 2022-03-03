#![feature(derive_default_enum)]

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

mod background;
mod button;
mod camera;
mod controls;
mod log;
use log::*;
mod screen;
mod sprite;
use sprite::SpriteHandles;
mod state;
mod options;

use state::*;

#[macro_use]
extern crate lazy_static;

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("papaparararan.ogg"));
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

    console_log!("Starting Game!");
    App::new()
        .init_resource::<SpriteHandles>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(start_background_audio.system())
        .add_plugin(Loading)
        .add_plugin(Menu)
        .add_plugin(Game)
        .add_plugin(GameOver)
        .add_plugin(Pause)
        .run();
}



