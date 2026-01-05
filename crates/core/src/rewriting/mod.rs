use crate::base::nodes::{HashNode, HashNodeInner, NodeStorage};

pub mod pattern;
pub mod substitution;
pub mod unifiable;

// Re-export the main types for convenience
pub use pattern::{Pattern, QuantifierType};
pub use substitution::Substitution;
pub use unifiable::{Unifiable, UnificationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteDirection {
    Both,
    Forward,
    Backward,
}

/// A rewrite rule for term transformation.
///
/// # Type Parameters
///
/// * `Node` - The expression type (must implement `HashNodeInner` and `Unifiable`)
pub struct RewriteRule<Node: HashNodeInner + Unifiable> {
    pub name: String,
    pub pattern: Pattern<Node>,
    pub replacement: Pattern<Node>,
    pub direction: RewriteDirection,
}

pub struct RewriteResult<Node: HashNodeInner> {
    pub term: HashNode<Node>,
    pub substitution: Substitution<Node>,
    pub rule_name: String,
}

impl<Node: HashNodeInner + Unifiable> RewriteRule<Node> {
    /// Create a new rewrite rule.
    pub fn new(
        name: impl Into<String>,
        pattern: Pattern<Node>,
        replacement: Pattern<Node>,
        direction: RewriteDirection,
    ) -> Self {
        Self {
            name: name.into(),
            pattern,
            replacement,
            direction,
        }
    }

    /// Create a bidirectional rewrite rule.
    pub fn bidirectional(name: impl Into<String>, pattern: Pattern<Node>, replacement: Pattern<Node>) -> Self {
        Self::new(name, pattern, replacement, RewriteDirection::Both)
    }

    /// Try to match the pattern against a term (forward direction).
    pub fn try_match(
        &self,
        term: &HashNode<Node>,
        store: &NodeStorage<Node>,
    ) -> Result<Substitution<Node>, UnificationError> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        Node::unify(&self.pattern, term, &Substitution::new(), store)
    }

    /// Try to match the replacement against a term (reverse direction).
    pub fn try_match_reverse(
        &self,
        term: &HashNode<Node>,
        store: &NodeStorage<Node>,
    ) -> Result<Substitution<Node>, UnificationError> {
        if matches!(self.direction, RewriteDirection::Forward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        Node::unify(&self.replacement, term, &Substitution::new(), store)
    }

    /// Check if this rule is bidirectional.
    pub fn is_bidirectional(&self) -> bool {
        matches!(self.direction, RewriteDirection::Both)
    }

    /// Apply this rule to a term (forward direction).
    pub fn apply(
        &self,
        term: &HashNode<Node>,
        store: &NodeStorage<Node>,
    ) -> Option<HashNode<Node>> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return None;
        }

        let subst = self.try_match(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.replacement,
            &subst,
            store,
        ))
    }

    /// Apply this rule to a term (reverse direction).
    pub fn apply_reverse(
        &self,
        term: &HashNode<Node>,
        store: &NodeStorage<Node>,
    ) -> Option<HashNode<Node>> {
        if matches!(self.direction, RewriteDirection::Forward) {
            return None;
        }

        let subst = self.try_match_reverse(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.pattern,
            &subst,
            store,
        ))
    }
}

/// Apply a substitution to a pattern.
fn apply_substitution_to_pattern<T: HashNodeInner + Clone>(
    pattern: &Pattern<T>,
    subst: &Substitution<T>,
    store: &NodeStorage<T>,
) -> HashNode<T> {
    match pattern {
        Pattern::Variable(idx) => {
            subst.get(*idx).cloned().unwrap_or_else(|| panic!("Variable /{} should be bound in substitution", idx))
        }
        Pattern::Wildcard => {
            panic!("Wildcard should not appear in replacement pattern")
        }
        Pattern::Constant(c) => HashNode::from_store(c.clone(), store),
        Pattern::Compound { opcode, args } => {
            let substituted_args: Vec<HashNode<T>> = args
                .iter()
                .map(|arg| apply_substitution_to_pattern(arg, subst, store))
                .collect();
            let len = substituted_args.len();
            T::construct_from_parts(*opcode, substituted_args, store).unwrap_or_else(|| {
                panic!("Invalid opcode: {} with {} children", opcode, len)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_rule() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::var(0);
        let replacement = Pattern::constant(42u64);

        let rule = RewriteRule::bidirectional(
            "test_rule",
            pattern.clone(),
            replacement.clone(),
        );

        // Forward: match pattern (var 0) against term (42) - should succeed
        assert!(rule.try_match(&term, &store).is_ok());
        // Reverse: match replacement (constant 42) against term (42) - should succeed
        assert!(rule.try_match_reverse(&term, &store).is_ok());
        assert!(rule.is_bidirectional());
    }
}
