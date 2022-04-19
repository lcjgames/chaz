use bevy::prelude::*;

use super::direction;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn apply_gravity(&mut self, time: f32) {
        let max_speed = 300.0;
        let gravity_acceleration = 500.0;
        self.y -= gravity_acceleration * time;
        limit(&mut self.y, max_speed);
    }
    fn increase(&mut self, direction: direction::Direction) {
        let max_speed = 250.0;
        let speed_increase = 10.0;
        self.x += speed_increase * f32::from(direction);
        limit(&mut self.x, max_speed);
    }
    fn decrease(&mut self) {
        if self.x.abs() < 10.0 {
            self.x = 0.0
        } else {
            self.x *= 0.9
        };
    }
    pub fn update(&mut self, direction: Option<direction::Direction>) {
        match direction {
            None => self.decrease(),
            Some(dir) => self.increase(dir),
        }
    }
    pub fn stop_left(&mut self) {
        if self.x < 0.0 {
            self.x = 0.0;
        }
    }
    pub fn stop_right(&mut self) {
        if self.x > 0.0 {
            self.x = 0.0;
        }
    }
    pub fn stop_top(&mut self) {
        if self.y > 0.0 {
            self.y = 0.0;
        }
    }
}

fn limit(value: &mut f32, limit: f32) {
    if *value > limit {
        *value = limit;
    }
    if *value < -limit {
        *value = -limit;
    }
}
