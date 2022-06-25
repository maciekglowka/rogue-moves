use bevy::prelude::*;

use crate::states::GameState;

pub mod cursor;
mod input;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input::mouse_press);

        app.add_system_set(
            SystemSet::on_enter(GameState::LoadAssets)
                .with_system(cursor::load_assets)
        );
        app.add_event::<cursor::DrawCursorEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(cursor::draw_cursor)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::PlayerTurn)
                .with_system(cursor::clear_cursor)
        );
    }
}
