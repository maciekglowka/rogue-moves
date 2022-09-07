use bevy::prelude::*;

use crate::states::{AnimationState, FadeState, GameState};

pub mod board_renderer;
mod fx;
pub mod item_renderer;
pub mod sprites;
pub mod unit_renderer;
mod utils;

pub const TILE_SIZE: f32 = 64.;

pub const MAP_Z: f32 = 0.;
pub const MASK_Z: f32 = 1.;
pub const UNIT_Z: f32 = 10.;
pub const ITEM_Z: f32 = 5.;
pub const CURSOR_Z: f32 = 100.;
pub const OVERLAY_Z: f32 = 200.;

pub const UNIT_SPEED: f32 = 20.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadAssets)
                .with_system(board_renderer::load_assets)
                .with_system(unit_renderer::load_assets)
                .with_system(item_renderer::load_assets)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Spawning)
                .with_system(board_renderer::draw_board)
                .with_system(unit_renderer::draw_units)
                .with_system(item_renderer::draw_items)
        );
        app.add_system_set(
            SystemSet::on_update(AnimationState::Animating)
                .with_system(unit_renderer::animate_units)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Spawning)
                .with_system(camera_center)
        );
        app.insert_resource(
            unit_renderer::SpriteTimer::new()
        );
        app.add_system(
            unit_renderer::animate_sprites
        );

        // FADE
        app.add_system_set(
            SystemSet::on_enter(FadeState::In)
                .with_system(fx::draw_overlay)
        );
        app.add_system_set(
            SystemSet::on_update(FadeState::In)
                .with_system(fx::fade_overlay_in)
        );
        app.add_system_set(
            SystemSet::on_update(FadeState::Out)
                .with_system(fx::fade_overlay_out)
        );
        app.add_system_set(
            SystemSet::on_exit(FadeState::Out)
                .with_system(fx::clear_overlay)
        );
    }
}

pub fn camera_center(
    mut camera_query: Query<&mut Transform, (Without<crate::units::player::Player>, With<crate::camera::MainCamera>)>
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let half = TILE_SIZE * (crate::board::SIZE / 2) as f32;
        camera_transform.translation.x = half;
        camera_transform.translation.y = half;
    }
}