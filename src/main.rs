use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod systems;
mod resources;

use systems::*;
use resources::*;

fn main() {
    // Bevy's default plugins, window rendering input etc
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "Asteroid Dodge".to_string(),
                resolution: WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        // Our internally developed resources (global game state)
        .init_resource::<GameState>()
        .init_resource::<SpawnTimer>()
        // Startup Systems
        .add_systems(Startup, (setup_camera, spawn_player))
        // Update Systems (run every frame)
        .add_systems(Update, (
            player_movement,
            spawn_asteroids,
            move_asteroids,
            check_collisions,
            cleanup_offscreen,
        ))
        .run();
}
