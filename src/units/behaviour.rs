use bevy::prelude::*;

use crate::board::{
    Board,
    Position,
    tile::Tile
};
use crate::vectors::{DIAGONAL_DIRECTIONS, ORTHO_DIRECTIONS, Vector2Int};
use super::action::{Action, ActionType};

#[derive(Clone)]
pub struct Behaviour {
    pub pattern: Vec::<Vector2Int>,
    pub action_type: ActionType
}

impl Behaviour {
    pub fn possible_positions(
        &self,
        source: Vector2Int,
        // tile_query: &Query<&Position, With<Tile>>
        board: &Board
    ) -> Vec::<Vector2Int> {
        let mut positions = Vec::new();
        for v in &self.pattern {
            let p = source + *v;
            if !board.tiles.contains_key(&p) { continue; }

            positions.push(p);
        }
        positions
    }
}

pub fn get_ortho_pattern(range: u8) -> Vec::<Vector2Int> {
    return ranged_positions(&ORTHO_DIRECTIONS, range)
}

fn ranged_positions<'a, T> (directions: &'a T, range: u8) -> Vec<Vector2Int> 
where &'a T: std::iter::IntoIterator<Item=&'a Vector2Int> {
    let mut positions = Vec::new();
    for idx in 1..=range {
        for dir in directions {
            positions.push(
                *dir * idx as i32
            )
        }
    }
    positions
}