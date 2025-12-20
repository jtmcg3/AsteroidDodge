// main.rs
//! Asteroid Dodge - A physics-based space survival game
//!
//! This game demonstrates:
//! - Avian Physics integration with Bevy ECS
//! - Procedural polygon generation
//! - Event-driven collision handling
//! - Idiomatic Rust patterns

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

mod components;
mod resources;
mod shapes;
mod systems;

use resources::*;
use systems::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() {
    App::new()
        // Rust Concept: Plugin composition
        // Bevy apps are built by composing plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Dodge".to_string(),
                #[cfg(not(target_arch = "wasm32"))]
                resolution: bevy::window::WindowResolution::new(800, 600),
                #[cfg(not(target_arch = "wasm32"))]
                resizable: false,

                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                #[cfg(target_arch = "wasm32")]
                prevent_default_event_handling: true,
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#game-canvas".to_string()),
                ..default()
            }),
            ..default()
        }))
        // Add Avian Physics plugin
        // Rust Concept: Plugin configuration
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(HanabiPlugin)
        // Configure physics behavior
        // Rust Concept: Resource insertion for configuration
        .insert_resource(Gravity(Vec2::ZERO)) // No gravity in space!
        // Initialize our game resources
        .init_state::<AppState>()
        .init_resource::<GameData>()
        .init_resource::<SpawnTimer>()
        .init_resource::<AsteroidSpawnConfig>()
        .init_resource::<PhysicsConfig>()
        .init_resource::<DifficultyConfig>()
        // Startup systems (run once at launch)
        // Rust Concept: System scheduling with tuples
        .add_systems(OnEnter(AppState::Menu), (setup_menu,))
        .add_systems(
            Update,
            (handle_menu_input,).run_if(in_state(AppState::Menu)),
        )
        .add_systems(OnEnter(AppState::Playing), (spawn_player,))
        // .add_systems(OnEnter(AppState::GameOver))
        .add_systems(
            Startup,
            (
                setup_camera,
                spawn_background,
                setup_health_display,
                setup_score_display,
                // WASM-specific: Add browser logging setup
                #[cfg(target_arch = "wasm32")]
                setup_browser_logging,
            ),
        )
        // Update systems (run every frame)
        // Rust Concept: System sets for organization
        .add_systems(
            Update,
            (
                // Input handling
                player_movement,
                player_fire,
                // Spawning and cleanup
                spawn_asteroids,
                cleanup_offscreen,
                move_projectiles,
                cleanup_projectiles,
                // Physics and collision (handled by Avian automatically)
                handle_collisions_simple,
                handle_projectile_collisions,
                // UI updates
                update_health_display,
                update_score_display,
                update_thruster_visuals,
                update_thruster_audio,
                // Rendering (debug visualization)
                //draw_asteroid_shapes,
            )
                .run_if(in_state(AppState::Playing)),
        )
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

// Required for WASM builds
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    // Initialize panic hook for better error messages in browser console
    console_error_panic_hook::set_once();

    main();
}

// Helper system for WASM logging setup
#[cfg(target_arch = "wasm32")]
fn setup_browser_logging() {
    use web_sys::console;
    console::log_1(&"Asteroid Dodge initialized for web!".into());
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
        app.add_plugins(MinimalPlugins).init_resource::<GameData>();

        // If this doesn't panic, our setup is valid
        app.update();
    }
}
