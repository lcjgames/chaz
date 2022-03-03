use bevy::prelude::*;

use std::fmt::*;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub difficulty: Difficulty,
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

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
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
