use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use bevy::prelude::*;
use bevy::asset::HandleId;
use crate::sprite::SpriteVariant::Sprite;

#[derive(Default)]
pub struct SpriteHandles {
    pub handles: HashMap<String, Vec<Handle<Image>>>,
}

impl SpriteHandles {
    pub fn id_list(&self) -> impl Iterator<Item = HandleId> + '_ {
        self.handles.iter()
            .map(|(_, handles)| handles.iter().map(|h| h.id))
            .flatten()
    }
}

#[derive(Component)]
pub struct SpriteTimer {
    pub timer: Timer,
}

impl SpriteTimer {
    pub fn from_seconds(duration: f32) -> Self {
        SpriteTimer {
            timer: Timer::from_seconds(duration, true)
        }
    }
}

#[derive(Clone)]
pub enum SpriteVariant {
    Sprite(&'static str),
    SpriteSheet(SpriteType)
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SpriteType {
    BlueBG,
    BrownBG,
    WhiteBG,
    GreenBG,
    IdleGreen,
    WalkGreen,
    JumpGreen,
    Blue,
    Pink,
    Yellow,
    Beige,
    SpikeBall,
    Fish,
    Block,
    Hedgehog,
    BabyJeremy,
    Jeremy,
    Angel,
    Ground,
    Heart,
}

impl FromStr for SpriteType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BlueBG" => Ok(SpriteType::BlueBG),
            "BrownBG" => Ok(SpriteType::BrownBG),
            "WhiteBG" => Ok(SpriteType::WhiteBG),
            "GreenBG" => Ok(SpriteType::GreenBG),
            "IdleGreen" => Ok(SpriteType::IdleGreen),
            "WalkGreen" => Ok(SpriteType::WalkGreen),
            "JumpGreen" => Ok(SpriteType::JumpGreen),
            "Blue" => Ok(SpriteType::Blue),
            "Pink" => Ok(SpriteType::Pink),
            "Yellow" => Ok(SpriteType::Yellow),
            "Beige" => Ok(SpriteType::Beige),
            "SpikeBall" => Ok(SpriteType::SpikeBall),
            "Fish" => Ok(SpriteType::Fish),
            "Block" => Ok(SpriteType::Block),
            "Hedgehog" => Ok(SpriteType::Hedgehog),
            "BabyJeremy" => Ok(SpriteType::BabyJeremy),
            "Jeremy" => Ok(SpriteType::Jeremy),
            "Angel" => Ok(SpriteType::Angel),
            "Ground" => Ok(SpriteType::Ground),
            "Heart" => Ok(SpriteType::Heart),
            _ => Err(()),
        }
    }
}

impl fmt::Display for SpriteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SpriteTypeStates {
    Empty,
    Half,
    Full,
    Closed,
    Open,
    Idle,
    Idle0,
    Idle1,
    Idle2,
    Surprised,
    Pissed,
    AloneGrass,
    RightGrass,
    LeftRightGrass,
    LeftGrass,
    DownGrass,
    DownGrassRight,
    DownGrassLeftRight,
    DownGrassLeft,
    Above,
    LeftAbove,
    RightAbove,
    BelowAbove,
    RightEmpty,
    LeftEmpty,
    BelowEmpty,
    BelowLeftEmpty,
    BelowRightEmpty,
    AboveLeftEmpty,
    AboveRightEmpty,
}

lazy_static!{
//TODO: maybe use enums instead of strings lol
pub static ref SPRITES: HashMap<SpriteType, HashMap<SpriteTypeStates, &'static str>> = HashMap::from([
    (SpriteType::BlueBG, HashMap::from([
        (SpriteTypeStates::Empty, "pixel-platformer/Background/background_0000.png"),
        (SpriteTypeStates::Half, "pixel-platformer/Background/background_0001.png"),
        (SpriteTypeStates::Full, "pixel-platformer/Background/background_0002.png"),
    ])),
    (SpriteType::BrownBG, HashMap::from([
        (SpriteTypeStates::Empty, "pixel-platformer/Background/background_0003.png"),
        (SpriteTypeStates::Half, "pixel-platformer/Background/background_0004.png"),
        (SpriteTypeStates::Full, "pixel-platformer/Background/background_0005.png"),
    ])),
    (SpriteType::WhiteBG, HashMap::from([
        (SpriteTypeStates::Empty, "pixel-platformer/Background/background_0006.png"),
        (SpriteTypeStates::Half, "pixel-platformer/Background/background_0007.png"),
        (SpriteTypeStates::Full, "pixel-platformer/Background/background_0008.png"),
    ])),
    (SpriteType::GreenBG, HashMap::from([
        (SpriteTypeStates::Empty, "pixel-platformer/Background/background_0009.png"),
        (SpriteTypeStates::Half, "pixel-platformer/Background/background_0010.png"),
        (SpriteTypeStates::Full, "pixel-platformer/Background/background_0011.png"),
    ])),
    (SpriteType::IdleGreen, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0000.png"),
    ])),
    (SpriteType::WalkGreen, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0000.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0001.png"),
    ])),
    (SpriteType::JumpGreen, HashMap::from([
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0001.png"),
    ])),
    (SpriteType::Blue, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0002.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0003.png"),
    ])),
    (SpriteType::Pink, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0004.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0005.png"),
    ])),
    (SpriteType::Yellow, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0006.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0007.png"),
    ])),
    (SpriteType::SpikeBall, HashMap::from([
        (SpriteTypeStates::Idle, "pixel-platformer/Characters/character_0008.png"),
    ])),
    (SpriteType::Beige, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0009.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0010.png"),
    ])),
    (SpriteType::Fish, HashMap::from([
        (SpriteTypeStates::Closed, "pixel-platformer/Characters/character_0011.png"),
        (SpriteTypeStates::Open, "pixel-platformer/Characters/character_0012.png"),
    ])),
    (SpriteType::Block, HashMap::from([
        (SpriteTypeStates::Surprised, "pixel-platformer/Characters/character_0013.png"),
        (SpriteTypeStates::Pissed, "pixel-platformer/Characters/character_0014.png"),
    ])),
    (SpriteType::Hedgehog, HashMap::from([
        (SpriteTypeStates::Idle0, "pixel-platformer/Characters/character_0015.png"),
        (SpriteTypeStates::Idle1, "pixel-platformer/Characters/character_0016.png"),
        (SpriteTypeStates::Idle2, "pixel-platformer/Characters/character_0017.png"),
    ])),
    (SpriteType::BabyJeremy, HashMap::from([
        (SpriteTypeStates::Idle0, "pixel-platformer/Characters/character_0018.png"),
        (SpriteTypeStates::Idle1, "pixel-platformer/Characters/character_0019.png"),
        (SpriteTypeStates::Idle0, "pixel-platformer/Characters/character_0020.png"),
    ])),
    (SpriteType::Jeremy, HashMap::from([
        (SpriteTypeStates::Idle0, "pixel-platformer/Characters/character_0021.png"),
        (SpriteTypeStates::Idle1, "pixel-platformer/Characters/character_0022.png"),
        (SpriteTypeStates::Idle2, "pixel-platformer/Characters/character_0023.png"),
    ])),
    (SpriteType::Angel, HashMap::from([
        (SpriteTypeStates::Idle0, "pixel-platformer/Characters/character_0024.png"),
        (SpriteTypeStates::Idle1, "pixel-platformer/Characters/character_0025.png"),
        (SpriteTypeStates::Idle2, "pixel-platformer/Characters/character_0026.png"),
    ])),
    (SpriteType::Ground, HashMap::from([
        (SpriteTypeStates::AloneGrass, "pixel-platformer/Tiles/tile_0000.png"),
        (SpriteTypeStates::RightGrass, "pixel-platformer/Tiles/tile_0001.png"),
        (SpriteTypeStates::LeftRightGrass, "pixel-platformer/Tiles/tile_0002.png"),
        (SpriteTypeStates::LeftGrass, "pixel-platformer/Tiles/tile_0003.png"),
        (SpriteTypeStates::DownGrass, "pixel-platformer/Tiles/tile_0020.png"),
        (SpriteTypeStates::DownGrassRight, "pixel-platformer/Tiles/tile_0021.png"),
        (SpriteTypeStates::DownGrassLeftRight, "pixel-platformer/Tiles/tile_0022.png"),
        (SpriteTypeStates::DownGrassLeft, "pixel-platformer/Tiles/tile_0023.png"),
        (SpriteTypeStates::Full, "pixel-platformer/Tiles/tile_0122.png"),
        (SpriteTypeStates::Above, "pixel-platformer/Tiles/tile_0140.png"),
        (SpriteTypeStates::LeftAbove, "pixel-platformer/Tiles/tile_0143.png"),
        (SpriteTypeStates::RightAbove, "pixel-platformer/Tiles/tile_0141.png"),
        (SpriteTypeStates::BelowAbove, "pixel-platformer/Tiles/tile_0120.png"),
        (SpriteTypeStates::LeftEmpty, "pixel-platformer/Tiles/tile_0121.png"),
        (SpriteTypeStates::RightEmpty, "pixel-platformer/Tiles/tile_0123.png"),
        (SpriteTypeStates::BelowEmpty, "pixel-platformer/Tiles/tile_0142.png"),
        (SpriteTypeStates::BelowLeftEmpty, "pixel-platformer/Tiles/tile_0005.png"),
        (SpriteTypeStates::BelowRightEmpty, "pixel-platformer/Tiles/tile_0004.png"),
        (SpriteTypeStates::AboveLeftEmpty, "pixel-platformer/Tiles/tile_0025.png"),
        (SpriteTypeStates::AboveRightEmpty, "pixel-platformer/Tiles/tile_0024.png"),
    ])),
    (SpriteType::Heart, HashMap::from([
        (SpriteTypeStates::Full, "pixel-platformer/Tiles/tile_0044.png"),
        (SpriteTypeStates::Half, "pixel-platformer/Tiles/tile_0045.png"),
        (SpriteTypeStates::Empty, "pixel-platformer/Tiles/tile_0046.png"),
    ])),
    ("torch light", HashMap::from([
        ("effect", "torch-light-effect.png"),
    ])),
]);
}

pub fn load_sprites(name: SpriteType, asset_server: &Res<AssetServer>) -> Vec<Handle<Image>> {
    SPRITES[&name].iter().map(
            |(_, &path)| asset_server.load(path)
        ).collect()
}

pub fn spawn(
    name: String,
    sprite_handles: &Res<SpriteHandles>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Image>>,
) -> Handle<TextureAtlas> {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles[&name].iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(textures).unwrap();
    texture_atlases.add(texture_atlas)
}
