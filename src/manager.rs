use bevy::prelude::*;

use crate::states::GameState;
use crate::units::npc::NPC;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(next_level)
        );
    }
}

fn next_level(
    mut game_state: ResMut<State<GameState>>,
    npc_query: Query<&NPC>
) {
    if npc_query.is_empty() {
        game_state.set(GameState::MapGenerate);
    }
}