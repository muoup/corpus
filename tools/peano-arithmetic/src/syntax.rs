use core::fmt;

use corpus_classical_logic::{BinaryTruth, ClassicalOperator};
use corpus_core::expression::{DomainContent, DomainExpression};
use corpus_core::nodes::{HashNode, HashNodeInner, NodeStorage, Hashing};
use corpus_core::rewriting::RewriteRule;

pub type PeanoExpression = DomainExpression<BinaryTruth, PeanoContent>;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PeanoContent {
    Equals(
        HashNode<ArithmeticExpression>,
        HashNode<ArithmeticExpression>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ArithmeticExpression {
    Add(
        HashNode<ArithmeticExpression>,
        HashNode<ArithmeticExpression>,
    ),
    Successor(HashNode<ArithmeticExpression>),
    Number(u64),
    DeBruijn(u32),
}

impl fmt::Display for PeanoContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeanoContent::Equals(left, right) => write!(f, "{} = {}", left, right),
        }
    }
}

impl DomainContent<BinaryTruth> for PeanoContent {
    type Operator = ClassicalOperator;
}

impl fmt::Display for ArithmeticExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticExpression::Add(left, right) => write!(f, "({} + {})", left, right),
            ArithmeticExpression::Successor(inner) => write!(f, "S({})", inner),
            ArithmeticExpression::Number(n) => write!(f, "{}", n),
            ArithmeticExpression::DeBruijn(idx) => write!(f, "/{}", idx),
        }
    }
}

impl HashNodeInner for PeanoContent {
    fn hash(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => {
                let hashes = vec![left.hash(), right.hash()];
                Hashing::root_hash(Hashing::opcode("equals"), &hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => 1 + left.size() + right.size(),
        }
    }

    fn decompose(&self) -> Option<(u64, Vec<HashNode<Self>>)> {
        None
    }

    fn rewrite_any_subterm<F>(
        &self,
        node: &HashNode<Self>,
        _store: &NodeStorage<Self>,
        try_rewrite: &F,
    ) -> Option<HashNode<Self>>
    where
        F: Fn(&HashNode<Self>) -> Option<HashNode<Self>>,
    {
        // Try rewriting the full equality first
        if let Some(rewritten) = try_rewrite(node) {
            return Some(rewritten);
        }

        // Then try rewriting subterms (the arithmetic expressions on each side)
        match self {
            PeanoContent::Equals(_left, _right) => {
                // Note: We can't directly call try_rewrite on left/right because they
                // are HashNode<ArithmeticExpression> but try_rewrite expects HashNode<PeanoContent>.
                // This will be handled by creating wrapper rewrite rules that know how to
                // apply arithmetic rules to equality subterms.
                //
                // For now, this method just tries the top-level rewrite. The subterm rewriting
                // will be handled by the wrapper rules created by wrap_arithmetic_rule_for_equality.
                None
            }
        }
    }
}

impl HashNodeInner for ArithmeticExpression {
    fn hash(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => {
                Hashing::root_hash(Hashing::opcode("add"), &[left.hash(), right.hash()])
            }
            ArithmeticExpression::Successor(inner) => {
                Hashing::root_hash(Hashing::opcode("successor"), &[inner.hash()])
            }
            ArithmeticExpression::Number(n) => Hashing::root_hash(Hashing::opcode("number"), &[*n]),
            ArithmeticExpression::DeBruijn(idx) => {
                Hashing::root_hash(Hashing::opcode("debruijn"), &[*idx as u64])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpression::Successor(inner) => 1 + inner.size(),
            ArithmeticExpression::Number(_) => 1,
            ArithmeticExpression::DeBruijn(_) => 1,
        }
    }

    fn decompose(&self) -> Option<(u64, Vec<HashNode<Self>>)> {
        match self {
            ArithmeticExpression::Add(left, right) => {
                Some((Hashing::opcode("add"), vec![left.clone(), right.clone()]))
            }
            ArithmeticExpression::Successor(inner) => {
                Some((Hashing::opcode("successor"), vec![inner.clone()]))
            }
            ArithmeticExpression::Number(_) | ArithmeticExpression::DeBruijn(_) => None,
        }
    }

    fn rewrite_any_subterm<F>(
        &self,
        node: &HashNode<Self>,
        store: &NodeStorage<Self>,
        try_rewrite: &F,
    ) -> Option<HashNode<Self>>
    where
        F: Fn(&HashNode<Self>) -> Option<HashNode<Self>>,
    {
        // Try rewriting self first
        if let Some(rewritten) = try_rewrite(node) {
            return Some(rewritten);
        }

        // Then try subterms recursively
        match self {
            ArithmeticExpression::Add(left, right) => {
                // Try rewriting left subterm
                if let Some(new_left) = left.value.rewrite_any_subterm(left, store, try_rewrite) {
                    return Some(HashNode::from_store(
                        ArithmeticExpression::Add(new_left, right.clone()),
                        store,
                    ));
                }
                // Try rewriting right subterm
                if let Some(new_right) = right.value.rewrite_any_subterm(right, store, try_rewrite) {
                    return Some(HashNode::from_store(
                        ArithmeticExpression::Add(left.clone(), new_right),
                        store,
                    ));
                }
                None
            }
            ArithmeticExpression::Successor(inner) => {
                // Try rewriting inner subterm
                inner.value.rewrite_any_subterm(inner, store, try_rewrite)
                    .map(|new_inner| HashNode::from_store(
                        ArithmeticExpression::Successor(new_inner),
                        store,
                    ))
            }
            // Number and DeBruijn have no subterms
            ArithmeticExpression::Number(_) | ArithmeticExpression::DeBruijn(_) => None,
        }
    }

    fn construct_from_parts(
        opcode: u64,
        children: Vec<HashNode<Self>>,
        store: &NodeStorage<Self>,
    ) -> Option<HashNode<Self>> {
        match opcode {
            // Unfortunately due to the limitations of Rust's constexpr engine, Hashing::opcode("...") is not compile-time
            // and thus we cannot use a true match here. Instead we have to fudge it with if statements.
            
            o if o == Hashing::opcode("add") && children.len() == 2 => {
                Some(HashNode::from_store(
                    ArithmeticExpression::Add(children[0].clone(), children[1].clone()),
                    store,
                ))
            }
            o if o == Hashing::opcode("successor") && children.len() == 1 => {
                Some(HashNode::from_store(
                    ArithmeticExpression::Successor(children[0].clone()),
                    store,
                ))
            }
            o if o == Hashing::opcode("number") && children.len() == 1 => {
                let n = children[0].hash();
                Some(HashNode::from_store(ArithmeticExpression::Number(n), store))
            }
            o if o == Hashing::opcode("debruijn") && children.len() == 1 => {
                let idx = children[0].hash() as u32;
                Some(HashNode::from_store(ArithmeticExpression::DeBruijn(idx), store))
            }
            _ => None,
        }
    }
}

/// Get all possible rewrites of a PeanoContent (equality) by applying
/// arithmetic rewrite rules to its subterms.
///
/// This function takes a list of arithmetic rewrite rules and applies them
/// to both the left and right sides of the equality, generating new equalities.
pub fn get_all_rewrites_for_equality(
    equality: &HashNode<PeanoContent>,
    _store: &NodeStorage<PeanoContent>,
    arithmetic_rules: &[RewriteRule<ArithmeticExpression>],
) -> Vec<HashNode<PeanoContent>> {
    let mut rewrites = Vec::new();

    let PeanoContent::Equals(left, right) = equality.value.as_ref();
    
    // Create an arithmetic expression store for applying rules
    let arith_store = NodeStorage::<ArithmeticExpression>::new();

    // Try applying each arithmetic rule to the left subterm
    for rule in arithmetic_rules {
        // Forward direction: apply pattern to get replacement
        if let Some(new_left) = rule.apply(left, &arith_store) {
            let new_content = PeanoContent::Equals(new_left, right.clone());
            rewrites.push(HashNode::from_store(new_content, _store));
        }

        // Reverse direction: apply replacement to get pattern
        if let Some(new_left) = rule.apply_reverse(left, &arith_store) {
            let new_content = PeanoContent::Equals(new_left, right.clone());
            rewrites.push(HashNode::from_store(new_content, _store));
        }

        // Try the right subterm too
        if let Some(new_right) = rule.apply(right, &arith_store) {
            let new_content = PeanoContent::Equals(left.clone(), new_right);
            rewrites.push(HashNode::from_store(new_content, _store));
        }

        if let Some(new_right) = rule.apply_reverse(right, &arith_store) {
            let new_content = PeanoContent::Equals(left.clone(), new_right);
            rewrites.push(HashNode::from_store(new_content, _store));
        }
    }
    
    rewrites
}

/// Wrapper for compatibility - wraps arithmetic rules for use with equalities.
///
/// This creates dummy RewriteRule<PeanoContent> entries that can be added to the prover.
/// The actual rewriting logic is in get_all_rewrites_for_equality.
pub fn wrap_arithmetic_rules_for_equality(
    rules: Vec<RewriteRule<ArithmeticExpression>>,
) -> Vec<RewriteRule<PeanoContent>> {
    // For now, create dummy wildcard rules - the actual rewriting
    // will be handled by a custom implementation in the prover
    rules.into_iter().map(|rule| {
        RewriteRule::bidirectional(
            rule.name.clone(),
            corpus_core::rewriting::Pattern::Wildcard,
            corpus_core::rewriting::Pattern::Wildcard,
        )
    }).collect()
}

/// Apply successor injectivity rewrite: S(x) = S(y) -> x = y
///
/// If both sides of the equality are successor expressions, rewrite to
/// the equality of their inner terms.
pub fn apply_successor_injectivity(
    equality: &HashNode<PeanoContent>,
    store: &NodeStorage<PeanoContent>,
) -> Option<HashNode<PeanoContent>> {
    let PeanoContent::Equals(left, right) = equality.value.as_ref();
    
    // Check if both sides are Successor expressions
    let ArithmeticExpression::Successor(left_inner) = left.value.as_ref() else {
        return None;
    };
    
    let ArithmeticExpression::Successor(right_inner) = right.value.as_ref() else {
        return None;
    };

    // Create new equality: left_inner = right_inner
    let new_content = PeanoContent::Equals(left_inner.clone(), right_inner.clone());
    Some(HashNode::from_store(new_content, store))
}
