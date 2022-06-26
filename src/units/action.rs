use crate::vectors::{Vector2Int, vector_line};
use crate::board::{Blocker, Position};

#[derive(Clone)]
pub enum ActionType {
    Walk
}

pub fn get_validator(
    action_type: &ActionType
) -> impl ActionValidator {
    match action_type {
        ActionType::Walk => WalkValidator
    }
}

pub trait ActionValidator {
    fn is_valid(
        &self,
        source: Vector2Int,
        target: Vector2Int,
        blockers: &Vec<(&Position, &Blocker)>
    ) -> bool;
}

pub struct WalkValidator;

impl ActionValidator for WalkValidator {
    fn is_valid(
        &self,
        source: Vector2Int,
        target: Vector2Int,
        blockers: &Vec<(&Position, &Blocker)>
    ) -> bool {
        !has_line_blockers(source, target, blockers)
    }
}

fn has_line_blockers(
    a: Vector2Int,
    b: Vector2Int,
    blockers: &Vec<(&Position, &Blocker)>
) -> bool {
    let line = vector_line(a, b);
    if line.len() < 2 { return false; }
    for idx in 1..line.len() - 1 {
        if blockers.iter().find(|a| a.0.v == line[idx]).is_some() { return true; }
    }

    return false
}