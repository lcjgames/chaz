use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::sprite::*;

#[derive(Component, Default)]
pub struct Background; //TODO: layers with paralax?

const IMAGE_SIZE: f32 = 24.0;
const SCALE: f32 = 30.0;
const TILE_SIZE: f32 = IMAGE_SIZE * SCALE;
const CLOUD_HEIGHT: f32 = 100.0;

fn snap_to_grid(position: Vec3) -> Vec3 {
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
    0.0 < difference && difference < TILE_SIZE
}

fn in_horizontal_strip(tile_start: &Vec3, position: &Vec3) -> bool {
    let difference = position.y - tile_start.y;
    0.0 < difference && difference < TILE_SIZE
}

fn get_image(position: &Vec3) -> &'static str {
    let image = if position.y < CLOUD_HEIGHT - TILE_SIZE { SpriteTypeStates::Full } else if position.y >= CLOUD_HEIGHT { SpriteTypeStates::Empty } else { SpriteTypeStates::Half };
    SPRITES[&SpriteType::BlueBG][&image]
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    query: Query<(&Background, &Transform)>,
) {
    let window = windows.get_primary().unwrap();
    let start_position = Vec3::new(-window.width()/2.0, -window.height()/2.0, 0.0);
    spawn_tiles(
        start_position,
        1+(window.width() / TILE_SIZE).ceil() as i32,
        1+(window.height() / TILE_SIZE).ceil() as i32,
        &mut commands,
        &asset_server,
        &query
    );
}

pub fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    background_query: Query<(&Background, &Transform)>,
    camera_query: Query<(&MainCamera, &Transform)>,
) {
    let camera_position = camera_query.single().1.translation;
    let window = windows.get_primary().unwrap();
    let start_position = Vec3::new(camera_position.x - window.width()/2.0, camera_position.y - window.height()/2.0, 0.0);
    spawn_tiles(
        start_position,
        (window.width() / TILE_SIZE).ceil() as i32,
        (window.height() / TILE_SIZE).ceil() as i32,
        &mut commands,
        &asset_server,
        &background_query
    );
    //TODO: despawn_tiles
}

fn spawn_tiles(
    start_position: Vec3,
    horizontal_count: i32,
    vertical_count: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<(&Background, &Transform)>,
) {
    let start_position = snap_to_grid(start_position);
    for i in -3..3+horizontal_count {
        for j in -3..3+vertical_count {
            let position = start_position + Vec3::new(i as f32*TILE_SIZE, j as f32*TILE_SIZE, 0.0);
            if query.iter().any(|(_, tile_transform)| in_tile(&tile_transform.translation, &position)) {
                break;
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

pub fn clear_background(
    mut commands: Commands,
    query: Query<(Entity, &Background)>,
) {
    for (id, _) in query.iter() {
        commands.entity(id).despawn();
    }
}
