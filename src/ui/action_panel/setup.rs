use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::TextureId};

use crate::setup::SetupSystems;

pub struct ActionPanelSetupPlugin;

impl Plugin for ActionPanelSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_icon_image_assets.in_set(SetupSystems::Ui))
            .add_systems(
                Update,
                (modify_icon_color, load_icon_texture_ids)
                    .chain()
                    .run_if(resource_exists::<SidePanelIconImages>)
                    .run_if(not(resource_exists::<SidePanelIconTextureIds>)),
            );
    }
}

#[derive(Resource)]
pub struct SidePanelIconTextureIds {
    pub particle_editor: TextureId,
    pub settings: TextureId,
}

#[derive(Resource)]
pub struct SidePanelIconImages {
    pub particle_editor: Handle<Image>,
    pub settings: Handle<Image>,
}

fn load_icon_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SidePanelIconImages {
        particle_editor: asset_server.load("icons/atom.png"),
        settings: asset_server.load("icons/document.png"),
    })
}

fn modify_icon_color(mut images: ResMut<Assets<Image>>, icons: Res<SidePanelIconImages>) {
    let light_gray: [u8; 3] = [200, 200, 200];

    for handle in [&icons.particle_editor, &icons.settings] {
        let Some(image) = images.get_mut(handle) else {
            continue;
        };
        let Some(data) = image.data.as_mut() else {
            continue;
        };
        for pixel in data.chunks_mut(4) {
            if pixel[3] > 0 {
                pixel[0] = light_gray[0];
                pixel[1] = light_gray[1];
                pixel[2] = light_gray[2];
            }
        }
    }
}

fn load_icon_texture_ids(
    mut commands: Commands,
    mut contexts: EguiContexts,
    icons: Res<SidePanelIconImages>,
) {
    commands.insert_resource(SidePanelIconTextureIds {
        particle_editor: contexts.add_image(bevy_egui::EguiTextureHandle::Strong(
            icons.particle_editor.clone(),
        )),
        settings: contexts.add_image(bevy_egui::EguiTextureHandle::Strong(icons.settings.clone())),
    });
}
