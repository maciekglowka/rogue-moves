use bevy::prelude::*;
use std::collections::VecDeque;

use crate::board::Position;
use crate::states::{GameState, AnimationState};
use crate::vectors::Vector2Int;

use super::action::ActionType;
use super::behaviour::{Behaviour, get_ortho_pattern};
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
    mut npc_queue: ResMut<NPCQueue>
) {
    if game_state.current() != &GameState::NPCTurn { return; }
    npc_queue.current = None;
}

pub fn move_npc(
    mut npc_queue: ResMut<NPCQueue>,
    mut npc_query: Query<&mut Position, With<NPC>>,
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

    npc_queue.current = Some(entity);
    if let Ok(mut position) = npc_query.get_mut(entity) {
        position.v += Vector2Int::new(1, 0);
        animation_state.set(AnimationState::Animating);
    };
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
            .insert(Unit { 
                ap: 1,
                behaviour: behaviour.clone()
            });
    }
}