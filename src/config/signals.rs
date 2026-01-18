use std::path::PathBuf;

use bevy::prelude::*;

pub struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveInitConfigSignal>()
            .add_message::<LoadInitConfigSignal>()
            .add_systems(Update, msgr_load_init_config);
    }
}

#[derive(Event, Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct SaveInitConfigSignal(pub PathBuf);

/// Signal to load init config from the config asset source.
/// The path is relative to the config directory (e.g., "settings/init.scn.ron").
#[derive(Event, Message, Default, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct LoadInitConfigSignal(pub String);

/// System to load init config from RON scene file using the asset server.
fn msgr_load_init_config(
    mut msgr_load_init_config: MessageReader<LoadInitConfigSignal>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    for signal in msgr_load_init_config.read() {
        let asset_path = format!("config://{}", signal.0);
        let scene_handle: Handle<DynamicScene> = asset_server.load(&asset_path);
        scene_spawner.spawn_dynamic(scene_handle);
        info!("Loading init config from {}", asset_path);
    }
}
