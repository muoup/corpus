// Base module - core abstractions for the corpus system

// Declare all submodules
pub mod axioms;
pub mod nodes;
pub mod truth;
pub mod variables;

// Re-export all submodule items for convenience
pub use axioms::*;
pub use nodes::*;
pub use truth::*;
pub use variables::*;

pub trait LogicSystem {
    type TruthType;
}