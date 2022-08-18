use bevy::prelude::*;

use crate::states::{AnimationState, GameState};

pub mod cursor;
mod input;
mod main_menu;
mod player_menu;
mod status;

pub struct RedrawUIEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(input::mouse_press_menu)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(main_menu::draw_menu)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(main_menu::clear_menu)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(input::mouse_press_game_over)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(cursor::clear_cursor)
                .with_system(status::clear_status)
                .with_system(player_menu::clear_menu)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadAssets)
                .with_system(cursor::load_assets)
                .with_system(player_menu::load_assets)
                .with_system(load_assets)
        );
        app.add_event::<RedrawUIEvent>();
        app.add_event::<player_menu::PlayerButtonClickEvent>();
        app.add_system_set(
            SystemSet::on_enter(GameState::PlayerTurn)
                .with_system(input::reset_input_assets)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(cursor::draw_cursor)
                .with_system(input::mouse_press_game)
                .with_system(status::draw_status)
                .with_system(player_menu::draw_menu)
                .with_system(player_menu::button_click)
        );
        app.add_system_set(
            SystemSet::on_enter(AnimationState::Animating)
                .with_system(cursor::clear_cursor)
        );
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>    
) {
    let handle = asset_server.load("pixel.ttf");
    asset_list.0.push(handle.clone_untyped());

    commands.insert_resource(
        FontAssets { 
            font: handle
        }
    );
}

pub struct FontAssets {
    font: Handle<Font>
}