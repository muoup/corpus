//! Classical logic pattern implementations.

use crate::expression::{ClassicalLogicalExpression, DomainContent, LogicalStorage};
use crate::BinaryTruth;
use corpus_core::base::nodes::{HashNode, Hashing};
use corpus_core::rewriting::patterns::{AsRewriteRules, Rewritable};
use corpus_core::{HashNodeInner, RewriteDirection, RewriteRule};

#[derive(Debug, Clone, PartialEq)]
pub enum ClassicalLogicPattern<D: DomainContent>
where
    D::AsPattern: Clone + HashNodeInner,
{
    Compound { opcode: u64, operands: Vec<Self> },
    BooleanConstant(BinaryTruth),
    DomainPattern(D::AsPattern),
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

        ClassicalLogicalExpression::Equals(..)
        | ClassicalLogicalExpression::BooleanConstant(_)
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

impl<D: DomainContent> Rewritable for ClassicalLogicalExpression<D>
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
            ClassicalLogicalExpression::Equals(l, r) => ClassicalLogicPattern::Compound {
                opcode: Hashing::opcode("EQUALS"),
                operands: vec![
                    ClassicalLogicPattern::DomainPattern(
                        l.value.decompose_to_pattern(&store.domain_storage),
                    ),
                    ClassicalLogicPattern::DomainPattern(
                        r.value.decompose_to_pattern(&store.domain_storage),
                    ),
                ],
            },
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
        let pattern = self.decompose_to_pattern(store);

        if pattern.hash() == from.hash() {
            // Here we would normally apply the substitution, but for simplicity,
            // we will assume a direct match and return the 'to' pattern as a new expression.
            // In a full implementation, we would need to handle variable bindings.
            match to {
                ClassicalLogicPattern::Compound { opcode, operands } => {
                    recompose_logical_operator(
                        *opcode,
                        operands
                            .iter()
                            .map(|op| {
                                // This is a simplification; in a full implementation, we would
                                // need to convert patterns back to expressions properly.
                                HashNode::from_store(
                                    ClassicalLogicalExpression::BooleanConstant(BinaryTruth::True), // Placeholder
                                    &store.logical_storage,
                                )
                            })
                            .collect(),
                        store,
                    )
                }
                ClassicalLogicPattern::BooleanConstant(b) => Some(HashNode::from_store(
                    ClassicalLogicalExpression::BooleanConstant(*b),
                    &store.logical_storage,
                )),
                ClassicalLogicPattern::DomainPattern(..) => {
                    // Convert domain pattern back to domain expression
                    // Placeholder implementation
                    None
                }
            }
        } else {
            None
        }
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

            ClassicalLogicalExpression::Equals(..) => {
                // Rewrite rules do not work outside of the context of a boolean system
                // e.g. we must know that two expressions are *equal* to declare a rewrite
            }

            ClassicalLogicalExpression::BooleanConstant(_) => {}
            ClassicalLogicalExpression::_phantom() => unreachable!(),
        }

        rewrites
    }
}

impl<D: DomainContent> AsRewriteRules for ClassicalLogicalExpression<D>
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
            ClassicalLogicalExpression::Equals(l, r) => {
                let left_pattern = ClassicalLogicPattern::DomainPattern(
                    l.value.decompose_to_pattern(&store.domain_storage),
                );
                let right_pattern = ClassicalLogicPattern::DomainPattern(
                    r.value.decompose_to_pattern(&store.domain_storage),
                );

                // Create bidirectional rewrite rules for equality
                rewrites.push(RewriteRule::new(
                    name,
                    left_pattern,
                    right_pattern,
                    RewriteDirection::Both,
                ));
            }

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
