use bevy::prelude::*;
use std::collections::VecDeque;

use crate::board::{
    Blocker, Board, Position,
    utils::get_spawn_position,
    tile::{Tile, TileInteractionEvent}
};
use crate::states::{GameState, AnimationState};
use crate::vectors::Vector2Int;

use super::data::get_unit_behaviour;
use super::player::Player;
use super::Unit;

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
    mut unit_query: Query<&mut Unit>,
    tile_query: Query<(&Position, &Tile)>,
    mut ev_tile: EventWriter<TileInteractionEvent>
) {
    if game_state.current() != &GameState::NPCTurn { return; }

    if let Some(entity) = npc_queue.current {
        let position = unit_position.get(entity).unwrap().1;

        if let Some(killed) = super::check_unit_interaction(
            entity,
            position, 
            &unit_position) {
                commands.entity(killed).despawn_recursive();
            };

        let mut unit = unit_query.get_mut(entity).unwrap();
        let turn_end = unit.handle_move_end();
        npc_queue.current = None;
        ev_tile.send(TileInteractionEvent(entity));
    }
}

pub fn move_npc(
    mut npc_queue: ResMut<NPCQueue>,
    mut npc_query: Query<(&mut Position, &mut Unit, &Blocker), (With<NPC>, Without<Player>)>,
    board_query: Query<&Board>,
    blocker_query: Query<(&Position, &Blocker), (Without<NPC>, Without<Player>)>,
    player_query: Query<(&Position, &Blocker), With<Player>>,
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

    if let Ok((_, mut unit, _)) = npc_query.get_mut(entity) {
        unit.handle_turn_start();
        if unit.ap == 0 { return; }
    }

    let (player_position, player_blocker) = match player_query.get_single() {
        Ok(r) => r,
        Err(_) => return
    };

    let mut new_position_v = None;

    if let Ok((position, unit, _)) = npc_query.get(entity) {
        let mut blockers:  Vec<(&Position, &Blocker)> = blocker_query.iter().collect();
        let npc_blockers: Vec<(&Position, &Blocker)> = npc_query.iter().map(|(p, _, b)| (p, b)).collect();
        let npc_positions: Vec<(&Position)> = npc_query.iter().map(|(p, _, _)| p).collect();
        blockers.push((player_position, player_blocker));
        blockers.extend(npc_blockers);

        let board = board_query.get_single().unwrap();

        new_position_v = get_best_move(
            &unit,
            position.v,
            board,
            &blockers,
            &player_position,
            npc_positions
        );
    }
   
    if let Ok((mut position, _, _)) = npc_query.get_mut(entity) {  
        match new_position_v {
            Some(v) => {
                position.v = v;
            }
            _ => ()
        }
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
    commands: &mut Commands,
    blocker_positions: &mut Vec<Vector2Int>,
    board: &Board,
    rank_sum: u32
) { 
    let kinds = super::utils::get_npc_set(rank_sum);
    for kind in kinds {
        let position = get_spawn_position(blocker_positions, board);
        if position.is_none() { continue; }

        blocker_positions.push(position.unwrap());

        commands.spawn()
            .insert(Position { v: position.unwrap() })
            .insert(NPC)
            .insert(Blocker { is_targetable: true })
            .insert(Unit { 
                ap: super::BASE_AP,
                behaviour: get_unit_behaviour(&kind),
                kind: kind,
                state: super::UnitState::Active
            });
    }
}

fn get_best_move(
    unit: &Unit,
    source: Vector2Int,
    board: &Board,
    blockers: &Vec<(&Position, &Blocker)>,
    player_position: &Position,
    npc_positions: Vec<&Position>
) -> Option<Vector2Int> {

    let positions = unit.behaviour.possible_positions(source, board, blockers);

    if positions.len() == 0 {
        return None;
    }

    let mut rated = Vec::new();
    for v in positions {
        // let mut rank = match v.dist(player_position.v) {
        //     d if d >=2. || d < 1. => d,
        //     _ => 5.
        // };
        let mut rank = v.dist(player_position.v);
        if npc_positions.iter().any(|p| p.v == v) {
            rank += 50.;
        }
        rated.push((rank, v));
    }
    
    rated.sort_by_key(|a| (100. * a.0) as u32);
    Some(rated[0].1)
}