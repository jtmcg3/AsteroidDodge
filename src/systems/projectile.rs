use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// Move projectiles forward
pub fn move_projectiles(
    mut query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
    config: Res<PhysicsConfig>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Projectiles move in their local Y direction (forward)
        let forward = (transform.rotation * Vec3::Y).truncate();
        transform.translation += (forward * config.projectile_speed * time.delta_secs()).extend(0.0);
    }
}

/// Cleanup expired projectiles
pub fn cleanup_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
