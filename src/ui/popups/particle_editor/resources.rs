use std::time::Duration;

use bevy::{ecs::query::QueryData, platform::collections::HashMap, prelude::*};
use bevy_egui::EguiPrimaryContextPass;
use bevy_falling_sand::prelude::*;

use crate::chunk_effects::{BurnEffect, GasEffect, GlowEffect, LiquidEffect};
use crate::particles::ParticleCategory;
use crate::ui::UiSystems;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NameDraft>()
            .add_systems(
                EguiPrimaryContextPass,
                (
                    synchronize_editor_registry.run_if(resource_changed::<ParticleTypeRegistry>),
                    refresh_name_draft.run_if(condition_should_refresh_name_draft),
                )
                    .chain()
                    .before(UiSystems::ParticleEditor),
            )
            .add_systems(
                Update,
                refresh_particle_labels.run_if(condition_particle_movement_changed),
            );
    }
}

#[derive(Resource)]
pub struct ParticleTypesSavedMessageConfiguration {
    pub fade_duration: Duration,
    pub colors: [u8; 3],
}

impl Default for ParticleTypesSavedMessageConfiguration {
    fn default() -> Self {
        Self {
            fade_duration: Duration::from_secs(8),
            colors: [40, 200, 40],
        }
    }
}

/// Known category ordering. Categories matching these names appear first in this order.
const CATEGORY_ORDER: &[&str] = &["Wall", "Solid", "Movable Solid", "Liquid", "Gas"];

#[derive(Resource, Default)]
pub struct ParticleCategoryLabels {
    /// Ordered list of (category_name, particle_names).
    pub categories: Vec<(String, Vec<String>)>,
}

impl ParticleCategoryLabels {
    pub fn push(&mut self, category: &str, name: String) {
        if let Some((_, names)) = self.categories.iter_mut().find(|(c, _)| c == category) {
            names.push(name);
        } else {
            self.categories.push((category.to_string(), vec![name]));
        }
    }

    /// Sort categories: known categories first in CATEGORY_ORDER, then remaining alphabetically,
    /// with "Other" always last.
    pub fn sort(&mut self) {
        self.categories.sort_by(|(a, _), (b, _)| {
            fn rank(s: &str) -> (u8, usize) {
                if let Some(idx) = CATEGORY_ORDER.iter().position(|&c| c == s) {
                    (0, idx)
                } else if s == "Other" {
                    (2, 0)
                } else {
                    (1, 0)
                }
            }
            rank(a).cmp(&rank(b)).then_with(|| a.cmp(b))
        });
    }

    pub fn categories(&self) -> impl Iterator<Item = (&str, &Vec<String>)> {
        self.categories.iter().map(|(k, v)| (k.as_str(), v))
    }
}

#[derive(Resource, Copy, Clone, PartialEq, Debug, Reflect)]
pub struct SelectedParticle(pub Entity);

#[derive(Resource, Default, Clone, Debug, Reflect)]
pub struct EditorState {
    pub map: HashMap<Entity, ParticleData>,
}

#[derive(Clone, Debug, Reflect)]
pub struct CachedMovementState {
    pub movement: Movement,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
    pub resistor: ParticleResistor,
}

impl Default for CachedMovementState {
    fn default() -> Self {
        Self {
            movement: Movement::default(),
            density: Density(1000),
            speed: Speed::new(1, 3),
            momentum: Momentum::default(),
            resistor: ParticleResistor(0.0),
        }
    }
}

fn default_editor_gradient() -> ColorGradient {
    ColorGradient {
        hsv_interpolation: true,
        ..ColorGradient::default()
    }
}

fn default_editor_texture() -> TextureSource {
    match ColorProfile::texture(String::new()).source {
        ColorSource::Texture(t) => t,
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct ParticleData {
    pub cached_movement: CachedMovementState,
    pub timed_lifetime: TimedLifetime,
    pub chance_lifetime: ChanceLifetime,
    pub chance_mutation: ChanceMutation,
    pub static_rigid_body: StaticRigidBodyParticle,
    pub palette: Palette,
    pub gradient: ColorGradient,
    pub texture: TextureSource,
    pub burns: Flammable,
    pub contact_reaction: ContactReaction,
    pub corrosive: Corrosive,
    pub corrodible: Corrodible,
}

impl Default for ParticleData {
    fn default() -> Self {
        let cached_movement = CachedMovementState::default();
        let timed_lifetime = TimedLifetime::new(Duration::from_millis(10000));
        let chance_lifetime = ChanceLifetime::new(0.01, Duration::from_millis(100));
        let chance_mutation =
            ChanceMutation::from_string(String::new(), 0.01, Duration::from_millis(100));
        let static_rigid_body = StaticRigidBodyParticle;
        let burns = Flammable::new(
            Duration::from_millis(1000),
            Duration::from_millis(100),
            0.5,
            None,
            0.01,
            true,
            1.0,
            false,
            false,
        );
        let palette = Palette::default();
        let gradient = default_editor_gradient();
        let texture = default_editor_texture();
        let contact_reaction = ContactReaction::default();
        let corrosive = Corrosive::new(0.5, Duration::from_millis(100));
        let corrodible = Corrodible;
        Self {
            cached_movement,
            timed_lifetime,
            chance_lifetime,
            chance_mutation,
            static_rigid_body,
            palette,
            gradient,
            texture,
            burns,
            contact_reaction,
            corrosive,
            corrodible,
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct ParticleDataQuery {
    pub core: CoreQuery,
    pub movement: MovementQuery,
    pub physics: PhysicsQuery,
    pub color: ColorQuery,
    pub reactions: ReactionsQuery,
    pub effects: EffectsQuery,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct CoreQuery {
    pub particle_type: &'static mut ParticleType,
    pub timed_lifetime: Option<&'static mut TimedLifetime>,
    pub chance_lifetime: Option<&'static mut ChanceLifetime>,
    pub chance_mutation: Option<&'static mut ChanceMutation>,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct MovementQuery {
    pub movement: Option<&'static mut Movement>,
    pub density: Option<&'static mut Density>,
    pub speed: Option<&'static mut Speed>,
    pub momentum: Option<&'static Momentum>,
    pub resistor: Option<&'static ParticleResistor>,
    pub category: Option<&'static mut ParticleCategory>,
    pub air_resistance: Option<&'static mut AirResistance>,
}

#[derive(QueryData)]
pub(crate) struct PhysicsQuery {
    pub static_rigid_body: Option<&'static StaticRigidBodyParticle>,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct ColorQuery {
    pub profile: &'static mut ColorProfile,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct ReactionsQuery {
    pub burns: Option<&'static mut Flammable>,
    pub contact_reaction: Option<&'static mut ContactReaction>,
    pub corrosive: Option<&'static mut Corrosive>,
    pub corrodible: Option<&'static Corrodible>,
}

#[derive(QueryData)]
pub(crate) struct EffectsQuery {
    pub liquid: Option<&'static LiquidEffect>,
    pub gas: Option<&'static GasEffect>,
    pub glow: Option<&'static GlowEffect>,
    pub burn: Option<&'static BurnEffect>,
}

fn build_cached_movement(
    movement: Option<&Movement>,
    density: Option<&Density>,
    speed: Option<&Speed>,
    momentum: Option<&Momentum>,
    resistor: Option<&ParticleResistor>,
    cached: &CachedMovementState,
) -> CachedMovementState {
    CachedMovementState {
        movement: movement.cloned().unwrap_or_else(|| cached.movement.clone()),
        density: density.copied().unwrap_or(cached.density),
        speed: speed.copied().unwrap_or(cached.speed),
        momentum: momentum.copied().unwrap_or(cached.momentum),
        resistor: resistor.copied().unwrap_or(cached.resistor),
    }
}

fn synchronize_editor_registry(
    mut commands: Commands,
    query: Query<ParticleDataQuery>,
    particle_registry: Res<ParticleTypeRegistry>,
    editor_state: Res<EditorState>,
) {
    let defaults = ParticleData::default();
    let mut new_state = EditorState::default();

    for entity in particle_registry.entities() {
        let cached = editor_state.map.get(entity);

        if let Ok(data) = query.get(*entity) {
            let (palette, gradient, texture) = match &data.color.profile.source {
                ColorSource::Palette(p) => (
                    p.clone(),
                    cached
                        .map(|c| c.gradient.clone())
                        .unwrap_or_else(default_editor_gradient),
                    cached
                        .map(|c| c.texture.clone())
                        .unwrap_or_else(default_editor_texture),
                ),
                ColorSource::Gradient(g) => (
                    cached.map(|c| c.palette.clone()).unwrap_or_default(),
                    g.clone(),
                    cached
                        .map(|c| c.texture.clone())
                        .unwrap_or_else(default_editor_texture),
                ),
                ColorSource::Texture(t) => (
                    cached.map(|c| c.palette.clone()).unwrap_or_default(),
                    cached
                        .map(|c| c.gradient.clone())
                        .unwrap_or_else(default_editor_gradient),
                    t.clone(),
                ),
            };

            let cached_movement = cached
                .map(|c| c.cached_movement.clone())
                .unwrap_or_else(|| defaults.cached_movement.clone());

            let new_cached = build_cached_movement(
                data.movement.movement,
                data.movement.density,
                data.movement.speed,
                data.movement.momentum,
                data.movement.resistor,
                &cached_movement,
            );

            let particle_data = ParticleData {
                cached_movement: new_cached,
                timed_lifetime: data
                    .core
                    .timed_lifetime
                    .cloned()
                    .unwrap_or_else(|| defaults.timed_lifetime.clone()),
                chance_lifetime: data
                    .core
                    .chance_lifetime
                    .cloned()
                    .unwrap_or_else(|| defaults.chance_lifetime.clone()),
                chance_mutation: data
                    .core
                    .chance_mutation
                    .cloned()
                    .unwrap_or_else(|| defaults.chance_mutation.clone()),
                static_rigid_body: data
                    .physics
                    .static_rigid_body
                    .cloned()
                    .unwrap_or(defaults.static_rigid_body),
                palette,
                gradient,
                texture,
                burns: data
                    .reactions
                    .burns
                    .cloned()
                    .unwrap_or_else(|| defaults.burns.clone()),
                contact_reaction: data
                    .reactions
                    .contact_reaction
                    .cloned()
                    .unwrap_or_else(|| defaults.contact_reaction.clone()),
                corrosive: data
                    .reactions
                    .corrosive
                    .cloned()
                    .unwrap_or_else(|| defaults.corrosive.clone()),
                corrodible: data
                    .reactions
                    .corrodible
                    .copied()
                    .unwrap_or(defaults.corrodible),
            };

            new_state.map.insert(*entity, particle_data);
        }
    }

    commands.insert_resource(new_state);
}

fn refresh_particle_labels(
    mut commands: Commands,
    particles: Query<(&ParticleType, Option<&ParticleCategory>), With<ParticleType>>,
) {
    let mut labels = ParticleCategoryLabels::default();
    for (ptype, category) in &particles {
        let cat = category.map(|c| c.0.as_str()).unwrap_or("Other");
        labels.push(cat, ptype.name.to_string());
    }
    labels.sort();
    commands.insert_resource(labels);
}

fn condition_particle_movement_changed(
    movement: Query<Entity, Changed<Movement>>,
    categories: Query<Entity, Changed<ParticleCategory>>,
) -> bool {
    !movement.is_empty() || !categories.is_empty()
}

/// Buffered name for the currently-selected particle.
///
/// The Name text field in the editor writes to this buffer instead of the live
/// `ParticleType` component, so partially-typed values can't collide with another
/// particle's registered name (which would otherwise clobber the other particle's
/// identity). The buffer is committed to the entity by the "Save Particle" button.
#[derive(Resource, Default)]
pub struct NameDraft {
    pub entity: Option<Entity>,
    pub name: String,
}

fn condition_should_refresh_name_draft(
    selected: Option<Res<SelectedParticle>>,
    draft: Res<NameDraft>,
    query: Query<(), With<ParticleType>>,
) -> bool {
    match selected {
        None => draft.entity.is_some(),
        Some(s) => {
            if draft.entity != Some(s.0) {
                return true;
            }
            // Selected entity vanished — clear the stale draft once.
            !query.contains(s.0) && (draft.entity.is_some() || !draft.name.is_empty())
        }
    }
}

fn refresh_name_draft(
    selected: Option<Res<SelectedParticle>>,
    mut draft: ResMut<NameDraft>,
    query: Query<&ParticleType>,
) {
    let Some(selected) = selected else {
        draft.entity = None;
        draft.name.clear();
        return;
    };

    match query.get(selected.0) {
        Ok(particle_type) => {
            draft.entity = Some(selected.0);
            draft.name = particle_type.name.to_string();
        }
        Err(_) => {
            draft.entity = None;
            draft.name.clear();
        }
    }
}
