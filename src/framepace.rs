use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

pub struct AdaptiveFramepacePlugin;

impl Plugin for AdaptiveFramepacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .init_resource::<AdaptiveFramepaceConfig>()
            .insert_resource(FramepaceSettings {
                limiter: Limiter::from_framerate(60.0),
            })
            .add_systems(Update, adaptive_framerate_limiter);
    }
}

#[derive(Resource)]
pub struct AdaptiveFramepaceConfig {
    /// Target framerate when performance is good
    pub target_fps: f64,
    /// Minimum framerate limit we'll drop to
    pub min_fps: f64,
    /// How far below CURRENT limit before we consider struggling
    /// (e.g., 5.0 means if limited to 60 and getting 54, we're struggling)
    pub struggle_threshold: f64,
    /// Current effective limit
    pub current_limit: f64,
    /// Rolling average of recent FPS measurements
    fps_history: Vec<f64>,
    /// How many samples to keep for averaging
    pub history_size: usize,
    /// Frames at target before we try recovering
    frames_at_target: u32,
    /// Frames struggling before we drop
    frames_struggling: u32,
    /// Frames needed at target before attempting recovery
    pub frames_before_recovery: u32,
    /// Frames needed struggling before dropping limit
    pub frames_before_drop: u32,
}

impl Default for AdaptiveFramepaceConfig {
    fn default() -> Self {
        Self {
            target_fps: 60.0,
            min_fps: 30.0,
            struggle_threshold: 5.0,
            current_limit: 60.0,
            fps_history: Vec::with_capacity(60),
            history_size: 60,
            frames_at_target: 0,
            frames_struggling: 0,
            frames_before_recovery: 180, // ~3 seconds at limit before recovering
            frames_before_drop: 120,     // ~2 seconds of struggling before dropping
        }
    }
}

fn adaptive_framerate_limiter(
    diagnostics: Res<DiagnosticsStore>,
    mut config: ResMut<AdaptiveFramepaceConfig>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        return;
    };

    let Some(current_fps) = fps_diagnostic.smoothed() else {
        return;
    };

    // Update history
    if config.fps_history.len() >= config.history_size {
        config.fps_history.remove(0);
    }
    config.fps_history.push(current_fps);

    // Need enough samples before making decisions
    if config.fps_history.len() < config.history_size {
        return;
    }

    let avg_fps: f64 = config.fps_history.iter().sum::<f64>() / config.fps_history.len() as f64;

    // Struggling = can't hit our CURRENT limit (not target)
    // If we're limited to 50 and averaging 50, that's fine - we're hitting our limit
    let struggling = avg_fps < (config.current_limit - config.struggle_threshold);

    // Hitting limit = we're within 1 FPS of current limit
    let hitting_limit = avg_fps >= (config.current_limit - 1.0);

    if struggling && config.current_limit > config.min_fps {
        config.frames_at_target = 0;
        config.frames_struggling += 1;

        if config.frames_struggling >= config.frames_before_drop {
            // Drop limit to match what we can actually achieve
            let new_limit = (avg_fps - 1.0).max(config.min_fps);
            config.current_limit = new_limit;
            framepace.limiter = Limiter::from_framerate(config.current_limit);
            config.frames_struggling = 0;
        }
    } else if hitting_limit && config.current_limit < config.target_fps {
        config.frames_struggling = 0;
        config.frames_at_target += 1;

        if config.frames_at_target >= config.frames_before_recovery {
            // Try bumping up the limit
            let new_limit = (config.current_limit + 5.0).min(config.target_fps);
            config.current_limit = new_limit;
            framepace.limiter = Limiter::from_framerate(config.current_limit);
            config.frames_at_target = 0;
        }
    } else {
        // In between - not struggling but not quite hitting limit either
        config.frames_struggling = 0;
        config.frames_at_target = 0;
    }
}
