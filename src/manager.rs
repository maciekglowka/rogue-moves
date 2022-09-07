use bevy::prelude::*;

use crate::states::{FadeState, GameState};
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
    mut fade_state: ResMut<State<FadeState>>,
    npc_query: Query<&NPC>
) {
    if npc_query.is_empty() {
        // game_state.set(GameState::MapGenerate);
        fade_state.set(FadeState::In);
    }
}