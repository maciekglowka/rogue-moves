use super::action::{walk_validator, jump_validator};
use super::behaviour::{
    Behaviour, get_omni_pattern, get_ortho_pattern, get_knight_pattern, get_ram_pattern,
    get_player_pattern, get_diagonal_pattern
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
        UnitKind::Ram => {
            Behaviour {
                pattern: get_ram_pattern(),
                validator: walk_validator
            }
        },
        UnitKind::Frog => {
            Behaviour {
                pattern: get_diagonal_pattern(1),
                validator: jump_validator
            }
        },
        UnitKind::Bear => {
            Behaviour {
                pattern: get_ortho_pattern(2),
                validator: walk_validator
            }
        },
        UnitKind::Hen => {
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
        UnitKind::Stork => {
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
        UnitKind::Ram => 2,
        UnitKind::Frog => 2,
        UnitKind::Bear => 3,
        UnitKind::Hen => 1,
        // UnitKind::Cat => 3,
        UnitKind::Stork => 3,
        // UnitKind::Puma => 3,
        UnitKind::Player => 0
    }
}

pub fn get_npc_types() -> Vec<UnitKind> {
    vec![
        UnitKind::Ram,
        UnitKind::Frog,
        UnitKind::Bear,
        UnitKind::Hen,
        // UnitKind::Cat,
        // UnitKind::Puma,
        UnitKind::Stork
    ]
}