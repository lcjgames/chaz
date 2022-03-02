use bevy::prelude::*;

use crate::background::*;
use crate::camera::*;
use crate::controls::Controls;
use crate::options::{Difficulty, Options};
use crate::state::{AppState, GameOverEvent};
use crate::sprite::*;

mod direction;

mod hitbox;
use hitbox::*;

mod map;
use map::*;

mod player;
use player::*;

mod positions;
use positions::*;

mod velocity;
use velocity::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RivalPositions>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(reset_camera_position))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_background))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(load_level))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animation))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_direction))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_spritesheet))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_ground_collision))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_enemy_collision))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(check_win))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(camera_movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_background))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(out_of_bounds))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(record_player_position))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_rival_position));
    }
}

fn load_level(
    rival_positions: Res<RivalPositions>,
    options: Res<Options>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    crate::console_log!("{:?}", options);
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
                    Tile::Win => { entity.insert( WinHitbox(hitbox) ); },
                    Tile::Player => {
                        entity.insert_bundle(PlayerBundle {
                            ground_hitbox: PlayerGroundHitbox(hitbox.clone()),
                            enemy_hitbox: PlayerEnemyHitbox(hitbox),
                            ..Default::default()
                        });
                    },
                    Tile::Rival => {
                        entity.insert_bundle(RivalBundle {
                            positions: rival_positions.0.clone(),
                            ..Default::default()
                        });
                        let mut spawn_torch = |scale| {
                            entity.with_children(|parent| {
                                parent.spawn_bundle(SpriteBundle {
                                    texture: asset_server.get_handle("torch-light-effect.png"),
                                    transform: Transform::from_scale(Vec3::splat(scale)),
                                    ..Default::default()
                                });
                            });
                        };
                        match options.difficulty {
                            Difficulty::Training => {},
                            Difficulty::Easy => spawn_torch(3.0),
                            Difficulty::Medium => spawn_torch(2.0),
                            Difficulty::Hard => spawn_torch(1.0),
                            Difficulty::Zatoichi => {
                                commands.spawn_bundle(SpriteBundle {
                                    texture: asset_server.load("zatoichi-vision.png"),
                                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)).with_scale(Vec3::splat(100.0)),
                                    ..Default::default()
                                });
                            },
                        };
                    },
                    Tile::Blue | Tile::Jeremy => {
                        entity.insert(EnemyHitbox(hitbox));
                    },
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
    mut query: Query<(&mut Character, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
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
    mut query: Query<(&mut Character, &Controls, &mut Velocity, &mut direction::Direction)>,
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
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&MainCamera, &mut Transform), Without<Player>>,
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
    mut player_query: Query<(&mut Character, &PlayerGroundHitbox, &mut Transform, &mut Velocity), Without<GroundHitbox>>,
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
    mut game_over: EventWriter<GameOverEvent>,
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &EnemyHitbox, &Transform), Without<PlayerGroundHitbox>>,
    mut player_query: Query<(&Character, &PlayerEnemyHitbox, &Transform, &mut Velocity), Without<GroundHitbox>>,
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
                    _ => {
                        game_over.send(GameOverEvent {
                            secondary_message: Some("Killed by an enemy".to_string()),
                            ..Default::default()
                        });
                        state.set(AppState::GameOver).unwrap();
                    },
                };
            }
        }
    }
}

fn check_win(
    mut rival_positions: ResMut<RivalPositions>,
    mut game_over: EventWriter<GameOverEvent>,
    mut state: ResMut<State<AppState>>,
    player_query: Query<(&Player, &PlayerGroundHitbox, &Transform, &Positions)>,
    win_tile_query: Query<(&WinHitbox, &Transform), Without<Player>>,
) {
    for (_, player_hitbox, player_transform, player_positions) in player_query.iter() {
        for (win_hitbox, win_transform) in win_tile_query.iter() {
            if let Some(_) = player_hitbox.0.collide(&player_transform.translation, &win_hitbox.0, &win_transform.translation) {
                rival_positions.0 = Positions {
                    values: player_positions.values.iter().map(|p| *p - Vec3::new(0.0, 0.0, 1.0)).collect(),
                    ..Default::default()
                };
                //To change the hardcoded path, uncomment the code below,
                //then copy and paste this output into the map
                //TODO: a more sophisticated way to do this
                /*
                use crate::log::*;
                console_log!("rival_positions: Positions {{");
                console_log!("values: vec![");
                for position in positions.values.iter() {
                    console_log!("Vec3::new({}, {}, 1.0),", position.x, position.y)
                }
                console_log!("].iter().copied().collect(), //TODO: is there a better way to do this?");
                console_log!("timer: Timer::from_seconds({}, true),", positions.timer.duration().as_secs_f32());
                console_log!("}}");
                */
                game_over.send(GameOverEvent {
                    main_message: "You\nwin".to_string(),
                    ..Default::default()
                });
                state.set(AppState::GameOver).unwrap();
            }
        }
    }
}

fn out_of_bounds(
    mut game_over: EventWriter<GameOverEvent>,
    mut state: ResMut<State<AppState>>,
    windows: Res<Windows>,
    player_query: Query<(&Character, &Transform)>,
    camera_query: Query<(&MainCamera, &Transform), Without<Character>>,
) {
    let (_, camera_position) = camera_query.single();

    let window = windows.get_primary().unwrap();
    let screen_bottom = camera_position.translation.y - window.height() / 2.0;

    for (_, transform) in player_query.iter() {
        if transform.translation.y < screen_bottom {
            game_over.send(GameOverEvent {
                secondary_message: Some("Fell from a great height".to_string()),
                ..Default::default()
            });
            state.set(AppState::GameOver).unwrap();
        }
    }
}

fn record_player_position(
    time: Res<Time>,
    mut query: Query<(&Player, &Transform, &mut Positions)>,
) {
    for (_, transform, mut positions) in query.iter_mut() {
        positions.timer.tick(time.delta());
        if positions.timer.finished() {
            positions.values.push_back(transform.translation);
        }
    }
}

fn update_rival_position(
    mut game_over: EventWriter<GameOverEvent>,
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut query: Query<(&Rival, &mut Transform, &mut Positions)>,
) {
    for (_, mut transform, mut positions) in query.iter_mut() {
        if positions.values.is_empty() {
            game_over.send(GameOverEvent {
                secondary_message: Some("Your rival was faster".to_string()),
                ..Default::default()
            });
            state.set(AppState::GameOver).unwrap();
            return;
        }
        positions.timer.tick(time.delta());
        transform.translation = if positions.timer.finished() {
            positions.values.pop_front().unwrap()
        } else {
            let proportion = positions.timer.elapsed_secs() / positions.timer.duration().as_secs_f32();
            // use crate::log::*;
            // console_log!("elapsed = {}, duration = {}, proportion = {}", positions.timer.elapsed_secs(), positions.timer.duration().as_secs_f32(), proportion);
            //TODO: bug! when timer duration is large, you can see the character is not transitioning smoothly
            proportion*positions.values[0] + (1.0-proportion)*transform.translation
        }
    }
}
