use bevy::prelude::*;

//TODO: derive this for all hitboxes for comfort
trait Collide {
    fn collide<T: Collide>(&self, other: &T) -> Option<CollisionType>;
}

#[derive(Clone, Default)]
pub struct Hitbox {
    pub relative_position: Vec3,
    pub size: Vec2,
}

pub enum CollisionType {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Collision {
    pub collision_type: CollisionType,
    pub overlap: f32,
}

impl Hitbox {
    pub fn collide(&self, position: &Vec3, other: &Hitbox, other_position: &Vec3) -> Option<Collision> {
        Hitbox::inner_collide(
            *other_position + other.relative_position,
            other.size,
            *position + self.relative_position,
            self.size,
        )
    }

    //TODO: refactor, since I just copied this and added the overlap
    pub fn inner_collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
        let a_min = a_pos.truncate() - a_size / 2.0;
        let a_max = a_pos.truncate() + a_size / 2.0;

        let b_min = b_pos.truncate() - b_size / 2.0;
        let b_max = b_pos.truncate() + b_size / 2.0;

        // check to see if the two rectangles are intersecting
        if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
            // check to see if we hit on the left or right side
            let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
            {
                (Some(CollisionType::Left), b_min.x - a_max.x)
            } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
                (Some(CollisionType::Right), a_min.x - b_max.x)
            } else {
                (None, 0.0)
            };

            // check to see if we hit on the top or bottom side
            let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
            {
                (Some(CollisionType::Bottom), b_min.y - a_max.y)
            } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
                (Some(CollisionType::Top), a_min.y - b_max.y)
            } else {
                (None, 0.0)
            };

            // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
            match (x_collision, y_collision) {
                (Some(x_collision), Some(y_collision)) => {
                    if y_depth.abs() < x_depth.abs() {
                        Some(Collision{ collision_type: y_collision, overlap: -y_depth })
                    } else {
                        Some(Collision{ collision_type: x_collision, overlap: -x_depth })
                    }
                }
                (Some(x_collision), None) => Some(Collision{ collision_type: x_collision, overlap: -x_depth }),
                (None, Some(y_collision)) => Some(Collision{ collision_type: y_collision, overlap: -y_depth }),
                (None, None) => None,
            }
        } else {
            None
        }
    }
}

#[derive(Component, Default)]
pub struct PlayerGroundHitbox(pub Hitbox);

#[derive(Component, Default)]
pub struct GroundHitbox(pub Hitbox);

#[derive(Component, Default)]
pub struct WinHitbox(pub Hitbox);

#[derive(Component, Default)]
pub struct PlayerEnemyHitbox(pub Hitbox);

#[derive(Component, Default)]
pub struct EnemyHitbox(pub Hitbox);
