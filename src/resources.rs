// Resources are singletons, one instance per app
use bevy::prelude::*;

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu, // Start Screen
    Playing,  // Active Gameplay - will have levels and such
    Paused,   // Non-active Gameplay, halt cycles?
    GameOver, // Game Over Screen + High Scores }
}

#[derive(Resource)]
struct PlayerSettings {
    ship_type: ShipType,
    name: String,
}

#[derive(Debug, Default)]
pub enum ShipType {
    #[default]
    SimpleShip,
}

#[derive(Resource)]
struct HighScores {
    scores: Vec<(String, u32)>, // name, score
}

#[derive(Resource)]
pub struct GameData {
    pub score: u32,
    pub is_game_over: bool,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            is_game_over: false,
        }
    }
}

// Timer things for spawning things
#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
    pub elapsed_time: f32,
}

// Difficulty config for spawn timer
#[derive(Resource)]
pub struct DifficultyConfig {
    pub initial_interval: f32,
    pub min_interval: f32,
    pub curve_steepness: f32,
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        Self {
            initial_interval: 1.5,
            min_interval: 0.1,
            curve_steepness: 0.1, // lower = more gradual increase in difficulty
        }
    }
}

impl DifficultyConfig {
    // calc spawn interval based on elapsed time
    pub fn calculate_interval(&self, elapsed_time: f32) -> f32 {
        self.min_interval
            + (self.initial_interval - self.min_interval)
                / (1.0 + self.curve_steepness * elapsed_time)
    }
}

// TODO i think increasing spawn rate as score goes up is a good idea
impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            // Spawn every 1.5 sec
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            elapsed_time: 0.0,
        }
    }
}

// Asteroid Spawning config
#[derive(Resource)]
pub struct AsteroidSpawnConfig {
    pub min_speed: f32,
    pub max_speed: f32,
    pub spawn_y: f32,
    pub spawn_x_range: f32,
}

impl Default for AsteroidSpawnConfig {
    fn default() -> Self {
        Self {
            min_speed: 100.0,
            max_speed: 250.0,
            spawn_x_range: 370.0,
            spawn_y: 320.0,
        }
    }
}

// Physics configuration resource
#[derive(Resource)]
pub struct PhysicsConfig {
    pub thruster_force: f32,
    pub rotation_torque: f32,
    pub reverse_thrust_force: f32,
    pub drag: f32,
    pub angular_drag: f32,
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            thruster_force: 3000.0,
            rotation_torque: 30000.0,
            reverse_thrust_force: 3000.0,
            drag: 1.0,
            angular_drag: 1.0,
            projectile_speed: 500.0,
            projectile_lifetime: 2.0,
        }
    }
}

/// Display player score (simple text for now)
///
/// Rust Concept: One-shot systems for UI setup
pub fn setup_score_display(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        ScoreDisplay,
    ));
}

/// Marker component for Score UI
#[derive(Component)]
pub(crate) struct ScoreDisplay;

/// Update score display
///
/// Rust Concept: Optional query results
/// Using Option<> for queries that might not have results
pub fn update_score_display(
    spawn_timer: Res<SpawnTimer>,
    mut text_query: Query<&mut Text, With<ScoreDisplay>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    // Rust Concept: String formatting with format! macro
    *text = Text::new(format!("Score: {:.0}", spawn_timer.elapsed_time));
}
