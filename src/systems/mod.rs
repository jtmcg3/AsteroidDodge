//! Rust Concept: Module organization
//! Breaking code into logical modules makes it easier to navigate and maintain

pub mod player;
pub mod asteroid;
pub mod collision;

// Re-export all public functions for convenience
// Rust Concept: Selective re-exports
// This allows users to import everything with `use systems::*;`
pub use player::*;
pub use asteroid::*;
pub use collision::*;
