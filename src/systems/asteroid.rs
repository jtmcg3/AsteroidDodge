use core::f32;

use crate::components::*;
use crate::resources::*;
use crate::shapes::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

/// Spawn asteroids at intervals with procedurally generated shapes
///
/// Rust Concept: Complex system with multiple resources
pub fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    difficulty: Res<DifficultyConfig>,
    config: Res<AsteroidSpawnConfig>,
    time: Res<Time>,
    game_state: Res<GameData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if game_state.is_game_over {
        return;
    }

    // update elapsed time
    spawn_timer.elapsed_time += time.delta_secs();
    // Tick the timer
    spawn_timer.timer.tick(time.delta());

    // Rust Concept: Guard clause pattern
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let new_interval = difficulty.calculate_interval(spawn_timer.elapsed_time);

    //reset spawn timer with new duration
    spawn_timer
        .timer
        .set_duration(std::time::Duration::from_secs_f32(new_interval));
    spawn_timer.timer.reset();

    // Rust Concept: Creating thread-local RNG
    // This is cheaper than using a global RNG with locking
    let mut rng = rand::rng();

    // Random spawn position
    let x = rng.random_range(-config.spawn_x_range..config.spawn_x_range);
    let y = config.spawn_y;

    // Random size
    // Rust Concept: Match expression for weighted random choice
    let size = match rng.random_range(0..100) {
        0..=40 => AsteroidSize::Small,   // 40% chance
        41..=75 => AsteroidSize::Medium, // 35% chance
        _ => AsteroidSize::Large,        // 25% chance
    };

    // Random velocity (downward with slight horizontal variation)
    let speed_y = -rng.random_range(config.min_speed..config.max_speed);
    let speed_x = rng.random_range(-50.0..50.0);

    let velocity = Vec2::new(speed_x, speed_y);
    let position = Vec3::new(x, y, 0.0);

    spawn_asteroid_entity(
        &mut commands,
        &mut meshes,
        &mut materials,
        position,
        velocity,
        size,
    );
}

// Helper function to spawn asteroids
pub fn spawn_asteroid_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    velocity: Vec2,
    size: AsteroidSize,
) {
    let mut rng = rand::rng();
    // 1. Generate Asteroid shape
    let generator = IrregularPolygonGenerator::new(size.vertex_count(), size.radius());
    let mut vertices = generator.generate(&mut rng);

    // Ensure vertices are in the correct order for physics
    ensure_ccw(&mut vertices);

    // Simplify to avoid tiny edges that can cause physics issues
    simplify_polygon(&mut vertices, 5.0);

    // Rust Concept: Clone for both visual and physics
    // We need separate copies because the systems might modify them
    let visual_vertices = vertices.clone();

    // Create physics collider from vertices
    // Rust Concept: Error handling with expect
    // This converts Result to a panic with a custom message if it fails
    let collider =
        Collider::convex_hull(vertices).expect("Failed to create convex hull for asteroid");

    // Create visual mesh from polygon entities
    let mesh = create_polygon_mesh(&visual_vertices);
    let mesh_handle = meshes.add(mesh);

    // create material with asteroid color
    let material = materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.7)));

    // Spawn the asteroid entity
    // Rust Concept: Long tuple of components
    // Bevy can handle arbitrarily many components in a tuple
    commands.spawn((
        // Visual (we'll render custom mesh later)
        Mesh2d(mesh_handle),
        MeshMaterial2d(material),
        Transform::from_translation(position),
        // Game components
        Asteroid,
        size,
        PolygonMesh::new(visual_vertices),
        Cleanup,
        // Physics components
        RigidBody::Dynamic,
        collider,
        CollisionEventsEnabled,
        LinearVelocity(velocity),
        AngularVelocity(rng.random_range(-2.0..2.0)),
        Mass(size.mass()),
        // Restitution (bounciness) - asteroids bounce off each other a bit
        Restitution::new(0.8),
    ));
}

/// Clean up asteroids that have left the screen
///
/// Rust Concept: Filtering entities with With<> query filter
pub fn cleanup_offscreen(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Asteroid>, With<Cleanup>)>,
) {
    for (entity, transform) in &query {
        // Despawn if below the screen
        if transform.translation.y < -350.0 {
            // Rust Concept: Entity despawning
            // This marks the entity for removal
            commands.entity(entity).despawn();
        }
    }
}

/// System to render asteroid polygons
///
/// Rust Concept: Gizmos for debug rendering
/// This is temporary - in a full game you'd use mesh rendering
#[cfg(feature = "debug")]
pub fn draw_asteroid_shapes(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &PolygonMesh), With<Asteroid>>,
) {
    for (transform, mesh) in &query {
        let color = Color::srgb(0.8, 0.5, 0.3);

        // Draw lines between vertices
        // Rust Concept: Enumerate for indices
        // This gives us both the index and the value
        for (i, &vertex) in mesh.vertices.iter().enumerate() {
            let next_vertex = mesh.vertices[(i + 1) % mesh.vertices.len()];

            // Transform vertices to world space
            // Rust Concept: Transform composition
            let world_vertex = transform.transform_point(vertex.extend(0.0));
            let world_next = transform.transform_point(next_vertex.extend(0.0));

            gizmos.line_2d(world_vertex.truncate(), world_next.truncate(), color);
        }
    }
}

// Creating a polygon mesh
fn create_polygon_mesh(vertices: &[Vec2]) -> Mesh {
    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, PrimitiveTopology};

    // we need at least 3 vertices for a polygon
    assert!(vertices.len() >= 3, "Polygon must have at least 3 vertices");

    // Use fan triangulation
    let mut indices = Vec::new();
    for i in 1..(vertices.len() - 1) {
        indices.push(0u32);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    //convert vec2 to vec3 by adding z=0
    // iterator map for transformation
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| [v.x, v.y, 0.0]).collect();

    // normals will all point forward for 2d
    let normals: Vec<[f32; 3]> = vertices.iter().map(|_| [0.0, 0.0, 1.0]).collect();

    // create UV texture coordinates - simple mapping (bounding box calc)
    let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
    let max_x = vertices
        .iter()
        .map(|v| v.x)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
    let max_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);

    let width = max_x - min_x;
    let height = max_y - min_y;

    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| [(v.x - min_x) / width, (v.y - min_y) / height])
        .collect();

    // build the meth with bevys mesh builder pattern
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// System to show collision shapes for debugging
///
/// Rust Concept: Conditional compilation
/// We can enable this with a feature flag
#[cfg(feature = "debug")]
pub fn draw_colliders(mut gizmos: Gizmos, query: Query<(&Transform, &Collider), With<Asteroid>>) {
    for (transform, collider) in &query {
        // Extract polygon vertices from collider
        // This is a bit advanced - we're pattern matching on the collider type
        if let Some(polygon) = collider.as_convex_polygon() {
            let color = Color::srgba(0.0, 1.0, 0.0, 0.3);

            for (i, &vertex) in polygon.vertices().iter().enumerate() {
                let next_vertex = polygon.vertices()[(i + 1) % polygon.vertices().len()];

                let world_vertex = transform.transform_point(vertex.extend(0.0));
                let world_next = transform.transform_point(next_vertex.extend(0.0));

                gizmos.line_2d(world_vertex.truncate(), world_next.truncate(), color);
            }
        }
    }
}
