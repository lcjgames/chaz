use std::str::FromStr;
use bevy::prelude::{Vec2, Vec3};

use super::hitbox::Hitbox;
use super::positions::Positions;

use crate::sprite::{SpriteType, SpriteTypeStates, SpriteVariant};
use crate::Timer;
use crate::log::console_log;

const LEVEL_00: &str = include_str!("../../../assets/maps/00_map.txt");
const LEVEL_01: &str = include_str!("../../../assets/maps/01_map.txt");
const LEVEL_02: &str = include_str!("../../../assets/maps/02_map.txt");
const LEVEL_03: &str = include_str!("../../../assets/maps/03_map.txt");

const POSITIONS_00: &str = include_str!("../../../assets/positions/00_positions.txt");
const POSITIONS_01: &str = include_str!("../../../assets/positions/01_positions.txt");
const POSITIONS_02: &str = include_str!("../../../assets/positions/02_positions.txt");
const POSITIONS_03: &str = include_str!("../../../assets/positions/03_positions.txt");

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Ground,
    Win,
    Player,
    Rival,
    Blue,
    Jeremy,
    Blocky,
    Npc(SpriteType), //TODO: remove
}

impl FromStr for Tile {

    type Err = ();

    fn from_str(input: &str) -> Result<Tile, Self::Err> {
        match input {
            "Empty" => Ok(Tile::Empty),
            "Ground" => Ok(Tile::Ground),
            "Win" => Ok(Tile::Win),
            "Player" => Ok(Tile::Player),
            "Rival" => Ok(Tile::Rival),
            "Blue" => Ok(Tile::Blue),
            "Jeremy" => Ok(Tile::Jeremy),
            "Blocky" => Ok(Tile::Blocky),
            _ => Err(()),
        }
    }
}

impl Tile {
    pub const SIZE: f32 = 18.0;
    pub fn connects_to(self, other: Tile) -> bool {
        match (self, other) {
            (Tile::Ground, Tile::Ground) => true,
            (_, _) => false,
        }
    }
}

pub struct TileInfo {
    pub tile_type: Tile,
    pub position: Vec3,
    pub image: SpriteVariant,
    pub hitbox: Option<Hitbox>,
}

type Line = Vec<Tile>;

pub struct Map {
    values: Vec<Line>,
    pub rival_positions: Positions,
}

impl Map {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 20;
    fn left(&self, i: usize, j: usize) -> Tile {
        if i > 0 { self.values[i-1][j] } else { Tile::Empty }
    }
    fn right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH { self.values[i+1][j] } else { Tile::Empty }
    }
    fn below(&self, i: usize, j: usize) -> Tile {
        if j > 0 { self.values[i][j-1] } else { Tile::Empty }
    }
    fn above(&self, i: usize, j: usize) -> Tile {
        if j+1 < Self::HEIGHT { self.values[i][j+1] } else { Tile::Empty }
    }
    fn below_left(&self, i: usize, j: usize) -> Tile {
        if i > 0 && j > 0 { self.values[i-1][j-1] } else { Tile::Empty }
    }
    fn below_right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH && j > 0 { self.values[i+1][j-1] } else { Tile::Empty }
    }
    fn above_left(&self, i: usize, j: usize) -> Tile {
        if i > 0 && j+1 < Self::HEIGHT { self.values[i-1][j+1] } else { Tile::Empty }
    }
    fn above_right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH && j+1 < Self::HEIGHT { self.values[i+1][j+1] } else { Tile::Empty }
    }
    pub fn get_tile_info(&self, i: usize, j: usize) -> Option<TileInfo> {
        use crate::sprite::SPRITES;

        let position = |layer| {
            let start_point = Vec3::new(-20.0 * Tile::SIZE,-((Self::HEIGHT/2) as f32) * Tile::SIZE, layer);
            start_point + Tile::SIZE * Vec3::new(i as f32, j as f32, 0.0)
        };

        let tile = self[i][j];
        match tile {
            Tile::Empty => None,
            Tile::Ground => {
                let left = tile.connects_to(self.left(i, j));
                let right = tile.connects_to(self.right(i,j));
                let below = tile.connects_to(self.below(i, j));
                let above = tile.connects_to(self.above(i, j));
                let below_left = tile.connects_to(self.below_left(i, j));
                let below_right = tile.connects_to(self.below_right(i, j));
                let above_left = tile.connects_to(self.above_left(i, j));
                let above_right = tile.connects_to(self.above_right(i, j));
                let image_key = match (above, left, right, below) {
                    (false, false, false, false) => SpriteTypeStates::AloneGrass,
                    (false, true, false, false) => SpriteTypeStates::LeftGrass,
                    (false, false, true, false) => SpriteTypeStates::RightGrass,
                    (false, true, true, false) => SpriteTypeStates::LeftRightGrass,
                    (false, false, false, true) => SpriteTypeStates::DownGrass,
                    (false, true, false, true) => SpriteTypeStates::DownGrassLeft,
                    (false, false, true, true) => SpriteTypeStates::DownGrassRight,
                    (false, true, true, true) => SpriteTypeStates::DownGrassLeftRight,
                    (true, false, false, false) => SpriteTypeStates::Above,
                    (true, true, false, false) => SpriteTypeStates::LeftAbove,
                    (true, false, true, false) => SpriteTypeStates::RightAbove,
                    (true, true, true, false) => SpriteTypeStates::BelowEmpty,
                    (true, false, false, true) => SpriteTypeStates::BelowAbove,
                    (true, true, false, true) => SpriteTypeStates::RightEmpty,
                    (true, false, true, true) => SpriteTypeStates::LeftEmpty,
                    (true, true, true, true) => match (below_left, below_right, above_left, above_right) {
                        //TODO: missing some cases due to not having the images
                        (false, _, _, _) => SpriteTypeStates::BelowLeftEmpty,
                        (_, false, _, _) => SpriteTypeStates::BelowRightEmpty,
                        (_, _, false, _) => SpriteTypeStates::AboveLeftEmpty,
                        (_, _, _, false) => SpriteTypeStates::AboveRightEmpty,
                        (_, _, _, _) => SpriteTypeStates::Full,
                    }
                };
                let hitbox = if image_key == SpriteTypeStates::Full {
                    None
                } else {
                    Some(Hitbox {
                        relative_position: Vec3::default(),
                        size: Vec2::new(Tile::SIZE, Tile::SIZE),
                    })
                };
                Some(TileInfo {
                    tile_type: tile,
                    position: position(0.5),
                    image: SpriteVariant::Sprite(SPRITES[&SpriteType::Ground][&image_key]),
                    hitbox,
                })
            },
            Tile::Win => Some(TileInfo {
                tile_type: tile,
                position: position(3.0),
                image: SpriteVariant::Sprite(SPRITES[&SpriteType::Heart][&SpriteTypeStates::Full]),
                hitbox: Some(Hitbox {
                    relative_position: Vec3::default(), //TODO: better values
                    size: Vec2::new(5.0, 5.0), //TODO: better values
                }),
            }),
            Tile::Player => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(2.0),
                    image: SpriteVariant::SpriteSheet(SpriteType::IdleGreen),
                    hitbox: Some(Hitbox {
                        relative_position: Vec3::default(), //TODO: better values
                        size: Vec2::new(Tile::SIZE, Tile::SIZE), //TODO: better values
                    }),
                })
            },
            Tile::Rival => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.5),
                    image: SpriteVariant::SpriteSheet(SpriteType::IdleBlue),
                    hitbox: Some(Hitbox {
                        relative_position: Vec3::default(), //TODO: better values
                        size: Vec2::new(Tile::SIZE, Tile::SIZE), //TODO: better values
                    }),
                })
            },
            Tile::Blue => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.0),
                    image: SpriteVariant::SpriteSheet(SpriteType::IdleBlue),
                    hitbox: Some(Hitbox {
                        relative_position: Vec3::default(), //TODO: better values
                        size: Vec2::new(Tile::SIZE, Tile::SIZE), //TODO: better values
                    }),
                })
            },
            Tile::Jeremy => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.0),
                    image: SpriteVariant::SpriteSheet(SpriteType::Jeremy),
                    hitbox: Some(Hitbox {
                        relative_position: Vec3::default(), //TODO: better values
                        size: Vec2::new(Tile::SIZE, Tile::SIZE), //TODO: better values
                    }),
                })
            },
            Tile::Blocky => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.0),
                    image: SpriteVariant::Sprite(SPRITES[&SpriteType::Blocky][&SpriteTypeStates::Surprised]),
                    hitbox: Some(Hitbox {
                        relative_position: Vec3::default(), //TODO: better values
                        size: Vec2::new(Tile::SIZE, Tile::SIZE), //TODO: better values
                    }),
                })
            },
            Tile::Npc(name) => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.0),
                    image: SpriteVariant::SpriteSheet(name),
                    hitbox: None,
                })
            },
        }
    }
    pub fn tile_info_iter(&self) -> impl Iterator<Item = Option<TileInfo>> + '_ {
        self.iter().map(|(i, j)| self.get_tile_info(i, j))
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..Map::WIDTH).flat_map(|i| (0..Map::HEIGHT).map(move |j| (i, j)))
    }
}

impl std::ops::Deref for Map {
    type Target = Vec<Line>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::Index<usize> for Map {
    type Output = Line;
    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

pub const LEVEL_COUNT: usize = 4;

fn convert_map_text_to_tiles(map_str_from_file: Vec<Vec<&str>>) -> Vec<Vec<Tile>> {
    map_str_from_file.into_iter().map(|x|{
        x.into_iter().map(|v| {
            Tile::from_str(v).unwrap()
        }).collect()
    }).collect()
}

fn read_map_from_file(map: &str) -> Vec<Vec<Tile>>{
    return convert_map_text_to_tiles(Vec::from_iter(map
        .lines()
        .map(|line| {
            line
                .split(' ')
                .collect::<Vec<&str>>()
        })));
}

fn read_positions_from_file(pos: &str) -> Vec<Vec3> {
    return Vec::from_iter(pos
        .lines()
        .map(|s| {
            let mut values = s.split(',').map(|v| {
                f32::from_str(v).unwrap()
            });
            Vec3::new(values.next().unwrap(), values.next().unwrap(), values.next().unwrap())
        }));
}



pub fn read_map(i: usize) -> Map {
    match i {
        0 => Map {
            values: read_map_from_file(LEVEL_00),
            // To get rival_positions, play in training mode, then copy the output into the file:
            // ../../../assets/positions/
            rival_positions: Positions {
                values: read_positions_from_file(POSITIONS_00).iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        1 => Map {
            values: read_map_from_file(LEVEL_01),
            /// To get rival_positions, play in training mode, then copy the output into the file:
            // ../../../assets/positions/
            rival_positions: Positions {
                values: read_positions_from_file(POSITIONS_01).iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        2 => Map {
            values: read_map_from_file(LEVEL_02),
            // To get rival_positions, play in training mode, then copy the output into the file:
            // ../../../assets/positions/
            rival_positions: Positions {
                values: read_positions_from_file(POSITIONS_02).iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        3 => Map {
            values: read_map_from_file(LEVEL_03),
            // To get rival_positions, play in training mode, then copy the output into the file:
            // ../../../assets/positions/
            rival_positions: Positions {
                values: read_positions_from_file(POSITIONS_03).iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        LEVEL_COUNT.. => panic!("Tried to access a level greater than level count"),
        _ => panic!(),
    }
}

#[derive(Clone, Default)]
pub enum RivalLevelPositions {
    #[default]
    HardCoded,
    Stolen(Positions),
}

#[derive(Clone, Default)]
pub struct RivalPositions(pub [RivalLevelPositions; LEVEL_COUNT]);
