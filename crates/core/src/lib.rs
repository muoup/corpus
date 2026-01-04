pub mod expression;
pub mod logic;
pub mod nodes;
pub mod opcodes;
pub mod patterns;
pub mod rewriting;
pub mod truth;
pub mod variables;

// Re-export rewriting for convenience
pub use rewriting::{Pattern, Substitution, Unifiable, UnificationError, RewriteDirection, RewriteRule};

// Re-export new traits
pub use opcodes::OpcodeMapper;
pub use patterns::PatternDecomposer;
pub use variables::VariableExtractor;
