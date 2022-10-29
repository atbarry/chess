use bevy::prelude::*;
use chess::StarterPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StarterPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

