use rand::Rng;

use super::ItemKind;

pub fn is_passive(kind: ItemKind) -> bool {
    match kind {
        ItemKind::Armor => true,
        ItemKind::SpeedMushroom => false,
        ItemKind::StopMushroom => false
    }
}

pub fn get_random_kind() -> ItemKind {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0.0..1.0) {
        a if a < 0.1 => ItemKind::Armor,
        a if a < 0.45 => ItemKind::StopMushroom,
        _ => ItemKind::SpeedMushroom
    }
}