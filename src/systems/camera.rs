use crate::components::{Camera, DamageSource};
use crate::events::DamageEvent;
use crate::resources::{CameraTarget, ScreenShake};
use bevy::prelude::*;
use rand::Rng;

/// Setup the camera
///
/// Rust Concept: Simple startup system
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera));
}

pub fn trigger_screen_shake(
    mut events: MessageReader<DamageEvent>,
    mut shaker: ResMut<ScreenShake>,
) {
    for event in events.read() {
        shaker.trauma += ({
            match event.source_type {
                DamageSource::AsteroidEntity(size) => size.damage() / 60.0_f32,
            }
        })
        .min(1.0);
        info!("Trauma: {}, Offset: {:?}", shaker.trauma, shaker.offset);
    }
}

pub fn update_screen_shake(mut shaker: ResMut<ScreenShake>, time: Res<Time>) {
    //if active: calculate random offset based on intensity
    if shaker.trauma > 0.001 {
        let magnitude = shaker.trauma * shaker.trauma * shaker.max_offset;
        let mut rng = rand::rng();
        shaker.offset = Vec3::new(
            rng.random_range(-magnitude..magnitude),
            rng.random_range(-magnitude..magnitude),
            0.0,
        );
        info!("Trauma: {}, Offset: {:?}", shaker.trauma, shaker.offset);
        shaker.trauma = (shaker.trauma - shaker.decay_rate * time.delta_secs()).max(0.0);
        info!("Trauma: {}, Offset: {:?}", shaker.trauma, shaker.offset);
    } else {
        shaker.offset = Vec3::ZERO;
        shaker.trauma = 0.0;
    }
}

pub fn apply_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    target: ResMut<CameraTarget>,
    shaker: ResMut<ScreenShake>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    transform.translation = target.position + shaker.offset;
    info!(
        "Applied camera position: {:?} to {:?}",
        target.position + shaker.offset,
        transform.translation
    );
}
