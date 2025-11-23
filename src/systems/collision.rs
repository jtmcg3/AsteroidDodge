use bevy::prelude::*;
use avian2d::prelude::CollisionStart;
use crate::components::*;
use crate::resources::*;

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
        // Check both orderings of the collision
        // Rust Concept: Helper function to keep code DRY (Don't Repeat Yourself)
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
    }
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
        crate::systems::player::spawn_player(commands, asset_server, physics_config);
    }
}
