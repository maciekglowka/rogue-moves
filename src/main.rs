use bevy::prelude::*;

mod assets;
mod board;
mod camera;
mod graphics;
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

    app.add_plugins(DefaultPlugins);
    app.init_resource::<assets::AssetList>();

    app.add_state(states::GameState::LoadAssets);
    app.add_state(states::AnimationState::Idle);

    app.add_system_set(
        SystemSet::on_update(states::GameState::LoadAssets)
            .with_system(assets::check_asset_loading)
    );

    app.add_plugin(board::BoardPlugin);
    app.add_plugin(units::UnitsPlugin);
    app.add_plugin(graphics::GraphicsPlugin);

    app.add_plugin(ui::UIPlugin);
    app.add_plugin(manager::ManagerPlugin);

    app.add_startup_system(camera::spawn_camera);
    app.run();
}
