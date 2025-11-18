use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;

// Startup system, runs once
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 0.0),
        Player,
        Velocity::new(0.0, 0.0),
        Health::new(100.0),
    ));
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    // query returns an iterator
    for (mut transform, mut velocity) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        // idiomatic pattern matching in Rust
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA){
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD){
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW){
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS){
            direction.y -= 1.0;
        }
        // normalize to prevent diagonal speed boost
        if direction != Vec2::ZERO{
            direction = direction.normalize();
        }

        let speed = 300.0;
        velocity.value = direction * speed;

        // update position based on velocity
        transform.translation.x += velocity.value.x * time.delta_secs();
        transform.translation.y += velocity.value.y * time.delta_secs();

        // clamp to screen bounds
        transform.translation.x = transform.translation.x.clamp(-370.0, 370.0);
        transform.translation.y = transform.translation.y.clamp(-270.0, 270.0);
    }
}

// Asteroid spawning

pub fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    game_state: Res<GameState>,
){
    if game_state.is_game_over {
        return;
    }

    // tick the timer
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        let mut rng = rand::rng();

        // random x position at top of screen
        let x = rng.random_range(-370.0..370.0);
        let y = 320.0;

        // Random Size
        let size = rng.random_range(20.0..75.0);

        // random speed
        let speed_y = rng.random_range(-200.0..-100.0);

        commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.3, 0.2),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(x,y,0.0),
            Asteroid,
            Velocity::new(0.0, speed_y),
        ));
    }
}

// CRASH into me

pub fn move_asteroids(
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_secs();
        transform.translation.y += velocity.value.y * time.delta_secs();
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut asteroid_query: Query<(Entity, &Transform, &Sprite), With<Asteroid>>,
    mut player_query: Query<(Entity, &Transform, &Sprite, &mut Health), With<Player>>,
) {
    // rusts iterator methods are powerful and idiomatic
    for (player_entity, player_transform, player_sprite, mut health) in player_query.iter_mut() {
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ONE);

        for (asteroid_entity, asteroid_transform, asteroid_sprite) in asteroid_query.iter_mut() {
            let asteroid_size = asteroid_sprite.custom_size.unwrap_or(Vec2::ONE);

            let collision = collide_aabb(
                player_transform.translation,
                player_size,
                asteroid_transform.translation,
                asteroid_size,
            );

            if collision {
                // Despawn Asteroid
                commands.entity(asteroid_entity).despawn();

                //Damage Player
                health.current -= 20.0;

                if health.is_dead() {
                    commands.entity(player_entity).despawn();
                    game_state.is_game_over = true;
                }

                break; //Can only hit one asteroid per frame
            }
        }
    }
}

// helper function
fn collide_aabb(
    pos_a: Vec3,
    size_a: Vec2,
    pos_b: Vec3,
    size_b: Vec2,
) -> bool {
    let half_size_a = size_a / 2.0;
    let half_size_b = size_b / 2.0;

    (pos_a.x - half_size_a.x < pos_b.x + half_size_b.x) &&
    (pos_a.x + half_size_a.x > pos_b.x - half_size_b.x) &&
    (pos_a.y - half_size_a.y < pos_b.y + half_size_b.y) &&
    (pos_a.y + half_size_a.y > pos_b.y - half_size_b.y)
}

pub fn cleanup_offscreen(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Asteroid>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -350.0 {
            commands.entity(entity).despawn();
        }
    }
}
