use bevy::{
    prelude::*,
    render,
    sprite
};

use crate::board;
use crate::states::{FadeState, GameState};

use super::OVERLAY_Z;
use super::board_renderer::MASK_RANGE;

const FADE_RATE: f32 = 5.;

#[derive(Component)]
pub struct Overlay {
    pub counter: f32
}

pub fn draw_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let min_v = 0.;
    let max_v = (board::SIZE + MASK_RANGE) as f32;

    let verts = vec!(
        [min_v, min_v, 0.],
        [min_v, max_v, 0.],
        [max_v, max_v, 0.],
        [max_v, min_v, 0.],
    );

    let normals = vec!([0.0, 1.0, 0.0]; 4);
    let uvs = vec!([0., 0.], [0., 1.], [1., 1.], [1., 0.]);
    let tris = vec!(0, 2, 1, 0, 3, 2);

    let mut mesh = Mesh::new(render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(render::mesh::Indices::U32(tris)));
    
    let offset = - 0.5 * super::TILE_SIZE * MASK_RANGE as f32;

    commands
        .spawn_bundle(sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(mesh)),
            material: materials.add(
                ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })
            ),
            transform: Transform::default()
                .with_scale(Vec3::new(super::TILE_SIZE, super::TILE_SIZE, 1.0))
                .with_translation(Vec3::new(offset, offset, OVERLAY_Z)),
            ..Default::default()
        })
        .insert(Overlay { counter: 0. });
}

pub fn clear_overlay(
    mut commands: Commands,
    query: Query<Entity, With<Overlay>>
) {
    if let Ok(entity) = query.get_single() {
        commands
            .entity(entity)
            .despawn_recursive();
    }
}

pub fn fade_overlay_in(
    mut query: Query<(&Handle<ColorMaterial>, &mut Overlay)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fade_state: ResMut<State<FadeState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Ok((handle, mut overlay)) = query.get_single_mut() {
        if let Some(mut material) = materials.get_mut(handle) {
            overlay.counter += FADE_RATE * time.delta_seconds();
            material.color.set_a(ease(overlay.counter));
            if overlay.counter >= 1. { 
                fade_state.set(FadeState::Out);
                match game_state.current() {
                    GameState::PlayerTurn => game_state.set(GameState::MapGenerate).unwrap(),
                    GameState::MainMenu => game_state.set(GameState::MapGenerate).unwrap(),
                    _ => ()
                }
            }
        }
    }
}

pub fn fade_overlay_out(
    mut query: Query<(&Handle<ColorMaterial>, &mut Overlay)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fade_state: ResMut<State<FadeState>>
) {
    if let Ok((handle, mut overlay)) = query.get_single_mut() {
        if let Some(mut material) = materials.get_mut(handle) {
            overlay.counter -= FADE_RATE * time.delta_seconds();
            material.color.set_a(ease(overlay.counter));

            if overlay.counter <= 0. { fade_state.set(FadeState::Hidden); }
        }
    }
}

fn ease(t: f32) -> f32 {
    t * t * (3. - 2. * t)
}