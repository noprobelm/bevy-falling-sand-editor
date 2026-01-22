//! Sets up a custom log layer for capturing logs, exposing them as a resource within the ECS.
use bevy::log::BoxedLayer;
use bevy::log::tracing_subscriber::Layer;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

/// Creates a custom log layer and inserts the `LogCapture` resource into the app.
pub fn console_capture_layer(app: &mut App) -> Option<BoxedLayer> {
    let log_capture = LogCapture::default();
    let layer = ConsoleCaptureLayer {
        logs: log_capture.logs.clone(),
    };
    app.insert_resource(log_capture);
    Some(Box::new(layer) as BoxedLayer)
}

#[derive(Resource, Clone, Default)]
pub struct LogCapture {
    logs: Arc<Mutex<Vec<String>>>,
}

impl LogCapture {
    /// Drain all captured logs, returning them and clearing the internal buffer.
    #[allow(unused)]
    pub fn drain(&self) -> Vec<String> {
        self.logs
            .lock()
            .map(|mut logs| std::mem::take(&mut *logs))
            .unwrap_or_default()
    }

    /// Returns the number of captured logs.
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.logs.lock().map(|logs| logs.len()).unwrap_or(0)
    }

    /// Returns true if there are no captured logs.
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// `Layer` that captures logs to a shared buffer intended for use by the console.
struct ConsoleCaptureLayer {
    logs: Arc<Mutex<Vec<String>>>,
}

impl<S: tracing::Subscriber> Layer<S> for ConsoleCaptureLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: bevy::log::tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = StringVisitor::default();
        event.record(&mut visitor);

        if let Ok(mut logs) = self.logs.lock() {
            logs.push(format!("[{}] {}", event.metadata().level(), visitor.0));
        }
    }
}

#[derive(Default)]
struct StringVisitor(String);

impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = format!("{:?}", value);
        }
    }
}
