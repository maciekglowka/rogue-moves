use super::action::{walk_validator, jump_validator};
use super::behaviour::{
    Behaviour, get_omni_pattern, get_ortho_pattern, get_knight_pattern, get_turtle_pattern, get_frog_pattern,
    get_cat_pattern, get_puma_pattern, get_rat_pattern, get_player_pattern
};
use super::{UnitKind, Unit};

pub fn get_unit_behaviour(kind: &UnitKind) -> Behaviour {
    match kind {
        UnitKind::Player => {
            Behaviour {
                pattern: get_player_pattern(),
                validator: walk_validator
            }
        },
        UnitKind::Turtle => {
            Behaviour {
                pattern: get_turtle_pattern(),
                validator: walk_validator
            }
        },
        // UnitKind::Frog => {
        //     Behaviour {
        //         pattern: get_frog_pattern(),
        //         validator: jump_validator
        //     }
        // },
        UnitKind::Goblin => {
            Behaviour {
                pattern: get_ortho_pattern(2),
                validator: walk_validator
            }
        },
        UnitKind::Rat => {
            Behaviour {
                pattern: get_ortho_pattern(1),
                validator: walk_validator
            }
        },
        // UnitKind::Cat => {
        //     Behaviour {
        //         pattern: get_cat_pattern(),
        //         validator: jump_validator
        //     }
        // },
        UnitKind::Knight => {
            Behaviour {
                pattern: get_knight_pattern(),
                validator: jump_validator
            }
        },
        // UnitKind::Puma => {
        //     Behaviour {
        //         pattern: get_puma_pattern(),
        //         validator: jump_validator
        //     }
        // }
    }
}

pub fn get_unit_rank(kind: &UnitKind) -> u32 {
    match kind {
        UnitKind::Turtle => 2,
        // UnitKind::Frog => 2,
        UnitKind::Goblin => 3,
        UnitKind::Rat => 1,
        // UnitKind::Cat => 3,
        UnitKind::Knight => 3,
        // UnitKind::Puma => 3,
        UnitKind::Player => 0
    }
}

pub fn get_npc_types() -> Vec<UnitKind> {
    vec![
        UnitKind::Turtle,
        // UnitKind::Frog,
        UnitKind::Goblin,
        UnitKind::Rat,
        // UnitKind::Cat,
        // UnitKind::Puma,
        UnitKind::Knight
    ]
}