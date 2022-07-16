use rand::prelude::SliceRandom;

use crate::vectors::Vector2Int;

use super::Board;

pub fn get_spawn_position(
    blocker_positions: &Vec<Vector2Int>,
    board: &Board,
) -> Option<Vector2Int> {
    let positions = get_possible_spawn_positions(blocker_positions, board);
    match positions.choose(&mut rand::thread_rng()) {
        Some(v) => Some(*v),
        None => None
    }
}

fn get_possible_spawn_positions(
    blocker_positions: &Vec<Vector2Int>,
    board: &Board
) -> Vec<Vector2Int> {

    board.tiles.keys()
        .filter(|v|
            !blocker_positions
                .iter()
                .any(|a| a == *v)
        )
        .map(|v| *v)
        .collect()
}