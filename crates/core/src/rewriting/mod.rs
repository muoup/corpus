pub mod patterns;

use crate::{
    base::nodes::{HashNode, HashNodeInner},
    rewriting::patterns::Rewritable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteDirection {
    Both,
    Forward,
    Backward,
}

pub trait Pattern<Node: HashNodeInner> {
    fn matches(&self, expr: &HashNode<Node>) -> bool;
    fn transform(this: &Node, from: &Self, to: &Self) -> Option<Node>;
}

/// A rewrite rule for term transformation.
///
/// # Type Parameters
///
/// * `Node` - The expression type (must implement `HashNodeInner` and `Unifiable`)
pub struct RewriteRule<Node: Rewritable> {
    pub name: String,
    pub pattern: Node::AsPattern,
    pub replacement: Node::AsPattern,
    pub direction: RewriteDirection,

    _phantom: std::marker::PhantomData<Node>,
}

impl<Node: Rewritable> RewriteRule<Node> {
    /// Create a new rewrite rule.
    pub fn new(
        name: &str,
        pattern: Node::AsPattern,
        replacement: Node::AsPattern,
        direction: RewriteDirection,
    ) -> Self {
        Self {
            name: name.to_string(),
            pattern,
            replacement,
            direction,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn apply(
        &self,
        term: &HashNode<Node>,
        store: &Node::Storage,
    ) -> Option<HashNode<Node>> {
        term.value.as_ref()
            .try_rewrite(&self.pattern, &self.replacement, store)
    }
}
