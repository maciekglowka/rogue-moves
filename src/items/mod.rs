use bevy::prelude::*;

use crate::board::{
    Blocker, Board, Position,
    utils::get_spawn_position
};
use crate::command::{CommandEvent, CommandType};
use crate::units::{
    player::{Player, PlayerData},
    Unit
};
use crate::ui::RedrawUIEvent;
use crate::states::GameState;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UseItemEvent>();
        app.add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(clear_items)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::MapGenerate)
                .with_system(spawn_items)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(clear_items)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(use_item)
        );
    }
}

#[derive(Clone, Component)]
pub struct Item;

fn spawn_items(
    mut commands: Commands,
    board_query: Query<&Board>,
    blocker_query: Query<&Position, With<Blocker>>
) {
    let board = board_query.get_single().unwrap();
    let blocker_positions = blocker_query.iter()
        .map(|a| a.v)
        .collect();
    if let Some(position) = get_spawn_position(&blocker_positions, &board) {
        commands.spawn()
            .insert(Position { v: position })
            .insert(Item);
    }
}

fn clear_items(
    mut commands: Commands,
    query: Query<Entity, With<Item>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct UseItemEvent(pub usize);

pub fn use_item(
    mut ev_use_item: EventReader<UseItemEvent>,
    mut ev_ui: EventWriter<RedrawUIEvent>,
    mut ev_command: EventWriter<CommandEvent>,
    mut player_data: ResMut<PlayerData>,
    player_query: Query<Entity, With<Player>>
) {
    for ev in ev_use_item.iter() {
        if let Ok(entity) = player_query.get_single() {
            player_data.items.remove(ev.0);
            ev_command.send(CommandEvent(CommandType::AddAP(entity, 1)));
            ev_ui.send(RedrawUIEvent);
        }
    }
}