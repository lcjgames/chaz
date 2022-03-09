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
            "Tile::Empty" => Ok(Tile::Empty),
            "Tile::Ground" => Ok(Tile::Ground),
            "Tile::Win" => Ok(Tile::Win),
            "Tile::Player" => Ok(Tile::Player),
            "Tile::Rival" => Ok(Tile::Rival),
            "Tile::Blue" => Ok(Tile::Blue),
            "Tile::Jeremy" => Ok(Tile::Jeremy),
            "Tile::Blocky" => Ok(Tile::Blocky),
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


pub fn read_map(i: usize) -> Map {

    match i {
        0 => Map {
            values: read_map_from_file(LEVEL_00),
            // To get rival_positions, play in training mode, then copy the output into the source code
            rival_positions: Positions {
                values: vec![
                    Vec3::new(-144.0, -38.077168, 1.5),
                    Vec3::new(-144.0, -46.82327, 1.5),
                    Vec3::new(-144.0, -59.751152, 1.5),
                    Vec3::new(-144.0, -74.26811, 1.5),
                    Vec3::new(-144.0, -90.0, 1.5),
                    Vec3::new(-144.0, -90.0, 1.5),
                    Vec3::new(-139.343, -58.015656, 1.5),
                    Vec3::new(-128.882, -35.980125, 1.5),
                    Vec3::new(-112.338005, -18.829432, 1.5),
                    Vec3::new(-89.90501, -6.755717, 1.5),
                    Vec3::new(-64.88001, 0.3516388, 1.5),
                    Vec3::new(-39.880013, 2.4495692, 1.5),
                    Vec3::new(-18.980013, 0.3651594, 1.5),
                    Vec3::new(10.1199875, -8.354365, 1.5),
                    Vec3::new(35.044987, -21.20883, 1.5),
                    Vec3::new(60.144985, -39.178925, 1.5),
                    Vec3::new(85.069984, -62.008316, 1.5),
                    Vec3::new(110.094986, -90.0, 1.5),
                    Vec3::new(135.11998, -90.0, 1.5),
                    Vec3::new(160.09499, -90.0, 1.5),
                    Vec3::new(180.94498, -90.0, 1.5),
                    Vec3::new(205.96997, -90.0, 1.5),
                    Vec3::new(230.94496, -70.81357, 1.5),
                    Vec3::new(260.06995, -42.63766, 1.5),
                    Vec3::new(285.06992, -23.864616, 1.5),
                    Vec3::new(310.14493, -10.059103, 1.5),
                    Vec3::new(335.04492, -1.3242289, 1.5),
                    Vec3::new(355.89493, 2.172445, 1.5),
                    Vec3::new(380.8949, 1.7798836, 1.5),
                    Vec3::new(405.4449, -3.5955331, 1.5),
                    Vec3::new(421.32983, -14.004316, 1.5),
                    Vec3::new(429.7753, -29.469477, 1.5),
                    Vec3::new(434.23346, -36.0, 1.5),
                    Vec3::new(436.8794, -36.0, 1.5),
                    Vec3::new(441.97083, -36.0, 1.5),
                    Vec3::new(453.10394, -21.570906, 1.5),
                    Vec3::new(470.15335, 3.7694502, 1.5),
                    Vec3::new(488.9925, 21.14817, 1.5),
                    Vec3::new(514.4923, 39.665504, 1.5),
                    Vec3::new(524.5698, 48.687542, 1.5),
                    Vec3::new(532.15283, 55.468765, 1.5),
                    Vec3::new(535.00134, 56.156643, 1.5),
                    Vec3::new(537.14465, 51.285, 1.5),
                    Vec3::new(537.6797, 41.680565, 1.5),
                    Vec3::new(538.6717, 27.083603, 1.5),
                    Vec3::new(544.87225, 7.464031, 1.5),
                    Vec3::new(549.30994, -12.788387, 1.5),
                    Vec3::new(552.6183, -46.224026, 1.5),
                    Vec3::new(556.934, -38.32505, 1.5),
                    Vec3::new(565.1689, -18.05822, 1.5),
                    Vec3::new(580.397, 1.4627812, 1.5),
                    Vec3::new(598.78687, 16.11577, 1.5),
                    Vec3::new(609.4985, 25.782082, 1.5),
                    Vec3::new(615.8163, 30.713228, 1.5),
                    Vec3::new(618.52985, 29.52129, 1.5),
                    Vec3::new(621.7358, 24.711466, 1.5),
                    Vec3::new(631.0355, 14.407701, 1.5),
                    Vec3::new(646.3954, -0.96572757, 1.5),
                    Vec3::new(667.76526, -21.359419, 1.5),
                    Vec3::new(692.8392, -1.71417, 1.5),
                    Vec3::new(721.88916, 25.377523, 1.5),
                    Vec3::new(746.8642, 43.27065, 1.5),
                    Vec3::new(756.168, 54.385864, 1.5),
                    Vec3::new(761.97314, 64.086784, 1.5),
                    Vec3::new(771.13513, 66.87051, 1.5),
                    Vec3::new(791.0131, 64.91288, 1.5),
                    Vec3::new(814.35504, 57.802727, 1.5),
                    Vec3::new(836.325, 54.0, 1.5),
                    Vec3::new(865.575, 54.0, 1.5),
                    Vec3::new(885.13873, 53.863876, 1.5),
                    Vec3::new(900.0, 48.99455, 1.5),
                    Vec3::new(899.838, 39.457592, 1.5),
                    Vec3::new(896.51697, 27.637619, 1.5),
                    Vec3::new(884.88794, 5.3232527, 1.5),
                    Vec3::new(880.33704, 0.000000059604645, 1.5),
                    Vec3::new(874.51807, 0.00000044703484, 1.5),
                    Vec3::new(859.35706, -0.00000020861626, 1.5),
                    Vec3::new(843.6451, -0.000000014901161, 1.5),
                    Vec3::new(819.65, -0.00000008940697, 1.5),
                    Vec3::new(794.55005, -1.3987854, 1.5),
                    Vec3::new(792.0, -9.163851, 1.5),
                    Vec3::new(793.66296, -21.275791, 1.5),
                    Vec3::new(801.15607, -38.367607, 1.5),
                    Vec3::new(814.62805, -54.0, 1.5),
                    Vec3::new(834.0941, -54.0, 1.5),
                    Vec3::new(858.6382, -54.0, 1.5),
                    Vec3::new(883.6882, -54.142803, 1.5),
                    Vec3::new(900.0, -56.905354, 1.5),
                    Vec3::new(900.0, -64.89949, 1.5),
                    Vec3::new(899.006, -80.36253, 1.5),
                    Vec3::new(894.0109, -95.66847, 1.5),
                    Vec3::new(880.0209, -108.0, 1.5),
                    Vec3::new(861.4998, -108.0, 1.5),
                    Vec3::new(845.50604, -108.0, 1.5),
                    Vec3::new(840.0141, -108.0, 1.5),
                    Vec3::new(828.5491, -108.0, 1.5),
                    Vec3::new(815.64044, -108.0, 1.5),
                    Vec3::new(804.82355, -110.01769, 1.5),
                    Vec3::new(800.8634, -115.514885, 1.5),
                    Vec3::new(801.643, -129.03699, 1.5),
                    Vec3::new(808.8252, -146.08884, 1.5),
                    Vec3::new(821.9853, -162.0, 1.5),
                    Vec3::new(841.15314, -162.0, 1.5),
                    Vec3::new(861.26324, -162.0, 1.5),
                    Vec3::new(886.46326, -162.0, 1.5),
                    Vec3::new(915.5132, -162.0, 1.5),
                    Vec3::new(940.5132, -162.0, 1.5),
                    Vec3::new(961.3132, -162.0, 1.5),
                    Vec3::new(990.4883, -152.1096, 1.5),
                    Vec3::new(1011.41327, -129.77518, 1.5),
                    Vec3::new(1036.3633, -107.720535, 1.5),
                    Vec3::new(1065.5383, -88.24857, 1.5),
                    Vec3::new(1090.5635, -76.972176, 1.5),
                    Vec3::new(1115.5386, -72.13944, 1.5),
                    Vec3::new(1140.5635, -75.89794, 1.5),
                    Vec3::new(1165.5386, -84.64315, 1.5),
                    Vec3::new(1190.5636, -98.41184, 1.5),
                    Vec3::new(1215.5387, -117.14717, 1.5),
                    Vec3::new(1240.4135, -140.76582, 1.5),
                    Vec3::new(1265.5636, -162.0, 1.5),
                    Vec3::new(1290.5251, -162.0, 1.5),
                    Vec3::new(1315.5753, -134.04384, 1.5),
                    Vec3::new(1340.5754, -111.1458, 1.5),
                    Vec3::new(1365.5255, -93.27749, 1.5),
                    Vec3::new(1386.3005, -82.19874, 1.5),
                ].iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        1 => Map {
            values: read_map_from_file(LEVEL_01),
            // To get rival_positions, play in training mode, then copy the output into the source code
            rival_positions: Positions {
                values: vec![
                    Vec3::new(-306.0, -92.98272, 1.5),
                    Vec3::new(-306.0, -100.931335, 1.5),
                    Vec3::new(-306.0, -111.49583, 1.5),
                    Vec3::new(-306.0, -128.37755, 1.5),
                    Vec3::new(-306.0, -144.0, 1.5),
                    Vec3::new(-305.01498, -144.0, 1.5),
                    Vec3::new(-298.55597, -144.0, 1.5),
                    Vec3::new(-286.041, -144.0, 1.5),
                    Vec3::new(-267.593, -144.0, 1.5),
                    Vec3::new(-247.74796, -144.0, 1.5),
                    Vec3::new(-222.54797, -144.0, 1.5),
                    Vec3::new(-193.623, -144.0, 1.5),
                    Vec3::new(-168.623, -134.13324, 1.5),
                    Vec3::new(-143.573, -107.78819, 1.5),
                    Vec3::new(-118.573006, -86.499954, 1.5),
                    Vec3::new(-93.67302, -72.0, 1.5),
                    Vec3::new(-72.848015, -74.08173, 1.5),
                    Vec3::new(-43.64801, -82.84996, 1.5),
                    Vec3::new(-18.623013, -95.78784, 1.5),
                    Vec3::new(6.4269876, -113.76372, 1.5),
                    Vec3::new(31.401987, -136.67456, 1.5),
                    Vec3::new(56.426987, -144.0, 1.5),
                    Vec3::new(81.47698, -144.0, 1.5),
                    Vec3::new(102.251976, -144.0, 1.5),
                    Vec3::new(131.40198, -144.0, 1.5),
                    Vec3::new(156.45, -144.0, 1.5),
                    Vec3::new(181.5, -144.0, 1.5),
                    Vec3::new(206.49998, -144.0, 1.5),
                    Vec3::new(231.57498, -144.0, 1.5),
                    Vec3::new(252.34998, -144.0, 1.5),
                    Vec3::new(281.49997, -144.0, 1.5),
                    Vec3::new(306.525, -144.0, 1.5),
                    Vec3::new(331.525, -144.0, 1.5),
                    Vec3::new(356.475, -144.0, 1.5),
                    Vec3::new(381.475, -144.0, 1.5),
                    Vec3::new(402.37497, -144.0, 1.5),
                    Vec3::new(431.49994, -144.0, 1.5),
                    Vec3::new(456.4999, -134.26778, 1.5),
                    Vec3::new(481.57492, -107.95804, 1.5),
                    Vec3::new(506.52493, -86.75826, 1.5),
                    Vec3::new(531.5999, -72.0, 1.5),
                    Vec3::new(556.4999, -74.8934, 1.5),
                    Vec3::new(577.37494, -81.143394, 1.5),
                    Vec3::new(606.50006, -95.70395, 1.5),
                    Vec3::new(631.525, -113.62146, 1.5),
                    Vec3::new(656.4751, -136.4803, 1.5),
                    Vec3::new(666.0, -144.0, 1.5),
                    Vec3::new(666.0, -144.0, 1.5),
                    Vec3::new(666.0, -138.96, 1.5),
                    Vec3::new(666.0, -111.93306, 1.5),
                    Vec3::new(662.49304, -89.83975, 1.5),
                    Vec3::new(653.0751, -72.786156, 1.5),
                    Vec3::new(637.6271, -51.64958, 1.5),
                    Vec3::new(616.093, -26.161238, 1.5),
                    Vec3::new(610.32404, -5.6946087, 1.5),
                    Vec3::new(604.45905, 7.5408354, 1.5),
                    Vec3::new(589.42206, 17.85888, 1.5),
                    Vec3::new(569.97504, 14.115435, 1.5),
                    Vec3::new(545.4051, 5.3184195, 1.5),
                    Vec3::new(520.4551, -8.414939, 1.5),
                    Vec3::new(503.837, -18.0, 1.5),
                    Vec3::new(499.31302, -18.0, 1.5),
                    Vec3::new(490.99704, -18.0, 1.5),
                    Vec3::new(486.0, -18.0, 1.5),
                    Vec3::new(486.0, -18.0, 1.5),
                    Vec3::new(486.0, -18.0, 1.5),
                    Vec3::new(485.505, 9.92804, 1.5),
                    Vec3::new(480.042, 16.624332, 1.5),
                    Vec3::new(468.57898, 10.419893, 1.5),
                    Vec3::new(451.03098, 0.0, 1.5),
                    Vec3::new(427.79196, -0.41365957, 1.5),
                    Vec3::new(402.66696, -5.0401344, 1.5),
                    Vec3::new(377.74197, -14.6043005, 1.5),
                    Vec3::new(352.61697, -18.0, 1.5),
                    Vec3::new(331.817, -18.0, 1.5),
                    Vec3::new(306.94202, -18.0, 1.5),
                    Vec3::new(277.71698, -18.0, 1.5),
                    Vec3::new(252.76697, -18.0, 1.5),
                    Vec3::new(227.74197, -18.0, 1.5),
                    Vec3::new(202.79198, -18.0, 1.5),
                    Vec3::new(177.71698, -18.0, 1.5),
                    Vec3::new(156.69199, -18.0, 1.5),
                    Vec3::new(131.717, -18.0, 1.5),
                    Vec3::new(102.691986, -18.0, 1.5),
                    Vec3::new(77.64199, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -18.0, 1.5),
                    Vec3::new(54.0, -3.1761155, 1.5),
                    Vec3::new(50.509995, 17.86552, 1.5),
                    Vec3::new(41.052998, 14.110871, 1.5),
                    Vec3::new(25.552996, 5.369171, 1.5),
                    Vec3::new(3.9619951, 0.00000044703484, 1.5),
                    Vec3::new(-20.938005, -2.8937345, 1.5),
                    Vec3::new(-45.888004, -10.792334, 1.5),
                    Vec3::new(-71.01301, -18.0, 1.5),
                    Vec3::new(-95.963005, -18.0, 1.5),
                    Vec3::new(-121.013, -18.0, 1.5),
                    Vec3::new(-141.73799, -18.0, 1.5),
                    Vec3::new(-170.91298, -18.0, 1.5),
                    Vec3::new(-195.88799, -18.0, 1.5),
                    Vec3::new(-220.93799, -18.0, 1.5),
                    Vec3::new(-245.86298, -18.0, 1.5),
                    Vec3::new(-270.98798, -18.0, 1.5),
                    Vec3::new(-295.91293, -18.0, 1.5),
                    Vec3::new(-320.86295, -18.0, 1.5),
                    Vec3::new(-324.0, -18.0, 1.5),
                    Vec3::new(-324.0, -18.0, 1.5),
                    Vec3::new(-324.0, -18.0, 1.5),
                    Vec3::new(-324.0, -18.0, 1.5),
                    Vec3::new(-323.023, -18.0, 1.5),
                    Vec3::new(-318.05804, 5.6028647, 1.5),
                    Vec3::new(-304.13406, 32.92606, 1.5),
                    Vec3::new(-285.61407, 53.349785, 1.5),
                    Vec3::new(-270.0, 80.45109, 1.5),
                    Vec3::new(-269.00302, 102.49095, 1.5),
                    Vec3::new(-262.555, 106.60102, 1.5),
                    Vec3::new(-252.50998, 101.707825, 1.5),
                    Vec3::new(-235.08397, 91.2813, 1.5),
                    Vec3::new(-207.64595, 89.17661, 1.5),
                    Vec3::new(-182.64597, 83.774445, 1.5),
                    Vec3::new(-157.59596, 73.346016, 1.5),
                    Vec3::new(-136.77097, 72.0, 1.5),
                    Vec3::new(-107.57096, 72.0, 1.5),
                    Vec3::new(-86.79596, 72.0, 1.5),
                    Vec3::new(-57.69596, 72.0, 1.5),
                    Vec3::new(-32.67096, 72.0, 1.5),
                    Vec3::new(-7.5959597, 72.0, 1.5),
                    Vec3::new(17.37904, 72.0, 1.5),
                    Vec3::new(42.37904, 72.0, 1.5),
                    Vec3::new(67.35404, 72.0, 1.5),
                    Vec3::new(88.25404, 72.0, 1.5),
                    Vec3::new(117.42904, 72.0, 1.5),
                    Vec3::new(142.32904, 72.0, 1.5),
                    Vec3::new(167.35405, 72.0, 1.5),
                    Vec3::new(192.30406, 72.0, 1.5),
                    Vec3::new(217.42908, 72.0, 1.5),
                    Vec3::new(238.2, 72.0, 1.5),
                    Vec3::new(267.3, 72.0, 1.5),
                    Vec3::new(270.0, 72.0, 1.5),
                    Vec3::new(270.0, 72.0, 1.5),
                    Vec3::new(270.0, 72.0, 1.5),
                    Vec3::new(270.0, 72.0, 1.5),
                    Vec3::new(270.0, 72.0, 1.5),
                    Vec3::new(270.0, 91.43744, 1.5),
                    Vec3::new(273.509, 107.57915, 1.5),
                    Vec3::new(282.987, 102.959656, 1.5),
                    Vec3::new(298.43298, 93.40797, 1.5),
                    Vec3::new(319.90195, 90.0, 1.5),
                    Vec3::new(344.82693, 89.86716, 1.5),
                    Vec3::new(365.87692, 87.05393, 1.5),
                    Vec3::new(394.97696, 77.30948, 1.5),
                    Vec3::new(419.852, 72.0, 1.5),
                    Vec3::new(444.87695, 72.0, 1.5),
                    Vec3::new(469.85196, 72.0, 1.5),
                    Vec3::new(494.85196, 72.0, 1.5),
                    Vec3::new(519.87695, 72.0, 1.5),
                    Vec3::new(544.43695, 72.0, 1.5),
                    Vec3::new(560.6533, 72.0, 1.5),
                    Vec3::new(578.4041, 72.0, 1.5),
                    Vec3::new(601.8789, 72.0, 1.5),
                    Vec3::new(619.54315, 72.0, 1.5),
                    Vec3::new(627.69244, 72.0, 1.5),
                    Vec3::new(633.2793, 72.0, 1.5),
                    Vec3::new(636.5437, 72.0, 1.5),
                    Vec3::new(640.8403, 72.0, 1.5),
                    Vec3::new(646.95386, 72.0, 1.5),
                    Vec3::new(650.1889, 72.0, 1.5),
                    Vec3::new(649.1524, 72.0, 1.5),
                    Vec3::new(643.7153, 72.0, 1.5),
                    Vec3::new(640.3396, 72.0, 1.5),
                    Vec3::new(638.54913, 72.0, 1.5),
                    Vec3::new(638.0084, 75.15, 1.5),
                    Vec3::new(638.0084, 102.18495, 1.5),
                    Vec3::new(638.0084, 124.27816, 1.5),
                    Vec3::new(638.0084, 141.34906, 1.5),
                    Vec3::new(636.34045, 160.54735, 1.5),
                    Vec3::new(627.0765, 189.70628, 1.5),
                    Vec3::new(612.5814, 209.28955, 1.5),
                    Vec3::new(594.982, 223.86421, 1.5),
                    Vec3::new(584.73895, 233.42691, 1.5),
                    Vec3::new(579.23425, 238.03273, 1.5),
                    Vec3::new(576.35925, 237.61107, 1.5),
                    Vec3::new(572.9272, 232.1917, 1.5),
                    Vec3::new(564.0024, 221.78001, 1.5),
                    Vec3::new(557.15155, 206.34071, 1.5),
                    Vec3::new(553.52185, 185.94539, 1.5),
                    Vec3::new(550.3972, 162.0, 1.5),
                    Vec3::new(541.93097, 162.0, 1.5),
                    Vec3::new(539.015, 162.0, 1.5),
                    Vec3::new(530.9341, 181.00664, 1.5),
                    Vec3::new(517.3531, 205.73488, 1.5),
                    Vec3::new(497.9531, 225.23521, 1.5),
                    Vec3::new(473.45212, 239.82304, 1.5),
                    Vec3::new(448.47714, 249.40062, 1.5),
                    Vec3::new(423.45215, 253.9955, 1.5),
                    Vec3::new(402.67712, 254.00543, 1.5),
                    Vec3::new(373.45212, 248.17027, 1.5),
                    Vec3::new(348.55212, 237.79756, 1.5),
                    Vec3::new(323.52713, 222.38751, 1.5),
                    Vec3::new(300.8191, 201.94415, 1.5),
                    Vec3::new(287.9006, 176.36237, 1.5),
                    Vec3::new(279.4946, 162.0, 1.5),
                    Vec3::new(265.89664, 162.0, 1.5),
                    Vec3::new(246.93277, 162.0, 1.5),
                    Vec3::new(233.5178, 162.0, 1.5),
                    Vec3::new(221.96486, 162.0, 1.5),
                    Vec3::new(208.10416, 162.0, 1.5),
                    Vec3::new(195.39665, 162.0, 1.5),
                    Vec3::new(190.62122, 162.0, 1.5),
                    Vec3::new(187.01727, 162.0, 1.5),
                    Vec3::new(185.4616, 162.0, 1.5),
                    Vec3::new(185.12846, 162.0, 1.5),
                    Vec3::new(185.12846, 162.0, 1.5),
                    Vec3::new(184.96246, 162.0, 1.5),
                    Vec3::new(180.44145, 162.0, 1.5),
                    Vec3::new(174.18317, 162.0, 1.5),
                    Vec3::new(170.86214, 162.0, 1.5),
                    Vec3::new(169.09187, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(168.55083, 162.0, 1.5),
                    Vec3::new(167.53682, 176.72156, 1.5),
                    Vec3::new(162.57481, 198.19135, 1.5),
                    Vec3::new(148.58582, 222.46503, 1.5),
                    Vec3::new(130.1528, 237.81827, 1.5),
                    Vec3::new(106.1568, 255.13794, 1.5),
                    Vec3::new(81.131805, 281.4163, 1.5),
                    Vec3::new(56.106804, 302.68787, 1.5),
                    Vec3::new(31.131805, 318.91742, 1.5),
                    Vec3::new(6.106805, 330.17896, 1.5),
                    Vec3::new(-18.968195, 336.43726, 1.5),
                    Vec3::new(-43.868195, 337.67722, 1.5),
                    Vec3::new(-68.84319, 333.9266, 1.5),
                    Vec3::new(-93.94319, 325.13794, 1.5),
                    Vec3::new(-118.455696, 311.42624, 1.5),
                    Vec3::new(-134.34962, 292.69284, 1.5),
                    Vec3::new(-143.06004, 268.8978, 1.5),
                    Vec3::new(-154.6209, 240.22246, 1.5),
                    Vec3::new(-172.17656, 210.25247, 1.5),
                    Vec3::new(-195.38182, 180.34248, 1.5),
                ].iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        2 => Map {
            values: read_map_from_file(LEVEL_02),
            // To get rival_positions, play in training mode, then copy the output into the source code
            rival_positions: Positions {
                values: vec![
                    Vec3::new(-306.0, -91.71106, 1.5),
                    Vec3::new(-306.0, -144.13123, 1.5),
                    Vec3::new(-306.0, -144.13448, 1.5),
                    Vec3::new(-305.00702, -129.58846, 1.5),
                    Vec3::new(-298.55103, -104.18108, 1.5),
                    Vec3::new(-286.03806, -83.7083, 1.5),
                    Vec3::new(-267.61407, -68.31083, 1.5),
                    Vec3::new(-243.56708, -57.857822, 1.5),
                    Vec3::new(-222.61707, -52.971973, 1.5),
                    Vec3::new(-197.81706, -51.720715, 1.5),
                    Vec3::new(-172.59204, -55.50261, 1.5),
                    Vec3::new(-143.69205, -66.087616, 1.5),
                    Vec3::new(-118.66705, -80.65209, 1.5),
                    Vec3::new(-93.64206, -90.14112, 1.5),
                    Vec3::new(-68.54206, -92.940796, 1.5),
                    Vec3::new(-43.542065, -100.87804, 1.5),
                    Vec3::new(-18.642065, -113.751854, 1.5),
                    Vec3::new(-0.00000047683716, -131.70924, 1.5),
                    Vec3::new(0.0000002682209, -144.11552, 1.5),
                    Vec3::new(0.00000023841858, -124.656624, 1.5),
                    Vec3::new(-0.00000011920929, -100.08249, 1.5),
                    Vec3::new(1.6649998, -80.42105, 1.5),
                    Vec3::new(9.141, -65.79374, 1.5),
                    Vec3::new(19.911, -57.46818, 1.5),
                    Vec3::new(42.045, -51.597233, 1.5),
                    Vec3::new(66.59599, -51.985603, 1.5),
                    Vec3::new(91.07099, -57.350883, 1.5),
                    Vec3::new(106.93124, -67.73601, 1.5),
                    Vec3::new(115.369865, -83.170784, 1.5),
                    Vec3::new(117.92466, -103.58098, 1.5),
                    Vec3::new(114.72788, -128.97287, 1.5),
                    Vec3::new(112.0124, -144.13284, 1.5),
                    Vec3::new(112.93547, -133.33405, 1.5),
                    Vec3::new(121.38318, -104.084015, 1.5),
                    Vec3::new(133.04018, -83.62599, 1.5),
                    Vec3::new(139.9248, -68.232414, 1.5),
                    Vec3::new(145.86246, -57.814137, 1.5),
                    Vec3::new(157.51671, -52.401134, 1.5),
                    Vec3::new(167.94453, -51.713272, 1.5),
                    Vec3::new(175.78133, -56.58327, 1.5),
                    Vec3::new(179.132, -66.15253, 1.5),
                    Vec3::new(182.64648, -78.05452, 1.5),
                    Vec3::new(194.59459, -60.890903, 1.5),
                    Vec3::new(208.11296, -40.760033, 1.5),
                    Vec3::new(233.91325, -18.4819, 1.5),
                    Vec3::new(258.91327, -4.7422647, 1.5),
                    Vec3::new(283.9633, 4.0251174, 1.5),
                    Vec3::new(309.03827, 7.775669, 1.5),
                    Vec3::new(329.98828, 7.031781, 1.5),
                    Vec3::new(354.88824, 1.6080183, 1.5),
                    Vec3::new(375.57355, -10.986875, 1.5),
                    Vec3::new(384.67505, -24.260674, 1.5),
                    Vec3::new(392.49216, -36.057243, 1.5),
                    Vec3::new(404.24316, -36.13778, 1.5),
                    Vec3::new(418.80478, -36.06962, 1.5),
                    Vec3::new(445.23114, -36.132847, 1.5),
                    Vec3::new(466.40613, -36.162, 1.5),
                    Vec3::new(495.3061, -12.620117, 1.5),
                    Vec3::new(520.2811, 11.067036, 1.5),
                    Vec3::new(537.6522, 27.234835, 1.5),
                    Vec3::new(549.10004, 41.61547, 1.5),
                    Vec3::new(555.968, 52.25612, 1.5),
                    Vec3::new(556.94867, 55.733776, 1.5),
                    Vec3::new(553.0119, 55.28336, 1.5),
                    Vec3::new(548.8852, 48.47561, 1.5),
                    Vec3::new(549.7668, 37.18687, 1.5),
                    Vec3::new(556.7519, 20.945053, 1.5),
                    Vec3::new(569.67444, 17.86878, 1.5),
                    Vec3::new(585.1946, 17.853794, 1.5),
                    Vec3::new(608.789, 17.86222, 1.5),
                    Vec3::new(637.93896, 14.112961, 1.5),
                    Vec3::new(662.93896, 5.366207, 1.5),
                    Vec3::new(683.714, -5.7053127, 1.5),
                    Vec3::new(712.91394, 12.401342, 1.5),
                    Vec3::new(737.8639, 36.09005, 1.5),
                    Vec3::new(762.96387, 54.906616, 1.5),
                    Vec3::new(783.7389, 66.66936, 1.5),
                    Vec3::new(812.8889, 96.35092, 1.5),
                    Vec3::new(837.694, 119.36172, 1.5),
                    Vec3::new(853.6475, 137.19485, 1.5),
                    Vec3::new(862.9703, 150.12067, 1.5),
                    Vec3::new(866.23596, 158.0137, 1.5),
                    Vec3::new(864.36194, 160.79155, 1.5),
                    Vec3::new(855.7599, 158.84848, 1.5),
                    Vec3::new(850.46277, 151.72122, 1.5),
                    Vec3::new(846.3018, 139.66174, 1.5),
                    Vec3::new(843.70245, 125.75727, 1.5),
                    Vec3::new(837.6556, 100.448166, 1.5),
                    Vec3::new(825.9031, 73.35637, 1.5),
                    Vec3::new(808.22687, 91.28053, 1.5),
                    Vec3::new(785.16705, 114.198235, 1.5),
                    Vec3::new(769.29803, 132.16118, 1.5),
                    Vec3::new(761.99164, 143.21422, 1.5),
                    Vec3::new(756.1869, 152.96858, 1.5),
                    Vec3::new(749.7772, 155.72691, 1.5),
                    Vec3::new(736.48157, 154.46756, 1.5),
                    Vec3::new(713.53955, 146.70924, 1.5),
                    Vec3::new(688.7907, 134.63249, 1.5),
                    Vec3::new(663.74066, 117.51547, 1.5),
                    Vec3::new(665.837, 107.86716, 1.5),
                    Vec3::new(665.833, 127.05939, 1.5),
                    Vec3::new(661.31195, 151.7343, 1.5),
                    Vec3::new(650.8899, 171.23123, 1.5),
                    Vec3::new(634.31995, 185.86598, 1.5),
                    Vec3::new(611.8579, 195.42595, 1.5),
                    Vec3::new(586.9329, 199.99419, 1.5),
                    Vec3::new(566.108, 199.9991, 1.5),
                    Vec3::new(540.90796, 195.36205, 1.5),
                    Vec3::new(516.033, 185.80794, 1.5),
                    Vec3::new(486.908, 183.6839, 1.5),
                    Vec3::new(461.83298, 210.0168, 1.5),
                    Vec3::new(436.80798, 231.2552, 1.5),
                    Vec3::new(411.88297, 247.45828, 1.5),
                    Vec3::new(386.93298, 258.69925, 1.5),
                    Vec3::new(361.88297, 264.97418, 1.5),
                    Vec3::new(336.88297, 266.22806, 1.5),
                    Vec3::new(311.90796, 262.4926, 1.5),
                    Vec3::new(290.83298, 255.45412, 1.5),
                    Vec3::new(261.90796, 240.01866, 1.5),
                    Vec3::new(236.85796, 221.23843, 1.5),
                    Vec3::new(211.88297, 197.51817, 1.5),
                    Vec3::new(191.05795, 173.92422, 1.5),
                    Vec3::new(179.001, 161.86388, 1.5),
                    Vec3::new(172.51, 161.86055, 1.5),
                    Vec3::new(159.99698, 161.86055, 1.5),
                    Vec3::new(145.03699, 161.86388, 1.5),
                    Vec3::new(117.60898, 161.86716, 1.5),
                    Vec3::new(92.60897, 161.86055, 1.5),
                    Vec3::new(71.70897, 161.86552, 1.5),
                    Vec3::new(42.633965, 161.86716, 1.5),
                    Vec3::new(17.583967, 161.9155, 1.5),
                    Vec3::new(-7.3660336, 161.86055, 1.5),
                    Vec3::new(-28.191034, 161.86552, 1.5),
                    Vec3::new(-57.49103, 161.85036, 1.5),
                    Vec3::new(-78.29103, 161.86388, 1.5),
                    Vec3::new(-107.466034, 161.8555, 1.5),
                    Vec3::new(-128.34103, 161.85036, 1.5),
                    Vec3::new(-157.41605, 161.86552, 1.5),
                    Vec3::new(-182.49104, 161.85379, 1.5),
                    Vec3::new(-207.36604, 161.86055, 1.5),
                ].iter().copied().collect(), //TODO: is there a better way to do this?
                timer: Timer::from_seconds(0.1, true),
            }
        },
        3 => Map {
            values: read_map_from_file(LEVEL_03),
            // To get rival_positions, play in training mode, then copy the output into the source code
            rival_positions: Positions {
                values: vec![
                    Vec3::new(-342.0, -74.91702, 1.5),
                    Vec3::new(-342.0, -82.88066, 1.5),
                    Vec3::new(-342.0, -93.30903, 1.5),
                    Vec3::new(-342.0, -113.653824, 1.5),
                    Vec3::new(-342.0, -126.00001, 1.5),
                    Vec3::new(-341.004, -126.0, 1.5),
                    Vec3::new(-332.92902, -126.0, 1.5),
                    Vec3::new(-322.13998, -121.05, 1.5),
                    Vec3::new(-303.65396, -93.94956, 1.5),
                    Vec3::new(-275.66693, -68.796005, 1.5),
                    Vec3::new(-250.56693, -52.494293, 1.5),
                    Vec3::new(-229.11693, -42.561108, 1.5),
                    Vec3::new(-200.66695, -35.03131, 1.5),
                    Vec3::new(-175.54195, -33.780014, 1.5),
                    Vec3::new(-150.61694, -37.52911, 1.5),
                    Vec3::new(-129.89197, -44.445755, 1.5),
                    Vec3::new(-106.112015, -60.047203, 1.5),
                    Vec3::new(-95.73311, -78.70411, 1.5),
                    Vec3::new(-90.20705, -102.45256, 1.5),
                    Vec3::new(-85.22877, -126.0, 1.5),
                    Vec3::new(-71.718056, -126.0, 1.5),
                    Vec3::new(-56.99134, -126.0, 1.5),
                    Vec3::new(-29.81411, -100.99891, 1.5),
                    Vec3::new(-4.8391104, -90.82347, 1.5),
                    Vec3::new(20.26089, -96.27475, 1.5),
                    Vec3::new(39.88414, -104.62085, 1.5),
                    Vec3::new(55.78235, -122.06989, 1.5),
                    Vec3::new(62.26894, -126.0, 1.5),
                    Vec3::new(64.325676, -125.99999, 1.5),
                    Vec3::new(65.69809, -126.00001, 1.5),
                    Vec3::new(69.675896, -126.0, 1.5),
                    Vec3::new(78.27759, -122.23661, 1.5),
                    Vec3::new(96.90629, -91.0057, 1.5),
                    Vec3::new(119.28735, -69.79703, 1.5),
                    Vec3::new(140.06235, -55.963734, 1.5),
                    Vec3::new(169.18733, -42.43608, 1.5),
                    Vec3::new(190.08733, -36.875076, 1.5),
                    Vec3::new(219.16232, -34.97987, 1.5),
                    Vec3::new(244.18732, -38.740868, 1.5),
                    Vec3::new(265.0873, -45.743977, 1.5),
                    Vec3::new(294.16232, -39.552753, 1.5),
                    Vec3::new(315.06232, -17.99502, 1.5),
                    Vec3::new(343.8298, 6.288747, 1.5),
                    Vec3::new(358.49734, 20.275003, 1.5),
                    Vec3::new(368.22406, 32.12499, 1.5),
                    Vec3::new(372.69653, 37.540604, 1.5),
                    Vec3::new(374.7811, 38.209232, 1.5),
                    Vec3::new(377.99646, 34.431522, 1.5),
                    Vec3::new(387.08923, 25.536932, 1.5),
                    Vec3::new(398.4154, 11.914133, 1.5),
                    Vec3::new(405.28314, -9.19795, 1.5),
                    Vec3::new(411.62265, -30.90305, 1.5),
                    Vec3::new(424.2371, -59.30522, 1.5),
                    Vec3::new(435.06308, -89.42522, 1.5),
                    Vec3::new(440.7769, -119.245224, 1.5),
                    Vec3::new(445.87097, -144.0, 1.5),
                    Vec3::new(459.2033, -144.0, 1.5),
                    Vec3::new(473.6547, -135.22122, 1.5),
                    Vec3::new(498.38153, -107.24417, 1.5),
                    Vec3::new(521.58154, -87.66304, 1.5),
                    Vec3::new(546.58154, -71.40327, 1.5),
                    Vec3::new(571.60657, -60.169117, 1.5),
                    Vec3::new(600.83154, -53.351, 1.5),
                    Vec3::new(622.78156, -38.305035, 1.5),
                    Vec3::new(650.8316, -10.337046, 1.5),
                    Vec3::new(671.6066, 6.3048697, 1.5),
                    Vec3::new(700.7317, 23.82735, 1.5),
                    Vec3::new(721.6066, 32.210594, 1.5),
                    Vec3::new(750.70667, 38.071278, 1.5),
                    Vec3::new(775.7817, 37.644012, 1.5),
                    Vec3::new(800.80664, 32.247345, 1.5),
                    Vec3::new(825.8066, 21.850239, 1.5),
                    Vec3::new(850.7066, 6.533561, 1.5),
                    Vec3::new(871.6316, -10.1738405, 1.5),
                    Vec3::new(900.83167, -39.35258, 1.5),
                    Vec3::new(923.5025, -68.89765, 1.5),
                    Vec3::new(936.72125, -98.92765, 1.5),
                    Vec3::new(951.9206, -128.83765, 1.5),
                    Vec3::new(973.1936, -144.0, 1.5),
                    Vec3::new(998.1548, -144.0, 1.5),
                    Vec3::new(1018.9298, -144.0, 1.5),
                    Vec3::new(1048.15, -144.0, 1.5),
                    Vec3::new(1073.2002, -144.0, 1.5),
                    Vec3::new(1098.1754, -144.0, 1.5),
                    Vec3::new(1123.1254, -144.0, 1.5),
                    Vec3::new(1144.2253, -144.0, 1.5),
                    Vec3::new(1152.0, -144.0, 1.5),
                    Vec3::new(1152.0, -132.13058, 1.5),
                    Vec3::new(1152.0, -106.55062, 1.5),
                    Vec3::new(1152.0, -88.11219, 1.5),
                    Vec3::new(1152.0, -63.766026, 1.5),
                    Vec3::new(1152.0, -35.547115, 1.5),
                    Vec3::new(1152.0, -19.602997, 1.5),
                    Vec3::new(1150.3291, -4.985032, 1.5),
                    Vec3::new(1142.8431, 4.5932636, 1.5),
                    Vec3::new(1128.3373, 9.212085, 1.5),
                    Vec3::new(1116.7552, 8.10708, 1.5),
                    Vec3::new(1112.0516, 3.2428055, 1.5),
                    Vec3::new(1108.465, -9.472504, 1.5),
                    Vec3::new(1107.1428, -22.615808, 1.5),
                    Vec3::new(1106.6078, -36.0, 1.5),
                    Vec3::new(1108.2349, -31.08, 1.5),
                    Vec3::new(1114.8239, -5.867016, 1.5),
                    Vec3::new(1126.4528, 14.820813, 1.5),
                    Vec3::new(1146.7458, 41.463272, 1.5),
                    Vec3::new(1152.0, 68.109634, 1.5),
                    Vec3::new(1154.531, 85.054115, 1.5),
                    Vec3::new(1164.8761, 102.32953, 1.5),
                    Vec3::new(1177.9862, 110.9646, 1.5),
                    Vec3::new(1195.3097, 116.55439, 1.5),
                    Vec3::new(1202.2339, 116.546745, 1.5),
                    Vec3::new(1206.9063, 111.95873, 1.5),
                    Vec3::new(1206.3685, 100.29673, 1.5),
                    Vec3::new(1201.7778, 87.74538, 1.5),
                    Vec3::new(1198.0302, 72.0, 1.5),
                    Vec3::new(1194.8861, 72.0, 1.5),
                    Vec3::new(1186.4285, 72.0, 1.5),
                    Vec3::new(1169.108, 89.8389, 1.5),
                    Vec3::new(1151.2649, 111.060394, 1.5),
                    Vec3::new(1126.6918, 131.25598, 1.5),
                    Vec3::new(1097.5918, 156.08234, 1.5),
                    Vec3::new(1072.6168, 181.46254, 1.5),
                    Vec3::new(1050.6418, 199.66182, 1.5),
                    Vec3::new(1022.6417, 217.29813, 1.5),
                    Vec3::new(1001.2802, 227.748, 1.5),
                    Vec3::new(991.1672, 232.61595, 1.5),
                    Vec3::new(984.24994, 233.80875, 1.5),
                    Vec3::new(980.64, 230.06451, 1.5),
                    Vec3::new(974.29095, 219.36293, 1.5),
                    Vec3::new(964.6515, 207.42554, 1.5),
                    Vec3::new(947.2546, 188.05023, 1.5),
                    Vec3::new(920.9967, 180.0, 1.5),
                    Vec3::new(900.1717, 180.0, 1.5),
                    Vec3::new(875.1217, 177.0526, 1.5),
                    Vec3::new(864.0, 168.66911, 1.5),
                    Vec3::new(864.0, 162.0, 1.5),
                    Vec3::new(864.951, 162.0, 1.5),
                ].iter().copied().collect(), //TODO: is there a better way to do this?
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
