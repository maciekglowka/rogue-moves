use bevy::prelude::*;
use bevy::{render, sprite};

use crate::board::{Blocker, Board, Position};
use crate::graphics::{TILE_SIZE, CURSOR_Z};
use crate::units::{
    Unit,
    player::{Player, PlayerData}
};
use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct Cursor;

pub struct CursorAssets {
    material: Handle<ColorMaterial>,
}

pub fn draw_cursor(
    mut commands: Commands,
    mut ev_draw_cursor: EventReader<super::RedrawUIEvent>,
    cursor_query: Query<Entity, With<Cursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<CursorAssets>,
    input_assets: Res<super::input::InputAssets>,
    blocker_query: Query<(&Position, &Blocker), Without<Unit>>,
    unit_query: Query<(&Unit, &Position, &Blocker)>,
    player_query: Query<Entity, With<Player>>,
    board_query: Query<&Board>,
    player_data: Res<PlayerData>,
) {
    for _ in ev_draw_cursor.iter() {
        destroy_cursor(&mut commands, &cursor_query);

        let player_entity = match player_query.get_single() {
            Ok(e) => e,
            _ => return
        };
        if let Ok((player_unit, _, _)) = unit_query.get(player_entity) {
            if player_unit.ap == 0 { return; }
        }

        let entity = match input_assets.selected_npc {
            Some(e) => e,
            None => {
                player_entity
            }
        };

        let behaviour = match input_assets.selected_npc {
            Some(e) => {
                if let Ok((u, _, _)) = unit_query.get(e) { &u.behaviour } else { return; }
            },
            None => {
                &player_data.current_behaviour
            }
        };

        let board = board_query.get_single().unwrap();

        let (_, position, _) = unit_query.get(entity).unwrap();
    
        let mut blockers:  Vec<(&Position, &Blocker)> = blocker_query.iter().collect();
        let unit_blockers: Vec<(&Position, &Blocker)> = unit_query.iter().map(|(_, p, b)| (p, b)).collect();
        blockers.extend(unit_blockers);
    
        let range = behaviour.possible_positions(
            position.v, 
            board,
            &blockers
        );
    
        let mesh = create_cursor_mesh(&range);
    
        commands.spawn_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(mesh)),
            material: assets.material.clone(),
            transform: Transform::default()
                .with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 0.0))
                .with_translation(Vec3::new(0., 0., CURSOR_Z)),
            ..Default::default()
        })
        .insert(Cursor);
    }
}

pub fn clear_cursor(
    mut commands: Commands,
    query: Query<Entity, With<Cursor>>,
) {
    destroy_cursor(&mut commands, &query);
}

fn destroy_cursor(
    commands: &mut Commands,
    query: &Query<Entity, With<Cursor>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
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

    let color = Color::Rgba { red: 0.84, green: 0.85, blue: 0.84, alpha: 1. };
    let material_handle = materials.add(
        ColorMaterial{ color: color, texture: Some(texture_handle)}
    );

    commands.insert_resource(
        CursorAssets { 
            material: material_handle,
        }
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
        verts.push([position.x as f32, position.y as f32, 0.]);
        verts.push([position.x as f32, position.y as f32 + 1.0, 0.]);
        verts.push([position.x as f32 + 1.0, position.y as f32 + 1.0, 0.]);
        verts.push([position.x as f32 + 1.0, position.y as f32, 0.]);

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