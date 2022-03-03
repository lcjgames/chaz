use bevy::prelude::*;

#[derive(Clone, Component, Copy, Default, PartialEq)]
pub enum Direction {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn from_input(left_pressed: bool, right_pressed: bool) -> Option<Self> {
        let mut direction = 0;
        if left_pressed {
            direction -= 1;
        }
        if right_pressed {
            direction += 1;
        }
        if direction > 0 {
            Some(Direction::Right)
        } else if direction < 0 {
            Some(Direction::Left)
        } else {
            None
        }
    }
    pub fn update(&mut self, velocity: &Vec3) {
        if velocity.x > 0.0 {
            *self = Direction::Right;
        }
        if velocity.x < 0.0 {
            *self = Direction::Left;
        }
    }
}

impl From<Direction> for f32 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left | Direction::Down => -1.0,
            Direction::Right | Direction::Up => 1.0,
        }
    }
}
