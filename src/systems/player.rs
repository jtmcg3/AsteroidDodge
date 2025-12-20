use crate::components::*;
use crate::resources::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

/// Create a thruster particle effect
fn create_thruster_effect(effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
    use bevy_hanabi::prelude::*;

    // Orange/yellow flame gradient
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0)); // Bright yellow
    gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.1, 0.8)); // Orange
    gradient.add_key(1.0, Vec4::new(0.8, 0.2, 0.0, 0.0)); // Dark orange fade

    let mut module = Module::default();

    // Spawn particles in a small cone
    let init_pos = SetPositionCone3dModifier {
        base_radius: module.lit(2.0),
        top_radius: module.lit(0.5),
        height: module.lit(5.0),
        dimension: ShapeDimension::Volume,
    };

    // Particles move along the cone axis (backward for thruster)
    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::NEG_Y), // Shoot backward
        speed: module.lit(50.0),
    };

    let lifetime = module.lit(0.3);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let spawner = SpawnerSettings::rate(80.0.into());

    let effect = EffectAsset::new(8192, spawner, module)
        .with_name("thruster")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec3::new(3.0, 3.0, 1.0)),
            screen_space_size: false,
        });

    effects.add(effect)
}

/// Spawn the player entity
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<PhysicsConfig>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Create thruster effect
    let thruster_effect = create_thruster_effect(&mut effects);
    let thruster_sound: Handle<AudioSource> =
        asset_server.load("audio/kenney_sci-fi-sounds/Audio/thrusterFire_004.ogg");

    commands
        .spawn((
            // Visual representation
            // Visual representation
            Sprite {
                image: asset_server.load("sprites/ship_G.png"),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            // Game components
            Player,
            Health::new(100.0),
            Velocity::new(0.0, 0.0),
            // Physics components
            RigidBody::Dynamic,
            Collider::triangle(
                Vec2::new(0.0, 20.0),
                Vec2::new(-20.0, -15.0),
                Vec2::new(20.0, -15.0),
            ),
            Mass(15.0),
            CollisionEventsEnabled,
            // Physics config
            ConstantForce::default(),
            ConstantTorque::default(),
            LinearDamping(config.drag),
            AngularDamping(config.angular_drag),
        ))
        .with_children(|children| {
            // Left main thruster (under left wing, fires backward)
            children.spawn((
                Name::new("LeftMainThruster"),
                ParticleEffect::new(thruster_effect.clone()),
                Transform::from_translation(Vec3::new(-12.0, -15.0, -1.0)),
                Thruster::Left,
                ThrusterType::Main,
            ));

            // Right main thruster (under right wing, fires backward)
            children.spawn((
                Name::new("RightMainThruster"),
                ParticleEffect::new(thruster_effect.clone()),
                Transform::from_translation(Vec3::new(12.0, -15.0, -1.0)),
                Thruster::Right,
                ThrusterType::Main,
            ));

            // Left reverse thruster (front left, fires forward)
            children.spawn((
                Name::new("LeftReverseThruster"),
                ParticleEffect::new(thruster_effect.clone()),
                Transform::from_translation(Vec3::new(-10.0, 15.0, -1.0))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)), // Rotate 180°
                Thruster::Left,
                ThrusterType::Reverse,
            ));

            // Right reverse thruster (front right, fires forward)
            children.spawn((
                Name::new("RightReverseThruster"),
                ParticleEffect::new(thruster_effect),
                Transform::from_translation(Vec3::new(10.0, 15.0, -1.0))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)), // Rotate 180°
                Thruster::Right,
                ThrusterType::Reverse,
            ));

            // Add the audio component
            children.spawn((
                AudioPlayer(thruster_sound),
                PlaybackSettings::LOOP.paused(),
                ThrusterAudio,
            ));
        });
}

/// Handle player movement with keyboard input
///
/// Rust Concept: Multiple query parameters
/// We can query different entity sets in the same system
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    physics_config: Res<PhysicsConfig>,
    mut query: Query<(&mut ConstantForce, &mut ConstantTorque, &Transform), With<Player>>,
) {
    // Rust Concept: Early return pattern
    // If no player exists, just return
    let Ok((mut constant_force, mut constant_torque, transform)) = query.single_mut() else {
        return;
    };

    // Calculate movement direction from input
    // Rust Concept: Accumulator pattern
    // Reset forces each frame since we are simulating thrusters
    constant_force.0 = Vec2::ZERO;
    constant_torque.0 = 0.0;

    // which thrusters are active
    let mut left_thruster_active = false;
    let mut right_thruster_active = false;
    let mut reverse_active = false;

    // Rust Concept: if expressions (not statements)
    // left arrow fires right thruster
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        right_thruster_active = true;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        left_thruster_active = true;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        left_thruster_active = true;
        right_thruster_active = true;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        reverse_active = true;
    }

    // Apply Physics

    let forward = (transform.rotation * Vec3::Y).truncate();

    //left thruster
    if left_thruster_active {
        constant_force.0 += forward * physics_config.thruster_force;
        constant_torque.0 -= physics_config.rotation_torque;
    }

    //right thruster
    if right_thruster_active {
        constant_force.0 += forward * physics_config.thruster_force;
        constant_torque.0 += physics_config.rotation_torque;
    }

    //reverse thruster
    if reverse_active {
        constant_force.0 -= forward * physics_config.reverse_thrust_force;
    }
}

pub fn update_thruster_audio(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&AudioSink, With<ThrusterAudio>>,
) {
    // We expect only one thruster audio entity
    let Ok(sink) = query.single() else {
        return;
    };
    // Check if any thruster key is pressed
    let is_thrusting = keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::ArrowRight)
        || keyboard.pressed(KeyCode::KeyD);
    if is_thrusting {
        if sink.is_paused() {
            sink.play();
        }
    } else {
        if !sink.is_paused() {
            sink.pause();
        }
    }
}

/// Handle player firing
pub fn player_fire(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    config: Res<PhysicsConfig>,
    asset_server: Res<AssetServer>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(transform) = query.single() else {
        return;
    };

    // Spawn projectile at ship's nose
    // Offset slightly forward so it doesn't spawn inside the ship
    let forward = (transform.rotation * Vec3::Y).truncate();
    let spawn_pos = transform.translation.truncate() + forward * 30.0;

    commands.spawn((
        Sprite {
            // Use a small circle or existing asset
            // For now, we'll use a small custom size sprite if no asset
            custom_size: Some(Vec2::new(10.0, 20.0)),
            color: Color::srgb(1.0, 0.8, 0.2),
            ..default()
        },
        Transform::from_translation(spawn_pos.extend(0.0)).with_rotation(transform.rotation),
        Projectile,
        Lifetime::new(config.projectile_lifetime),
        // Physics for collision detection
        RigidBody::Kinematic, // Kinematic so it moves manually but detects collisions
        Collider::rectangle(10.0, 20.0),
        Sensor, // Sensor so it doesn't physically push things
    ));
}

/// Keep player within screen bounds
///
/// Rust Concept: Mutable queries with bounds checking
pub fn constrain_player_position(mut query: Query<&mut Transform, With<Player>>) {
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
    game_data: Res<GameData>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    if let Ok(health) = player_query.single() {
        // Player is alive, show current health
        *text = Text::new(format!("Health: {:.0}", health.current()));
    } else if game_data.is_game_over {
        // Player is dead (despawned), show 0
        *text = Text::new("Health: 0");
    }
}

/// Update thruster particle effects based on input
pub fn update_thruster_visuals(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Thruster, &ThrusterType, &mut Visibility)>,
) {
    let mut left_main_active = false;
    let mut right_main_active = false;
    let mut left_reverse_active = false;
    let mut right_reverse_active = false;

    // Logic matches player_movement
    // Left Arrow -> Fires Right Thruster
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        right_main_active = true;
    }
    // Right Arrow -> Fires Left Thruster
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        left_main_active = true;
    }
    // Up Arrow -> Fires BOTH main thrusters
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        left_main_active = true;
        right_main_active = true;
    }
    // Down Arrow -> Fires BOTH reverse thrusters
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        left_reverse_active = true;
        right_reverse_active = true;
    }

    for (thruster, thruster_type, mut visibility) in query.iter_mut() {
        let active = match (thruster, thruster_type) {
            (Thruster::Left, ThrusterType::Main) => left_main_active,
            (Thruster::Right, ThrusterType::Main) => right_main_active,
            (Thruster::Left, ThrusterType::Reverse) => left_reverse_active,
            (Thruster::Right, ThrusterType::Reverse) => right_reverse_active,
        };

        *visibility = if active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
