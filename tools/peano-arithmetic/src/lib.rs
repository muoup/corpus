pub mod parsing;
pub mod syntax;
pub mod axioms;
pub mod opcodes;
pub mod patterns;
pub mod prover;
pub mod rewrite;

pub use prover::{Prover, ProofResult, ProofState, ProofStep};
pub use opcodes::PeanoOpcodeMapper;