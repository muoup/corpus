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
    pub substitution: Substitution<T>,
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
    ) -> Result<Substitution<T>, UnificationError> {
        if matches!(self.direction, RewriteDirection::Backward) {
            return Err(UnificationError::CannotUnify("Wrong direction".into()));
        }
        T::unify(&self.pattern, term, &Substitution::new(), store)
    }

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

    pub fn is_bidirectional(&self) -> bool {
        matches!(self.direction, RewriteDirection::Both)
    }

    pub fn apply<F>(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
        construct_compound: F,
    ) -> Option<HashNode<T>>
    where
        F: Fn(u8, Vec<HashNode<T>>, &NodeStorage<T>) -> HashNode<T>,
    {
        if matches!(self.direction, RewriteDirection::Backward) {
            return None;
        }

        let subst = self.try_match(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.replacement,
            &subst,
            store,
            &construct_compound,
        ))
    }

    pub fn apply_reverse<F>(
        &self,
        term: &HashNode<T>,
        store: &NodeStorage<T>,
        construct_compound: F,
    ) -> Option<HashNode<T>>
    where
        F: Fn(u8, Vec<HashNode<T>>, &NodeStorage<T>) -> HashNode<T>,
    {
        if matches!(self.direction, RewriteDirection::Forward) {
            return None;
        }

        let subst = self.try_match_reverse(term, store).ok()?;
        Some(apply_substitution_to_pattern(
            &self.pattern,
            &subst,
            store,
            &construct_compound,
        ))
    }
}

fn apply_substitution_to_pattern<T: HashNodeInner + Clone, F>(
    pattern: &Pattern<T>,
    subst: &Substitution<T>,
    store: &NodeStorage<T>,
    construct_compound: &F,
) -> HashNode<T>
where
    F: Fn(u8, Vec<HashNode<T>>, &NodeStorage<T>) -> HashNode<T>,
{
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
                .map(|arg| apply_substitution_to_pattern(arg, subst, store, construct_compound))
                .collect();
            construct_compound(*opcode, substituted_args, store)
        }
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
