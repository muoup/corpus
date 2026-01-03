use corpus_rewriting::{Rewriter, RewriteRule};

use crate::syntax::SumNode;

static PeanoRewriter: Rewriter<SumNode> = {
    let mut rewriter = vec![
        // Rule one:
        //   (\forall x, y, z...) f(x, y, z) == g(x, y, z)
        //      -> If g(x, y, z).size() < f(x, y, z).size() then
        //           rewrite f(x, y, z) to g(x, y, z)
    ];
    
    Rewriter::new(rewriter)
};