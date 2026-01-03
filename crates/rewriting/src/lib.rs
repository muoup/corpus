use corpus_core::nodes::{HashNode, HashNodeInner};

pub struct Rewriter<Node: HashNodeInner> {
    rules: Vec<RewriteRule<Node>>,
}

pub struct RewriteResult<'a, Node: HashNodeInner> {
    input: &'a [Node],
    allowed_rewrites: Vec<RewriteStep<Node>>,
}

pub struct RewriteRule<Node: HashNodeInner> {
    pattern: Box<dyn Fn(&[Node]) -> Option<RewriteStep<Node>>>,
}

pub struct RewriteStep<Node: HashNodeInner> {
    process: Box<dyn Fn(&[Node]) -> HashNode<Node>>,
}

impl<Node: HashNodeInner> Rewriter<Node> {
    pub const fn new(rules: Vec<RewriteRule<Node>>) -> Self {
        Self { rules }
    }

    pub fn try_apply_rules<'a>(&self, node: &'a [Node]) -> RewriteResult<'a, Node> {
        RewriteResult {
            input: node,
            allowed_rewrites: self
                .rules
                .iter()
                .filter_map(|rule| (rule.pattern)(node))
                .collect(),
        }
    }
}

impl<Node: HashNodeInner> RewriteResult<'_, Node> {
    pub const fn get_allowed_rewrites(&self) -> &Vec<RewriteStep<Node>> {
        &self.allowed_rewrites
    }

    pub fn apply_rewrite(&self, index: usize) -> Option<HashNode<Node>> {
        self.allowed_rewrites
            .get(index)
            .map(|step| step.apply(self.input))
    }
}

impl<Node: HashNodeInner> RewriteRule<Node> {
    pub fn new<F>(pattern: F) -> Self
    where
        F: 'static + Fn(&[Node]) -> Option<RewriteStep<Node>>,
    {
        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl<Node: HashNodeInner> RewriteStep<Node> {
    pub fn new<F>(process: F) -> Self
    where
        F: 'static + Fn(&[Node]) -> HashNode<Node>,
    {
        Self {
            process: Box::new(process),
        }
    }

    pub fn apply(&self, input: &[Node]) -> HashNode<Node> {
        (self.process)(input)
    }
}

unsafe impl<Node> Send for Rewriter<Node> where Node: HashNodeInner {}
unsafe impl<Node> Sync for Rewriter<Node> where Node: HashNodeInner {}
