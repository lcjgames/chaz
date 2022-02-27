use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct Positions {
    pub values: VecDeque::<Vec3>,
    pub timer: Timer,
}

impl Default for Positions {
    fn default() -> Self {
        Self {
            values: VecDeque::with_capacity(1000),
            timer: Timer::from_seconds(0.1, true),
        }
    }
}
