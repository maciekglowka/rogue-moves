use bevy::prelude::*;

use crate::units::{Unit, UnitKind};
use crate::board::Position;
use crate::states::AnimationState;

use super::{TILE_SIZE, UNIT_Z, UNIT_SPEED};

#[derive(Component)]
pub struct UnitRenderer;

pub fn animate_units(
    mut unit_query: Query<(&Position, &mut Transform)>,
    time: Res<Time>,
    mut animation_state: ResMut<State<AnimationState>>
) {
    let mut animating = false;
    for (position, mut transform) in unit_query.iter_mut() {
        let target = Vec3::new(
            (position.v.x as f32 + 0.5) * TILE_SIZE,
            (position.v.y as f32 + 0.5) * TILE_SIZE,
            UNIT_Z
        );
        if (target-transform.translation).length() > 0.1 {
            transform.translation = transform.translation.lerp(
                target,
                UNIT_SPEED * time.delta_seconds()
            );
            animating = true;
        }
    }

    if !animating {
        animation_state.set(AnimationState::Idle);
    }
}

pub fn draw_units(
    mut commands: Commands,
    unit_query: Query<(Entity, &Unit, &Position), Without<UnitRenderer>>,
    sprite_sheet: Res<UnitSprites>
) {
    for (entity, unit, position) in unit_query.iter() {
        let mut sprite = TextureAtlasSprite::new(
            get_sprite_idx(&unit.kind)
        );
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        commands.entity(entity)
            .insert(UnitRenderer)
            .insert_bundle(SpriteSheetBundle{
                sprite: sprite,
                texture_atlas: sprite_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        TILE_SIZE * (position.v.x as f32 + 0.5),
                        TILE_SIZE * (position.v.y as f32 + 0.5),
                        UNIT_Z
                    ),
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}

// pub fn camera_follow(
//     player_query: Query<&Transform, With<crate::units::player::Player>>,
//     mut camera_query: Query<&mut Transform, (Without<crate::units::player::Player>, With<crate::camera::MainCamera>)>
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         if let Ok(mut camera_transform) = camera_query.get_single_mut() {
//             camera_transform.translation.x = player_transform.translation.x;
//             camera_transform.translation.y = player_transform.translation.y;
//         }
//     }
// }

pub struct UnitSprites(pub Handle<TextureAtlas>);

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList> 
) {
    let image_handle = asset_server.load("units.png");
    asset_list.0.push(image_handle.clone_untyped());

    let atlas = TextureAtlas::from_grid(image_handle, Vec2::splat(16.0), 2, 8);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(UnitSprites(atlas_handle));
}

fn get_sprite_idx(kind: &UnitKind) -> usize {
    match kind {
        UnitKind::Player => 0,
        UnitKind::Ram => 2,
        UnitKind::Frog => 6,
        UnitKind::Hen => 4,
        UnitKind::Bear => 8,
        UnitKind::Stork => 10,
        UnitKind::Fox => 12,
        UnitKind::Wolf => 14
    }
}