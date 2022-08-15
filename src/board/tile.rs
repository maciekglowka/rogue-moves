use bevy::prelude::*;

pub enum TileKind {
    Floor,
    Wall,
    Stair,
    Bush
}

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind
}