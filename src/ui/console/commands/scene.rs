use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy_falling_sand::prelude::{ParticleSceneRegistry, SpawnSceneSignal};

use super::parse_position;
use crate::console_command::ConsoleCommand;

#[derive(Default)]
pub struct SceneConsoleCommand;

impl ConsoleCommand for SceneConsoleCommand {
    fn name(&self) -> &'static str {
        "scene"
    }

    fn description(&self) -> &'static str {
        "Scene operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(SceneSpawnConsoleCommand),
            Box::new(SceneListConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct SceneSpawnConsoleCommand;

impl ConsoleCommand for SceneSpawnConsoleCommand {
    fn name(&self) -> &'static str {
        "spawn"
    }

    fn description(&self) -> &'static str {
        "Spawn a scene at a position. Usage: scene spawn <name> <x>,<y> [--overwrite]"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.len() < 2 {
            warn!("Usage: scene spawn <name> <x>,<y> [--overwrite]");
            return;
        }

        let scene_name = &args[0];

        let position_args: Vec<&String> =
            args[1..].iter().filter(|a| *a != "--overwrite").collect();
        let position_slice: Vec<String> = position_args.into_iter().cloned().collect();

        match parse_position::<IVec2>(&position_slice) {
            Ok(center) => {
                let name = scene_name.clone();
                info!("Spawning scene '{name}' at {center}");
                commands.queue(move |world: &mut World| {
                    let registry = world.resource::<ParticleSceneRegistry>();
                    let handle = registry.scenes.iter().find(|(path, _)| {
                        let filename = path
                            .rsplit('/')
                            .next()
                            .unwrap_or(path)
                            .strip_suffix(".scn.ron")
                            .unwrap_or(path);
                        filename == name
                    });

                    let Some((_, handle)) = handle else {
                        warn!(
                            "Scene '{}' not found. Use 'scene list' to see available scenes.",
                            name
                        );
                        return;
                    };

                    let handle = handle.clone();
                    world
                        .resource_mut::<Messages<SpawnSceneSignal>>()
                        .write(SpawnSceneSignal::new(handle, center));
                });
            }
            Err(e) => {
                warn!("{e}");
            }
        }
    }
}

#[derive(Default)]
pub struct SceneListConsoleCommand;

impl ConsoleCommand for SceneListConsoleCommand {
    fn name(&self) -> &'static str {
        "list"
    }

    fn description(&self) -> &'static str {
        "List available scenes"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.queue(|world: &mut World| {
            let registry = world.resource::<ParticleSceneRegistry>();
            if registry.scenes.is_empty() {
                info!("No scenes loaded");
            } else {
                for path in registry.scenes.keys() {
                    let name = path
                        .rsplit('/')
                        .next()
                        .unwrap_or(path)
                        .strip_suffix(".scn.ron")
                        .unwrap_or(path);
                    info!("  {name}");
                }
            }
        });
    }
}

#[derive(Resource)]
pub(super) struct SceneFolderHandle(#[allow(dead_code)] Handle<LoadedFolder>);

pub(super) fn load_scene_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load_folder("scenes");
    commands.insert_resource(SceneFolderHandle(handle));
}
