//! Rust Concept: Module organization
//! Breaking code into logical modules makes it easier to navigate and maintain

pub mod asteroid;
pub mod background;
pub mod collision;
pub mod game_over;
pub mod health;
pub mod loading;
pub mod menu;
pub mod player;
pub mod projectile;

// Re-export all public functions for convenience
// Rust Concept: Selective re-exports
// This allows users to import everything with `use systems::*;`
pub use asteroid::*;
pub use background::*;
pub use collision::*;
pub use game_over::*;
pub use health::*;
pub use loading::*;
pub use menu::*;
pub use player::*;
pub use projectile::*;
