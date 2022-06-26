use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use crate::vectors::Vector2Int;
use crate::states::{GameState, SetupLabel};

pub mod tile;

const SIZE: u8 = 8;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int
}

#[derive(Component)]
pub struct Blocker;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(generate_board)
                .label(SetupLabel::Board)
        );
    }
}

#[derive(Component)]
pub struct Board {
    pub tiles: HashMap<Vector2Int, Entity>
}

pub fn generate_board(
    mut commands: Commands
) {
    let mut rng = rand::thread_rng();
    let mut tiles = HashMap::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            let v = Vector2Int::new(x as i32, y as i32);

            let kind = match rng.gen_range(0.0..1.0) {
                a if a < 0.1 => tile::TileKind::Wall,
                _ => tile::TileKind::Floor
            };

            let is_blocker = match kind {
                tile::TileKind::Wall => true,
                _ => false
            };

            let tile = commands.spawn()
                .insert(tile::Tile{kind: kind})
                .insert(Position { v: v })
                .id();

            if is_blocker {
             commands.entity(tile)
                .insert(Blocker);
            };

            tiles.insert(v, tile);
        }
    }

    let tile_vec: Vec<Entity> = tiles.to_owned().into_values().collect();
    commands.spawn()
        .insert(Board {tiles})
        .push_children(&tile_vec);
}