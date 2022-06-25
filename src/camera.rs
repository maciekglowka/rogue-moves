use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn_bundle(camera)
        .insert(MainCamera);
}