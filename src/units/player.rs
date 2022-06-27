use bevy::prelude::*;

use crate::board::{Blocker, Board, Position};
use crate::ui;
use crate::states::{AnimationState, GameState};
use crate::vectors::Vector2Int;

use super::action::ActionType;
use super::{Unit, behaviour};
use super::behaviour::{Behaviour, get_omni_pattern};

#[derive(Component)]
pub struct Player;

pub struct PlayerData {
    pub current_behaviour: behaviour::Behaviour
}

pub struct MovePlayerEvent(pub Vector2Int);

pub fn start_player_turn(
    mut ev_ui: EventWriter<ui::cursor::DrawCursorEvent>,
    mut player_query: Query<&mut Unit, With<Player>>,
) {
    if let Ok(mut unit) = player_query.get_single_mut() {
        unit.ap = 2;
        ev_ui.send(ui::cursor::DrawCursorEvent);
    }
}

pub fn move_player(
    mut ev: EventReader<MovePlayerEvent>,
    mut query: Query<(Entity, &mut Position), With<Player>>,
    mut animation_state: ResMut<State<AnimationState>>
) {
    if animation_state.current() == &AnimationState::Animating { return ; }
    for ev in ev.iter() {
        if let Ok((entity, mut position)) = query.get_single_mut() {
            position.v = ev.0;
            animation_state.set(AnimationState::Animating);
        }
    }
}

pub fn tick(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut player_data: ResMut<PlayerData>,
    player_query: Query<Entity, With<Player>>,
    unit_position: Query<(Entity, &Position), With<Unit>>,
    mut unit_query: Query<&mut Unit>,
    mut ev_ui: EventWriter<ui::cursor::DrawCursorEvent>,
) {
    if game_state.current() != &GameState::PlayerTurn { return; }

    let entity = player_query.get_single().unwrap();

    if let Ok((_, position)) = unit_position.get(entity) {
        match super::check_unit_interaction(entity, position, &unit_position) {
            Some(killed) => {
                let killed_unit = unit_query.get(killed).unwrap();
                player_data.current_behaviour = killed_unit.behaviour.clone();
                commands.entity(killed).despawn_recursive();
            },
            None => {
                player_data.current_behaviour = get_base_player_behaviour(); 
            }
        };            
    }
    
    let mut unit = unit_query.get_mut(entity).unwrap();
    let turn_end = unit.handle_turn_end();

    if turn_end {
        game_state.set(GameState::NPCTurn);
    } else {
        ev_ui.send(ui::cursor::DrawCursorEvent);
    }
}

pub fn spawn_player(
    mut commands: &mut Commands,
) {
    let behaviour = get_base_player_behaviour();

    commands.spawn()
        .insert(Position { v: Vector2Int::new(0, 0) })
        .insert(Player)
        .insert(Blocker { is_targetable: true })
        .insert(Unit { 
            ap: 2,
            behaviour: behaviour
        });
}

pub fn get_base_player_behaviour() -> Behaviour {
    let pattern = get_omni_pattern(1);
    Behaviour {
        pattern: pattern,
        action_type: ActionType::Walk
    }
}