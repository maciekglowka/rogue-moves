use bevy::prelude::*;

use crate::board::{Board, Blocker, Position};
use crate::units::{
    Unit,
    npc::NPC,
    player::MovePlayerEvent
};
use crate::states::GameState;
use crate::vectors::Vector2Int;


pub fn mouse_press_menu(
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<State<GameState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        game_state.set(GameState::MapGenerate);
    }
}

pub fn mouse_press_game(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
    mut ev_player: EventWriter<MovePlayerEvent>,
    unit_query: Query<(Entity, &Position), With<NPC>>,
    mut assets: ResMut<super::cursor::CursorAssets>,
    mut ev_ui: EventWriter<super::cursor::DrawCursorEvent>
) {
    if buttons.just_pressed(MouseButton::Left) {        
        if let Some(world_pos) = mouse_to_world(&windows, &camera_query) {
            let v = Vector2Int::from_world(world_pos.x, world_pos.y);
            ev_player.send(MovePlayerEvent(v));
        };
    }

    if buttons.just_pressed(MouseButton::Right) { 
        if let Some(world_pos) = mouse_to_world(&windows, &camera_query) {
            let v = Vector2Int::from_world(world_pos.x, world_pos.y);
            for (entity, position) in unit_query.iter() {
                if position.v != v { continue; }
    
                if assets.npc != Some(entity) {
                    assets.npc = Some(entity);
                    ev_ui.send(super::cursor::DrawCursorEvent);
                }
                return;
            }
        }

    }

    if buttons.just_released(MouseButton::Right) {
        assets.npc = None; 
        ev_ui.send(super::cursor::DrawCursorEvent);
    }
}

// pub fn mouse_hover(
//     windows: Res<Windows>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
//     unit_query: Query<(Entity, &Position), With<NPC>>,
//     mut assets: ResMut<super::cursor::CursorAssets>,
//     mut ev_ui: EventWriter<super::cursor::DrawCursorEvent>
// ) {
//     if let Some(world_pos) = mouse_to_world(windows, &camera_query) {
//         let v = Vector2Int::from_world(world_pos.x, world_pos.y);
//         for (entity, position) in unit_query.iter() {
//             if position.v != v { continue; }

//             if assets.npc != Some(entity) {
//                 assets.npc = Some(entity);
//                 ev_ui.send(super::cursor::DrawCursorEvent);
//             }
//             return;
//         }
//     }
//     if !assets.npc.is_none() { 
//         assets.npc = None; 
//         ev_ui.send(super::cursor::DrawCursorEvent);
//     }
// }

fn mouse_to_world(
    windows: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get_primary().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        return Some(world_pos);
    }
    None
}