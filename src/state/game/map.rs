use bevy::prelude::{Vec2, Vec3};

use super::hitbox::Hitbox;

use crate::sprite::{SpriteType, SpriteTypeStates, SpriteVariant};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Ground,
    Player,
    Blue,
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
            Tile::Blue => {
                Some(TileInfo {
                    tile_type: tile,
                    position: position(1.0),
                    image: SpriteVariant::SpriteSheet(SpriteType::Blue),
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
    LEVEL_0
}

const LEVEL_0: Map = Map { values: [
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
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Player, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Blue, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Npc(SpriteType::Pink), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Npc(SpriteType::Yellow), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Npc(SpriteType::Jeremy), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
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
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
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
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
]};