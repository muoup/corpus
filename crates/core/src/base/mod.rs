// Base module - core abstractions for the corpus system

// Declare all submodules
pub mod axioms;
pub mod expression;
pub mod logic;
pub mod nodes;
pub mod patterns;
pub mod truth;
pub mod variables;

// Re-export all submodule items for convenience
pub use axioms::*;
pub use expression::*;
pub use logic::*;
pub use nodes::*;
pub use patterns::*;
pub use truth::*;
pub use variables::*;
