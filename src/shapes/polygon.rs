use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

// trait for shape generator (trait based polymorphisms)
pub trait ShapeGenerator {
    fn generate(&self, rng: &mut impl Rng) -> Vec<Vec2>;
}

/*
generator for irregular polygons
1. place vertices in a circle
2. Randomly varying the radius for each vertex
3. Adding a slight angular variation
*/ 
pub struct IrregularPolygonGenerator {
    pub vertex_count: usize,
    pub base_radius: f32,
    pub radius_variation: f32,
    pub angle_variation: f32,
}

impl IrregularPolygonGenerator {
    pub fn new(vertex_count: usize, base_radius: f32) -> Self {
        Self {
            vertex_count,
            base_radius,
            radius_variation: 0.3, // 30% variation in radius
            angle_variation: 0.2, //small angular jitter
        }
    }

    // generat e a single vertex with variation (private helper methods)
    fn generate_vertex(&self, index: usize, rng: &mut impl Rng) -> Vec2 {
        //calculate base angle for the vertex. type inference with as f32 cast
        let base_angle = (index as f32 / self.vertex_count as f32) * 2.0 * PI;

        //add some random angular variation (range syntax for random generation)
        let angle_offset = rng.random_range(-self.angle_variation..self.angle_variation);
        let angle = base_angle + angle_offset;

        // vary the radius randomly
        let radius_factor = rng.random_range(
            1.0 - self.radius_variation..1.0 + self.radius_variation
        );
        let radius = self.base_radius * radius_factor;

        // Direct return without return keyword
        Vec2::new(
            angle.cos() * radius,
            angle.sin() * radius,
        )
    }
}

impl ShapeGenerator for IrregularPolygonGenerator {
    fn generate(&self, rng: &mut impl Rng) -> Vec<Vec2> {
        // Rust Concept: Iterator chains for collection building
        // This is idiomatic Rust - functional style with zero-cost abstractions
        (0..self.vertex_count)
            .map(|i| self.generate_vertex(i, rng))
            .collect()
    }
}

/// Ensure a polygon's vertices are in counter-clockwise order
/// 
/// Rust Concept: In-place mutation with &mut
/// This modifies the vector directly rather than creating a new one
pub fn ensure_ccw(vertices: &mut Vec<Vec2>) {
    if !is_ccw(vertices) {
        vertices.reverse();
    }
}

/// Check if vertices are in counter-clockwise order using the shoelace formula
/// 
/// Rust Concept: Window iteration
/// windows(2) creates an iterator over overlapping pairs
fn is_ccw(vertices: &[Vec2]) -> bool {
    // Rust Concept: Slice patterns with [a, b]
    // This destructures the pair elegantly
    let area: f32 = vertices
        .windows(2)
        .map(|pair| {
            if let [a, b] = pair {
                (b.x - a.x) * (b.y + a.y)
            } else {
                0.0
            }
        })
        .sum();
    
    // Also add the wrap-around edge
    let last = vertices.last().unwrap();
    let first = vertices.first().unwrap();
    let final_area = area + (first.x - last.x) * (first.y + last.y);
    
    final_area < 0.0  // Negative area means CCW
}

/// Create vertices for a regular polygon (for testing/simple cases)
/// 
/// Rust Concept: Generic functions with trait bounds
/// This works with any number that can be converted to/from f32
pub fn regular_polygon(num_vertices: usize, radius: f32) -> Vec<Vec2> {
    (0..num_vertices)
        .map(|i| {
            let angle = (i as f32 / num_vertices as f32) * 2.0 * PI;
            Vec2::new(angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

/// Simplify a polygon by removing vertices that are too close together
/// 
/// Rust Concept: Filtering with retain
/// This is more efficient than creating a new Vec
pub fn simplify_polygon(vertices: &mut Vec<Vec2>, min_distance: f32) {
    if vertices.len() < 3 {
        return;
    }
    
    let min_dist_squared = min_distance * min_distance;
    
    // Rust Concept: retain with closure
    // Keep only vertices that are far enough from the previous one
    let mut prev = vertices[0];
    vertices.retain(|&vertex| {
        let dist_squared = prev.distance_squared(vertex);
        let keep = dist_squared >= min_dist_squared;
        if keep {
            prev = vertex;
        }
        keep
    });
    
    // Ensure we still have enough vertices for a valid polygon
    if vertices.len() < 3 {
        *vertices = regular_polygon(6, 20.0); // Fallback to regular hexagon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Rust Concept: Unit testing
    /// Testing is built into Rust - no external framework needed
    #[test]
    fn test_regular_polygon_count() {
        let vertices = regular_polygon(6, 10.0);
        assert_eq!(vertices.len(), 6);
    }
    
    #[test]
    fn test_ccw_detection() {
        // Square in CCW order
        let ccw_square = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];
        assert!(is_ccw(&ccw_square));
        
        // Square in CW order
        let cw_square = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
        ];
        assert!(!is_ccw(&cw_square));
    }
    
    #[test]
    fn test_ensure_ccw() {
        let mut vertices = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
        ];
        
        ensure_ccw(&mut vertices);
        assert!(is_ccw(&vertices));
    }
}
