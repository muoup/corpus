//! Classical logic pattern implementations.

use crate::expression::{ClassicalLogicalExpression, DomainContent, LogicalStorage};
use crate::BinaryTruth;
use corpus_core::base::nodes::{HashNode, Hashing};
use corpus_core::rewriting::patterns::{AsRewriteRules, Rewritable};
use corpus_core::{HashNodeInner, RewriteDirection, RewriteRule};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ClassicalLogicPattern<D: DomainContent>
where
    D::AsPattern: Clone + HashNodeInner,
{
    Compound { opcode: u64, operands: Vec<Self> },
    BooleanConstant(BinaryTruth),
    DomainPattern(D::AsPattern),
}

/// Substitution mapping De Bruijn indices to logical expressions.
type Substitution<D> = HashMap<u32, HashNode<ClassicalLogicalExpression<D>>>;

/// Match a pattern against an expression, producing a substitution if successful.
fn match_pattern<D: DomainContent + std::fmt::Debug>(
    expr: &HashNode<ClassicalLogicalExpression<D>>,
    pattern: &ClassicalLogicPattern<D>,
    store: &LogicalStorage<D>,
) -> Option<Substitution<D>>
where
    D::AsPattern: HashNodeInner,
{
    match pattern {
        ClassicalLogicPattern::BooleanConstant(b) => {
            match expr.value.as_ref() {
                ClassicalLogicalExpression::BooleanConstant(bt) if bt == b => {
                    Some(Substitution::new())
                }
                _ => None,
            }
        }
        ClassicalLogicPattern::Compound { opcode, operands } => {
            match expr.value.as_ref() {
                ClassicalLogicalExpression::And(l, r)
                    if *opcode == Hashing::opcode("AND") && operands.len() == 2 =>
                {
                    let mut subst = match_pattern(l, &operands[0], store)?;
                    subst.extend(match_pattern(r, &operands[1], store)?);
                    Some(subst)
                }
                ClassicalLogicalExpression::Or(l, r)
                    if *opcode == Hashing::opcode("OR") && operands.len() == 2 =>
                {
                    let mut subst = match_pattern(l, &operands[0], store)?;
                    subst.extend(match_pattern(r, &operands[1], store)?);
                    Some(subst)
                }
                ClassicalLogicalExpression::Not(x)
                    if *opcode == Hashing::opcode("NOT") && operands.len() == 1 =>
                {
                    match_pattern(x, &operands[0], store)
                }
                ClassicalLogicalExpression::Imply(l, r)
                    if *opcode == Hashing::opcode("IMPLY") && operands.len() == 2 =>
                {
                    let mut subst = match_pattern(l, &operands[0], store)?;
                    subst.extend(match_pattern(r, &operands[1], store)?);
                    Some(subst)
                }
                ClassicalLogicalExpression::Iff(l, r)
                    if *opcode == Hashing::opcode("IFF") && operands.len() == 2 =>
                {
                    let mut subst = match_pattern(l, &operands[0], store)?;
                    subst.extend(match_pattern(r, &operands[1], store)?);
                    Some(subst)
                }
                ClassicalLogicalExpression::ForAll(x)
                    if *opcode == Hashing::opcode("FORALL") && operands.len() == 1 =>
                {
                    match_pattern(x, &operands[0], store)
                }
                ClassicalLogicalExpression::Exists(x)
                    if *opcode == Hashing::opcode("EXISTS") && operands.len() == 1 =>
                {
                    match_pattern(x, &operands[0], store)
                }
                _ => None,
            }
        }
        ClassicalLogicPattern::DomainPattern(_domain_pattern) => {
            // Domain patterns are handled directly in try_rewrite, not through the generic match/apply system
            // Return None here since domain pattern matching uses a different code path
            None
        }
    }
}

/// Apply a substitution to a pattern to produce an expression.
fn apply_substitution<D: DomainContent + std::fmt::Debug>(
    pattern: &ClassicalLogicPattern<D>,
    substitution: &Substitution<D>,
    store: &LogicalStorage<D>,
) -> Option<HashNode<ClassicalLogicalExpression<D>>>
where
    D::AsPattern: HashNodeInner,
{
    match pattern {
        ClassicalLogicPattern::BooleanConstant(b) => Some(HashNode::from_store(
            ClassicalLogicalExpression::BooleanConstant(*b),
            &store.logical_storage,
        )),
        ClassicalLogicPattern::Compound { opcode, operands } => {
            let resolved_operands: Vec<_> = operands
                .iter()
                .map(|p| apply_substitution(p, substitution, store))
                .collect::<Option<Vec<_>>>()?;

            if *opcode == Hashing::opcode("AND") && resolved_operands.len() == 2 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::And(
                        resolved_operands[0].clone(),
                        resolved_operands[1].clone(),
                    ),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("OR") && resolved_operands.len() == 2 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::Or(
                        resolved_operands[0].clone(),
                        resolved_operands[1].clone(),
                    ),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("NOT") && resolved_operands.len() == 1 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::Not(resolved_operands[0].clone()),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("IMPLY") && resolved_operands.len() == 2 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::Imply(
                        resolved_operands[0].clone(),
                        resolved_operands[1].clone(),
                    ),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("IFF") && resolved_operands.len() == 2 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::Iff(
                        resolved_operands[0].clone(),
                        resolved_operands[1].clone(),
                    ),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("FORALL") && resolved_operands.len() == 1 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::ForAll(resolved_operands[0].clone()),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("EXISTS") && resolved_operands.len() == 1 {
                Some(HashNode::from_store(
                    ClassicalLogicalExpression::Exists(resolved_operands[0].clone()),
                    &store.logical_storage,
                ))
            } else if *opcode == Hashing::opcode("EQUALS") && resolved_operands.len() == 2 {
                // For Equals, domain bridging is complex
                // Placeholder: return None for now
                None
            } else {
                None
            }
        }
        ClassicalLogicPattern::DomainPattern(_) => {
            // Domain patterns are handled directly in try_rewrite, not through the generic match/apply system
            None
        }
    }
}

fn decompose_logical_operator<D: DomainContent>(
    expr: &ClassicalLogicalExpression<D>,
) -> (u64, Vec<HashNode<ClassicalLogicalExpression<D>>>) {
    match expr {
        ClassicalLogicalExpression::And(lhs, rhs) => {
            (Hashing::opcode("AND"), vec![lhs.clone(), rhs.clone()])
        }
        ClassicalLogicalExpression::Or(lhs, rhs) => {
            (Hashing::opcode("OR"), vec![lhs.clone(), rhs.clone()])
        }
        ClassicalLogicalExpression::Not(inner) => (Hashing::opcode("NOT"), vec![inner.clone()]),
        ClassicalLogicalExpression::Imply(lhs, rhs) => {
            (Hashing::opcode("IMPLY"), vec![lhs.clone(), rhs.clone()])
        }
        ClassicalLogicalExpression::Iff(lhs, rhs) => {
            (Hashing::opcode("IFF"), vec![lhs.clone(), rhs.clone()])
        }
        ClassicalLogicalExpression::ForAll(inner) => {
            (Hashing::opcode("FORALL"), vec![inner.clone()])
        }
        ClassicalLogicalExpression::Exists(inner) => {
            (Hashing::opcode("EXISTS"), vec![inner.clone()])
        }

        ClassicalLogicalExpression::BooleanConstant(_)
        | ClassicalLogicalExpression::DomainContent(_)
        | ClassicalLogicalExpression::_phantom(..) => unreachable!(),
    }
}

fn recompose_logical_operator<D: DomainContent>(
    opcode: u64,
    operands: Vec<HashNode<ClassicalLogicalExpression<D>>>,
    store: &LogicalStorage<D>,
) -> Option<HashNode<ClassicalLogicalExpression<D>>> {
    match opcode {
        x if x == Hashing::opcode("AND") && operands.len() == 2 => Some(HashNode::from_store(
            ClassicalLogicalExpression::And(operands[0].clone(), operands[1].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("OR") && operands.len() == 2 => Some(HashNode::from_store(
            ClassicalLogicalExpression::Or(operands[0].clone(), operands[1].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("NOT") && operands.len() == 1 => Some(HashNode::from_store(
            ClassicalLogicalExpression::Not(operands[0].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("IMPLY") && operands.len() == 2 => Some(HashNode::from_store(
            ClassicalLogicalExpression::Imply(operands[0].clone(), operands[1].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("IFF") && operands.len() == 2 => Some(HashNode::from_store(
            ClassicalLogicalExpression::Iff(operands[0].clone(), operands[1].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("FORALL") && operands.len() == 1 => Some(HashNode::from_store(
            ClassicalLogicalExpression::ForAll(operands[0].clone()),
            &store.logical_storage,
        )),
        x if x == Hashing::opcode("EXISTS") && operands.len() == 1 => Some(HashNode::from_store(
            ClassicalLogicalExpression::Exists(operands[0].clone()),
            &store.logical_storage,
        )),
        _ => None,
    }
}

impl<D: DomainContent + std::fmt::Debug> Rewritable for ClassicalLogicalExpression<D>
where
    D::AsPattern: Clone + HashNodeInner,
{
    type AsPattern = ClassicalLogicPattern<D>;
    type Storage = LogicalStorage<D>;

    fn decompose_to_pattern(&self, store: &Self::Storage) -> Self::AsPattern {
        match self {
            ClassicalLogicalExpression::And(l, r) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("AND"),
                operands: vec![
                    l.value.decompose_to_pattern(store),
                    r.value.decompose_to_pattern(store),
                ],
            },
            ClassicalLogicalExpression::Or(l, r) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("OR"),
                operands: vec![
                    l.value.decompose_to_pattern(store),
                    r.value.decompose_to_pattern(store),
                ],
            },
            ClassicalLogicalExpression::Not(inner) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("NOT"),
                operands: vec![inner.value.decompose_to_pattern(store)],
            },
            ClassicalLogicalExpression::Imply(l, r) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("IMPLY"),
                operands: vec![
                    l.value.decompose_to_pattern(store),
                    r.value.decompose_to_pattern(store),
                ],
            },
            ClassicalLogicalExpression::Iff(l, r) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("IFF"),
                operands: vec![
                    l.value.decompose_to_pattern(store),
                    r.value.decompose_to_pattern(store),
                ],
            },
            ClassicalLogicalExpression::ForAll(inner) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("FORALL"),
                operands: vec![inner.value.decompose_to_pattern(store)],
            },
            ClassicalLogicalExpression::Exists(inner) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("EXISTS"),
                operands: vec![inner.value.decompose_to_pattern(store)],
            },
            ClassicalLogicalExpression::DomainContent(domain_content) => {
                ClassicalLogicPattern::DomainPattern(
                    domain_content.value.decompose_to_pattern(&store.domain_storage),
                )
            }
            ClassicalLogicalExpression::BooleanConstant(b) => {
                ClassicalLogicPattern::BooleanConstant(*b)
            }
            ClassicalLogicalExpression::_phantom() => unreachable!(),
        }
    }

    fn try_rewrite(
        &self,
        from: &Self::AsPattern,
        to: &Self::AsPattern,
        store: &Self::Storage,
    ) -> Option<HashNode<Self>> {
        // Special case: if both from and to are DomainPatterns, use domain-level rewriting directly
        if let (ClassicalLogicPattern::DomainPattern(from_pattern), ClassicalLogicPattern::DomainPattern(to_pattern)) = (from, to) {
            // We need to match the domain pattern against the domain content
            let ClassicalLogicalExpression::DomainContent(domain_content) = self else {
                return None;
            };

            // Use the domain's try_rewrite to match and rewrite at the domain level
            let rewritten_domain = domain_content.value.try_rewrite(from_pattern, to_pattern, &store.domain_storage)?;

            // Wrap the result back in DomainContent
            return Some(HashNode::from_store(
                ClassicalLogicalExpression::DomainContent(rewritten_domain),
                &store.logical_storage,
            ));
        }

        // Special case: if from is DomainPattern and to is BooleanConstant
        // Check if domain matches pattern and return the boolean constant
        if let ClassicalLogicPattern::DomainPattern(from_pattern) = from {
            if let ClassicalLogicPattern::BooleanConstant(b) = to {
                let ClassicalLogicalExpression::DomainContent(domain_content) = self else {
                    return None;
                };

                // Use try_rewrite with same pattern to check structural match
                // If successful, the domain content matches the pattern structure
                if domain_content.value.try_rewrite(from_pattern, from_pattern, &store.domain_storage).is_some() {
                    return Some(HashNode::from_store(
                        ClassicalLogicalExpression::BooleanConstant(*b),
                        &store.logical_storage,
                    ));
                }
            }
        }

        // Standard case: use generic pattern matching and substitution
        // Match 'from' pattern against self to get substitution
        let substitution = match_pattern(
            &HashNode::from_store(self.clone(), &store.logical_storage),
            from,
            store,
        )?;
        // Apply substitution to 'to' pattern to get result
        apply_substitution(to, &substitution, store)
    }

    fn get_recursive_rewrites(
        &self,
        from: &Self::AsPattern,
        to: &Self::AsPattern,
        store: &Self::Storage,
    ) -> Vec<HashNode<Self>> {
        let mut rewrites = vec![];

        if let Some(self_rewrite) = self.try_rewrite(from, to, store) {
            rewrites.push(self_rewrite);
        }

        match self {
            ClassicalLogicalExpression::And(..)
            | ClassicalLogicalExpression::Or(..)
            | ClassicalLogicalExpression::Not(..)
            | ClassicalLogicalExpression::Imply(..)
            | ClassicalLogicalExpression::Iff(..)
            | ClassicalLogicalExpression::ForAll(..)
            | ClassicalLogicalExpression::Exists(..) => {
                let (opcode, operands) = decompose_logical_operator(self);

                for (i, operand) in operands.iter().enumerate() {
                    for operand_rewrite in operand.value.get_recursive_rewrites(from, to, store) {
                        let mut new_operands = operands.clone();
                        new_operands[i] = operand_rewrite;
                        if let Some(recomposed) =
                            recompose_logical_operator(opcode, new_operands, store)
                        {
                            rewrites.push(recomposed);
                        }
                    }
                }
            }

            ClassicalLogicalExpression::DomainContent(domain_content) => {
                // Recursively try rewriting at the domain level
                // This allows arithmetic rules to be applied to operands of equality expressions

                // Extract domain patterns from the logical patterns (if they are DomainPattern variants)
                let (domain_from, domain_to) = match (from, to) {
                    (ClassicalLogicPattern::DomainPattern(df), ClassicalLogicPattern::DomainPattern(dt)) => (Some(df), Some(dt)),
                    _ => (None, None),
                };

                // If we have domain patterns, use them for domain-level rewriting
                if let (Some(df), Some(dt)) = (domain_from, domain_to) {
                    for domain_rewrite in domain_content.value.get_recursive_rewrites(df, dt, &store.domain_storage) {
                        rewrites.push(HashNode::from_store(
                            ClassicalLogicalExpression::DomainContent(domain_rewrite),
                            &store.logical_storage,
                        ));
                    }
                }
            }

            ClassicalLogicalExpression::BooleanConstant(_) => {}
            ClassicalLogicalExpression::_phantom() => unreachable!(),
        }

        rewrites
    }
}

impl<D: DomainContent + std::fmt::Debug> AsRewriteRules for ClassicalLogicalExpression<D>
where
    D::AsPattern: Clone + HashNodeInner,
{
    fn decompose_to_rewrite_rules(
        &self,
        name: &str,
        store: &Self::Storage,
    ) -> Vec<RewriteRule<Self>> {
        let mut rewrites = vec![];

        match self {
            ClassicalLogicalExpression::Imply(l, r) => {
                let left_pattern = l.value.decompose_to_pattern(store);
                let right_pattern = r.value.decompose_to_pattern(store);

                // Create a unidirectional rewrite rule for implication
                rewrites.push(RewriteRule::new(
                    name,
                    left_pattern,
                    right_pattern,
                    RewriteDirection::Forward,
                ));
            }

            ClassicalLogicalExpression::Iff(l, r) => {
                let left_pattern = l.value.decompose_to_pattern(store);
                let right_pattern = r.value.decompose_to_pattern(store);

                // Create bidirectional rewrite rules for biconditional
                rewrites.push(RewriteRule::new(
                    name,
                    left_pattern,
                    right_pattern,
                    RewriteDirection::Both,
                ));
            }

            ClassicalLogicalExpression::Exists(..) => {
                // Not quite sure how to handle existential quantifiers in rewrite rules yet
            }

            ClassicalLogicalExpression::And(..)
            | ClassicalLogicalExpression::Or(..)
            | ClassicalLogicalExpression::Not(..)
            | ClassicalLogicalExpression::ForAll(..)
            | ClassicalLogicalExpression::BooleanConstant(_) => {
                let (_, operands) = decompose_logical_operator(self);

                for operand in operands {
                    let operand_rewrites = operand.value.decompose_to_rewrite_rules(name, store);
                    rewrites.extend(operand_rewrites);
                }
            }

            ClassicalLogicalExpression::DomainContent(domain_content) => {
                // Generate a rewrite rule that maps the domain pattern to true
                // This allows axioms like reflexivity (EQ x x) to match and rewrite to true
                let domain_pattern = domain_content.value.decompose_to_pattern(&store.domain_storage);
                let true_pattern = ClassicalLogicPattern::BooleanConstant(BinaryTruth::True);

                rewrites.push(RewriteRule::new(
                    name,
                    ClassicalLogicPattern::DomainPattern(domain_pattern),
                    true_pattern,
                    RewriteDirection::Forward,
                ));
            } 
            
            ClassicalLogicalExpression::_phantom(..) => unreachable!(),
        }

        rewrites
    }
}

impl<D: DomainContent> HashNodeInner for ClassicalLogicPattern<D>
where
    D::AsPattern: Clone + HashNodeInner,
{
    fn hash(&self) -> u64 {
        match self {
            ClassicalLogicPattern::Compound { opcode, operands } => {
                let mut all_hashes = vec![*opcode];
                for operand in operands {
                    all_hashes.push(operand.hash());
                }
                Hashing::root_hash(1, &all_hashes)
            }
            ClassicalLogicPattern::BooleanConstant(b) => {
                let all_hashes = vec![Hashing::opcode("BOOLEAN_CONSTANT"), b.hash()];
                Hashing::root_hash(1, &all_hashes)
            }
            ClassicalLogicPattern::DomainPattern(dp) => {
                let all_hashes = vec![Hashing::opcode("DOMAIN_PATTERN"), dp.hash()];
                Hashing::root_hash(1, &all_hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            ClassicalLogicPattern::Compound { operands, .. } => {
                1 + operands.iter().map(|op| op.size()).sum::<u64>()
            }
            ClassicalLogicPattern::BooleanConstant(_) => 1,
            ClassicalLogicPattern::DomainPattern(_) => 1,
        }
    }
}
