use rand::prelude::SliceRandom;

use super::UnitKind;
use super::data;

pub fn get_npc_set(target_sum: u32) -> Vec<UnitKind> {
    let npc_kinds = data::get_npc_types();
    let mut output = Vec::new();
    let mut sum = 0;

    while sum < target_sum {
        let kind = npc_kinds.choose(&mut rand::thread_rng()).unwrap();
        let rank = data::get_unit_rank(kind);
        if rank <= target_sum - sum {
            sum += rank;
            output.push(*kind);
        }
    }
    output
}