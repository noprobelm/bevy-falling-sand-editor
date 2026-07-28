//! Writes the user-facing particle manifest after particle definitions are persisted.

use std::fs::File;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy_falling_sand::prelude::{ParticleType, ParticleTypesPersistedSignal};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::super::{ParticleCategory, ParticleName};

const PARTICLE_MANIFEST_SCHEMA_VERSION: u32 = 1;
const PARTICLE_MANIFEST_HASH_DOMAIN: &[u8] = b"bevy-falling-sand-particle-manifest-v1\0";

#[derive(Serialize)]
struct ParticleManifest {
    schema_version: u32,
    version: String,
    particles: Vec<ParticleManifestEntry>,
}

#[derive(Serialize, Debug)]
struct ParticleManifestEntry {
    id: usize,
    name: String,
    category: String,
}

pub(super) fn write_particle_manifest(
    mut persisted: MessageReader<ParticleTypesPersistedSignal>,
    particle_types: Query<(
        &ParticleType,
        Option<&ParticleName>,
        Option<&ParticleCategory>,
    )>,
) {
    let paths: Vec<PathBuf> = persisted.read().map(|message| message.0.clone()).collect();
    if paths.is_empty() {
        return;
    }

    let mut particles: Vec<ParticleManifestEntry> = particle_types
        .iter()
        .map(|(particle_type, name, category)| ParticleManifestEntry {
            id: particle_type.id().get(),
            name: name
                .map(|name| name.0.clone())
                .unwrap_or_else(|| format!("Particle {}", particle_type.id().get())),
            category: category
                .map(|category| category.0.clone())
                .unwrap_or_else(|| "Other".to_string()),
        })
        .collect();
    particles.sort_by(|left, right| {
        left.category
            .cmp(&right.category)
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| left.id.cmp(&right.id))
    });

    let version = manifest_version(&particles);
    let manifest = ParticleManifest {
        schema_version: PARTICLE_MANIFEST_SCHEMA_VERSION,
        version,
        particles,
    };

    for particle_types_path in paths {
        let manifest_path = manifest_path_for(&particle_types_path);
        match File::create(&manifest_path)
            .map_err(serde_json::Error::io)
            .and_then(|file| serde_json::to_writer_pretty(file, &manifest))
        {
            Ok(()) => info!("Particle manifest saved to: {:?}", manifest_path),
            Err(error) => error!(
                "Failed to save particle manifest to {:?}: {}",
                manifest_path, error
            ),
        }
    }
}

fn manifest_version(particles: &[ParticleManifestEntry]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(PARTICLE_MANIFEST_HASH_DOMAIN);
    for particle in particles {
        hasher.update((particle.id as u64).to_le_bytes());
        hash_string(&mut hasher, &particle.name);
        hash_string(&mut hasher, &particle.category);
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn hash_string(hasher: &mut Sha256, value: &str) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value.as_bytes());
}

fn manifest_path_for(particle_types_path: &Path) -> PathBuf {
    let file_name = particle_types_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("particles");
    let stem = file_name
        .strip_suffix(".scn.ron")
        .or_else(|| file_name.strip_suffix(".ron"))
        .unwrap_or(file_name);
    particle_types_path.with_file_name(format!("{stem}.particle-manifest.json"))
}
