use bevy::prelude::*;

use crate::board::{Board, Blocker, Position};
use crate::units::{
    Unit,
    npc::NPC,
    player::MovePlayerEvent
};
use crate::states::GameState;
use crate::vectors::Vector2Int;

use super::player_menu::PlayerButtonClickEvent;

pub struct InputAssets {
    pub selected_npc: Option<Entity>,
    pub clicked_button: Option<Entity>
}

pub fn reset_input_assets(
    mut commands: Commands
) {
    commands.insert_resource(InputAssets {
        selected_npc: None,
        clicked_button: None
    });
}


pub fn mouse_press_menu(
    mut buttons: ResMut<Input<MouseButton>>,
    mut game_state: ResMut<State<GameState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        buttons.clear();
        game_state.set(GameState::MapGenerate);
    }
}

pub fn mouse_press_game_over(
    mut buttons: ResMut<Input<MouseButton>>,
    mut game_state: ResMut<State<GameState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        buttons.clear();
        game_state.set(GameState::MainMenu);
    }
}

pub fn mouse_press_game(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
    mut ev_player: EventWriter<MovePlayerEvent>,
    unit_query: Query<(Entity, &Position), With<NPC>>,
    mut assets: ResMut<InputAssets>,
    mut ev_ui: EventWriter<super::RedrawUIEvent>,
    mut interactions: Query<(&Interaction, Entity, &mut UiColor), (Changed<Interaction>, With<Button>)>,
    mut ev_player_button: EventWriter<PlayerButtonClickEvent>,
) {
    let mut menu_clicked = false;
    for (interaction, entity, mut color) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                menu_clicked = true;
                assets.clicked_button = Some(entity);
                *color =super::player_menu::BUTTON_COLOR_CLICK.into();
            },
            Interaction::None => {
                assets.clicked_button = None;
                *color = super::player_menu::BUTTON_COLOR.into();
            }
            _ => {}
        }
    }

    if buttons.just_pressed(MouseButton::Left) && !menu_clicked {        
        if let Some(world_pos) = mouse_to_world(&windows, &camera_query) {
            let v = Vector2Int::from_world(world_pos.x, world_pos.y);
            ev_player.send(MovePlayerEvent(v));
        };
    }

    if buttons.just_released(MouseButton::Left) {
        if let Some(entity) = assets.clicked_button {
            assets.clicked_button = None; 
            ev_player_button.send(PlayerButtonClickEvent(entity));
        }
    }

    if buttons.just_pressed(MouseButton::Right) { 
        if let Some(world_pos) = mouse_to_world(&windows, &camera_query) {
            let v = Vector2Int::from_world(world_pos.x, world_pos.y);
            for (entity, position) in unit_query.iter() {
                if position.v != v { continue; }
    
                if assets.selected_npc != Some(entity) {
                    assets.selected_npc = Some(entity);
                    ev_ui.send(super::RedrawUIEvent);
                }
                break;
            }
        }

    }

    if buttons.just_released(MouseButton::Right) {
        assets.selected_npc = None; 
        ev_ui.send(super::RedrawUIEvent);
    }
}


fn mouse_to_world(
    windows: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get_primary().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        return Some(world_pos);
    }
    None
}