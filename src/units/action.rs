use crate::vectors::Vector2Int;

#[derive(Clone)]
pub enum ActionType {
    Walk
}

pub fn action_by_type(
    action_type: ActionType,
    source: Vector2Int,
    target: Vector2Int
) -> impl Action {
    match action_type {
        ActionType::Walk => Walk::new(source, target)
    }
}

pub trait Action {
    fn new(source: Vector2Int, target: Vector2Int) -> Self;
    fn is_valid() -> bool;
}

pub struct Walk {}

impl Action for Walk {
    fn new(source: Vector2Int, target: Vector2Int) -> Walk {
        Walk {}
    }
    fn is_valid() -> bool {
        true
    }
}