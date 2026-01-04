// Base module - core abstractions for the corpus system

// Declare all submodules
pub mod expression;
pub mod logic;
pub mod nodes;
pub mod opcodes;
pub mod patterns;
pub mod truth;
pub mod variables;

// Re-export all submodule items for convenience
pub use expression::*;
pub use logic::*;
pub use nodes::*;
pub use opcodes::*;
pub use patterns::*;
pub use truth::*;
pub use variables::*;
