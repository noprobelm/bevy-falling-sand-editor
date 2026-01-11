use crate::particles::SelectedParticle;
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_falling_sand::color::{ColorAssignment, ColorSource, Palette};
use bevy_falling_sand::prelude::*;
use std::time::Duration;

/// Query data tuple for fetching particle type properties.
///
/// IMPORTANT: When adding a new material type, add it to this tuple.
type ParticlePropertiesQueryData = (
    Option<&'static Density>,
    Option<&'static Speed>,
    Option<&'static Momentum>,
    Option<&'static ColorProfile>,
    Option<&'static ChangesColor>,
    Option<&'static TimedLifetime>,
    Option<&'static ChanceLifetime>,
    Option<&'static Burns>,
    Option<&'static Fire>,
    Option<&'static Wall>,
    Option<&'static Solid>,
    Option<&'static MovableSolid>,
    Option<&'static Liquid>,
    Option<&'static Gas>,
    Option<&'static Insect>,
);

#[derive(Message, Debug, Clone)]
pub struct LoadParticleIntoEditor {
    pub particle_name: String,
}

#[derive(Message, Debug, Clone)]
pub struct CreateNewParticle {
    pub duplicate_from: Option<String>,
}

#[derive(Message, Debug, Clone)]
pub struct ApplyEditorChangesAndReset {
    pub editor_entity: Entity,
}

#[derive(Clone, Debug, Component)]
pub struct ParticleEditorData {
    pub name: String,

    pub material_state: MaterialState,

    pub density: u32,
    pub max_speed: u8,
    pub has_momentum: bool,

    pub color_source: ColorSource,
    pub color_assignment: ColorAssignment,
    pub changes_color: Option<ChangesColorConfig>,

    pub lifetime: Option<LifetimeConfig>,

    pub fluidity: Option<u8>,
    pub liquid_resistance: Option<f64>,
    pub air_resistance: Option<f64>,

    pub burns_config: Option<BurnsConfig>,

    pub fire_config: Option<FireConfig>,

    pub is_new: bool,

    pub is_modified: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChangesColorConfig {
    Chance(f64),
    Timed {
        duration_ms: u64,
        duration_str: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LifetimeConfig {
    Timed {
        duration_ms: u64,
        duration_str: String,
    },
    Chance {
        chance: f64,
        tick_rate_ms: u64,
        tick_rate_str: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialState {
    Wall,
    Solid,
    MovableSolid,
    Liquid,
    Gas,
    Insect,
    Other,
}

#[derive(Clone, Debug)]
pub struct BurnsConfig {
    pub duration: Duration,
    pub tick_rate: Duration,
    pub duration_str: String,
    pub tick_rate_str: String,
    pub chance_destroy_per_tick: Option<f64>,
    pub reaction: Option<ReactionConfig>,
    pub burning_colors: Option<Vec<Color>>,
    pub spreads_fire: Option<FireConfig>,
    pub ignites_on_spawn: bool,
}

#[derive(Clone, Debug)]
pub struct FireConfig {
    pub burn_radius: f32,
    pub chance_to_spread: f64,
    pub destroys_on_spread: bool,
}

#[derive(Clone, Debug)]
pub struct ReactionConfig {
    pub produces: String,
    pub chance_to_produce: f64,
}

impl Default for ParticleEditorData {
    fn default() -> Self {
        Self {
            name: "New Particle".to_string(),
            material_state: MaterialState::Solid,
            density: 100,
            max_speed: 3,
            has_momentum: false,
            color_source: ColorSource::Palette(Palette {
                index: 0,
                colors: vec![Color::srgba_u8(128, 128, 128, 255)],
            }),
            color_assignment: ColorAssignment::Sequential,
            changes_color: None,
            lifetime: None,
            fluidity: None,
            liquid_resistance: None,
            air_resistance: None,
            burns_config: None,
            fire_config: None,
            is_new: true,
            is_modified: false,
        }
    }
}

impl ParticleEditorData {
    pub fn from_particle_type(
        name: String,
        particle_query: &Query<ParticlePropertiesQueryData, With<ParticleType>>,
        entity: Entity,
    ) -> Option<Self> {
        let components = particle_query.get(entity).ok()?;

        let (
            density,
            speed,
            momentum,
            color_profile,
            changes_color,
            timed_lifetime,
            chance_lifetime,
            burns,
            fire,
            wall,
            solid,
            movable_solid,
            liquid,
            gas,
            insect,
        ) = components;

        let material_state = if wall.is_some() {
            MaterialState::Wall
        } else if solid.is_some() {
            MaterialState::Solid
        } else if movable_solid.is_some() {
            MaterialState::MovableSolid
        } else if liquid.is_some() {
            MaterialState::Liquid
        } else if gas.is_some() {
            MaterialState::Gas
        } else if insect.is_some() {
            MaterialState::Insect
        } else {
            MaterialState::Other
        };

        let fluidity = if let Some(liquid) = liquid {
            Some(liquid.fluidity as u8)
        } else if let Some(gas) = gas {
            Some(gas.fluidity as u8)
        } else {
            None
        };

        let (liquid_resistance, air_resistance) = if let Some(movable_solid) = movable_solid {
            (
                Some(movable_solid.liquid_resistance),
                Some(movable_solid.air_resistance),
            )
        } else if let Some(liquid) = liquid {
            (Some(liquid.liquid_resistance), None)
        } else {
            (None, None)
        };

        let burns_config = burns.map(|burns| BurnsConfig {
            duration: burns.duration,
            tick_rate: burns.tick_rate,
            duration_str: burns.duration.as_millis().to_string(),
            tick_rate_str: burns.tick_rate.as_millis().to_string(),
            chance_destroy_per_tick: burns.chance_destroy_per_tick,
            reaction: burns.reaction.as_ref().map(|r| ReactionConfig {
                produces: r.produces.name.to_string(),
                chance_to_produce: r.chance_to_produce,
            }),
            burning_colors: burns.color.as_ref().map(|cp| match &cp.source {
                ColorSource::Palette(palette) => palette.colors.clone(),
                ColorSource::Gradient(gradient) => {
                    // For gradients, sample some colors
                    let mut colors = Vec::new();
                    for i in 0..gradient.steps.min(10) {
                        let t = i as f32 / (gradient.steps - 1) as f32;
                        colors.push(gradient.start.mix(&gradient.end, t));
                    }
                    colors
                }
            }),
            spreads_fire: burns.spreads.as_ref().map(|f| FireConfig {
                burn_radius: f.burn_radius,
                chance_to_spread: f.chance_to_spread,
                destroys_on_spread: f.destroys_on_spread,
            }),
            ignites_on_spawn: burns.ignites_on_spawn,
        });

        let fire_config = fire.map(|f| FireConfig {
            burn_radius: f.burn_radius,
            chance_to_spread: f.chance_to_spread,
            destroys_on_spread: f.destroys_on_spread,
        });

        let (color_source, color_assignment) = if let Some(cp) = color_profile {
            (cp.source.clone(), cp.assignment.clone())
        } else {
            (
                ColorSource::Palette(Palette {
                    index: 0,
                    colors: vec![Color::srgba_u8(128, 128, 128, 255)],
                }),
                ColorAssignment::Sequential,
            )
        };

        let lifetime = if let Some(timed) = timed_lifetime {
            Some(LifetimeConfig::Timed {
                duration_ms: timed.duration().as_millis() as u64,
                duration_str: timed.duration().as_millis().to_string(),
            })
        } else if let Some(chance) = chance_lifetime {
            let tick_rate_ms = chance
                .tick_timer
                .as_ref()
                .map(|t| t.duration().as_millis() as u64)
                .unwrap_or(100);
            Some(LifetimeConfig::Chance {
                chance: chance.chance,
                tick_rate_ms,
                tick_rate_str: tick_rate_ms.to_string(),
            })
        } else {
            None
        };

        Some(Self {
            name,
            material_state,
            density: density.map(|d| d.0).unwrap_or(100),
            max_speed: speed.map(|v| v.max()).unwrap_or(3),
            has_momentum: momentum.is_some(),
            color_source,
            color_assignment,
            changes_color: changes_color.map(|cc| match cc {
                ChangesColor::Chance(chance) => ChangesColorConfig::Chance(*chance),
                ChangesColor::Timed(duration) => ChangesColorConfig::Timed {
                    duration_ms: duration.as_millis() as u64,
                    duration_str: duration.as_millis().to_string(),
                },
            }),
            lifetime,
            fluidity,
            liquid_resistance,
            air_resistance,
            burns_config,
            fire_config,
            is_new: false,
            is_modified: false,
        })
    }

    pub fn mark_saved(&mut self) {
        self.is_modified = false;
        self.is_new = false;
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct ParticleEditorRegistry {
    map: HashMap<String, Entity>,
}

impl ParticleEditorRegistry {
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Entity)> {
        self.map.iter()
    }

    pub fn insert(&mut self, name: String, entity: Entity) -> Option<Entity> {
        self.map.insert(name, entity)
    }

    pub fn get(&self, name: &str) -> Option<&Entity> {
        self.map.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Entity> {
        self.map.remove(name)
    }
}

pub fn sync_particle_editor_registry(
    mut commands: Commands,
    particle_type_map: Res<ParticleTypeRegistry>,
    mut particle_editor_registry: ResMut<ParticleEditorRegistry>,
    particle_query: Query<ParticlePropertiesQueryData, With<ParticleType>>,
    _editor_data_query: Query<&ParticleEditorData>,
) {
    for (name, &particle_entity) in particle_type_map.iter() {
        let name_string = name.to_string();

        if !particle_editor_registry.contains(&name_string) {
            if let Some(editor_data) = ParticleEditorData::from_particle_type(
                name_string.clone(),
                &particle_query,
                particle_entity,
            ) {
                let editor_entity = commands.spawn(editor_data).id();
                particle_editor_registry.insert(name_string, editor_entity);
            }
        }
    }

    let mut to_remove = Vec::new();
    for (name, &editor_entity) in particle_editor_registry.iter() {
        if !particle_type_map.contains(name) {
            commands.entity(editor_entity).despawn();
            to_remove.push(name.clone());
        }
    }

    for name in to_remove {
        particle_editor_registry.remove(&name);
    }
}

pub fn handle_load_particle_into_editor(
    mut commands: Commands,
    mut load_messages: MessageReader<LoadParticleIntoEditor>,
    particle_type_map: Res<ParticleTypeRegistry>,
    mut particle_editor_registry: ResMut<ParticleEditorRegistry>,
    particle_query: Query<ParticlePropertiesQueryData, With<ParticleType>>,
    mut current_editor: ResMut<CurrentEditorSelection>,
    selected_particle: Option<ResMut<SelectedParticle>>,
) {
    let mut selected_particle_mut = selected_particle;

    for message in load_messages.read() {
        if let Some(ref mut selected_particle) = selected_particle_mut {
            let static_name: &'static str =
                Box::leak(message.particle_name.clone().into_boxed_str());
            selected_particle.0 = Particle::new(static_name);
        }

        if let Some(&editor_entity) = particle_editor_registry.get(&message.particle_name) {
            current_editor.selected_entity = Some(editor_entity);
            continue;
        }

        if let Some(&particle_entity) = particle_type_map.get(&message.particle_name) {
            if let Some(editor_data) = ParticleEditorData::from_particle_type(
                message.particle_name.clone(),
                &particle_query,
                particle_entity,
            ) {
                let editor_entity = commands.spawn(editor_data).id();
                particle_editor_registry.insert(message.particle_name.clone(), editor_entity);
                current_editor.selected_entity = Some(editor_entity);
            }
        }
    }
}

pub fn handle_create_new_particle(
    mut commands: Commands,
    mut create_messages: MessageReader<CreateNewParticle>,
    mut particle_editor_registry: ResMut<ParticleEditorRegistry>,
    mut current_editor: ResMut<CurrentEditorSelection>,
    mut particle_type_map: ResMut<ParticleTypeRegistry>,
    selected_particle: Option<ResMut<SelectedParticle>>,
    particle_query: Query<ParticlePropertiesQueryData, With<ParticleType>>,
) {
    let mut selected_particle_mut = selected_particle;

    for message in create_messages.read() {
        let editor_data = if let Some(ref duplicate_from) = message.duplicate_from {
            if let Some(&particle_entity) = particle_type_map.get(duplicate_from) {
                if let Some(mut duplicated_data) = ParticleEditorData::from_particle_type(
                    duplicate_from.clone(),
                    &particle_query,
                    particle_entity,
                ) {
                    let unique_name =
                        generate_unique_particle_name_with_base(&particle_type_map, "New Particle");
                    duplicated_data.name = unique_name;
                    duplicated_data.is_new = true;
                    duplicated_data.is_modified = false;
                    duplicated_data
                } else {
                    let mut default_data = ParticleEditorData::default();
                    let unique_name =
                        generate_unique_particle_name_with_base(&particle_type_map, "New Particle");
                    default_data.name = unique_name;
                    default_data
                }
            } else {
                let mut default_data = ParticleEditorData::default();
                let unique_name =
                    generate_unique_particle_name_with_base(&particle_type_map, "New Particle");
                default_data.name = unique_name;
                default_data
            }
        } else {
            let mut editor_data = ParticleEditorData::default();
            let unique_name =
                generate_unique_particle_name_with_base(&particle_type_map, "New Particle");
            editor_data.name = unique_name;
            editor_data
        };

        apply_editor_data_to_particle_type(
            &mut commands,
            &editor_data,
            &mut particle_type_map,
            true,
            None, // New particle, no existing index to track
        );

        let mut final_editor_data = editor_data;
        final_editor_data.mark_saved();

        let editor_entity = commands.spawn(final_editor_data.clone()).id();
        particle_editor_registry.insert(final_editor_data.name.clone(), editor_entity);
        current_editor.selected_entity = Some(editor_entity);

        if let Some(ref mut selected_particle) = selected_particle_mut {
            let static_name: &'static str =
                Box::leak(final_editor_data.name.clone().into_boxed_str());
            selected_particle.0 = Particle::new(static_name);
        }
    }
}

fn generate_unique_particle_name_with_base(
    particle_type_map: &ParticleTypeRegistry,
    base_name: &str,
) -> String {
    let mut counter = 1;
    let mut name = base_name.to_string();

    while particle_type_map.contains(&name) {
        counter += 1;
        name = format!("{} {}", base_name, counter);
    }

    name
}

fn apply_editor_data_to_particle_type(
    commands: &mut Commands,
    editor_data: &ParticleEditorData,
    particle_type_map: &mut ResMut<ParticleTypeRegistry>,
    create_new: bool,
    existing_color_index: Option<usize>,
) -> Entity {
    let entity = if create_new {
        let static_name: &'static str = Box::leak(editor_data.name.clone().into_boxed_str());
        let entity = commands.spawn(ParticleType::new(static_name)).id();
        particle_type_map.insert(static_name, entity);
        entity
    } else {
        *particle_type_map
            .get(&editor_data.name)
            .expect("Particle type should exist")
    };

    commands
        .entity(entity)
        .remove::<(
            Density,
            Speed,
            Momentum,
            ColorProfile,
            ChangesColor,
            TimedLifetime,
            ChanceLifetime,
            Burns,
            Fire,
        )>()
        .remove::<AllMaterialComponents>();

    commands.entity(entity).insert(Density(editor_data.density));
    commands
        .entity(entity)
        .insert(Speed::new(1, editor_data.max_speed));

    if editor_data.has_momentum {
        commands.entity(entity).insert(Momentum::ZERO);
    }

    // Preserve the existing color index when updating, to avoid resetting gradient progress
    let mut color_source = editor_data.color_source.clone();
    if let Some(index) = existing_color_index {
        match &mut color_source {
            ColorSource::Palette(p) => p.index = index,
            ColorSource::Gradient(g) => g.index = index,
        }
    }
    commands.entity(entity).insert(ColorProfile {
        source: color_source,
        assignment: editor_data.color_assignment.clone(),
    });

    if let Some(ref config) = editor_data.changes_color {
        match config {
            ChangesColorConfig::Chance(chance) => {
                commands
                    .entity(entity)
                    .insert(ChangesColor::chance(*chance));
            }
            ChangesColorConfig::Timed { duration_ms, .. } => {
                commands.entity(entity).insert(ChangesColor::timer(
                    std::time::Duration::from_millis(*duration_ms),
                ));
            }
        }
    }

    if let Some(ref config) = editor_data.lifetime {
        match config {
            LifetimeConfig::Timed { duration_ms, .. } => {
                commands.entity(entity).insert(TimedLifetime::new(
                    std::time::Duration::from_millis(*duration_ms),
                ));
            }
            LifetimeConfig::Chance {
                chance,
                tick_rate_ms,
                ..
            } => {
                commands
                    .entity(entity)
                    .insert(ChanceLifetime::with_tick_rate(
                        *chance,
                        std::time::Duration::from_millis(*tick_rate_ms),
                    ));
            }
        }
    }

    match editor_data.material_state {
        MaterialState::Wall => {
            commands.entity(entity).insert(Wall);
        }
        MaterialState::Solid => {
            commands.entity(entity).insert(Solid);
        }
        MaterialState::MovableSolid => {
            let liquid_resistance = editor_data.liquid_resistance.unwrap_or(0.5);
            let air_resistance = editor_data.air_resistance.unwrap_or(0.5);
            commands
                .entity(entity)
                .insert(MovableSolid::new(liquid_resistance, air_resistance));
        }
        MaterialState::Liquid => {
            let fluidity = editor_data.fluidity.unwrap_or(3);
            let liquid_resistance = editor_data.liquid_resistance.unwrap_or(0.0);
            commands
                .entity(entity)
                .insert(Liquid::with_resistance(fluidity.into(), liquid_resistance));
        }
        MaterialState::Gas => {
            let fluidity = editor_data.fluidity.unwrap_or(3);
            commands.entity(entity).insert(Gas::new(fluidity.into()));
        }
        MaterialState::Insect => {
            commands.entity(entity).insert(Insect::new());
        }
        MaterialState::Other => {}
    }

    if let Some(ref burns_config) = editor_data.burns_config {
        let reaction = burns_config.reaction.as_ref().map(|r| {
            let static_name: &'static str = Box::leak(r.produces.clone().into_boxed_str());
            Reacting::new(Particle::new(static_name), r.chance_to_produce)
        });

        let spreads = burns_config.spreads_fire.as_ref().map(|f| Fire {
            burn_radius: f.burn_radius,
            chance_to_spread: f.chance_to_spread,
            destroys_on_spread: f.destroys_on_spread,
        });

        commands.entity(entity).insert(Burns::new(
            burns_config.duration,
            burns_config.tick_rate,
            burns_config.chance_destroy_per_tick,
            reaction,
            burns_config
                .burning_colors
                .as_ref()
                .map(|colors| ColorProfile {
                    source: ColorSource::Palette(Palette {
                        index: 0,
                        colors: colors.clone(),
                    }),
                    assignment: ColorAssignment::Sequential,
                }),
            spreads,
            burns_config.ignites_on_spawn,
        ));
    }

    if let Some(ref fire_config) = editor_data.fire_config {
        commands.entity(entity).insert(Fire {
            burn_radius: fire_config.burn_radius,
            chance_to_spread: fire_config.chance_to_spread,
            destroys_on_spread: fire_config.destroys_on_spread,
        });
    }

    entity
}

pub fn setup_initial_particle_selection(
    selected_particle: Res<SelectedParticle>,
    mut load_particle_messages: MessageWriter<LoadParticleIntoEditor>,
) {
    let particle_name = selected_particle.0.name.to_string();

    load_particle_messages.write(LoadParticleIntoEditor { particle_name });
}

/// System that automatically applies changes to particle types when ParticleEditorData is modified.
pub fn auto_save_editor_changes(
    mut commands: Commands,
    mut editor_data_query: Query<(Entity, &mut ParticleEditorData), Changed<ParticleEditorData>>,
    mut particle_type_map: ResMut<ParticleTypeRegistry>,
    mut particle_editor_registry: ResMut<ParticleEditorRegistry>,
    color_profile_query: Query<&ColorProfile, With<ParticleType>>,
) {
    for (editor_entity, mut editor_data) in editor_data_query.iter_mut() {
        // Skip if this is a new particle that hasn't been saved yet
        if editor_data.is_new {
            continue;
        }

        // Only apply if the particle type already exists
        if !particle_type_map.contains(&editor_data.name) {
            continue;
        }

        // Get the existing color index to preserve it during auto-save
        let existing_color_index = particle_type_map
            .get(&editor_data.name)
            .and_then(|&entity| color_profile_query.get(entity).ok())
            .map(|cp| match &cp.source {
                ColorSource::Palette(p) => p.index,
                ColorSource::Gradient(g) => g.index,
            });

        apply_editor_data_to_particle_type(
            &mut commands,
            &editor_data,
            &mut particle_type_map,
            false,
            existing_color_index,
        );

        // Update the registry if needed
        if !particle_editor_registry.contains(&editor_data.name) {
            particle_editor_registry.insert(editor_data.name.clone(), editor_entity);
        }

        editor_data.mark_saved();
    }
}

pub fn handle_apply_editor_changes_and_reset(
    mut commands: Commands,
    mut apply_messages: MessageReader<ApplyEditorChangesAndReset>,
    mut editor_data_query: Query<&mut ParticleEditorData>,
    mut particle_type_map: ResMut<ParticleTypeRegistry>,
    mut particle_editor_registry: ResMut<ParticleEditorRegistry>,
    mut reset_particle_children_messages: MessageWriter<
        bevy_falling_sand::prelude::ResetParticleTypeChildrenSignal,
    >,
    color_profile_query: Query<&ColorProfile, With<ParticleType>>,
) {
    for message in apply_messages.read() {
        if let Ok(mut editor_data) = editor_data_query.get_mut(message.editor_entity) {
            let create_new = editor_data.is_new || !particle_type_map.contains(&editor_data.name);

            // Preserve the existing color index when applying changes
            let existing_color_index = particle_type_map
                .get(&editor_data.name)
                .and_then(|&entity| color_profile_query.get(entity).ok())
                .map(|cp| match &cp.source {
                    ColorSource::Palette(p) => p.index,
                    ColorSource::Gradient(g) => g.index,
                });

            let particle_entity = apply_editor_data_to_particle_type(
                &mut commands,
                &editor_data,
                &mut particle_type_map,
                create_new,
                existing_color_index,
            );

            if create_new {
                particle_editor_registry.insert(editor_data.name.clone(), message.editor_entity);
            }

            editor_data.mark_saved();

            reset_particle_children_messages.write(
                bevy_falling_sand::prelude::ResetParticleTypeChildrenSignal::from_parent_handle(
                    particle_entity,
                ),
            );
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentEditorSelection {
    pub selected_entity: Option<Entity>,
}

