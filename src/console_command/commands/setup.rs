pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ConsoleAction>::default())
            .init_resource::<ConsoleCache>()
            .init_resource::<ConsoleInformationAreaState>()
            .init_resource::<ConsolePromptState>()
            .add_systems(Startup, setup_console_command_registry);
    }
}

fn setup_console_command_registry(mut commands: Commands) {
    let mut registry = ConsoleCommandRegistry::default();
    registry.register(HelpConsoleCommand);
    registry.register(ExitConsoleCommand);
    registry.register(ParticlesConsoleCommand);
    registry.register(BrushConsoleCommand);
    registry.register(ConwayConsoleCommand);
    registry.register(SceneConsoleCommand);
    registry.register(CanvasCommand);
    registry.register(SelectCommand);
    registry.register(SaveCommand);
    commands.insert_resource(registry);
}
