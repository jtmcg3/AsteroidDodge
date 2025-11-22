use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::*;
use crate::resources::*;

/// Spawn the player entity
/// 
/// Rust Concept: Commands pattern in Bevy
/// Commands queue entity creation/deletion to happen after the system runs
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        // Visual representation
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        // Game components
        Player,
        Health::new(100.0),
        Velocity::new(0.0, 0.0),
        // Physics components (from Avian)
        // Rust Concept: Tuple syntax in spawn
        // Each item in the tuple becomes a component
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),  // Simple circle collider for the player
        // Rust Concept: Type inference
        // Rust infers the Mass type from context
        Mass(2.0),
        // Lock rotation so player stays upright
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
    ));
}

/// Handle player movement with keyboard input
/// 
/// Rust Concept: Multiple query parameters
/// We can query different entity sets in the same system
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    physics_config: Res<PhysicsConfig>,
    _time: Res<Time>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    // Rust Concept: Early return pattern
    // If no player exists, just return
    let Ok(mut linear_velocity) = query.single_mut() else {
        return;
    };
    
    // Calculate movement direction from input
    // Rust Concept: Accumulator pattern
    let mut direction = Vec2::ZERO;
    
    // Rust Concept: if expressions (not statements)
    // Each if can be part of a larger expression
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    
    // Normalize direction to prevent faster diagonal movement
    // Rust Concept: Method chaining with conditional logic
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    
    // Apply velocity
    // Rust Concept: Direct field access through DerefMut
    // LinearVelocity can be treated like a Vec2
    linear_velocity.x = direction.x * physics_config.player_speed;
    linear_velocity.y = direction.y * physics_config.player_speed;
}

/// Keep player within screen bounds
/// 
/// Rust Concept: Mutable queries with bounds checking
pub fn constrain_player_position(
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    
    // Screen bounds (800x600 window)
    const MIN_X: f32 = -385.0;
    const MAX_X: f32 = 385.0;
    const MIN_Y: f32 = -285.0;
    const MAX_Y: f32 = 285.0;
    
    // Rust Concept: Clamp method for bounds
    // This is more elegant than nested if statements
    transform.translation.x = transform.translation.x.clamp(MIN_X, MAX_X);
    transform.translation.y = transform.translation.y.clamp(MIN_Y, MAX_Y);
}

/// Display player health (simple text for now)
/// 
/// Rust Concept: One-shot systems for UI setup
pub fn setup_health_display(mut commands: Commands) {
    commands.spawn((
        Text::new("Health: 100"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HealthDisplay,
    ));
}

/// Marker component for health UI
#[derive(Component)]
pub(crate) struct HealthDisplay;

/// Update health display
/// 
/// Rust Concept: Optional query results
/// Using Option<> for queries that might not have results
pub fn update_health_display(
    player_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthDisplay>>,
) {
    let Ok(health) = player_query.single() else {
        return;
    };
    
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };
    
    // Rust Concept: String formatting with format! macro
    *text = Text::new(format!("Health: {:.0}", health.current()));
}
