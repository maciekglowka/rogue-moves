use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
// use bevy_inspector_egui::WorldInspectorPlugin;

mod assets;
mod command;
mod board;
mod camera;
mod graphics;
mod items;
mod manager;
mod states;
mod ui;
mod units;
mod vectors;

fn main() {
    let mut app = App::new();
    app.insert_resource(
        WindowDescriptor {
            height: 800.,
            width: 800.,
            ..Default::default()
        }
    );
    app.insert_resource(ImageSettings::default_nearest());

    app.add_plugins(DefaultPlugins);
    // app.add_plugin(WorldInspectorPlugin::new());
    app.init_resource::<assets::AssetList>();

    app.add_state(states::GameState::LoadAssets);
    app.add_state(states::AnimationState::Idle);
    app.add_state(states::FadeState::Hidden);

    app.add_system_set(
        SystemSet::on_update(states::GameState::LoadAssets)
            .with_system(assets::check_asset_loading)
    );

    app.add_plugin(command::CommandPlugin);
    app.add_plugin(board::BoardPlugin);
    app.add_plugin(units::UnitsPlugin);
    app.add_plugin(items::ItemsPlugin);
    app.add_plugin(graphics::GraphicsPlugin);

    app.add_plugin(ui::UIPlugin);
    app.add_plugin(manager::ManagerPlugin);

    app.add_startup_system(camera::spawn_camera);
    app.run();
}
