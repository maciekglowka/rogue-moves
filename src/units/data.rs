use super::action::ActionType;
use super::behaviour::{Behaviour, get_omni_pattern, get_ortho_pattern};
use super::{UnitKind, Unit};

pub fn get_unit_behaviour(kind: &UnitKind) -> Behaviour {
    match kind {
        UnitKind::Player => {
            Behaviour {
                pattern: get_omni_pattern(1),
                action_type: ActionType::Walk
            }
        }
        UnitKind::Goblin => {
            Behaviour {
                pattern: get_ortho_pattern(2),
                action_type: ActionType::Walk
            }
        },
        UnitKind::Rat => {
            Behaviour {
                pattern: get_omni_pattern(1),
                action_type: ActionType::Walk
            }
        },
        UnitKind::Cat => {
            Behaviour {
                pattern: get_ortho_pattern(16),
                action_type: ActionType::Walk
            }
        }
    }
}

pub fn get_unit_rank(kind: &UnitKind) -> u32 {
    match kind {
        UnitKind::Goblin => 2,
        UnitKind::Rat => 1,
        UnitKind::Cat => 3,
        _ => 0
    }
}

pub fn get_npc_types() -> Vec<UnitKind> {
    vec![
        UnitKind::Goblin,
        UnitKind::Rat,
        UnitKind::Cat
    ]
}