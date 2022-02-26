use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::controls::Controls;
use crate::state::AppState;
use crate::sprite::*;

mod direction;

mod hitbox;
use hitbox::*;

mod map;
use map::*;

mod player;
use player::*;

mod velocity;
use velocity::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_background))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(load_level))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animation))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_direction))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_spritesheet))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_ground_collision))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_enemy_collision))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(camera_movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(out_of_bounds));
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 24.0;
    let layer = 0.0;
    for i in -10..11 {
        for j in -10..11 {
            //TODO: use one big image looping or just moving with the camera instead of creating a trillion entities
            let cloud_height = 3;
            let image = if j < cloud_height { SpriteTypeStates::Full } else if j == cloud_height { SpriteTypeStates::Half } else { SpriteTypeStates::Empty };
            let image = SPRITES[&SpriteType::BlueBG][&image];
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.get_handle(image),
                    transform: Transform::from_translation(Vec3::new(i as f32*tile_size, j as f32*tile_size, layer)),
                    ..Default::default()
                });
        }
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut spawn = |name| {
        spawn(
            name,
            &sprite_handles,
            &mut texture_atlases,
            &mut textures,
        )
    };
    for tile_info in read_map().tile_info_iter() {
        if let Some(tile_info) = tile_info {
            let mut entity = commands.spawn();
            match tile_info.image {
                SpriteVariant::Sprite(path) => entity.insert_bundle(SpriteBundle {
                        texture: asset_server.get_handle(path),
                        transform: Transform::from_translation(tile_info.position),
                        ..Default::default()
                    }),
                SpriteVariant::SpriteSheet(key) => entity.insert_bundle(SpriteSheetBundle {
                        texture_atlas: spawn(key.to_string()),
                        transform: Transform::from_translation(tile_info.position),
                        ..Default::default()
                    })
                    .insert(SpriteTimer::from_seconds(0.2)),
            };
            if let Some(hitbox) = tile_info.hitbox {
                match tile_info.tile_type {
                    Tile::Empty => panic!("Not possible to have a hitbox on an empty tile"),
                    Tile::Ground => { entity.insert(GroundHitbox(hitbox)); },
                    Tile::Player => {
                        entity.insert_bundle(PlayerBundle {
                            ground_hitbox: PlayerGroundHitbox(hitbox.clone()),
                            enemy_hitbox: PlayerEnemyHitbox(hitbox),
                            ..Default::default()
                        });
                    },
                    Tile::Blue => { entity.insert(EnemyHitbox(hitbox)); },
                    Tile::Npc(_) => {
                        todo!()
                    },
                }
            }
        }
    }
}

fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut SpriteTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut sprite_timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        sprite_timer.timer.tick(time.delta());
        if sprite_timer.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &direction::Direction)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = *direction == direction::Direction::Right;
    }
}

fn player_spritesheet(
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut query: Query<(&mut PlayerCharacter, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
) {
    for (mut player, mut sprite, mut texture_atlas_handle) in query.iter_mut() {
        if let Some(sheet) = player.update_spritesheet() {
            *texture_atlas_handle = spawn(sheet.to_string(), &sprite_handles, &mut texture_atlases, &mut textures);
            *sprite = TextureAtlasSprite::default();
        }
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerCharacter, &Controls, &mut Velocity, &mut direction::Direction)>,
) {
    for (mut player, controls, mut velocity, mut direction) in query.iter_mut() {
        let new_direction = direction::Direction::from_input(input.pressed(controls.left), input.pressed(controls.right));
        velocity.update(new_direction);
        if let Some(new_direction) = new_direction {
            *direction = new_direction;
        }
        player.update_walk_state(velocity.0.x);

        if input.just_pressed(controls.jump) {
            if let Ok(_) = player.try_jump() {
                velocity.0.y = 500.0;
            }
        }
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.apply_gravity(time.delta_seconds());
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn camera_movement(
    windows: Res<Windows>,
    player_query: Query<(&PlayerCharacter, &Transform)>,
    mut camera_query: Query<(&MainCamera, &mut Transform), Without<PlayerCharacter>>,
) {
    let window = windows.get_primary().unwrap();
    let horizontal_limit = window.width() * 0.3;

    let (_, player_position) = player_query.single();
    let player_position = player_position.translation.x;

    let (_, mut camera_position) = camera_query.single_mut();

    let left_limit = camera_position.translation.x - horizontal_limit;
    let right_limit = camera_position.translation.x + horizontal_limit;

    if player_position < left_limit {
        camera_position.translation.x = player_position + horizontal_limit;
    } else if player_position > right_limit {
        camera_position.translation.x = player_position - horizontal_limit;
    }
}

fn player_ground_collision(
    ground_query: Query<(&GroundHitbox, &Transform), Without<PlayerGroundHitbox>>,
    mut player_query: Query<(&mut PlayerCharacter, &PlayerGroundHitbox, &mut Transform, &mut Velocity), Without<GroundHitbox>>,
) {
    for (mut player, player_hitbox, mut player_transform, mut player_velocity) in player_query.iter_mut() {
        for (ground_hitbox, ground_transform) in ground_query.iter() {
            if let Some(collision) = player_hitbox.0.collide(&player_transform.translation, &ground_hitbox.0, &ground_transform.translation) {
                match collision.collision_type {
                    CollisionType::Bottom => {
                        player_transform.translation.y += collision.overlap;
                        if player_velocity.0.y < 0.0 {
                            player_velocity.0.y = 0.0;
                            player.hit_ground();
                        }
                    },
                    CollisionType::Top => {
                        player_transform.translation.y -= collision.overlap;
                        player_velocity.stop_top();
                    },
                    CollisionType::Left => {
                        player_transform.translation.x += collision.overlap;
                        player_velocity.stop_left();
                    },
                    CollisionType::Right => {
                        player_transform.translation.x -= collision.overlap;
                        player_velocity.stop_right();
                    },
                };
            }
        }
    }
}

fn player_enemy_collision(
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &EnemyHitbox, &Transform), Without<PlayerGroundHitbox>>,
    mut player_query: Query<(&PlayerCharacter, &PlayerGroundHitbox, &Transform, &mut Velocity), Without<GroundHitbox>>,
) {
    for (_, player_hitbox, player_transform, mut player_velocity) in player_query.iter_mut() {
        for (enemy_id, enemy_hitbox, enemy_transform) in enemy_query.iter() {
            if let Some(collision) = player_hitbox.0.collide(&player_transform.translation, &enemy_hitbox.0, &enemy_transform.translation) {
                match collision.collision_type {
                    CollisionType::Bottom => {
                        //TODO: change player and enemy states so that some animation plays or there is a chance to jump again or something
                        commands.entity(enemy_id).despawn();
                        player_velocity.0.y *= -1.0;
                    },
                    _ => { state.set(AppState::GameOver).unwrap(); },
                };
            }
        }
    }
}

fn out_of_bounds(
    mut state: ResMut<State<AppState>>,
    windows: Res<Windows>,
    player_query: Query<(&PlayerCharacter, &Transform)>,
    camera_query: Query<(&MainCamera, &Transform), Without<PlayerCharacter>>,
) {
    let (_, camera_position) = camera_query.single();

    let window = windows.get_primary().unwrap();
    let screen_bottom = camera_position.translation.y - window.height() / 2.0;

    for (_, transform) in player_query.iter() {
        if transform.translation.y < screen_bottom {
            state.set(AppState::GameOver).unwrap();
        }
    }
}