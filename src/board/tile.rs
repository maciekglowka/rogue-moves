use bevy::prelude::*;

pub enum TileKind {
    Floor,
    Wall
}

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind
}