use bevy::prelude::*;
use std::collections::VecDeque;

use crate::command::{CommandEvent, CommandType};

use crate::states::{AnimationState, GameState};
use crate::board::{
    Blocker,
    Board,
    Position,
    tile::{Tile, TileKind}
};
use crate::vectors::Vector2Int;

mod action;
pub mod behaviour;
mod data;
pub mod npc;
pub mod player;
mod utils;

const BASE_AP: u8 = 1;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player::reset_player_data);
        app.add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(player::reset_player_data)
                .with_system(clear_units)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Spawning)
                .with_system(spawn_units)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(clear_units)
        );

        app.add_event::<player::MovePlayerEvent>();

        app.add_system_set(
            SystemSet::on_enter(GameState::PlayerTurn)
                .with_system(player::start_player_turn)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(player::move_player)
                .with_system(player::player_status)
        );

        app.add_system_set(
            SystemSet::on_enter(GameState::NPCTurn)
                .with_system(npc::start_npc_turn)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::NPCTurn)
                .with_system(npc::move_npc)
        );

        app.add_system_set(
            SystemSet::on_enter(AnimationState::Idle)
                .with_system(player::tick)
                .with_system(npc::tick)
        );
        app.insert_resource(npc::NPCQueue {
            npcs: VecDeque::new(),
            current: None
        });
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnitKind {
    Player,
    Ram,
    Hen,
    Frog,
    Bear,
    Stork,
    Fox,
    Wolf
}

pub enum UnitState {
    Active,
    Paused
}

#[derive(Component)]
pub struct Unit {
    pub ap: u8,
    pub behaviour: behaviour::Behaviour,
    pub kind: UnitKind,
    pub state: UnitState
}

impl Unit {
    pub fn handle_move_end(
            &mut self
        ) {   
        self.ap = self.ap.saturating_sub(1);
    }
    
    pub fn handle_turn_start(&mut self) {
        match self.state {
            UnitState::Active => self.ap = BASE_AP,
            UnitState::Paused => {
                self.state = UnitState::Active;
                self.ap = 0
            }
        };
    }
}

fn spawn_units(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    board_query: Query<&Board>,
    blocker_query: Query<&Position, With<Blocker>>,
    mut player_data: ResMut<player::PlayerData>
) {
    player_data.level += 1;

    let board = match board_query.get_single() {
        Ok(b) => b,
        Err(_) => return
    };
    let mut blocker_positions: Vec<Vector2Int> = blocker_query.iter()
        .map(|a| a.v)
        .collect();

    match player::spawn_player(&mut commands, &board) {
        Some(v) => blocker_positions.push(v),
        None => ()
    };

    npc::spawn_npcs(&mut commands, &mut blocker_positions, &board, (player_data.level as f32).powf(1.5) as u32);
    game_state.set(GameState::PlayerTurn);
}

fn check_unit_interaction(
    entity: Entity,
    position: &Position,
    unit_position: &Query<(Entity, &Position), With<Unit>>
) -> Option<Entity> {
    for (other_entity, other_position) in unit_position.iter() {
        if other_entity == entity { continue; }
        if position.v== other_position.v {
            return Some(other_entity)
        }
    }
    None
}

fn clear_units(
    mut commands: Commands,
    query: Query<Entity, With<Unit>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}