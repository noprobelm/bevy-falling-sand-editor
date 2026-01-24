use bevy::prelude::*;
use bevy_falling_sand::prelude::{
    DespawnAllParticlesSignal, DespawnDynamicParticlesSignal, DespawnParticleTypeChildrenSignal,
    DespawnStaticParticlesSignal,
};

use crate::directive::Directive;

pub struct ParticlesDirectivePlugin;

impl Plugin for ParticlesDirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_despawn_dynamic_particles)
            .add_observer(on_despawn_static_particles)
            .add_observer(on_despawn_named_particles)
            .add_observer(on_despawn_all_particles);
    }
}

#[derive(Default)]
pub struct ParticlesDirective;

impl Directive for ParticlesDirective {
    fn name(&self) -> &'static str {
        "particles"
    }

    fn description(&self) -> &'static str {
        "Particle system operations"
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![
            Box::new(ParticlesResetCommand),
            Box::new(ParticlesDespawnCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesResetCommand;

impl Directive for ParticlesResetCommand {
    fn name(&self) -> &'static str {
        "reset"
    }

    fn description(&self) -> &'static str {
        "Reset particle-related components"
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![
            Box::new(ParticlesResetWallDirective),
            Box::new(ParticlesResetDynamicDirective),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesResetWallDirective;

impl Directive for ParticlesResetWallDirective {
    fn name(&self) -> &'static str {
        "wall"
    }

    fn description(&self) -> &'static str {
        "Reset wall particles"
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![Box::new(ParticlesResetWallAllCommand)]
    }
}

#[derive(Default)]
pub struct ParticlesResetDynamicDirective;

impl Directive for ParticlesResetDynamicDirective {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn description(&self) -> &'static str {
        "Reset dynamic particles"
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![Box::new(ParticlesResetDynamicAllCommand)]
    }
}

#[derive(Default)]
pub struct ParticlesResetWallAllCommand;

impl Directive for ParticlesResetWallAllCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Reset all wall particles"
    }

    fn run(&self, _args: &[String], _commands: &mut Commands) {
        info!("Resetting all wall particles to parent data");
    }
}

#[derive(Default)]
pub struct ParticlesResetDynamicAllCommand;

impl Directive for ParticlesResetDynamicAllCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Reset all dynamic particles"
    }

    fn run(&self, _args: &[String], _commands: &mut Commands) {
        info!("Resetting all dynamic particles to parent data")
    }
}

#[derive(Default)]
pub struct ParticlesDespawnCommand;

impl Directive for ParticlesDespawnCommand {
    fn name(&self) -> &'static str {
        "despawn"
    }

    fn description(&self) -> &'static str {
        "Despawn particles from the world"
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![
            Box::new(ParticlesDespawnDynamicDirective),
            Box::new(ParticlesDespawnStaticDirective),
            Box::new(ParticlesDespawnAllDirective),
            Box::new(ParticlesDespawnNamedDirective),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesDespawnDynamicDirective;

impl Directive for ParticlesDespawnDynamicDirective {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn description(&self) -> &'static str {
        "Despawn dynamic particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all dynamic particles from the world");
        commands.trigger(DespawnDynamicParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnStaticDirective;

impl Directive for ParticlesDespawnStaticDirective {
    fn name(&self) -> &'static str {
        "static"
    }

    fn description(&self) -> &'static str {
        "Despawn static particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all static particles from the world");
        commands.trigger(DespawnStaticParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnAllDirective;

impl Directive for ParticlesDespawnAllDirective {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Despawn all particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all particles from the world");
        commands.trigger(DespawnAllParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnNamedDirective;

impl Directive for ParticlesDespawnNamedDirective {
    fn name(&self) -> &'static str {
        "named"
    }

    fn description(&self) -> &'static str {
        "Despawn all particles of specified name from the world"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let name = args.join(" ");
        info!("Despawning all '{}' particles from the world", name);
        commands.trigger(DespawnParticleTypeChildrenSignal::from_name(&name));
    }
}

fn on_despawn_dynamic_particles(
    _trigger: On<DespawnDynamicParticlesSignal>,
    mut msgw_clear_dynamic_particles: MessageWriter<DespawnDynamicParticlesSignal>,
) {
    msgw_clear_dynamic_particles.write(DespawnDynamicParticlesSignal);
}

fn on_despawn_static_particles(
    _trigger: On<DespawnStaticParticlesSignal>,
    mut msgw_clear_static_particles: MessageWriter<DespawnStaticParticlesSignal>,
) {
    msgw_clear_static_particles.write(DespawnStaticParticlesSignal);
}

fn on_despawn_all_particles(
    _trigger: On<DespawnAllParticlesSignal>,
    mut msgw_clear_particle_map: MessageWriter<DespawnAllParticlesSignal>,
) {
    msgw_clear_particle_map.write(DespawnAllParticlesSignal);
}

fn on_despawn_named_particles(
    trigger: On<DespawnParticleTypeChildrenSignal>,
    mut evw_clear_particle_type_children: MessageWriter<DespawnParticleTypeChildrenSignal>,
) {
    evw_clear_particle_type_children.write(trigger.event().clone());
}
