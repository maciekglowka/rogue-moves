use bevy::prelude::*;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

use crate::board::{Blocker, Board, Position};
use crate::states::{GameState, AnimationState};
use crate::vectors::Vector2Int;

use super::action::ActionType;
use super::behaviour::{Behaviour, get_ortho_pattern};
use super::Unit;

const BASE_AP: u8 = 1;

pub struct NPCQueue {
    pub npcs: VecDeque<Entity>,
    pub current: Option<Entity>
}

#[derive(Component)]
pub struct NPC;

pub fn tick(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut npc_queue: ResMut<NPCQueue>,
    unit_position: Query<(Entity, &Position), With<Unit>>,
    mut unit_query: Query<&mut Unit>
) {
    if game_state.current() != &GameState::NPCTurn { return; }

    if let Some(entity) = npc_queue.current {

        if let Ok((_, position)) = unit_position.get(entity) {
            if let Some(killed) = super::check_unit_interaction(
                entity,
                position, 
                &unit_position) {
                    commands.entity(killed).despawn_recursive();
                };
        }

        let mut unit = unit_query.get_mut(entity).unwrap();
        let turn_end = unit.handle_turn_end();
        npc_queue.current = None;
    }
}

pub fn move_npc(
    mut npc_queue: ResMut<NPCQueue>,
    mut npc_query: Query<(&mut Position, &mut Unit, &Blocker), With<NPC>>,
    board_query: Query<&Board>,
    blocker_query: Query<(&Position, &Blocker), Without<NPC>>,
    mut game_state: ResMut<State<GameState>>,
    mut animation_state: ResMut<State<AnimationState>>
) {
    if npc_queue.current.is_some() { return; }

    let entity = match npc_queue.npcs.pop_front() {
        Some(e) => e,
        None => {
            game_state.set(GameState::PlayerTurn);
            return;
        }
    };

    let mut new_position_v = None;

    if let Ok((position, unit, _)) = npc_query.get(entity) {
        let mut blockers:  Vec<(&Position, &Blocker)> = blocker_query.iter().collect();
        let npc_blockers: Vec<(&Position, &Blocker)>  = npc_query.iter().map(|(p, _, b)| (p, b)).collect();
        blockers.extend(npc_blockers);

        let board = board_query.get_single().unwrap();

        new_position_v = get_best_move(
            &unit,
            position.v,
            board,
            &blockers
        );
    }
   
    if let Ok((mut position, mut unit, _)) = npc_query.get_mut(entity) {  
        match new_position_v {
            Some(v) => {
                position.v = v;
            }
            _ => ()
        }    
        unit.ap = BASE_AP;
        npc_queue.current = Some(entity);
    }

    animation_state.set(AnimationState::Animating);
}

pub fn start_npc_turn(
    npc_query: Query<Entity, With<NPC>>,
    mut queue: ResMut<NPCQueue>
) {
    let npcs = npc_query.iter().collect();
    queue.npcs = npcs;
    queue.current = None;
}

pub fn spawn_npcs(
    commands: &mut Commands
) {
    let pattern = get_ortho_pattern(2);
    let behaviour = Behaviour {
        pattern: pattern,
        action_type: ActionType::Walk
    };

    for x in 0..2 {
        commands.spawn()
            .insert(Position { v: Vector2Int::new(2 + 2*x, 3+x) })
            .insert(NPC)
            .insert(Blocker { is_targetable: true })
            .insert(Unit { 
                ap: BASE_AP,
                behaviour: behaviour.clone()
            });
    }
}

fn get_best_move(
    unit: &Unit,
    source: Vector2Int,
    board: &Board,
    blockers: &Vec<(&Position, &Blocker)>
) -> Option<Vector2Int> {
    let positions = unit.behaviour.possible_positions(source, board, blockers);
    match positions.choose(&mut rand::thread_rng()) {
        Some(v) => Some(*v),
        None => None
    }
}