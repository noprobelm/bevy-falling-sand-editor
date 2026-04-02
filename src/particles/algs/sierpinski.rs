use bevy::prelude::*;
use bevy_falling_sand::core::SpawnParticleSignal;

use crate::brush::SelectedParticle;

pub mod carpet {
    use super::*;

    #[derive(Event, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct SpawnSierpinskiCarpetEvent {
        pub center: IVec2,
        pub depth: u32,
    }

    pub fn spawn_sierpinski_carpet(
        trigger: On<SpawnSierpinskiCarpetEvent>,
        mut msgw_spawn_particles: MessageWriter<SpawnParticleSignal>,
        selected_particle: Single<&SelectedParticle>,
    ) {
        let positions = sierpinski_carpet(trigger.event().center, trigger.event().depth);
        positions.iter().for_each(|pos| {
            msgw_spawn_particles.write(SpawnParticleSignal::new(selected_particle.0.clone(), *pos));
        })
    }

    fn sierpinski_carpet(center: IVec2, depth: u32) -> Vec<IVec2> {
        let size = 3u32.pow(depth);
        let offset = IVec2::splat(size as i32 / 2);
        let mut filled = vec![];
        (0..size).for_each(|y| {
            (0..size).for_each(|x| {
                if is_filled(x, y) {
                    filled.push(IVec2::new(x as i32, y as i32) + center - offset);
                }
            })
        });
        filled
    }

    fn is_filled(x: u32, y: u32) -> bool {
        let mut x = x;
        let mut y = y;
        while x > 0 || y > 0 {
            if x % 3 == 1 && y % 3 == 1 {
                return false;
            }

            x /= 3;
            y /= 3;
        }
        true
    }
}

pub mod triangle {
    use super::*;

    #[derive(Event, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct SpawnSierpinskiTriangleEvent {
        pub center: IVec2,
        pub depth: u32,
    }

    pub fn spawn_sierpinski_triangle(
        trigger: On<SpawnSierpinskiTriangleEvent>,
        mut msgw_spawn_particles: MessageWriter<SpawnParticleSignal>,
        selected_particle: Single<&SelectedParticle>,
    ) {
        let positions = sierpinski_triangle(trigger.event().center, trigger.event().depth);
        positions.iter().for_each(|pos| {
            msgw_spawn_particles.write(SpawnParticleSignal::new(selected_particle.0.clone(), *pos));
        })
    }

    fn sierpinski_triangle(center: IVec2, depth: u32) -> Vec<IVec2> {
        let height = 2u32.pow(depth);
        let width = 2 * height - 1;
        let offset = IVec2::new(width as i32 / 2, height as i32 / 2);
        let mut filled = vec![];
        for row in 0..height {
            for col in 0..=row {
                // Pascal's triangle mod 2: C(row, col) is odd iff (col & row) == col
                if (col & row) == col {
                    let x = (height - 1 - row + 2 * col) as i32;
                    let y = (height - 1 - row) as i32;
                    filled.push(IVec2::new(x, y) + center - offset);
                }
            }
        }
        filled
    }
}
