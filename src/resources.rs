// Resources are singletons, one instance per app
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub score: u32,
    pub is_game_over: bool,
}

impl Default for GameState {
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
            min_interval:0.1,
            curve_steepness:0.1 // lower = more gradual increase in difficulty
        }
    }
}

impl DifficultyConfig {
    // calc spawn interval based on elapsed time
    pub fn calculate_interval(&self, elapsed_time: f32) -> f32 {
        self.min_interval + 
            (self.initial_interval - self.min_interval) /
            (1.0 + self.curve_steepness * elapsed_time)
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
    pub player_speed: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            player_speed: 300.0,
        }
    }
}