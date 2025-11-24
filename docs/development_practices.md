# Development Practices & Guidelines

This document serves as a reference for development practices, code style, and architecture for the `asteroid_dodge` project.

## 0. DO NOT WRITE CODE UNLESS I APPROVE IT, this is a learning exercise.

### Repository
- **Remote**: `git@github.com:jtmcg3/AsteroidDodge.git`
- **Branch**: `main`

## 1. Code Style & Standards

### Rust Idioms
- **Formatting**: All code must be formatted using `rustfmt`.
- **Linting**: Use `clippy` to catch common mistakes and improve code quality.
- **Pattern Matching**: Prefer pattern matching (`match`, `if let`) over complex `if-else` chains where appropriate.
- **Iterators**: Use iterator chains (`map`, `filter`, `fold`) instead of raw loops when transforming collections.
- **Error Handling**: Use `Result` and `Option` types effectively. Avoid `unwrap()` in production code; use `expect()` with a meaningful message or handle errors gracefully.

### Documentation
- **Module Documentation**: Use `//!` at the top of files to describe the module's purpose.
- **Item Documentation**: Use `///` for public structs, enums, functions, and traits.
- **Comments**: Use `//` for implementation details and "TODO" notes.

## 2. Bevy Architecture (v0.17.2)

### ECS Organization
- **Components**: Defined in `src/components.rs`.
    - Derive `Component` for all ECS components.
    - Keep components simple (POD - Plain Old Data) when possible.
    - Use marker components (empty structs) for tagging entities (e.g., `Player`, `Asteroid`).
- **Resources**: Defined in `src/resources.rs`.
    - Derive `Resource` for global state.
    - Implement `Default` for resources to allow `init_resource` usage.
    - Use resources for configuration (e.g., `PhysicsConfig`, `DifficultyConfig`).
- **Systems**: Defined in `src/systems/`.
    - Group related systems into modules (e.g., `player.rs`, `asteroid.rs`).
    - Use `Commands` for spawning/despawning.
    - Use `Query` for accessing components. Use filters (`With`, `Without`, `Changed`) to optimize queries.
    - Use `Res` and `ResMut` for accessing resources.

### Plugin Structure
- **Composition**: The app is built by composing plugins in `main.rs`.
- **Configuration**: Use `insert_resource` or `init_resource` to configure plugins and game state.
- **Ordering**: Use `.before()` and `.after()` to enforce system execution order when necessary (e.g., `constrain_player_position.after(player_movement)`).

### Physics (`avian2d`)
- Use `avian2d` components for physics bodies and colliders.
- Configure gravity and other physics settings via resources (e.g., `Gravity(Vec2::ZERO)`).

## 3. Project Structure

- `src/main.rs`: Entry point, app configuration, plugin setup.
- `src/components.rs`: All ECS components.
- `src/resources.rs`: Global resources and configurations.
- `src/systems/`: Game logic systems.
    - `player.rs`: Player movement and input.
    - `asteroid.rs`: Asteroid spawning and behavior.
    - `collision.rs`: Collision detection and handling.
- `src/shapes/`: Geometry generation (e.g., procedural polygons).

## 4. Testing

- **Unit Tests**: Place unit tests in a `tests` module within the file being tested (or `main.rs` for app-level tests).
- **App Tests**: Verify that the app builds and initializes correctly (as seen in `main.rs`).

## 5. Asset Management

- **Assets**: Store game assets (images, sounds) in the `assets/` directory (create if needed).
- **Loading**: Use `AssetServer` to load assets in startup systems.

## 6. Development Workflow

- **Run**: `cargo run`
- **Test**: `cargo test`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`
