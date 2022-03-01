use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::state::AppState;
use crate::sprite::*;

mod loading_bar;
use loading_bar::LoadingBar;

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::PreLoad)
            .add_system_set(SystemSet::on_enter(AppState::PreLoad).with_system(load_preloaded_textures))
            .add_system_set(SystemSet::on_update(AppState::PreLoad).with_system(check_preloaded_textures))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(spawn_camera))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_loading_bar))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_textures))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(remove_loading_bar));
    }
}

fn load_preloaded_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let handles = load_sprites(SpriteType::Heart, &asset_server);
    sprite_handles.handles.insert("loading".to_string(), handles);
}

fn check_preloaded_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let ids = sprite_handles.id_list();
    if let LoadState::Loaded = asset_server.get_group_load_state(ids) {
        state.set(AppState::Loading).unwrap();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn setup_loading_bar(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let texture_atlas= spawn(
        "loading".to_string(),
        &sprite_handles,
        &mut texture_atlases,
        &mut textures,
    );
    let n = 10;
    let interval = 100.0 / n as f32;
    for i in 0..n {
        let position = Vec3::new(-120.0 + 24.0 * (i as f32), -144.0, 0.0);
        let lower_bound = i as f32 * interval;
        let upper_bound = lower_bound + interval;
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(LoadingBar { lower_bound, upper_bound });
    }
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let mut load = |name: SpriteType| {
        let handles = load_sprites(name, &asset_server);
        sprite_handles.handles.insert(name.to_string(), handles);
    };
    for (&key, _) in SPRITES.iter() {
        load(key);
    }
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&LoadingBar, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    let ids = sprite_handles.id_list();
    let mut total = 0;
    let mut loaded = 0;
    for id in ids {
        total += 1;
        match asset_server.get_load_state(id) {
            LoadState::Loaded => loaded += 1,
            LoadState::Failed => panic!("Failed to load asset."),
            _ => {},
        }
    }
    if loaded == total {
        state.set(AppState::Menu).unwrap();
        return;
    }
    let percent = (100*loaded) as f32 / total as f32;
    for (loading_bar, mut sprite, texture_atlas_handle) in query.iter_mut() {
        let image = loading_bar.get_image(percent);
        let handle = asset_server.get_handle(image);
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = texture_atlas.get_texture_index(&handle).unwrap();
    }
}

fn remove_loading_bar(
    mut commands: Commands,
    query: Query<(Entity, &LoadingBar)>,
) {
    for (id, _) in query.iter() {
        commands.entity(id).despawn();
    }
}
