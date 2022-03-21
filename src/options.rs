use bevy::prelude::*;
use enum_iterator::IntoEnumIterator;
use std::fmt::*;

#[derive(Clone, Debug)]
pub struct LeaderBoardOptions {
    pub difficulty: Difficulty,
    pub level: usize,
}

impl Default for LeaderBoardOptions {
    fn default() -> Self {
        LeaderBoardOptions {
            difficulty: Difficulty::default(),
            level: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub name: String,
    pub difficulty: Difficulty,
    pub level: usize,
    pub music_volume: u32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            name: "Chaz".to_string(),
            difficulty: Difficulty::default(),
            level: 1,
            music_volume: 100,
        }
    }
}

impl Options {
    pub fn set<T: Option>(&mut self, t: T) {
        t.add_to(self);
    }
}

pub trait Option {
    fn add_to(self, options: &mut Options);
}

pub trait Next {
    fn next(self) -> Self; //TODO: implement with a macro
}

#[derive(Clone, Component, Copy, Debug, Default, IntoEnumIterator, PartialEq)]
pub enum Difficulty {
    Training,
    Easy,
    #[default]
    Medium,
    Hard,
    Zatoichi,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

impl Option for Difficulty {
    fn add_to(self, options: &mut Options) {
        options.difficulty = self;
    }
}

impl Next for Difficulty {
    fn next(self) -> Self {
        match self {
            Difficulty::Training => Difficulty::Easy,
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Zatoichi,
            Difficulty::Zatoichi => Difficulty::Training,
        }
    }
}
