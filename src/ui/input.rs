use bevy::prelude::*;

use crate::units::player::MovePlayerEvent;
use crate::states::GameState;
use crate::vectors::Vector2Int;


pub fn keyboard_press(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut player_data: ResMut<crate::units::player::PlayerData>,
    mut ev_ui: EventWriter<super::cursor::DrawCursorEvent>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        keyboard.clear();
        if let Some(behaviour) = &player_data.captured_behaviour {
            player_data.current_behaviour = behaviour.clone();
            player_data.captured_behaviour = None;
            ev_ui.send(super::cursor::DrawCursorEvent);
            println!("used");
        }
    }
}


pub fn mouse_press(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
    mut ev_player: EventWriter<MovePlayerEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        match game_state.current() {
            GameState::MainMenu => {
                game_state.set(GameState::MapGenerate);
            },
            GameState::PlayerTurn => {
                if let Some(world_pos) = click_to_world(windows, &camera_query) {
                    let v = Vector2Int::from_world(world_pos.x, world_pos.y);
                    ev_player.send(MovePlayerEvent(v));
                };
            }
            _ => {}
        };
    }
}

fn click_to_world(
    windows: Res<Windows>,
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