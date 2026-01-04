//! Opcode mapper for Peano Arithmetic expressions.
//!
//! This module implements the `OpcodeMapper` trait for `ArithmeticExpression`,
//! providing generic opcode-to-expression construction.

use corpus_core::nodes::{HashNode, NodeStorage, Hashing};
use corpus_core::opcodes::OpcodeMapper;
use crate::syntax::ArithmeticExpression;

/// Opcode mapper for Peano Arithmetic expressions.
///
/// This struct implements `OpcodeMapper<ArithmeticExpression>` to provide
/// generic construction of arithmetic expressions from opcodes.
#[derive(Debug, Clone, Copy)]
pub struct PeanoOpcodeMapper;

impl OpcodeMapper<ArithmeticExpression> for PeanoOpcodeMapper {
    fn construct(
        &self,
        opcode: u8,
        children: Vec<HashNode<ArithmeticExpression>>,
        store: &NodeStorage<ArithmeticExpression>,
    ) -> HashNode<ArithmeticExpression> {
        match opcode {
            o if o == Hashing::opcode("add") && children.len() == 2 => {
                let expr = ArithmeticExpression::Add(children[0].clone(), children[1].clone());
                HashNode::from_store(expr, store)
            }
            o if o == Hashing::opcode("successor") && children.len() == 1 => {
                let expr = ArithmeticExpression::Successor(children[0].clone());
                HashNode::from_store(expr, store)
            }
            o if o == Hashing::opcode("number") && children.len() == 1 => {
                // Extract number from child's hash
                let n = children[0].hash();
                let expr = ArithmeticExpression::Number(n);
                HashNode::from_store(expr, store)
            }
            o if o == Hashing::opcode("debruijn") && children.len() == 1 => {
                // Extract de bruijn index from child's hash
                let idx = children[0].hash() as u32;
                let expr = ArithmeticExpression::DeBruijn(idx);
                HashNode::from_store(expr, store)
            }
            _ => panic!("Invalid opcode: {} with {} children", opcode, children.len()),
        }
    }

    fn get_opcode(&self, expr: &HashNode<ArithmeticExpression>) -> Option<u8> {
        match expr.value.as_ref() {
            ArithmeticExpression::Add(_, _) => Some(Hashing::opcode("add")),
            ArithmeticExpression::Successor(_) => Some(Hashing::opcode("successor")),
            ArithmeticExpression::Number(_) => Some(Hashing::opcode("number")),
            ArithmeticExpression::DeBruijn(_) => Some(Hashing::opcode("debruijn")),
        }
    }

    fn is_valid_opcode(&self, opcode: u8) -> bool {
        matches!(opcode,
            o if o == Hashing::opcode("add") ||
            o == Hashing::opcode("successor") ||
            o == Hashing::opcode("number") ||
            o == Hashing::opcode("debruijn")
        )
    }

    fn arity_for_opcode(&self, opcode: u8) -> Option<usize> {
        match opcode {
            o if o == Hashing::opcode("add") => Some(2),
            o if o == Hashing::opcode("successor") => Some(1),
            o if o == Hashing::opcode("number") => Some(1),
            o if o == Hashing::opcode("debruijn") => Some(1),
            _ => None,
        }
    }
}
