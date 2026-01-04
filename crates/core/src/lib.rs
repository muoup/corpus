pub mod base;
pub mod proving;
pub mod rewriting;

// Re-export base module items for backwards compatibility
pub use base::*;

// Re-export proving for convenience
pub use proving::{Prover, CostEstimator, GoalChecker, SubtermRewritable, ProofState, ProofStep, ProofResult,
                 SizeHashCostEstimator, HashEqualityGoalChecker};

// Re-export rewriting for convenience
pub use rewriting::{Pattern, Substitution, Unifiable, UnificationError, RewriteDirection, RewriteRule};
