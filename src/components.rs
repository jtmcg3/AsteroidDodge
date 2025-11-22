use bevy::prelude::*;

// Marker Components - tags basically
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

// Velocity Component
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(x:f32, y:f32) -> Self {
        Self {value: Vec2::new(x,y) }
    }

    pub fn magnitude(&self) -> f32 {
        self.value.length()
    }
}

// Health Component
#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max, 
            max,
        }
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

    pub fn percent(&self) -> f32 {
        self.current / self.max
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
            Self::Small => 1.0,
            Self::Medium => 3.0,
            Self::Large => 7.0,
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
pub struct PolygonMesh {
    pub vertices: Vec<Vec2>,
}

impl PolygonMesh {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }
}
