use bevy::prelude::*;

use crate::units::{
    player::{Player, PlayerData},
    Unit
};


#[derive(Component)]
pub struct StatusBar;

fn destroy_status(
    commands: &mut Commands,
    status_query: &Query<Entity, With<StatusBar>>,
) {
    for entity in status_query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

pub fn clear_status(
    mut commands: Commands,
    status_query: Query<Entity, With<StatusBar>>,
) {
    destroy_status(&mut commands, &status_query);
}

pub fn draw_status(
    mut commands: Commands,
    status_query: Query<Entity, With<StatusBar>>,
    player_query: Query<&Unit, With<Player>>,
    player_data: Res<PlayerData>,
    assets: Res<super::FontAssets>,
    mut ev_draw_cursor: EventReader<super::cursor::DrawCursorEvent>,
) {
    for _ in ev_draw_cursor.iter() {
        destroy_status(&mut commands, &status_query);

        if let Ok(player) = player_query.get_single() {
            let s = format!("Level: {} | {}", player_data.level, "O".repeat(player.ap as usize));
            let color = Color::Rgba { red: 0.84, green: 0.85, blue: 0.84, alpha: 1. };
            commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(20.),
                        left: Val::Px(20.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    s,
                    TextStyle {
                        color: color,
                        font: assets.font.clone(),
                        font_size: 24.,
                        ..Default::default()
                    },
                    TextAlignment { 
                        ..Default::default()
                    }
                ),
                ..Default::default()
            })
            .insert(StatusBar);
        }      
    }
}
