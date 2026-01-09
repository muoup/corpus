pub mod base;
pub mod proving;
pub mod rewriting;

// Re-export base module items for backwards compatibility
pub use base::*;

// Re-export proving for convenience
pub use proving::{
    CostEstimator, GoalChecker, ProofResult, ProofState, ProofStep, Prover, ReflexiveGoalChecker,
    SizeCostEstimator,
};

// Re-export rewriting for convenience
pub use rewriting::{Pattern, RewriteDirection, RewriteRule};
