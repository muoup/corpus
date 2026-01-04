use crate::nodes::{HashNode, HashNodeInner, NodeStorage};
use crate::opcodes::OpcodeMapper;

pub mod pattern;
pub mod substitution;
pub mod unifiable;

// Re-export the main types for convenience
pub use pattern::{Pattern, QuantifierType};
pub use substitution::Substitution;
pub use unifiable::{Unifiable, UnificationError};

pub enum RewriteDirection {
    Both,
    Forward,
    Backward,
}

/// A rewrite rule with a stored opcode mapper for generic expression construction.
///
/// The `OpcodeMapper` allows this rule to work with any domain's expression types
/// without requiring hard-coded `construct_compound` functions.
///
/// # Type Parameters
///
/// * `T` - The expression type (must implement `HashNodeInner` and `Unifiable`)
/// * `M` - The opcode mapper type (must implement `OpcodeMapper<T>`)
pub struct RewriteRule<T: HashNodeInner + Unifiable, M: OpcodeMapper<T>> {
    pub name: String,
    pub pattern: Pattern<T>,
    pub replacement: Pattern<T>,
    pub direction: RewriteDirection,
    mapper: M,
}

pub struct RewriteResult<T: HashNodeInner> {
    pub term: HashNode<T>,
    pub substitution: Substitution<T>,
    pub rule_name: String,
}

impl<T: HashNodeInner + Unifiable, M: OpcodeMapper<T>> RewriteRule<T, M> {
    /// Create a new rewrite rule with the given mapper.
    pub fn new(
        name: impl Into<String>,
        pattern: Pattern<T>,
        replacement: Pattern<T>,
        direction: RewriteDirection,
        mapper: M,
    ) -> Self {
        Self {
            name: name.into(),
            pattern,
            replacement,
            direction,
            mapper,
        }
    }

    /// Create a bidirectional rewrite rule.
    pub fn bidirectional(name: impl Into<String>, pattern: Pattern<T>, replacement: Pattern<T>, mapper: M) -> Self {
        Self::new(name, pattern, replacement, RewriteDirection::Both, mapper)
    }

    /// Try to match the pattern against a term (forward direction).
    pub fn try_match(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Result<Substitution<T>, UnificationError> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        T::unify(&self.pattern, term, &Substitution::new(), store)
    }

    /// Try to match the replacement against a term (reverse direction).
    pub fn try_match_reverse(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Result<Substitution<T>, UnificationError> {
        if matches!(self.direction, RewriteDirection::Forward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        T::unify(&self.replacement, term, &Substitution::new(), store)
    }

    /// Check if this rule is bidirectional.
    pub fn is_bidirectional(&self) -> bool {
        matches!(self.direction, RewriteDirection::Both)
    }

    /// Apply this rule to a term (forward direction).
    pub fn apply(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Option<HashNode<T>> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return None;
        }

        let subst = self.try_match(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.replacement,
            &subst,
            store,
            &self.mapper,
        ))
    }

    /// Apply this rule to a term (reverse direction).
    pub fn apply_reverse(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Option<HashNode<T>> {
        if matches!(self.direction, RewriteDirection::Forward) {
            return None;
        }

        let subst = self.try_match_reverse(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.pattern,
            &subst,
            store,
            &self.mapper,
        ))
    }
}

/// Apply a substitution to a pattern using an opcode mapper.
fn apply_substitution_to_pattern<T: HashNodeInner + Clone, M: OpcodeMapper<T>>(
    pattern: &Pattern<T>,
    subst: &Substitution<T>,
    store: &NodeStorage<T>,
    mapper: &M,
) -> HashNode<T> {
    match pattern {
        Pattern::Variable(idx) => {
            subst.get(*idx).cloned().expect(&format!("Variable /{} should be bound in substitution", idx))
        }
        Pattern::Wildcard => {
            panic!("Wildcard should not appear in replacement pattern")
        }
        Pattern::Constant(c) => HashNode::from_store(c.clone(), store),
        Pattern::Compound { opcode, args } => {
            let substituted_args: Vec<HashNode<T>> = args
                .iter()
                .map(|arg| apply_substitution_to_pattern(arg, subst, store, mapper))
                .collect();
            mapper.construct(*opcode, substituted_args, store)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple test mapper for u64 (no compound expressions)
    struct TestMapper;
    impl OpcodeMapper<u64> for TestMapper {
        fn construct(&self, _opcode: u8, _children: Vec<HashNode<u64>>, _store: &NodeStorage<u64>) -> HashNode<u64> {
            panic!("u64 has no compound expressions")
        }
        fn get_opcode(&self, _expr: &HashNode<u64>) -> Option<u8> {
            None
        }
        fn is_valid_opcode(&self, _opcode: u8) -> bool {
            false
        }
    }

    #[test]
    fn test_variable_rule() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::var(0);
        let replacement = Pattern::constant(42u64);

        let mapper = TestMapper;
        let rule = RewriteRule::new(
            "test_rule",
            pattern.clone(),
            replacement.clone(),
            RewriteDirection::Both,
            mapper,
        );

        // Forward: match pattern (var 0) against term (42) - should succeed
        assert!(rule.try_match(&term, &store).is_ok());
        // Reverse: match replacement (constant 42) against term (42) - should succeed
        assert!(rule.try_match_reverse(&term, &store).is_ok());
        assert!(rule.is_bidirectional());
    }
}
