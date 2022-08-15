use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use crate::vectors::Vector2Int;
use crate::states::{GameState, SetupLabel};

pub mod tile;
pub mod utils;

pub const SIZE: u8 = 6;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int
}

#[derive(Component)]
pub struct Blocker {
    pub is_targetable: bool
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(clear_board)
                .label(SetupLabel::CleanUp)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::MapGenerate)
                .with_system(generate_board)
                .after(SetupLabel::CleanUp)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(clear_board)
        );
    }
}

#[derive(Component)]
pub struct Board {
    pub tiles: HashMap<Vector2Int, Entity>,
    pub stair_v: Vector2Int
}

pub fn generate_board(
    mut commands: Commands
) {
    let mut rng = rand::thread_rng();
    let mut tiles = HashMap::new();
    let stair_v = Vector2Int::new(rng.gen_range(0..SIZE) as i32, rng.gen_range(0..SIZE/2) as i32);

    for y in 0..SIZE {
        for x in 0..SIZE {
            let v = Vector2Int::new(x as i32, y as i32);

            let mut kind = match rng.gen_range(0.0..1.0) {
                a if a < 0.075 => {
                    match rng.gen_range(0.0..1.0) {
                        b if b < 0.5 => tile::TileKind::Wall,
                        _ => tile::TileKind::Bush
                    }
                },
                _ => tile::TileKind::Floor
            };

            if v == stair_v {
                kind = tile::TileKind::Stair;
            }

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
                .insert(Blocker { is_targetable: false });
            };

            tiles.insert(v, tile);
        }
    }

    let tile_vec: Vec<Entity> = tiles.to_owned().into_values().collect();
    commands.spawn()
        .insert(Board {tiles, stair_v})
        .push_children(&tile_vec);
}

fn clear_board(
    mut commands: Commands,
    query: Query<Entity, With<Board>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if game_state.current() == &GameState::MapGenerate {
        game_state.set(GameState::Spawning);
    }
}