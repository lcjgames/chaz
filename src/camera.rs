use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn reset_camera_position(mut camera_query: Query<(&MainCamera, &mut Transform)>) {
    for (_, mut transform) in camera_query.iter_mut() {
        *transform = OrthographicCameraBundle::new_2d().transform;
    }
}
