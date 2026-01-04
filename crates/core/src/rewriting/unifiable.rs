use crate::nodes::{HashNode, HashNodeInner, NodeStorage};
use crate::rewriting::pattern::Pattern;
use crate::rewriting::substitution::Substitution;

#[derive(Debug, Clone, PartialEq)]
pub enum UnificationError {
    OccursCheck(u32, u64),
    TypeMismatch,
    CannotUnify(String),
}

pub trait Unifiable: HashNodeInner + Clone {
    fn unify(
        pattern: &Pattern<Self>,
        term: &HashNode<Self>,
        subst: &Substitution<Self>,
        store: &NodeStorage<Self>,
    ) -> Result<Substitution<Self>, UnificationError>;

    fn occurs_check(var_index: u32, term: &HashNode<Self>, subst: &Substitution<Self>) -> bool;
}

impl<T: HashNodeInner + Clone> Unifiable for T {
    fn unify(
        pattern: &Pattern<Self>,
        term: &HashNode<Self>,
        subst: &Substitution<Self>,
        _store: &NodeStorage<Self>,
    ) -> Result<Substitution<Self>, UnificationError> {
        match pattern {
            Pattern::Variable(idx) => {
                if let Some(bound) = subst.get(*idx) {
                    if bound.hash() == term.hash() {
                        Ok(subst.clone())
                    } else {
                        Err(UnificationError::CannotUnify(format!(
                            "Variable /{} bound to different term",
                            idx
                        )))
                    }
                } else if Self::occurs_check(*idx, term, subst) {
                    Err(UnificationError::OccursCheck(*idx, term.hash()))
                } else {
                    let mut new_subst = subst.clone();
                    new_subst.bind(*idx, term.clone());
                    Ok(new_subst)
                }
            }
            Pattern::Wildcard => Ok(subst.clone()),
            Pattern::Constant(c) => {
                let c_hash = c.hash();
                if c_hash == term.hash() {
                    Ok(subst.clone())
                } else {
                    Err(UnificationError::TypeMismatch)
                }
            }
            Pattern::Compound { opcode: pat_opcode, args: pat_args } => {
                if pat_args.is_empty() {
                    return Err(UnificationError::CannotUnify("Empty compound pattern".into()));
                }

                let (term_opcode, term_children) = term.value.as_ref().decompose()
                    .ok_or_else(|| UnificationError::TypeMismatch)?;

                if *pat_opcode != term_opcode || pat_args.len() != term_children.len() {
                    return Err(UnificationError::CannotUnify("Structure mismatch".into()));
                }

                let mut new_subst = subst.clone();

                for (pat_arg, term_child) in pat_args.iter().zip(term_children.iter()) {
                    new_subst = Self::unify(pat_arg, term_child, &new_subst, _store)?;
                }

                Ok(new_subst)
            }
        }
    }

    fn occurs_check(var_index: u32, term: &HashNode<Self>, subst: &Substitution<Self>) -> bool {
        if subst.contains(var_index) {
            if let Some(bound) = subst.get(var_index) {
                if bound.hash() == term.hash() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{HashNode, NodeStorage};

    #[test]
    fn test_variable_unification() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::var(0);
        let subst = Substitution::new();

        let result = u64::unify(&pattern, &term, &subst, &store);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wildcard_unification() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::wildcard();
        let subst = Substitution::new();

        let result = u64::unify(&pattern, &term, &subst, &store);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_unification() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::constant(42u64);
        let subst = Substitution::new();

        let result = u64::unify(&pattern, &term, &subst, &store);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_mismatch() {
        let store = NodeStorage::new();
        let term = HashNode::from_store(42u64, &store);
        let pattern = Pattern::constant(99u64);
        let subst = Substitution::new();

        let result = u64::unify(&pattern, &term, &subst, &store);
        assert!(result.is_err());
    }
}
