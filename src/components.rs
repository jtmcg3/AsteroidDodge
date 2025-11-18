use bevy::prelude::*;

// Marker Components - tags basically
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

// Velocity Component
#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(x:f32, y:f32) -> Self {
        Self {value: Vec2::new(x,y) }
    }
}

// Health Component
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}