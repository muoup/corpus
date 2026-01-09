use core::fmt;
use std::collections::HashMap;

use corpus_classical_logic::{ClassicalLogicalExpression, DomainContent};
use corpus_core::{NodeStorage, nodes::{HashNode, HashNodeInner, Hashing}, rewriting::patterns::Rewritable};

use crate::PeanoStorage;

// NOTE: PeanoExpression (DomainExpression) has been removed as DomainExpression
// is no longer in the core crate. Domain-specific expressions should use
// ClassicalLogicalExpression directly.

/// Logical expression type for Peano Arithmetic with full first-order logic support.
/// This wraps PeanoDomainExpression in ClassicalLogicalExpression to enable quantifiers (∀, ∃) and
/// mixed logical operators (→, ∧, ∨, ¬, ↔).
pub type PeanoLogicalExpression = ClassicalLogicalExpression<PeanoDomainExpression>;

/// Hash node containing a Peano logical expression.
pub type PeanoLogicalNode = HashNode<PeanoLogicalExpression>;

#[derive(Debug, Clone, PartialEq)]
pub enum PeanoDomainExpression {
    Equality(
        HashNode<PeanoArithmeticExpression>,
        HashNode<PeanoArithmeticExpression>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PeanoArithmeticExpression {
    Add(
        HashNode<PeanoArithmeticExpression>,
        HashNode<PeanoArithmeticExpression>,
    ),
    Successor(HashNode<PeanoArithmeticExpression>),
    Number(u64),
    DeBruijn(u32),
}

impl HashNodeInner for PeanoDomainExpression {
    fn hash(&self) -> u64 {
        match self {
            PeanoDomainExpression::Equality(l, r) => {
                Hashing::root_hash(1, &[Hashing::opcode("EQUALITY"), l.hash(), r.hash()])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoDomainExpression::Equality(l, r) => 1 + l.size() + r.size(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PeanoArithmeticPattern {
    Variable(u32),
    Wildcard,
    Literal(u64),
    Compound { opcode: u64, args: Vec<PeanoArithmeticPattern> },
}

/// Pattern type for Peano domain-level expressions (e.g., Equality).
#[derive(Debug, Clone, PartialEq)]
pub enum PeanoDomainPattern {
    Equality(PeanoArithmeticPattern, PeanoArithmeticPattern),
}

impl HashNodeInner for PeanoDomainPattern {
    fn hash(&self) -> u64 {
        match self {
            PeanoDomainPattern::Equality(l, r) => {
                Hashing::root_hash(1, &[Hashing::opcode("EQUALITY"), l.hash(), r.hash()])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoDomainPattern::Equality(l, r) => 1 + l.size() + r.size(),
        }
    }
}

impl DomainContent for PeanoDomainExpression {}

/// Substitution mapping De Bruijn indices to expressions.
type Substitution = HashMap<u32, HashNode<PeanoArithmeticExpression>>;

/// Match a pattern against an expression, producing a substitution if successful.
fn match_pattern(
    expr: &HashNode<PeanoArithmeticExpression>,
    pattern: &PeanoArithmeticPattern,
    store: &NodeStorage<PeanoArithmeticExpression>,
) -> Option<Substitution> {
    match pattern {
        PeanoArithmeticPattern::Wildcard => Some(Substitution::new()),
        PeanoArithmeticPattern::Variable(idx) => {
            let mut subst = Substitution::new();
            subst.insert(*idx, expr.clone());
            Some(subst)
        }
        PeanoArithmeticPattern::Literal(n) => {
            match expr.value.as_ref() {
                PeanoArithmeticExpression::Number(m) if *m == *n => Some(Substitution::new()),
                _ => None,
            }
        }
        PeanoArithmeticPattern::Compound { opcode, args } => {
            match expr.value.as_ref() {
                PeanoArithmeticExpression::Add(l, r)
                    if *opcode == Hashing::opcode("add") && args.len() == 2 =>
                {
                    let mut subst = match_pattern(l, &args[0], store)?;
                    subst.extend(match_pattern(r, &args[1], store)?);
                    Some(subst)
                }
                PeanoArithmeticExpression::Successor(inner)
                    if *opcode == Hashing::opcode("successor") && args.len() == 1 =>
                {
                    match_pattern(inner, &args[0], store)
                }
                _ => None,
            }
        }
    }
}

/// Apply a substitution to a pattern to produce an expression.
fn apply_substitution(
    pattern: &PeanoArithmeticPattern,
    substitution: &Substitution,
    store: &NodeStorage<PeanoArithmeticExpression>,
) -> Option<HashNode<PeanoArithmeticExpression>> {
    match pattern {
        PeanoArithmeticPattern::Wildcard => None, // Cannot reconstruct from wildcard
        PeanoArithmeticPattern::Variable(idx) => substitution.get(idx).cloned(),
        PeanoArithmeticPattern::Literal(n) => Some(HashNode::from_store(
            PeanoArithmeticExpression::Number(*n),
            store,
        )),
        PeanoArithmeticPattern::Compound { opcode, args } => {
            let resolved_args: Vec<_> = args
                .iter()
                .map(|p| apply_substitution(p, substitution, store))
                .collect::<Option<Vec<_>>>()?;

            if *opcode == Hashing::opcode("add") && resolved_args.len() == 2 {
                Some(HashNode::from_store(
                    PeanoArithmeticExpression::Add(
                        resolved_args[0].clone(),
                        resolved_args[1].clone(),
                    ),
                    store,
                ))
            } else if *opcode == Hashing::opcode("successor") && resolved_args.len() == 1 {
                Some(HashNode::from_store(
                    PeanoArithmeticExpression::Successor(resolved_args[0].clone()),
                    store,
                ))
            } else {
                None
            }
        }
    }
}

impl Rewritable for PeanoArithmeticExpression {
    type AsPattern = PeanoArithmeticPattern;
    type Storage = NodeStorage<PeanoArithmeticExpression>;
    
    fn decompose_to_pattern(
        &self,
        store: &Self::Storage,
    ) -> Self::AsPattern {
        match self {
            PeanoArithmeticExpression::Add(l, r) => PeanoArithmeticPattern::Compound {
                opcode: Hashing::opcode("add"),
                args: vec![
                    l.value.decompose_to_pattern(store),
                    r.value.decompose_to_pattern(store),
                ],
            },
            PeanoArithmeticExpression::Successor(inner) => PeanoArithmeticPattern::Compound {
                opcode: Hashing::opcode("successor"),
                args: vec![inner.value.decompose_to_pattern(store)],
            },
            PeanoArithmeticExpression::Number(n) => PeanoArithmeticPattern::Literal(*n),
            PeanoArithmeticExpression::DeBruijn(idx) => PeanoArithmeticPattern::Variable(*idx),
        }
    }

    fn try_rewrite(
        &self,
        from: &Self::AsPattern,
        to: &Self::AsPattern,
        store: &Self::Storage,
    ) -> Option<HashNode<Self>> {
        // Match 'from' pattern against self to get substitution
        let self_node = HashNode::from_store(self.clone(), store);
        let substitution = match_pattern(&self_node, from, store)?;
        // Apply substitution to 'to' pattern to get result
        apply_substitution(to, &substitution, store)
    }

    fn get_recursive_rewrites(&self, from: &Self::AsPattern, to: &Self::AsPattern, store: &Self::Storage) -> Vec<HashNode<Self>> {
        let mut results = Vec::new();

        if let Some(rewrite) = self.try_rewrite(from, to, store) {
            results.push(rewrite);
        }

        match self {
            PeanoArithmeticExpression::Add(l, r) => {
                // Get rewrites from left side and wrap them back in Add
                for left_rewrite in l.value.get_recursive_rewrites(from, to, store) {
                    results.push(HashNode::from_store(
                        PeanoArithmeticExpression::Add(left_rewrite, r.clone()),
                        store,
                    ));
                }

                // Get rewrites from right side and wrap them back in Add
                for right_rewrite in r.value.get_recursive_rewrites(from, to, store) {
                    results.push(HashNode::from_store(
                        PeanoArithmeticExpression::Add(l.clone(), right_rewrite),
                        store,
                    ));
                }
            }
            PeanoArithmeticExpression::Successor(inner) => {
                // Get rewrites from inner and wrap them back in Successor
                for inner_rewrite in inner.value.get_recursive_rewrites(from, to, store) {
                    results.push(HashNode::from_store(
                        PeanoArithmeticExpression::Successor(inner_rewrite),
                        store,
                    ));
                }
            }
            PeanoArithmeticExpression::Number(_) | PeanoArithmeticExpression::DeBruijn(_) => {}
        }

        results
    }
}

impl Rewritable for PeanoDomainExpression {
    type AsPattern = PeanoDomainPattern;
    type Storage = PeanoStorage;

    fn decompose_to_pattern(&self, store: &Self::Storage) -> Self::AsPattern {
        match self {
            PeanoDomainExpression::Equality(l, r) => {
                PeanoDomainPattern::Equality(
                    l.value.decompose_to_pattern(&store.arithmetic_storage),
                    r.value.decompose_to_pattern(&store.arithmetic_storage),
                )
            }
        }
    }

    fn try_rewrite(&self, from: &Self::AsPattern, to: &Self::AsPattern, store: &Self::Storage) -> Option<HashNode<Self>> {
        match (from, to) {
            (PeanoDomainPattern::Equality(from_l, from_r), PeanoDomainPattern::Equality(to_l, to_r)) => {
                match self {
                    PeanoDomainExpression::Equality(l, r) => {
                        let l_subst = match_pattern(l, from_l, &store.arithmetic_storage)?;
                        let r_subst = match_pattern(r, from_r, &store.arithmetic_storage)?;

                        // Merge substitutions, checking for conflicts
                        let mut subst = l_subst;
                        for (key, value) in r_subst {
                            // If variable exists in both substitutions, values must match
                            if let Some(existing) = subst.get(&key) {
                                if existing != &value {
                                    return None;  // Conflict: variable has different values
                                }
                            } else {
                                subst.insert(key, value);
                            }
                        }

                        // Apply substitution to patterns
                        let new_l = apply_substitution(to_l, &subst, &store.arithmetic_storage)?;
                        let new_r = apply_substitution(to_r, &subst, &store.arithmetic_storage)?;

                        Some(HashNode::from_store(
                            PeanoDomainExpression::Equality(new_l, new_r),
                            &store.domain_content_storage,
                        ))
                    }
                }
            }
        }
    }

    fn get_recursive_rewrites(&self, from: &Self::AsPattern, to: &Self::AsPattern, store: &Self::Storage) -> Vec<HashNode<Self>> {
        let mut results = Vec::new();

        // Try rewriting at this level (domain-level pattern matching)
        if let Some(rewrite) = self.try_rewrite(from, to, store) {
            results.push(rewrite);
        }

        // Recursively rewrite arithmetic operands using arithmetic rules
        // When we have an equality like EQ (S(0 + 0)) (S(0)), we need to rewrite
        // the arithmetic expressions inside (0 + 0 -> 0) before checking equality.
        let PeanoDomainExpression::Equality(l, r) = self;
        // Apply all arithmetic rules to the left operand
        for rule in store.arithmetic_rules.iter() {
            for left_rewrite in rule.apply_recursive(l, &store.arithmetic_storage) {
                results.push(HashNode::from_store(
                    PeanoDomainExpression::Equality(left_rewrite, r.clone()),
                    &store.domain_content_storage,
                ));
            }
        }

        // Apply all arithmetic rules to the right operand
        for rule in store.arithmetic_rules.iter() {
            for right_rewrite in rule.apply_recursive(r, &store.arithmetic_storage) {
                results.push(HashNode::from_store(
                    PeanoDomainExpression::Equality(l.clone(), right_rewrite),
                    &store.domain_content_storage,
                ));
            }
        }

        results
    }
}

impl fmt::Display for PeanoArithmeticExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeanoArithmeticExpression::Add(left, right) => write!(f, "{} + {}", left, right),
            PeanoArithmeticExpression::Successor(inner) => write!(f, "S({})", inner),
            PeanoArithmeticExpression::Number(n) => write!(f, "{}", n),
            PeanoArithmeticExpression::DeBruijn(idx) => write!(f, "/{}", idx),
        }
    }
}

impl fmt::Display for PeanoDomainExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeanoDomainExpression::Equality(left, right) => write!(f, "{} = {}", left, right),
        }
    }
}

impl HashNodeInner for PeanoArithmeticExpression {
    fn hash(&self) -> u64 {
        match self {
            PeanoArithmeticExpression::Add(left, right) => {
                Hashing::root_hash(Hashing::opcode("add"), &[left.hash(), right.hash()])
            }
            PeanoArithmeticExpression::Successor(inner) => {
                Hashing::root_hash(Hashing::opcode("successor"), &[inner.hash()])
            }
            PeanoArithmeticExpression::Number(n) => Hashing::root_hash(Hashing::opcode("number"), &[*n]),
            PeanoArithmeticExpression::DeBruijn(idx) => {
                Hashing::root_hash(Hashing::opcode("debruijn"), &[*idx as u64])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoArithmeticExpression::Add(left, right) => 1 + left.size() + right.size(),
            PeanoArithmeticExpression::Successor(inner) => 1 + inner.size(),
            PeanoArithmeticExpression::Number(_) => 1,
            PeanoArithmeticExpression::DeBruijn(_) => 1,
        }
    }
}

impl HashNodeInner for PeanoArithmeticPattern {
    fn hash(&self) -> u64 {
        match self {
            PeanoArithmeticPattern::Variable(idx) => {
                Hashing::root_hash(Hashing::opcode("variable"), &[*idx as u64])
            }
            PeanoArithmeticPattern::Wildcard => Hashing::root_hash(Hashing::opcode("wildcard"), &[]),
            PeanoArithmeticPattern::Literal(n) => {
                Hashing::root_hash(Hashing::opcode("literal"), &[*n])
            }
            PeanoArithmeticPattern::Compound { opcode, args } => {
                let mut arg_hashes: Vec<u64> = args.iter().map(|arg| arg.hash()).collect();
                let mut all_hashes = vec![*opcode];
                all_hashes.append(&mut arg_hashes);
                Hashing::root_hash(1, &all_hashes)
            }
        }
    }
    
    fn size(&self) -> u64 {
        match self {
            PeanoArithmeticPattern::Variable(_) | PeanoArithmeticPattern::Wildcard | PeanoArithmeticPattern::Literal(_) => 1,
            PeanoArithmeticPattern::Compound { args, .. } => {
                1 + args.iter().map(|arg| arg.size()).sum::<u64>()
            }
        }
    }
}