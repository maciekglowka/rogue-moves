use bevy::prelude::*;
use bevy::ui::FocusPolicy;

const BUTTON_WIDTH: Val = Val::Px(64.);
const BUTTON_HEIGHT: Val = Val::Px(96.);
const BUTTON_MARGIN: Val = Val::Px(16.);
const MENU_PADDING: Val = Val::Px(16.);

pub const BUTTON_COLOR: Color = Color::WHITE;
pub const STATUS_COLOR: Color = Color::Rgba { red: 0.77, green: 0.67, blue: 0.58, alpha: 1. };
pub const BUTTON_COLOR_CLICK: Color = Color::GRAY;
pub const BUTTON_TEXT_COLOR: Color = Color::Rgba{ red: 0.27, green: 0.22, blue: 0.19, alpha: 1. };
pub const TRANSPARENT: Color = Color::Rgba{ red: 0., green: 0., blue: 0., alpha: 0. };

use crate::items::{UseItemEvent, ItemKind};
use crate::units::{
    Unit,
    player::{Player, PlayerData}
};

pub struct PlayerButtonClickEvent(pub Entity);
pub struct PlayerMenuAssets {
    pub button_image: Handle<Image>,
    pub boot_image: Handle<Image>,
    pub shield_image: Handle<Image>
}

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
    player_query: Query<&Unit, With<Player>>,
    font_assets: Res<super::FontAssets>,
    assets: Res<PlayerMenuAssets>,
    mut ev_redraw_ui: EventReader<super::RedrawUIEvent>,
) {
    for _ in ev_redraw_ui.iter() {
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

            draw_card_menu(&mut commands, &player_data, &assets, &font_assets, menu);
            draw_status_menu(&mut commands, &player_data, &player_query, &assets, menu);
    }
}

fn draw_card_menu(
    commands: &mut Commands,
    player_data: &Res<PlayerData>,
    assets: &Res<PlayerMenuAssets>,
    font_assets: &Res<super::FontAssets>,
    menu: Entity
) {
    commands.entity(menu)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle{
                style: Style {
                    size: Size::new(Val::Percent(50.), BUTTON_HEIGHT),
                    ..Default::default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
                .with_children(|parent| {
                    for (idx, item) in player_data.items.iter().enumerate() {
                        parent.spawn_bundle(
                            get_button_bundle(assets)
                        )
                        .with_children(|parent| {
                            parent.spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(64.), Val::Px(64.)),
                                    ..Default::default()
                                },
                                color: BUTTON_TEXT_COLOR.into(),
                                image: UiImage(assets.boot_image.clone()),
                                focus_policy: FocusPolicy::Pass,
                                ..Default::default()
                            });
                            parent.spawn_bundle(TextBundle::from_section(
                                match item.kind {
                                    ItemKind::SpeedMushroom => "+1",
                                    ItemKind::StopMushroom => "0",
                                    _ => ""
                                },
                                TextStyle {
                                    color: BUTTON_TEXT_COLOR,
                                    font: font_assets.font.clone(),
                                    font_size: 32.,
                                    ..Default::default()
                                }
                            ));
                        })
                        .insert(
                            PlayerButton{idx}
                        );
                    }
                });
        });
}

fn draw_status_menu(
    commands: &mut Commands,
    player_data: &Res<PlayerData>,
    player_query: &Query<&Unit, With<Player>>,
    assets: &Res<PlayerMenuAssets>,
    menu: Entity
) {
    commands.entity(menu)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle{
                style: Style {
                    size: Size::new(Val::Percent(50.), BUTTON_HEIGHT),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
                .with_children(|parent| {
                    // draw AP
                    if let Ok(player) = player_query.get_single() {
                        for _ in 0..player.ap {
                            parent.spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(64.), Val::Px(64.)),
                                    margin: UiRect::new(Val::Px(0.), BUTTON_MARGIN, Val::Px(0.), Val::Px(0.)),
                                    ..Default::default()
                                },
                                color: STATUS_COLOR.into(),
                                image: UiImage(assets.boot_image.clone()),
                                focus_policy: FocusPolicy::Pass,
                                ..Default::default()
                            });
                        }
                    }
                    // draw armor
                    for _ in 0..player_data.armor {
                        parent.spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(64.), Val::Px(64.)),
                                margin: UiRect::new(Val::Px(0.), BUTTON_MARGIN, Val::Px(0.), Val::Px(0.)),
                                ..Default::default()
                            },
                            color: STATUS_COLOR.into(),
                            image: UiImage(assets.shield_image.clone()),
                            focus_policy: FocusPolicy::Pass,
                            ..Default::default()
                        });
                    }
                });
        });
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
            size: Size::new(BUTTON_WIDTH, BUTTON_HEIGHT),
            margin: UiRect::new(Val::Px(0.), BUTTON_MARGIN, Val::Px(0.), Val::Px(0.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: BUTTON_COLOR.into(),
        image: UiImage(assets.button_image.clone()),
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
    let button_handle = asset_server.load("card.png");
    asset_list.0.push(button_handle.clone_untyped());


    let boot_handle = asset_server.load("boot.png");
    asset_list.0.push(boot_handle.clone_untyped());

    let shield_handle = asset_server.load("shield.png");
    asset_list.0.push(shield_handle.clone_untyped());

    commands.insert_resource(
        PlayerMenuAssets {
            button_image: button_handle,
            boot_image: boot_handle,
            shield_image: shield_handle
        }
    );
}