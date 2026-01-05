pub mod parsing;
pub mod syntax;
pub mod axioms;
pub mod patterns;
pub mod prover;
pub mod rewrite;

pub use prover::{PeanoProver, create_prover, ProofResult, ProofState, ProofStep, ProofResultExt};