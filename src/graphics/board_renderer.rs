use bevy::{
    prelude::*,
    render,
    sprite
};

use crate::board::{Board, Position, tile::Tile, tile::TileKind};
use super::{MAP_Z, TILE_SIZE};

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
    let mut verts = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut tris = Vec::new();

    let mut idx = 0;

    for (position, tile) in tile_query.iter() {
        verts.push([position.v.x as f32, position.v.y as f32, MAP_Z]);
        verts.push([position.v.x as f32, position.v.y as f32 + 1.0, MAP_Z]);
        verts.push([position.v.x as f32 + 1.0, position.v.y as f32 + 1.0, MAP_Z]);
        verts.push([position.v.x as f32 + 1.0, position.v.y as f32, MAP_Z]);

        for _ in 0..4 {
            normals.push([0.0, 1.0, 0.0]);
        }

        let uv = match tile.kind {
            TileKind::Floor => (0, 0),
            TileKind::Wall => (1, 0),
            TileKind::Stair => (2, 0),
            TileKind::Bush => (3, 0),
        };

        uvs.extend(super::sprites::atlas_uvs(uv.0, uv.1, 4));
        tris.extend([idx, idx + 2, idx + 1, idx, idx + 3, idx +2]);
        idx += 4;
    }

    let mut mesh = Mesh::new(render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(render::mesh::Indices::U32(tris)));

    commands
        .entity(board.0)
        .insert(BoardRenderer)
        .insert_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(mesh)),
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