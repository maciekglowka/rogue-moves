use bevy::{
    prelude::*,
    render,
    sprite
};

use crate::board::{Board, Position, tile::Tile, tile::TileKind};
use super::{MAP_Z, TILE_SIZE};
use super::utils::QuadMesh;

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

    commands
        .entity(board.0)
        .insert(BoardRenderer)
        .insert_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(base_quad.to_mesh())),
            material: assets.material.clone(),
            transform: Transform::default().with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.)),
        ..default()});
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
