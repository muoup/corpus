//! Opcode mapping trait for constructing expressions from opcodes.
//!
//! This trait provides a generic interface for opcode-to-expression construction.
//! It replaces hard-coded `construct_compound` functions with a trait-based approach
//! that works for any domain.

use crate::nodes::{HashNode, HashNodeInner, NodeStorage};

/// Trait for mapping opcodes to concrete expressions.
///
/// Domains implement this trait to provide their own opcode-to-expression construction
/// logic. This eliminates the need for hard-coded `construct_compound` functions.
///
/// # Example
///
/// ```rust,ignore
/// use corpus_core::opcodes::OpcodeMapper;
/// use corpus_core::nodes::{HashNode, NodeStorage};
///
/// struct MyMapper;
///
/// impl OpcodeMapper<MyType> for MyMapper {
///     fn construct(&self, opcode: u8, children: Vec<HashNode<MyType>>, store: &NodeStorage<MyType>) -> HashNode<MyType> {
///         // Implementation specific to MyType
///         todo!()
///     }
/// }
/// ```
pub trait OpcodeMapper<T: HashNodeInner> {
    /// Construct an expression from an opcode and children.
    ///
    /// This is called when applying substitutions to compound patterns.
    /// The implementation should map the opcode to the appropriate expression
    /// constructor for the domain.
    ///
    /// # Arguments
    ///
    /// * `opcode` - The operation code (usually from `Hashing::opcode("name")`)
    /// * `children` - The child expressions to combine
    /// * `store` - The node storage for creating new hash-consed nodes
    ///
    /// # Panics
    ///
    /// The default implementation panics on invalid opcodes. Implementations
    /// may choose to return `Option` or `Result` instead if needed.
    fn construct(
        &self,
        opcode: u8,
        children: Vec<HashNode<T>>,
        store: &NodeStorage<T>,
    ) -> HashNode<T>;

    /// Get the opcode for a given expression (if compound).
    ///
    /// Returns `None` if the expression is not a compound expression
    /// (e.g., a constant, variable, or atomic value).
    fn get_opcode(&self, expr: &HashNode<T>) -> Option<u8>;

    /// Check if an opcode is valid for this domain.
    ///
    /// Returns `true` if the opcode is recognized, `false` otherwise.
    /// This can be used for validation before attempting construction.
    fn is_valid_opcode(&self, opcode: u8) -> bool;

    /// Get the arity (number of children) expected for an opcode.
    ///
    /// Returns `Some(arity)` if the opcode is known and has a fixed arity,
    /// or `None` if the arity is variable or unknown.
    ///
    /// The default implementation always returns `None`.
    fn arity_for_opcode(&self, _opcode: u8) -> Option<usize> {
        None
    }
}
