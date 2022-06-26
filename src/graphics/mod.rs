use bevy::prelude::*;

use crate::states::{AnimationState, GameState};

pub mod board_renderer;
pub mod sprites;
pub mod unit_renderer;

pub const TILE_SIZE: f32 = 64.;

pub const MAP_Z: f32 = 0.;
pub const UNIT_Z: f32 = 10.;
pub const OVERLAY_Z: f32 = 100.;

pub const UNIT_SPEED: f32 = 20.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadAssets)
                .with_system(board_renderer::load_assets)
                .with_system(unit_renderer::load_assets)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::MapGenerate)
                .with_system(board_renderer::draw_board)
                .with_system(unit_renderer::draw_units)
        );
        app.add_system_set(
            SystemSet::on_update(AnimationState::Animating)
                .with_system(unit_renderer::animate_units)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(unit_renderer::camera_follow)
        );
    }
}
