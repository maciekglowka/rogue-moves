use bevy::prelude::*;
use bevy::{render, sprite};

use crate::graphics::{TILE_SIZE, OVERLAY_Z};
use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct Cursor;

pub struct CursorAssets {
    material: Handle<ColorMaterial>
}

pub struct DrawCursorEvent(pub Vec::<Vector2Int>);

pub fn draw_cursor(
    mut commands: Commands,
    mut ev_draw_cursor: EventReader<DrawCursorEvent>,
    cursor_query: Query<Entity, With<Cursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<CursorAssets>
) {
    for ev in ev_draw_cursor.iter() {
        destroy_cursor(&mut commands, &cursor_query);

        let mesh = create_cursor_mesh(&ev.0);

        commands.spawn_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(mesh)),
            material: assets.material.clone(),
            transform: Transform::default().with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 0.0)),
            ..Default::default()
        })
        .insert(Cursor);
    }
}

pub fn clear_cursor(
    mut commands: Commands,
    query: Query<Entity, With<Cursor>>
) {
    destroy_cursor(&mut commands, &query)
}

fn destroy_cursor(
    commands: &mut Commands,
    query: &Query<Entity, With<Cursor>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_list: ResMut<crate::assets::AssetList>    
) {
    let texture_handle = asset_server.load("cursor.png");
    asset_list.0.push(texture_handle.clone_untyped());

    let material_handle = materials.add(
        ColorMaterial{ color: Color::WHITE, texture: Some(texture_handle)}
    );

    commands.insert_resource(
        CursorAssets { material: material_handle }
    );
}

fn create_cursor_mesh(
    positions: &Vec<Vector2Int>
) -> Mesh {
    let mut verts = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut tris = Vec::new();

    let mut idx = 0;

    for position in positions.iter() {
        verts.push([position.x as f32, position.y as f32, OVERLAY_Z]);
        verts.push([position.x as f32, position.y as f32 + 1.0, OVERLAY_Z]);
        verts.push([position.x as f32 + 1.0, position.y as f32 + 1.0, OVERLAY_Z]);
        verts.push([position.x as f32 + 1.0, position.y as f32, OVERLAY_Z]);

        for _ in 0..4 {
            normals.push([0.0, 1.0, 0.0]);
        }

        uvs.extend(vec!(
            [0.0,0.0], [0.0,1.0], [1.0,1.0], [1.0,0.0]
        ));
        tris.extend([idx, idx + 2, idx + 1, idx, idx + 3, idx +2]);
        idx += 4;
    }

    let mut mesh = Mesh::new(render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(render::mesh::Indices::U32(tris)));
    mesh
}