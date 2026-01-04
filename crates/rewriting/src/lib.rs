use corpus_core::nodes::{HashNode, HashNodeInner, NodeStorage};
use corpus_unification::{Pattern, Substitution, Unifiable, UnificationError};

pub enum RewriteDirection {
    Both,
    Forward,
    Backward,
}

pub struct RewriteRule<T: HashNodeInner + Unifiable> {
    pub name: String,
    pub pattern: Pattern<T>,
    pub replacement: Pattern<T>,
    pub direction: RewriteDirection,
}

pub struct RewriteResult<T: HashNodeInner> {
    pub term: HashNode<T>,
    pub substitution: Substitution,
    pub rule_name: String,
}

impl<T: HashNodeInner + Unifiable> RewriteRule<T> {
    pub fn new(
        name: impl Into<String>,
        pattern: Pattern<T>,
        replacement: Pattern<T>,
        direction: RewriteDirection,
    ) -> Self {
        Self {
            name: name.into(),
            pattern,
            replacement,
            direction,
        }
    }

    pub fn bidirectional(name: impl Into<String>, pattern: Pattern<T>, replacement: Pattern<T>) -> Self {
        Self::new(name, pattern, replacement, RewriteDirection::Both)
    }

    pub fn try_match(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Result<Substitution, UnificationError> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        T::unify(&self.pattern, term, &Substitution::new(), store)
    }

    pub fn try_match_reverse(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
    ) -> Result<Substitution, UnificationError> {
        if matches!(self.direction, RewriteDirection::Forward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        T::unify(&self.replacement, term, &Substitution::new(), store)
    }

    pub fn is_bidirectional(&self) -> bool {
        matches!(self.direction, RewriteDirection::Both)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus_unification::Pattern;

    #[test]
    fn test_variable_rule() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::var(0);
        let replacement = Pattern::constant(99u64);

        let rule = RewriteRule::new(
            "test_rule",
            pattern.clone(),
            replacement.clone(),
            RewriteDirection::Both,
        );

        assert!(rule.try_match(&term, &store).is_ok());
        assert!(rule.try_match_reverse(&term, &store).is_ok());
        assert!(rule.is_bidirectional());
    }
}
