use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::sprite::*;

#[derive(Component, Default)]
pub struct Background; //TODO: layers with paralax?

const IMAGE_SIZE: f32 = 24.0;
const SCALE: f32 = 10.0;
const TILE_SIZE: f32 = IMAGE_SIZE * SCALE;
const CLOUD_HEIGHT: f32 = 100.0;

fn snap_to_grid(position: &Vec3) -> Vec3 {
    Vec3::new(snap_component(position.x), snap_component(position.y), position.z)
}

fn snap_component(component: f32) -> f32 {
    component - component % TILE_SIZE
}

fn in_tile(tile_start: &Vec3, position: &Vec3) -> bool {
    in_vertical_strip(tile_start, position) && in_horizontal_strip(tile_start, position)
}

fn in_vertical_strip(tile_start: &Vec3, position: &Vec3) -> bool {
    let difference = position.x - tile_start.x;
    0.0 <= difference && difference < TILE_SIZE
}

fn in_horizontal_strip(tile_start: &Vec3, position: &Vec3) -> bool {
    let difference = position.y - tile_start.y;
    0.0 <= difference && difference < TILE_SIZE
}

fn get_image(position: &Vec3) -> &'static str {
    let image = if position.y < CLOUD_HEIGHT - TILE_SIZE { SpriteTypeStates::Full } else if position.y >= CLOUD_HEIGHT { SpriteTypeStates::Empty } else { SpriteTypeStates::Half };
    SPRITES[&SpriteType::BlueBG][&image]
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    query: Query<(Entity, &Background, &Transform)>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec3::new(window.width(), window.height(), 0.0);
    let start_position = - window_size / 2.0;
    spawn_tiles(
        &start_position,
        &window_size,
        &mut commands,
        &asset_server,
        &query
    );
}

pub fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    background_query: Query<(Entity, &Background, &Transform)>,
    camera_query: Query<(&MainCamera, &Transform)>,
) {
    let camera_position = camera_query.single().1.translation;
    let window = windows.get_primary().unwrap();
    let window_size = Vec3::new(window.width(), window.height(), 0.0);
    let start_position = Vec3::new(camera_position.x, camera_position.y, 0.0) - window_size / 2.0;
    spawn_tiles(
        &start_position,
        &window_size,
        &mut commands,
        &asset_server,
        &background_query,
    );
    despawn_tiles(
        &start_position,
        &window_size,
        &mut commands,
        &background_query,
    );
}

fn spawn_tiles(
    start_position: &Vec3,
    window_size: &Vec3,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<(Entity, &Background, &Transform)>,
) {
    let horizontal_count = (window_size.x / TILE_SIZE).ceil() as i32;
    let vertical_count = (window_size.y / TILE_SIZE).ceil() as i32;
    let start_position = snap_to_grid(start_position);
    for i in -3..3+horizontal_count {
        for j in -3..3+vertical_count {
            let position = start_position + Vec3::new(i as f32*TILE_SIZE, j as f32*TILE_SIZE, 0.0);
            if query.iter()
                .map(|(_, _, tile_transform)| tile_transform.translation)
                .any(|tile_position| in_tile(&tile_position, &position)) {
                continue;
            }
            let image = get_image(&position);
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.get_handle(image),
                    transform: Transform::from_translation(position).with_scale(Vec3::splat(SCALE)),
                    ..Default::default()
                }).insert(Background::default());
        }
    }
}

fn despawn_tiles(
    start_position: &Vec3,
    window_size: &Vec3,
    commands: &mut Commands,
    query: &Query<(Entity, &Background, &Transform)>,
) {
    let despawn_margin = 4.5 * TILE_SIZE;
    let left_margin = start_position.x - despawn_margin;
    let bottom_margin = start_position.y - despawn_margin;
    let end_position = *start_position + *window_size;
    let right_margin = end_position.x + TILE_SIZE + despawn_margin;
    let upper_margin = end_position.y + TILE_SIZE + despawn_margin;
    for (id, _, transform) in query.iter() {
        let position = transform.translation;
        if position.x < left_margin || position.x > right_margin || position.y < bottom_margin || position.y > upper_margin {
            commands.entity(id).despawn();
        }
    }
}

pub fn clear_background(
    mut commands: Commands,
    query: Query<(Entity, &Background)>,
) {
    for (id, _) in query.iter() {
        commands.entity(id).despawn();
    }
}
