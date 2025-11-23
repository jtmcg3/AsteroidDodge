// main.rs
//! Asteroid Dodge - A physics-based space survival game
//! 
//! This game demonstrates:
//! - Avian Physics integration with Bevy ECS
//! - Procedural polygon generation
//! - Event-driven collision handling
//! - Idiomatic Rust patterns

use bevy::prelude::*;
use avian2d::prelude::*;

mod components;
mod resources;
mod shapes;
mod systems;

use resources::*;
use systems::*;

fn main() {
    App::new()
        // Rust Concept: Plugin composition
        // Bevy apps are built by composing plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Dodge - Physics Edition".to_string(),
                resolution: bevy::window::WindowResolution::new(800, 600),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // Add Avian Physics plugin
        // Rust Concept: Plugin configuration
        .add_plugins(PhysicsPlugins::default())
        // Configure physics behavior
        // Rust Concept: Resource insertion for configuration
        .insert_resource(Gravity(Vec2::ZERO)) // No gravity in space!
        // Initialize our game resources
        .init_resource::<GameState>()
        .init_resource::<SpawnTimer>()
        .init_resource::<AsteroidSpawnConfig>()
        .init_resource::<PhysicsConfig>()
        .init_resource::<DifficultyConfig>()
        // Startup systems (run once at launch)
        // Rust Concept: System scheduling with tuples
        .add_systems(Startup, (
            setup_camera,
            spawn_background,
            spawn_player,
            setup_health_display,
        ))
        // Update systems (run every frame)
        // Rust Concept: System sets for organization
        .add_systems(Update, (
            // Input handling
            player_movement,
            handle_restart,
            // Spawning and cleanup
            spawn_asteroids,
            cleanup_offscreen,
            // Physics and collision (handled by Avian automatically)
            handle_collisions_simple,
            // UI updates
            update_health_display,
            show_game_over,
            // Rendering (debug visualization)
            //draw_asteroid_shapes,
        ))
        // Rust Concept: System ordering
        // We can specify that certain systems run before others
        .add_systems(Update, constrain_player_position.after(player_movement))
        .run();
}

/// Setup the camera
/// 
/// Rust Concept: Simple startup system
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Rust Concept: Conditional compilation for tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_builds() {
        // Rust Concept: Testing that the app can be constructed
        // This catches common configuration errors
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<GameState>();
        
        // If this doesn't panic, our setup is valid
        app.update();
    }
}
