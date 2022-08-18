use bevy::prelude::*;

const BUTTON_SIZE: Val = Val::Px(64.);
const BUTTON_MARGIN: Val = Val::Px(10.);
const MENU_PADDING: Val = Val::Px(10.);

pub const BUTTON_COLOR: Color = Color::WHITE;
pub const BUTTON_COLOR_CLICK: Color = Color::GRAY;

use crate::items::UseItemEvent;
use crate::units::{
    player::PlayerData
};

pub struct PlayerButtonClickEvent(pub Entity);
pub struct PlayerMenuAssets(pub Handle<Image>);

#[derive(Component)]
pub struct PlayerMenu;

fn destroy_menu(
    commands: &mut Commands,
    menu_query: &Query<Entity, With<PlayerMenu>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

pub fn clear_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<PlayerMenu>>,
) {
    destroy_menu(&mut commands, &menu_query);
}

pub fn draw_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<PlayerMenu>>,
    player_data: Res<PlayerData>,
    // assets: Res<super::FontAssets>,
    assets: Res<PlayerMenuAssets>,
    mut ev_draw_cursor: EventReader<super::RedrawUIEvent>,
) {
    for _ in ev_draw_cursor.iter() {
        destroy_menu(&mut commands, &menu_query);

        let menu = commands
            .spawn_bundle(
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect::new(MENU_PADDING, MENU_PADDING, MENU_PADDING, MENU_PADDING),
                        ..Default::default()
                    },
                    color: UiColor::from(Color::NONE),
                    ..Default::default()
                }
            )
            .insert(PlayerMenu)
            .id();

        for (idx, item_entity) in player_data.items.iter().enumerate() {
            commands.entity(menu)
                .with_children(|parent| {
                    parent.spawn_bundle(
                        get_button_bundle(&assets)
                    )
                    .insert(
                        PlayerButton{idx}
                    );
                });
        }
    }
}

pub fn button_click(
    mut ev_button_click: EventReader<PlayerButtonClickEvent>,
    mut ev_ui: EventWriter<UseItemEvent>,
    button_query: Query<&PlayerButton>
) {
    for ev in ev_button_click.iter() {
        if let Ok(button) = button_query.get(ev.0) {
            ev_ui.send(UseItemEvent(button.idx));
        }
    }
}

fn get_button_bundle(
    assets: &Res<PlayerMenuAssets>,
) -> ButtonBundle {
    ButtonBundle{
        style: Style {
            size: Size::new(BUTTON_SIZE, BUTTON_SIZE),
            margin: UiRect::new(Val::Px(0.), BUTTON_MARGIN, Val::Px(0.), BUTTON_MARGIN),
            ..Default::default()
        },
        color: BUTTON_COLOR.into(),
        image: UiImage(assets.0.clone()),
        ..Default::default()
    }
}

#[derive(Component)]
pub struct PlayerButton {
    pub idx: usize
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList> 
) {
    let image_handle = asset_server.load("cursor.png");
    asset_list.0.push(image_handle.clone_untyped());

    commands.insert_resource(PlayerMenuAssets(image_handle));
}