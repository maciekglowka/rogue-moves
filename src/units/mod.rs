use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::{AnimationState, GameState, SetupLabel};
use crate::board::Position;

mod action;
mod behaviour;
mod npc;
pub mod player;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(spawn_units)
                .label(SetupLabel::Units)
                .after(SetupLabel::Board)
        );

        app.add_event::<player::MovePlayerEvent>();

        app.add_system_set(
            SystemSet::on_enter(GameState::PlayerTurn)
                .with_system(player::start_player_turn)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(player::move_player)
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

        app.insert_resource(player::PlayerData {
            current_behaviour: player::get_base_player_behaviour()
        });
        app.insert_resource(npc::NPCQueue {
            npcs: VecDeque::new(),
            current: None
        });
    }
}


#[derive(Component)]
pub struct Unit {
    ap: u8,
    behaviour: behaviour::Behaviour
}

impl Unit {
    pub fn handle_turn_end(&mut self) -> bool {   
        self.ap -= 1;
        
        match self.ap {
            0 => true,
            _ => false
        }
    }
}

fn spawn_units(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>
) {
    println!("Spawning units");
    player::spawn_player(&mut commands);
    npc::spawn_npcs(&mut commands);
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