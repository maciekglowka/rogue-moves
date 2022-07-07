use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;


#[derive(Component)]
pub struct UiCamera;


pub fn spawn_camera(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}