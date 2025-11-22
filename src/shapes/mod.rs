pub mod polygon;

pub use polygon::{
    ShapeGenerator,
    IrregularPolygonGenerator,
    ensure_ccw,
    simplify_polygon,
};
