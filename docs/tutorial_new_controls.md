# Tutorial: Implementing Physics-Based Controls

This tutorial will guide you through updating your spaceship controls to use a physics-based thruster system. Instead of setting velocity directly (arcade style), we will apply forces and torque to the ship, allowing for momentum and drift.

## Goal
We want to achieve the following control scheme:
- **Left Arrow**: Fire right-back thruster -> Pivot Left (CCW) + Accelerate Forward
- **Right Arrow**: Fire left-back thruster -> Pivot Right (CW) + Accelerate Forward
- **Up Arrow**: Fire both thrusters -> Accelerate Forward (Double force, no rotation)
- **Down Arrow**: Fire reverse thrusters -> Accelerate Backward

## Prerequisites
Ensure you have `avian2d` set up in your project (which you do!). We will use `ExternalForce` and `ExternalTorque` components.

## Step 1: Update Physics Configuration
First, let's add configuration for our new physics parameters so we can tune them easily.

Open `src/resources.rs` and update `PhysicsConfig`:

```rust
#[derive(Resource)]
pub struct PhysicsConfig {
    pub thruster_power: f32,    // Force applied by a single thruster
    pub rotation_power: f32,    // Torque applied by a single thruster
    pub reverse_power: f32,     // Force for reverse thrusters
    pub drag: f32,              // Linear damping (air resistance)
    pub angular_drag: f32,      // Angular damping (rotation resistance)
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            thruster_power: 10000.0, // Needs to be high for physics forces
            rotation_power: 50000.0, // Needs to be high for torque
            reverse_power: 5000.0,
            drag: 0.5,               // 0.0 = no drag (drifts forever), 1.0 = quick stop
            angular_drag: 2.0,       // Higher to stop spinning quickly
        }
    }
}
```

> [!NOTE]
> Physics forces often need much larger numbers than direct velocity values because they are affected by Mass and Delta Time.

## Step 2: Update Player Spawning
We need to change the components we spawn the player with.
1. Remove `LockedAxes::ROTATION_LOCKED` so the ship can turn.
2. Add `ConstantForce` and `ConstantTorque` to handle physics inputs (persistent forces).
3. Add `LinearDamping` and `AngularDamping` to control momentum.

Open `src/systems/player.rs` and modify `spawn_player`:

```rust
// Add these to your imports if not present
use avian2d::prelude::*; 

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<PhysicsConfig>) {
    commands.spawn((
        // ... (Sprite and other components stay the same)
        Sprite {
            image: asset_server.load("sprites/ship_G.png"),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Player,
        Health::new(100.0),
        
        // Physics Components
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(0.0, 20.0),
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
        ),
        Mass(2.0), // Mass affects how much Force is needed
        
        // NEW: Physics Control Components
        // Use ConstantForce for thrusters (we will update it every frame)
        ConstantForce::default(),
        ConstantTorque::default(),
        LinearDamping(config.drag),
        AngularDamping(config.angular_drag),
        
        // REMOVED: LockedAxes::ROTATION_LOCKED 
        CollisionEventsEnabled,
    ));
}
```

## Step 3: Implement Thruster Logic
Now for the fun part! We will rewrite `player_movement` to apply forces based on your thruster logic.

In `src/systems/player.rs`, replace `player_movement` with:

```rust
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<PhysicsConfig>,
    // We now query for ConstantForce/Torque and Transform
    mut query: Query<(&mut ConstantForce, &mut ConstantTorque, &Transform), With<Player>>,
) {
    let Ok((mut constant_force, mut constant_torque, transform)) = query.single_mut() else {
        return;
    };

    // Reset forces each frame (so they don't stick)
    constant_force.0 = Vec2::ZERO;
    constant_torque.0 = 0.0;

    // Determine which thrusters are active
    let mut left_thruster_active = false;
    let mut right_thruster_active = false;
    let mut reverse_active = false;

    // Control Mapping
    // Left Arrow -> Fires Right Thruster (pushes right side forward -> turns Left)
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        right_thruster_active = true;
    }
    // Right Arrow -> Fires Left Thruster (pushes left side forward -> turns Right)
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        left_thruster_active = true;
    }
    // Up Arrow -> Fires BOTH Thrusters
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        left_thruster_active = true;
        right_thruster_active = true;
    }
    // Down Arrow -> Reverse
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        reverse_active = true;
    }

    // Apply Physics
    // Get the forward direction vector from the ship's rotation
    // Assuming Y+ is "Forward" for your sprite
    // Note: Transform rotation is a Quat, we multiply by Vec3::Y and truncate to Vec2
    let forward = (transform.rotation * Vec3::Y).truncate();

    // Apply Left Thruster (Pushes Left side -> Turns Right + Moves Forward)
    if left_thruster_active {
        constant_force.0 += forward * config.thruster_power;
        constant_torque.0 -= config.rotation_power; // Negative torque = Clockwise (Right)
    }

    // Apply Right Thruster (Pushes Right side -> Turns Left + Moves Forward)
    if right_thruster_active {
        constant_force.0 += forward * config.thruster_power;
        constant_torque.0 += config.rotation_power; // Positive torque = Counter-Clockwise (Left)
    }

    // Note: If BOTH are active (Up Arrow), the Torques (+ and -) cancel out to 0,
    // and the Forces add up to 2x thruster_power. This is exactly what we want!

    // Apply Reverse
    if reverse_active {
        constant_force.0 -= forward * config.reverse_power;
    }
}
```

## Step 4: Verify and Tweak
1.  **Run the game**: `cargo run`
2.  **Test Controls**:
    - Press **Up**: You should fly straight.
    - Press **Left**: You should curve left.
    - Press **Right**: You should curve right.
    - Release keys: You should drift and slowly stop (due to Damping).
3.  **Tuning**:
    - If you spin too fast, lower `rotation_power` or increase `AngularDamping`.
    - If you drift too much, increase `LinearDamping`.
    - If you are too slow, increase `thruster_power`.

> [!TIP]
> **Screen Wrapping**: The current `constrain_player_position` system just clamps your position. With physics momentum, hitting this "invisible wall" might feel abrupt. Consider implementing a screen-wrapping system later where flying off the right side teleports you to the left side!
