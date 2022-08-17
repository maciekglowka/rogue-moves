use bevy::prelude::*;

use crate::board::{
    Blocker, Board, Position,
    utils::get_spawn_position,
    tile::Tile
};
use crate::items::Item;
use crate::ui;
use crate::states::{AnimationState, GameState};
use crate::vectors::Vector2Int;

use super::behaviour::Behaviour;
use super::data::get_unit_behaviour;
use super::{Unit, UnitKind};

#[derive(Component)]
pub struct Player;

pub struct PlayerData {
    pub current_behaviour: Behaviour,
    // pub captured_behaviour: Option<Behaviour>,
    pub level: u32,
    pub items: Vec<Item>
}

pub fn reset_player_data(
    mut commands: Commands
) {
    commands.insert_resource(PlayerData {
        current_behaviour: super::data::get_unit_behaviour(&UnitKind::Player),
        // captured_behaviour: None,
        level: 0,
        items: Vec::new()
    });
}

pub struct MovePlayerEvent(pub Vector2Int);

pub fn start_player_turn(
    mut ev_ui: EventWriter<ui::cursor::DrawCursorEvent>,
    mut player_query: Query<(Entity, &mut Unit), With<Player>>,
    mut player_data: ResMut<PlayerData>,
) {
    if let Ok((_entity, mut unit)) = player_query.get_single_mut() {
        unit.handle_turn_start();

        player_data.current_behaviour = super::data::get_unit_behaviour(&UnitKind::Player);
        if unit.ap > 0 {ev_ui.send(ui::cursor::DrawCursorEvent);}
    }
}

pub fn player_status(
    mut game_state: ResMut<State<GameState>>,
    player_query: Query<&Unit, With<Player>>
) {
    if let Ok(unit) = player_query.get_single() {
        if unit.ap == 0 {
            game_state.set(GameState::NPCTurn);
        }
    } else {
        game_state.set(GameState::GameOver);
    }
}

pub fn move_player(
    mut ev: EventReader<MovePlayerEvent>,
    mut query: Query<(Entity, &mut Position), With<Player>>,
    mut animation_state: ResMut<State<AnimationState>>,
    // mut game_state: ResMut<State<GameState>>,
    blocker_query: Query<(&Position, &Blocker), Without<Player>>,
    board_query: Query<&Board>,
    player_data: Res<PlayerData>,
) {
    if animation_state.current() == &AnimationState::Animating { return ; }
    for ev in ev.iter() {
        if let Ok((_, mut position)) = query.get_single_mut() {
            let board = board_query.get_single().unwrap();
            
            let mut range = player_data.current_behaviour.possible_positions(
                position.v, 
                board,
                &blocker_query.iter().collect()
            );

            // range.push(position.v);

            if !range.contains(&ev.0) {
                continue;
            }

            position.v = ev.0;
            animation_state.set(AnimationState::Animating);
        }
    }
}

pub fn tick(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut player_data: ResMut<PlayerData>,
    mut player_query: Query<(Entity, &mut Unit), With<Player>>,
    unit_position: Query<(Entity, &Position), With<Unit>>,
    mut unit_query: Query<&mut Unit, Without<Player>>,
    mut item_query: Query<(Entity, &Item, &Position)>,
    mut ev_ui: EventWriter<ui::cursor::DrawCursorEvent>,
    tile_query: Query<(&Position, &Tile)>
) {
    if game_state.current() != &GameState::PlayerTurn { return; }

    let (entity, mut unit) = player_query.get_single_mut().unwrap();
    let position = unit_position.get(entity).unwrap().1;

    match super::check_unit_interaction(entity, position, &unit_position) {
        Some(killed) => {
            let killed_unit = unit_query.get(killed).unwrap();                
            player_data.current_behaviour = killed_unit.behaviour.clone();
            unit.ap += 1;
            commands.entity(killed).despawn_recursive();
        },
        None => {}
    };   
    
    try_pick_item(
        &mut commands,
        position,
        &mut item_query,
        &mut player_data
    );

    unit.handle_turn_end(
        &tile_query,
        &position
    );

    // if turn_end {
    //     game_state.set(GameState::NPCTurn);
    // } else {
    //     ev_ui.send(ui::cursor::DrawCursorEvent);
    // }
    if unit.ap > 0 {ev_ui.send(ui::cursor::DrawCursorEvent); }
}

pub fn spawn_player(
    mut commands: &mut Commands,
    board: &Board,
    blocker_positions: &Vec<Vector2Int>
) -> Option<Vector2Int> {
    let behaviour = get_unit_behaviour(&UnitKind::Player);
    // let position = get_spawn_position(blocker_positions, board);
    let position = board.stair_v;
    // if position.is_some() {
    commands.spawn()
        .insert(Position { v: position })
        .insert(Player)
        .insert(Blocker { is_targetable: true })
        .insert(Unit { 
            ap: super::BASE_AP,
            behaviour: behaviour,
            kind: super::UnitKind::Player,
            state: super::UnitState::Active
        });
        // }

    Some(position)
}

fn try_pick_item(
    mut commands: &mut Commands,
    player_position: &Position,
    mut item_query: &mut Query<(Entity, &Item, &Position)>,
    mut player_data: &mut ResMut<PlayerData>,
) {
    for (entity, item, position) in item_query.iter() {
        if position.v != player_position.v { continue; }

        commands.entity(entity).despawn_recursive();
        player_data.items.push(item.clone());
    }
}
