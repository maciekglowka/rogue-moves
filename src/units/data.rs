// use super::action::ActionType;
use super::action::{walk_validator, jump_validator};
use super::behaviour::{Behaviour, get_omni_pattern, get_ortho_pattern, get_knight_pattern};
use super::{UnitKind, Unit};

pub fn get_unit_behaviour(kind: &UnitKind) -> Behaviour {
    match kind {
        UnitKind::Player => {
            Behaviour {
                pattern: get_omni_pattern(1),
                // action_type: ActionType::Walk,
                validator: walk_validator
            }
        }
        UnitKind::Goblin => {
            Behaviour {
                pattern: get_ortho_pattern(2),
                // action_type: ActionType::Walk
                validator: walk_validator
            }
        },
        UnitKind::Rat => {
            Behaviour {
                pattern: get_omni_pattern(1),
                // action_type: ActionType::Walk
                validator: walk_validator
            }
        },
        UnitKind::Cat => {
            Behaviour {
                pattern: get_ortho_pattern(16),
                // action_type: ActionType::Walk
                validator: walk_validator
            }
        },
        UnitKind::Knight => {
            Behaviour {
                pattern: get_knight_pattern(),
                // action_type: ActionType::Jump
                validator: jump_validator
            }
        },
        UnitKind::Puma => {
            Behaviour {
                pattern: get_omni_pattern(16),
                // action_type: ActionType::Walk
                validator: walk_validator
            }
        }
    }
}

pub fn get_unit_rank(kind: &UnitKind) -> u32 {
    match kind {
        UnitKind::Goblin => 2,
        UnitKind::Rat => 2,
        UnitKind::Cat => 3,
        UnitKind::Knight => 4,
        UnitKind::Puma => 5,
        UnitKind::Player => 0
    }
}

pub fn get_npc_types() -> Vec<UnitKind> {
    vec![
        UnitKind::Goblin,
        UnitKind::Rat,
        UnitKind::Cat,
        UnitKind::Puma,
        UnitKind::Knight
    ]
}