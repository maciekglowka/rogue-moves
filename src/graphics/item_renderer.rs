use bevy::prelude::*;

use crate::items::{Item, ItemKind};
use crate::board::Position;

pub struct ItemSprites(pub Handle<TextureAtlas>);

use super::{TILE_SIZE, ITEM_Z};

#[derive(Component)]
pub struct ItemRenderer;

pub fn draw_items(
    mut commands: Commands,
    item_query: Query<(Entity, &Item, &Position), Without<ItemRenderer>>,
    sprite_sheet: Res<ItemSprites>
) {
    for (entity, item, position) in item_query.iter() {
        let mut sprite = TextureAtlasSprite::new(
            get_sprite_idx(&item.kind)
        );
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        commands.entity(entity)
            .insert(ItemRenderer)
            .insert_bundle(SpriteSheetBundle{
                sprite: sprite,
                texture_atlas: sprite_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        TILE_SIZE * (position.v.x as f32 + 0.5),
                        TILE_SIZE * (position.v.y as f32 + 0.5),
                        ITEM_Z
                    ),
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList> 
) {
    let image_handle = asset_server.load("items.png");
    asset_list.0.push(image_handle.clone_untyped());

    let atlas = TextureAtlas::from_grid(image_handle, Vec2::splat(16.0), 2, 2);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(ItemSprites(atlas_handle));
}

fn get_sprite_idx(kind: &ItemKind) -> usize {
    match kind {
        ItemKind::StopMushroom => 0,
        ItemKind::SpeedMushroom => 2,
        ItemKind::Armor => 1,
    }
}