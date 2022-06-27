use crate::board::{
    Blocker,
    Board,
    Position
};
use crate::vectors::{DIAGONAL_DIRECTIONS, ORTHO_DIRECTIONS, Vector2Int};
use super::action::{ActionType, ActionValidator, get_validator};

#[derive(Clone)]
pub struct Behaviour {
    pub pattern: Vec::<Vector2Int>,
    pub action_type: ActionType
}

impl Behaviour {
    pub fn possible_positions(
        &self,
        source: Vector2Int,
        board: &Board,
        blockers: &Vec<(&Position, &Blocker)>
    ) -> Vec::<Vector2Int> {
        let mut positions = Vec::new();
        let validator = get_validator(&self.action_type);
        for v in &self.pattern {
            let p = source + *v;
            if !board.tiles.contains_key(&p) { continue; }

            if let Some(blocker) = blockers.iter().find(|a| a.0.v == p) {
                if !blocker.1.is_targetable { continue; }
            }

            if validator.is_valid(source, p, blockers) { positions.push(p); }
        }
        positions
    }
}

pub fn get_ortho_pattern(range: u8) -> Vec::<Vector2Int> {
    ranged_positions(&ORTHO_DIRECTIONS, range)
}

pub fn get_diagonal_pattern(range: u8) -> Vec::<Vector2Int> {
    ranged_positions(&DIAGONAL_DIRECTIONS, range)
}

pub fn get_omni_pattern(range: u8) -> Vec::<Vector2Int> {
    let mut dirs = DIAGONAL_DIRECTIONS.to_vec();
    dirs.extend(ORTHO_DIRECTIONS);
    ranged_positions(&dirs, range)
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