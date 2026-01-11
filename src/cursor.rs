use bevy::{camera::ViewportConversionError, prelude::*, window::PrimaryWindow};
use bevy_falling_sand::{core::ParticleMap, prelude::Particle};

use crate::particles::SelectedParticle;

use super::camera::MainCamera;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .init_resource::<HoveredParticle>()
            .add_systems(Update, (update_cursor_position, update_hovered_particle));
    }
}

#[derive(Default, Resource, Clone, Debug)]
pub struct CursorPosition {
    pub current: Vec2,
    pub previous: Vec2,
    pub previous_previous: Vec2,
}

impl CursorPosition {
    pub fn update(&mut self, new_coords: Vec2) {
        self.previous_previous = self.previous;
        self.previous = self.current;
        self.current = new_coords;
    }
}

#[derive(Default, Resource, Clone, Debug)]
pub struct HoveredParticle {
    pub particle: Option<Particle>,
}

pub fn update_cursor_position(
    mut coords: ResMut<CursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Result {
    let (camera, camera_transform) = q_camera.single()?;

    let window = q_window.single()?;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(
            |cursor| -> Option<std::result::Result<Ray3d, ViewportConversionError>> {
                Some(camera.viewport_to_world(camera_transform, cursor))
            },
        )
        .map(|ray| ray.unwrap().origin.truncate())
    {
        coords.update(world_position);
    }
    Ok(())
}

fn update_hovered_particle(
    cursor_position: Res<CursorPosition>,
    map: Res<ParticleMap>,
    particle_query: Query<&Particle>,
    mut hovered_particle: ResMut<HoveredParticle>,
) -> Result {
    let position = IVec2::new(
        cursor_position.current.x.floor() as i32,
        cursor_position.current.y.floor() as i32,
    );
    if let Some(entity) = map.get(&position) {
        let particle = particle_query.get(*entity)?;
        hovered_particle.particle = Some(particle.clone());
    } else {
        hovered_particle.particle = None
    }
    Ok(())
}

pub fn sample_hovered_particle(
    hovered_particle: Res<HoveredParticle>,
    mut selected_particle: ResMut<SelectedParticle>,
) {
    if let Some(particle) = &hovered_particle.particle {
        selected_particle.0 = particle.clone();
    }
}
