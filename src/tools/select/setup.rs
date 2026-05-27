use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<SelectAction>::default())
            .add_systems(Startup, (setup_overlay_image, setup_select_input));
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(super) enum SelectAction {
    AddToSelection,
}

/// Shared 1x1 translucent white image used by all overlay sprites.
#[derive(Resource)]
pub(super) struct OverlayImage(pub Handle<Image>);

fn setup_overlay_image(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![255, 255, 255, 128],
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );
    commands.insert_resource(OverlayImage(images.add(image)));
}

fn setup_select_input(mut commands: Commands) {
    let input_map = InputMap::default().with(SelectAction::AddToSelection, KeyCode::ControlLeft);
    commands.spawn(input_map);
}
