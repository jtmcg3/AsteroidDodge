use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_hanabi::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::asteroid::spawn_asteroid_entity;

/// Handle collisions between player and asteroids
/// 
/// Rust Concept: Event-driven systems
/// Avian emits collision events that we can listen to
pub fn handle_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(Entity, &mut Health), With<Player>>,
    asteroid_query: Query<(Entity, &AsteroidSize), With<Asteroid>>,
) {
    // Rust Concept: Event iteration
    // MessageReader provides an iterator over new events since last frame
    for event in collision_events.read() {
        let entity1 = event.body1.unwrap();
        let entity2 = event.body2.unwrap();
        // Try to identify which entity is the player and which is the asteroid
        // Rust Concept: Pattern matching with if let
        let collision_data = 
            if let Ok((player_ent, _)) = player_query.get(entity1) {
                if let Ok((asteroid_ent, size)) = asteroid_query.get(entity2) {
                    Some((player_ent, asteroid_ent, *size))
                } else {
                    None
                }
            } else if let Ok((player_ent, _)) = player_query.get(entity2) {
                if let Ok((asteroid_ent, size)) = asteroid_query.get(entity1) {
                    Some((player_ent, asteroid_ent, *size))
                } else {
                    None
                }
            } else {
                None
            };
        
        // Rust Concept: Using if let Some pattern
        if let Some((player_ent, asteroid_ent, size)) = collision_data {
            // Get mutable access to health
            if let Ok((_, mut health)) = player_query.get_mut(player_ent) {
                // Apply damage to player
                health.damage(size.damage());
                
                // Despawn the asteroid
                commands.entity(asteroid_ent).despawn();
                
                // Check if player is dead
                if health.is_dead() {
                    commands.entity(player_ent).despawn();
                    game_state.is_game_over = true;
                }
            }
        }
    }
}

// Let me rewrite this more clearly - the above is too complex for a tutorial!

/// Handle collisions between player and asteroids (simplified version)
/// 
/// Rust Concept: Breaking complex logic into helper functions
pub fn handle_collisions_simple(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(Entity, &mut Health), With<Player>>,
    asteroid_query: Query<(Entity, &AsteroidSize), With<Asteroid>>,
) {
    for event in collision_events.read() {
        let entity1 = event.body1.unwrap();
        let entity2 = event.body2.unwrap();
        
        // Check Player-Asteroid
        if let Some(collision) = check_player_asteroid_collision(
            entity1,
            entity2,
            &player_query,
            &asteroid_query,
        ) {
            handle_collision(
                collision,
                &mut commands,
                &mut player_query,
                &mut game_state,
            );
        }
        
        // Check Projectile-Asteroid
        // We can just despawn both for now
        // Rust Concept: Querying for specific components to identify collision type
        // We need a query for projectiles to check if one of the entities is a projectile
        // But we can't easily pass it into this helper function structure without changing signatures
        // So let's do it inline for now or add a helper
    }
}

/// Handle collisions between projectiles and asteroids
pub fn handle_projectile_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    mut game_state: ResMut<GameState>,
    projectile_query: Query<(Entity, &LinearVelocity), With<Projectile>>,
    asteroid_query: Query<(Entity, &AsteroidSize, &Transform, &LinearVelocity), With<Asteroid>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in collision_events.read() {
        let entity1 = event.body1.unwrap();
        let entity2 = event.body2.unwrap();
        
        let projectile_entity;
        let projectile_velocity;
        let asteroid_entity;
        let asteroid_size;
        let asteroid_position;
        let asteroid_velocity;
        
        if let Ok((proj_ent, proj_vel)) = projectile_query.get(entity1) {
            projectile_entity = proj_ent;
            projectile_velocity = proj_vel.0;
            if let Ok((ast_ent, size, transform, ast_vel)) = asteroid_query.get(entity2) {
                asteroid_entity = ast_ent;
                asteroid_size = size;
                asteroid_position = transform.translation;
                asteroid_velocity = ast_vel.0;
            } else {
                continue;
            }
        } else if let Ok((proj_ent, proj_vel)) = projectile_query.get(entity2) {
            projectile_entity = proj_ent;
            projectile_velocity = proj_vel.0;
            if let Ok((ast_ent, size, transform, ast_vel)) = asteroid_query.get(entity1) {
                asteroid_entity = ast_ent;
                asteroid_size = size;
                asteroid_position = transform.translation;
                asteroid_velocity = ast_vel.0;
            } else {
                continue;
            }
        } else {
            continue;
        }

        // Collision confirmed
        commands.entity(projectile_entity).despawn();
        commands.entity(asteroid_entity).despawn();
        
        // Add score based on asteroid size
        let score_value = match asteroid_size {
            AsteroidSize::Small => 100,
            AsteroidSize::Medium => 50,
            AsteroidSize::Large => 20,
        };
        game_state.score += score_value;
        
        // Spawn explosion particle effect
        spawn_explosion(&mut commands, &mut effects, asteroid_position);

        // 2. Determine children
        let children_sizes = match asteroid_size {
            AsteroidSize::Large => Some((AsteroidSize::Medium, AsteroidSize::Small)),
            AsteroidSize::Medium => Some((AsteroidSize::Small, AsteroidSize::Small)),
            AsteroidSize::Small => None,
        };
        if let Some((size1, size2)) = children_sizes {
            // 3. Calculate Velocities
            // Impact influence (projectile pushes asteroid)
            let impact_impulse = projectile_velocity * 0.4; // 40% of projectile speed transfers
            let base_velocity = asteroid_velocity + impact_impulse;
            // Split force (perpendicular to direction)
            let split_dir = Vec2::new(-base_velocity.y, base_velocity.x).normalize_or_zero();
            let split_speed = 100.0; // Adjust as needed
            
            let vel1 = base_velocity + (split_dir * split_speed);
            let vel2 = base_velocity - (split_dir * split_speed);
            // 4. Spawn Children
            // Note: You'll need to pass meshes/materials resources to this system
            spawn_asteroid_entity(&mut commands, &mut meshes, &mut materials, asteroid_position, vel1, size1);
            spawn_asteroid_entity(&mut commands, &mut meshes, &mut materials, asteroid_position, vel2, size2);
        }
    }
}   

/// Spawn an explosion particle effect at the given position
fn spawn_explosion(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    position: Vec3,
) {
    use bevy_hanabi::prelude::*;
    
    // Create color gradient for explosion
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.2, 1.0)); // Bright yellow
    gradient.add_key(0.3, Vec4::new(1.0, 0.4, 0.1, 1.0)); // Orange
    gradient.add_key(1.0, Vec4::new(0.3, 0.1, 0.0, 0.0)); // Dark red fade
    
    // Create module for expressions
    let mut module = Module::default();
    
    // Spawn particles in a sphere surface
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(5.0),
        dimension: ShapeDimension::Surface,
    };
    
    // Particles shoot outward
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(100.0),
    };
    
    let lifetime = module.lit(0.6);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    
    // Fast spawn rate for burst effect
    let spawner = SpawnerSettings::rate(200.0.into());
    
    let effect = EffectAsset::new(32768, spawner, module)
        .with_name("explosion")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec3::new(4.0, 4.0, 1.0)),
            screen_space_size: false,
        });
    
    let effect_handle = effects.add(effect);
    
    // Spawn the effect entity - will spawn particles for 0.3s then despawn
    commands.spawn((
        Name::new("Explosion"),
        ParticleEffect::new(effect_handle),
        Transform::from_translation(position),
        // Despawn after short time
        Lifetime::new(0.3),
    ));
}

/// Helper struct to represent a player-asteroid collision
/// 
/// Rust Concept: Custom types for clarity
struct PlayerAsteroidCollision {
    player_entity: Entity,
    asteroid_entity: Entity,
    damage: f32,
}

/// Check if two entities represent a player-asteroid collision
/// 
/// Rust Concept: Returning Option for "maybe found" results
fn check_player_asteroid_collision(
    entity1: Entity,
    entity2: Entity,
    player_query: &Query<(Entity, &mut Health), With<Player>>,
    asteroid_query: &Query<(Entity, &AsteroidSize), With<Asteroid>>,
) -> Option<PlayerAsteroidCollision> {
    // Try entity1 as player, entity2 as asteroid
    if player_query.get(entity1).is_ok() {
        if let Ok((asteroid_ent, size)) = asteroid_query.get(entity2) {
            return Some(PlayerAsteroidCollision {
                player_entity: entity1,
                asteroid_entity: asteroid_ent,
                damage: size.damage(),
            });
        }
    }
    
    // Try entity2 as player, entity1 as asteroid
    if player_query.get(entity2).is_ok() {
        if let Ok((asteroid_ent, size)) = asteroid_query.get(entity1) {
            return Some(PlayerAsteroidCollision {
                player_entity: entity2,
                asteroid_entity: asteroid_ent,
                damage: size.damage(),
            });
        }
    }
    
    None
}

/// Handle a confirmed player-asteroid collision
/// 
/// Rust Concept: Separation of concerns
/// This function only handles the collision response
fn handle_collision(
    collision: PlayerAsteroidCollision,
    commands: &mut Commands,
    player_query: &mut Query<(Entity, &mut Health), With<Player>>,
    game_state: &mut ResMut<GameState>,
) {
    // Get player health (we know it exists because we just checked)
    // Rust Concept: unwrap() when we're certain it won't panic
    let (player_entity, mut health) = player_query
        .get_mut(collision.player_entity)
        .unwrap();
    
    // Apply damage
    health.damage(collision.damage);
    
    // Remove asteroid
    //commands.entity(collision.asteroid_entity).despawn();
    
    // Check for game over
    if health.is_dead() {
        commands.entity(player_entity).despawn();
        game_state.is_game_over = true;
    }
}

/// Show game over screen
/// 
/// Rust Concept: State-dependent systems
/// This only runs when certain conditions are met
pub fn show_game_over(
    mut commands: Commands,
    game_state: Res<GameState>,
    query: Query<Entity, With<GameOverText>>,
) {
    // Only show if game is over and text doesn't exist
    if !game_state.is_game_over || !query.is_empty() {
        return;
    }
    
    commands.spawn((
        Text::new("GAME OVER\nPress R to Restart"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(20.0),
            top: Val::Percent(30.0),
            ..default()
        },
        GameOverText,
    ));
}

#[derive(Component)]
pub(crate) struct GameOverText;

/// Handle restart input
pub fn handle_restart(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    physics_config: Res<PhysicsConfig>,
    game_over_query: Query<Entity, With<GameOverText>>,
    asteroid_query: Query<Entity, With<Asteroid>>,
    effects: ResMut<Assets<EffectAsset>>,
) {
    if !game_state.is_game_over {
        return;
    }
    
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Reset game state
        game_state.is_game_over = false;
        game_state.score = 0;
        
        // Remove game over text
        for entity in &game_over_query {
            commands.entity(entity).despawn();
        }
        
        // Clear all asteroids
        // Rust Concept: Batch operations with for loops
        for entity in &asteroid_query {
            commands.entity(entity).despawn();
        }
        
        // Respawn player
        crate::systems::player::spawn_player(commands, asset_server, physics_config, effects);
    }
}
