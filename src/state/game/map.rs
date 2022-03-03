use bevy::prelude::{Vec2, Vec3};

use super::hitbox::Hitbox;
use super::positions::Positions;

use crate::sprite::{SpriteType, SpriteTypeStates, SpriteVariant};
use crate::Timer;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Ground,
    Win,
    Player,
    Rival,
    Blue,
    Jeremy,
    Npc(SpriteType), //TODO: turn into many values to get rid of string
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

type Line = [Tile; Map::HEIGHT];

pub struct Map {
    values: [Line; Map::WIDTH],
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
                position: position(0.5),
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
                    position: position(1.0),
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
    type Target = [Line; 100];
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

pub fn read_map() -> Map {
    //TODO: read map from a file
    Map {
        values: [
            //turn your head to the right to read this
            [Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Rival, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Player, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Jeremy, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Jeremy, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Npc(SpriteType::Block), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Jeremy, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
            [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Jeremy, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Win, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        ],
        rival_positions: Positions {
            values: vec![
                Vec3::new(-144.0, -38.077168, 1.0),
                Vec3::new(-144.0, -46.82327, 1.0),
                Vec3::new(-144.0, -59.751152, 1.0),
                Vec3::new(-144.0, -74.26811, 1.0),
                Vec3::new(-144.0, -90.0, 1.0),
                Vec3::new(-144.0, -90.0, 1.0),
                Vec3::new(-139.343, -58.015656, 1.0),
                Vec3::new(-128.882, -35.980125, 1.0),
                Vec3::new(-112.338005, -18.829432, 1.0),
                Vec3::new(-89.90501, -6.755717, 1.0),
                Vec3::new(-64.88001, 0.3516388, 1.0),
                Vec3::new(-39.880013, 2.4495692, 1.0),
                Vec3::new(-18.980013, 0.3651594, 1.0),
                Vec3::new(10.1199875, -8.354365, 1.0),
                Vec3::new(35.044987, -21.20883, 1.0),
                Vec3::new(60.144985, -39.178925, 1.0),
                Vec3::new(85.069984, -62.008316, 1.0),
                Vec3::new(110.094986, -90.0, 1.0),
                Vec3::new(135.11998, -90.0, 1.0),
                Vec3::new(160.09499, -90.0, 1.0),
                Vec3::new(180.94498, -90.0, 1.0),
                Vec3::new(205.96997, -90.0, 1.0),
                Vec3::new(230.94496, -70.81357, 1.0),
                Vec3::new(260.06995, -42.63766, 1.0),
                Vec3::new(285.06992, -23.864616, 1.0),
                Vec3::new(310.14493, -10.059103, 1.0),
                Vec3::new(335.04492, -1.3242289, 1.0),
                Vec3::new(355.89493, 2.172445, 1.0),
                Vec3::new(380.8949, 1.7798836, 1.0),
                Vec3::new(405.4449, -3.5955331, 1.0),
                Vec3::new(421.32983, -14.004316, 1.0),
                Vec3::new(429.7753, -29.469477, 1.0),
                Vec3::new(434.23346, -36.0, 1.0),
                Vec3::new(436.8794, -36.0, 1.0),
                Vec3::new(441.97083, -36.0, 1.0),
                Vec3::new(453.10394, -21.570906, 1.0),
                Vec3::new(470.15335, 3.7694502, 1.0),
                Vec3::new(488.9925, 21.14817, 1.0),
                Vec3::new(514.4923, 39.665504, 1.0),
                Vec3::new(524.5698, 48.687542, 1.0),
                Vec3::new(532.15283, 55.468765, 1.0),
                Vec3::new(535.00134, 56.156643, 1.0),
                Vec3::new(537.14465, 51.285, 1.0),
                Vec3::new(537.6797, 41.680565, 1.0),
                Vec3::new(538.6717, 27.083603, 1.0),
                Vec3::new(544.87225, 7.464031, 1.0),
                Vec3::new(549.30994, -12.788387, 1.0),
                Vec3::new(552.6183, -46.224026, 1.0),
                Vec3::new(556.934, -38.32505, 1.0),
                Vec3::new(565.1689, -18.05822, 1.0),
                Vec3::new(580.397, 1.4627812, 1.0),
                Vec3::new(598.78687, 16.11577, 1.0),
                Vec3::new(609.4985, 25.782082, 1.0),
                Vec3::new(615.8163, 30.713228, 1.0),
                Vec3::new(618.52985, 29.52129, 1.0),
                Vec3::new(621.7358, 24.711466, 1.0),
                Vec3::new(631.0355, 14.407701, 1.0),
                Vec3::new(646.3954, -0.96572757, 1.0),
                Vec3::new(667.76526, -21.359419, 1.0),
                Vec3::new(692.8392, -1.71417, 1.0),
                Vec3::new(721.88916, 25.377523, 1.0),
                Vec3::new(746.8642, 43.27065, 1.0),
                Vec3::new(756.168, 54.385864, 1.0),
                Vec3::new(761.97314, 64.086784, 1.0),
                Vec3::new(771.13513, 66.87051, 1.0),
                Vec3::new(791.0131, 64.91288, 1.0),
                Vec3::new(814.35504, 57.802727, 1.0),
                Vec3::new(836.325, 54.0, 1.0),
                Vec3::new(865.575, 54.0, 1.0),
                Vec3::new(885.13873, 53.863876, 1.0),
                Vec3::new(900.0, 48.99455, 1.0),
                Vec3::new(899.838, 39.457592, 1.0),
                Vec3::new(896.51697, 27.637619, 1.0),
                Vec3::new(884.88794, 5.3232527, 1.0),
                Vec3::new(880.33704, 0.000000059604645, 1.0),
                Vec3::new(874.51807, 0.00000044703484, 1.0),
                Vec3::new(859.35706, -0.00000020861626, 1.0),
                Vec3::new(843.6451, -0.000000014901161, 1.0),
                Vec3::new(819.65, -0.00000008940697, 1.0),
                Vec3::new(794.55005, -1.3987854, 1.0),
                Vec3::new(792.0, -9.163851, 1.0),
                Vec3::new(793.66296, -21.275791, 1.0),
                Vec3::new(801.15607, -38.367607, 1.0),
                Vec3::new(814.62805, -54.0, 1.0),
                Vec3::new(834.0941, -54.0, 1.0),
                Vec3::new(858.6382, -54.0, 1.0),
                Vec3::new(883.6882, -54.142803, 1.0),
                Vec3::new(900.0, -56.905354, 1.0),
                Vec3::new(900.0, -64.89949, 1.0),
                Vec3::new(899.006, -80.36253, 1.0),
                Vec3::new(894.0109, -95.66847, 1.0),
                Vec3::new(880.0209, -108.0, 1.0),
                Vec3::new(861.4998, -108.0, 1.0),
                Vec3::new(845.50604, -108.0, 1.0),
                Vec3::new(840.0141, -108.0, 1.0),
                Vec3::new(828.5491, -108.0, 1.0),
                Vec3::new(815.64044, -108.0, 1.0),
                Vec3::new(804.82355, -110.01769, 1.0),
                Vec3::new(800.8634, -115.514885, 1.0),
                Vec3::new(801.643, -129.03699, 1.0),
                Vec3::new(808.8252, -146.08884, 1.0),
                Vec3::new(821.9853, -162.0, 1.0),
                Vec3::new(841.15314, -162.0, 1.0),
                Vec3::new(861.26324, -162.0, 1.0),
                Vec3::new(886.46326, -162.0, 1.0),
                Vec3::new(915.5132, -162.0, 1.0),
                Vec3::new(940.5132, -162.0, 1.0),
                Vec3::new(961.3132, -162.0, 1.0),
                Vec3::new(990.4883, -152.1096, 1.0),
                Vec3::new(1011.41327, -129.77518, 1.0),
                Vec3::new(1036.3633, -107.720535, 1.0),
                Vec3::new(1065.5383, -88.24857, 1.0),
                Vec3::new(1090.5635, -76.972176, 1.0),
                Vec3::new(1115.5386, -72.13944, 1.0),
                Vec3::new(1140.5635, -75.89794, 1.0),
                Vec3::new(1165.5386, -84.64315, 1.0),
                Vec3::new(1190.5636, -98.41184, 1.0),
                Vec3::new(1215.5387, -117.14717, 1.0),
                Vec3::new(1240.4135, -140.76582, 1.0),
                Vec3::new(1265.5636, -162.0, 1.0),
                Vec3::new(1290.5251, -162.0, 1.0),
                Vec3::new(1315.5753, -134.04384, 1.0),
                Vec3::new(1340.5754, -111.1458, 1.0),
                Vec3::new(1365.5255, -93.27749, 1.0),
                Vec3::new(1386.3005, -82.19874, 1.0),
            ].iter().copied().collect(), //TODO: is there a better way to do this?
            timer: Timer::from_seconds(0.1, true),
        }
    }
}

#[derive(Clone)]
pub struct RivalPositions(pub Positions);

impl Default for RivalPositions {
    fn default() -> Self {
        Self(read_map().rival_positions)
    }
}
