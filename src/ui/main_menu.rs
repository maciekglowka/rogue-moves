use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

pub fn draw_menu(
    mut commands: Commands,
    assets: Res<super::FontAssets>
) {
    let text = "Left click to move\nRight hold to see other's moves";

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
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.font.clone(),
                        font_size: 32.,
                        ..Default::default()
                    },
                    TextAlignment { 
                        ..Default::default()
                    }
                ),
                ..Default::default()
            })
            .insert(MainMenu);
}