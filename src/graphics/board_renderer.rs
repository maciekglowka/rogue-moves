use bevy::{
    prelude::*,
    sprite
};

use crate::board;
use crate::board::{Board, Position, tile::Tile, tile::TileKind};
use crate::vectors::Vector2Int;
use super::{MAP_Z, MASK_Z, TILE_SIZE};
use super::utils::QuadMesh;

pub const MASK_RANGE: u8 = 8;

#[derive(Component)]
pub struct BoardRenderer;

pub fn draw_board(
    mut commands: Commands,
    board_query: Query<(Entity, &Board)>,
    tile_query: Query<(&Position, &Tile)>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<BoardRendererAssets>
) {
    let board = match board_query.get_single() {
        Ok(b) => b,
        Err(_) => return
    };

    let mut base_quad = QuadMesh::new(MAP_Z);

    for (position, tile) in tile_query.iter() {
        let base_uv = ((position.v.x as u8 + position.v.y as u8) % 2, 0);
        base_quad.add_quad(position, base_uv);

        let feature_uv = match tile.kind {
            TileKind::Floor => continue,
            TileKind::Wall => (0, 1),
            TileKind::Stair => (1, 1),
            TileKind::Bush => (2, 1),
        };
        base_quad.add_quad(position, feature_uv);
    }

    let mask = draw_mask();

    commands
        .entity(board.0)
        .insert(BoardRenderer)
        .insert_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(base_quad.to_mesh())),
            material: assets.material.clone(),
            global_transform: GlobalTransform::default(),
            transform: Transform::default().with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(sprite::MaterialMesh2dBundle {
                mesh: sprite::Mesh2dHandle(meshes.add(mask.to_mesh())),
                material: assets.material.clone(),
                transform: Transform::default(),
                ..default()
            });
        });
}

fn draw_mask() -> QuadMesh {
    let mut mask_quad = QuadMesh::new(MASK_Z);

    let mask_min = - (MASK_RANGE as i32);
    let mask_max = (board::SIZE + MASK_RANGE) as i32;

    for x in mask_min..mask_max {
        for y in mask_min..mask_max {
            if 0 <= x && x < board::SIZE as i32
                && 0 <= y && y < board::SIZE as i32 { continue; }
            let p = Position { v: Vector2Int::new(x, y)};

            let u = match (x, y) {
                (x, y) if y == -1 && x >= 0 && x < board::SIZE as i32 => 1,
                (x, y) if y == board::SIZE as i32 && x >= 0 && x < board::SIZE as i32 => 3,
                (x, y) if x == -1 && y >= 0 && y < board::SIZE as i32 => 2,
                (x, y) if x == board::SIZE as i32 && y >= 0 && y < board::SIZE as i32 => 4,
                _ => 0
            };
            let uv = (u, 2);
            mask_quad.add_quad(&p, uv);
        }
    }
    mask_quad
}

pub struct BoardRendererAssets {
    material: Handle<ColorMaterial>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_list: ResMut<crate::assets::AssetList>    
) {
    let texture_handle = asset_server.load("board.png");
    asset_list.0.push(texture_handle.clone_untyped());

    let material_handle = materials.add(
        ColorMaterial{ color: Color::WHITE, texture: Some(texture_handle)}
    );

    commands.insert_resource(
        BoardRendererAssets { material: material_handle }
    );
}
