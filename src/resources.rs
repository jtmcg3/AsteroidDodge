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
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            // Spawn every 1.5 sec
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        }
    }
}