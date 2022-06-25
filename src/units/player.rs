use bevy::prelude::*;

use crate::board::{Board, Position};
use crate::ui;
use crate::states::{AnimationState, GameState};
use crate::vectors::Vector2Int;

use super::action::ActionType;
use super::{Unit, behaviour};
use super::behaviour::{Behaviour, get_ortho_pattern};

#[derive(Component)]
pub struct Player;

pub struct PlayerData {
    pub current_behaviour: behaviour::Behaviour
}

pub struct MovePlayerEvent(pub Vector2Int);

pub fn start_player_turn(
    player_data: Res<PlayerData>,
    mut ev_ui: EventWriter<ui::cursor::DrawCursorEvent>,
    mut player_query: Query<(&mut Unit, &Position), With<Player>>,
    board_query: Query<&Board>
) {
    if let Ok((mut unit, position)) = player_query.get_single_mut() {
        unit.ap = 2;

        let board = board_query.get_single().unwrap();

        let range = player_data.current_behaviour.possible_positions(
            position.v, 
            board
        );
        ev_ui.send(ui::cursor::DrawCursorEvent(range));
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
    mut game_state: ResMut<State<GameState>>
) {
    if game_state.current() != &GameState::PlayerTurn { return; }
    game_state.set(GameState::NPCTurn);
}

pub fn spawn_player(
    mut commands: &mut Commands,
) {
    let behaviour = get_base_player_behaviour();

    commands.spawn()
        .insert(Position { v: Vector2Int::new(0, 0) })
        .insert(Player)
        .insert(Unit { 
            ap: 2,
            behaviour: behaviour
        });
}

pub fn get_base_player_behaviour() -> Behaviour {
    let pattern = get_ortho_pattern(2);
    Behaviour {
        pattern: pattern,
        action_type: ActionType::Walk
    }
}