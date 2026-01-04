use bevy::prelude::*;

// Marker Components - tags basically
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

// Health Component
#[derive(Component)]
pub struct Health {
    current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

#[derive(Component)]
pub enum DamageSource {
    AsteroidEntity(AsteroidSize),
}

impl DamageSource {
    pub fn damage(&self) -> f32 {
        match *self {
            DamageSource::AsteroidEntity(size) => size.damage(),
        }
    }
}

// Asteroid size categories
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    // get the radius
    pub fn radius(&self) -> f32 {
        match self {
            Self::Small => 15.0,
            Self::Medium => 30.0,
            Self::Large => 45.0,
        }
    }

    // get the mass for physics
    pub fn mass(&self) -> f32 {
        match self {
            Self::Small => 7.0,
            Self::Medium => 15.0,
            Self::Large => 35.0,
        }
    }

    // number of vertices for the polygon
    pub fn vertex_count(&self) -> usize {
        match self {
            Self::Small => 10,
            Self::Medium => 17,
            Self::Large => 27,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            Self::Small => 10.0,
            Self::Medium => 20.0,
            Self::Large => 30.0,
        }
    }
}

// Cleanup yo shit
#[derive(Component)]
pub struct Cleanup;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub enum Thruster {
    Left,
    Right,
}

#[derive(Component)]
pub enum ThrusterType {
    Main,    // Fires backward
    Reverse, // Fires forward
}

#[derive(Component)]
pub struct ThrusterAudio;
