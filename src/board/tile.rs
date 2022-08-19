use bevy::prelude::*;

use crate::command::{CommandEvent, CommandType};
use crate::units::Unit;

use super::Position;

pub struct TileInteractionEvent(pub Entity);

pub enum TileKind {
    Floor,
    Wall,
    Stair,
    Bush
}

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind
}

pub fn tile_interaction(
    mut ev_interaction: EventReader<TileInteractionEvent>,
    mut ev_command: EventWriter<CommandEvent>,
    mut unit_query: Query<(&mut Unit, &Position)>,
    tile_query: Query<(&Tile, &Position)>
) {
    for ev in ev_interaction.iter() {
        if let Ok((mut _unit, position)) = unit_query.get_mut(ev.0) {
            let (tile, _) = match tile_query.iter()
                .filter(|(_, p)| p.v == position.v)
                .next() {
                    Some(t) => t,
                    _ => return
                };
            
            match tile.kind {
                TileKind::Bush => {
                    ev_command.send(CommandEvent(CommandType::PauseUnit(ev.0)));
                },
                _ => ()
            }
        }
    }
}